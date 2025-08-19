use std::fs;
use std::path;
use zed_extension_api::{self as zed, LanguageServerId, Result};

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
        Ok(zed::Command {
            command: self.dotnet_binary_path(worktree)?.clone(),
            args: vec![
                "--roll-forward".to_string(),
                "Major".to_string(),
                self.language_server_path(language_server_id)?,
            ],
            env: Default::default(),
        })
    }
}

impl BicepExtension {
    fn dotnet_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let dotnet_path = match &self.dotnet_binary_path {
            Some(path) if fs::metadata(path).map_or(false, |stat| stat.is_file()) => path.clone(),
            Some(path) => worktree
                .which(path.clone().as_str())
                .ok_or_else(|| "DotNet 8.0+ must be installed for Bicep")?,
            None => worktree
                .which("dotnet")
                .ok_or_else(|| "DotNet 8.0+ must be installed for Bicep")?,
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

            make_all_files_executable(&version_dir)?;

            // Ensure the binary exists
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
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

fn make_all_files_executable(dir: &str) -> Result<(), String> {
    fn recurse(path: &str) -> Result<(), String> {
        let metadata = fs::metadata(path).map_err(|e| format!("metadata error: {e}"))?;
        if metadata.is_dir() {
            for entry in fs::read_dir(path).map_err(|e| format!("read_dir error: {e}"))? {
                let entry = entry.map_err(|e| format!("entry error: {e}"))?;
                recurse(&entry.path().to_string_lossy())?;
            }
        } else if metadata.is_file() {
            zed::make_file_executable(path)
                .map_err(|e| format!("make_file_executable error: {e}"))?;
        }
        Ok(())
    }
    recurse(dir.as_ref())
}

zed::register_extension!(BicepExtension);
