//! Zed extension for Bicep language support.
//!
//! This extension provides IntelliSense, error checking, and syntax support for Azure Bicep
//! files (`.bicep` and `.bicepparam`) by downloading and launching the official
//! [Bicep Language Server](https://github.com/Azure/bicep) via the .NET runtime.
//!
//! ## LSP Lifecycle
//!
//! 1. **Resolve `dotnet`** — Finds the `dotnet` binary on the system PATH (cached after first lookup).
//! 2. **Download language server** — Fetches the latest `bicep-langserver.zip` from GitHub releases
//!    and extracts it to a versioned directory. Old versions are cleaned up automatically.
//! 3. **Launch** — Runs `dotnet --roll-forward Major <path>/Bicep.LangServer.dll`.

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
            Some(path) if fs::metadata(path).is_ok_and(|stat| stat.is_file()) => path.clone(),
            _ => worktree
                .which("dotnet")
                .ok_or("dotnet not found. Install the .NET SDK 8.0+ from https://dotnet.microsoft.com/download. Note: the standalone Bicep CLI is not sufficient.")?,
        };
        self.dotnet_binary_path = Some(dotnet_path.clone());
        Ok(dotnet_path)
    }

    fn language_server_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let install_root = "bicep-language-servers";

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
            .ok_or_else(|| "no bicep-langserver.zip found".to_string())?;

        let version_name = format!("bicep-langserver-{}", release.version);
        let version_dir = format!("{install_root}/{version_name}");
        let lsp_path = format!("{}/Bicep.LangServer.dll", version_dir);

        if !fs::metadata(&lsp_path).is_ok_and(|stat| stat.is_file()) {
            // Download the asset
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            fs::create_dir_all(install_root).map_err(|e| {
                format!("failed to create installation directory {install_root}: {e}")
            })?;
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|err| format!("download error {}", err))?;

            // Clean up old versions
            let entries = fs::read_dir(install_root).map_err(|e| {
                format!("failed to list installation directory {install_root}: {e}")
            })?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                let file_type = entry
                    .file_type()
                    .map_err(|e| format!("failed to inspect directory entry type {e}"))?;
                if !file_type.is_dir() {
                    continue;
                }

                let entry_name = entry.file_name();
                let entry_name = entry_name.to_string_lossy();
                if entry_name.starts_with("bicep-langserver-") && entry_name != version_name {
                    let path = entry.path();
                    fs::remove_dir_all(&path).map_err(|e| {
                        format!(
                            "failed to remove old language server directory {}: {e}",
                            path.display()
                        )
                    })?;
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
