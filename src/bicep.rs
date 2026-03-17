use std::fs;
use std::path;
use zed_extension_api::{self as zed, LanguageServerId, Result};

const BRIDGE_SCRIPT_NAME: &str = "bicep-lsp-bridge.py";
const BRIDGE_SCRIPT_CONTENT: &str = include_str!("bicep-lsp-bridge.py");

struct BicepExtension {
    dotnet_binary_path: Option<String>,
}

impl zed::Extension for BicepExtension {
    fn new() -> Self {
        Self {
            dotnet_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let dotnet_path = self.dotnet_binary_path(worktree)?;
        let lsp_path = self.language_server_path(language_server_id)?;

        // When Python 3 is available, use a TCP socket bridge so that the
        // Bicep language server communicates over a TCP socket rather than
        // stdin/stdout. The bridge script starts a local TCP server, passes
        // --socket <port> to the language server, and then forwards between
        // Zed's stdin/stdout and that TCP connection.
        if let Some(python3_path) = worktree.which("python3") {
            let bridge_path = self.ensure_bridge_script()?;
            return Ok(zed::Command {
                command: python3_path,
                args: vec![
                    bridge_path,
                    dotnet_path,
                    "--roll-forward".to_string(),
                    "Major".to_string(),
                    lsp_path,
                ],
                env: Default::default(),
            });
        }

        // Fall back to stdin/stdout when Python 3 is not available.
        Ok(zed::Command {
            command: dotnet_path,
            args: vec![
                "--roll-forward".to_string(),
                "Major".to_string(),
                lsp_path,
            ],
            env: Default::default(),
        })
    }
}

impl BicepExtension {
    fn ensure_bridge_script(&self) -> Result<String> {
        fs::write(BRIDGE_SCRIPT_NAME, BRIDGE_SCRIPT_CONTENT)
            .map_err(|e| format!("Failed to write bridge script: {e}"))?;

        let abs_path = path::absolute(BRIDGE_SCRIPT_NAME)
            .map_err(|e| format!("Failed to get absolute path for bridge script: {e}"))?
            .to_str()
            .ok_or_else(|| "Bridge script path contains invalid UTF-8".to_string())?
            .to_string();

        Ok(abs_path)
    }

    fn dotnet_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let dotnet_path = match &self.dotnet_binary_path {
            Some(path) if fs::metadata(path).map_or(false, |stat| stat.is_file()) => path.clone(),
            _ => worktree
                .which("dotnet")
                .ok_or_else(|| "dotnet not found. Install the .NET SDK 8.0+ from https://dotnet.microsoft.com/download. Note: the standalone Bicep CLI is not sufficient.")?,
        };
        self.dotnet_binary_path = Some(dotnet_path.clone());
        Ok(dotnet_path)
    }

    fn language_server_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Get the latest release
        let release = zed::latest_github_release(
            "Azure/bicep",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        // Find the language server release asset
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == "bicep-langserver.zip")
            .ok_or_else(|| format!("no bicep-langserver.zip found"))?;

        let version_dir = format!("bicep-langserver-{}", release.version);
        let lsp_path = format!("{}/Bicep.LangServer.dll", version_dir);

        if !fs::metadata(&lsp_path).map_or(false, |stat| stat.is_file()) {
            // Download the asset
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|err| format!("download error {}", err))?;

            // Clean up old versions
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    let path = entry.path();
                    if path.is_dir() {
                        fs::remove_dir_all(&path).ok();
                    } else {
                        fs::remove_file(&path).ok();
                    }
                }
            }
        }

        let abs_path = path::absolute(&lsp_path)
            .map_err(|e| format!("failed to get absolute path {e}"))?
            .to_str()
            .unwrap()
            .to_string();
        Ok(abs_path)
    }
}

zed::register_extension!(BicepExtension);
