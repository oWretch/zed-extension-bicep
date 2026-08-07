//! Zed extension for Bicep language support.
//!
//! This extension provides IntelliSense, error checking, and syntax support for Azure Bicep
//! files (`.bicep` and `.bicepparam`) by installing and launching the official
//! [Bicep Language Server](https://github.com/Azure/bicep) as a .NET tool.
//!
//! ## LSP Lifecycle
//!
//! 1. **Install/update** — Uses `dotnet tool install/update Azure.Bicep.LangServer --tool-path
//!    <work_dir>/bicep-langserver` to place the `bicep-ls` binary at a known absolute path.
//! 2. **Launch** — Runs `<work_dir>/bicep-langserver/bicep-ls` directly.
//! 3. **Cleanup** — Removes the legacy `bicep-language-servers/` directory if present.

use std::fs;
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

const TOOL_DIR: &str = "bicep-langserver";
const PACKAGE_ID: &str = "azure.bicep.langserver";

struct BicepExtension {
    bicep_ls_path: Option<String>,
}

impl zed::Extension for BicepExtension {
    fn new() -> Self {
        Self {
            bicep_ls_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let bicep_ls = self.bicep_ls_path(language_server_id, worktree)?;
        Ok(zed::Command {
            command: bicep_ls,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "bicep": {}
        })))
    }
}

impl BicepExtension {
    fn bicep_ls_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Return cached path if the binary still exists.
        if let Some(path) = &self.bicep_ls_path {
            if fs::metadata(path).is_ok_and(|s| s.is_file()) {
                return Ok(path.clone());
            }
        }

        let dotnet = worktree.which("dotnet").ok_or(
            "dotnet not found. Install the .NET SDK 8.0+ from https://dotnet.microsoft.com/download.",
        )?;

        // Zed sets PWD to the absolute extension work directory in the WASM sandbox.
        // We need absolute paths so subprocess spawning resolves them correctly.
        let work_dir = std::env::var("PWD").map_err(|_| "could not read PWD from environment")?;
        let tool_path = format!("{work_dir}/{TOOL_DIR}");
        let bicep_ls = format!("{tool_path}/bicep-ls");

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        // Check if the tool is already installed in our local tool path.
        let already_installed = zed::Command {
            command: dotnet.clone(),
            args: vec![
                "tool".to_string(),
                "list".to_string(),
                "--tool-path".to_string(),
                tool_path.clone(),
                "--format".to_string(),
                "json".to_string(),
            ],
            env: Default::default(),
        }
        .output()
        .ok()
        .and_then(|out| serde_json::from_slice::<serde_json::Value>(&out.stdout).ok())
        .is_some_and(|json| {
            json["data"].as_array().is_some_and(|entries| {
                entries.iter().any(|e| {
                    e["packageId"]
                        .as_str()
                        .is_some_and(|id| id.eq_ignore_ascii_case(PACKAGE_ID))
                })
            })
        });

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        let subcommand = if already_installed {
            "update"
        } else {
            "install"
        };
        let output = zed::Command {
            command: dotnet,
            args: vec![
                "tool".to_string(),
                subcommand.to_string(),
                PACKAGE_ID.to_string(),
                "--tool-path".to_string(),
                tool_path,
            ],
            env: Default::default(),
        }
        .output()
        .map_err(|e| format!("failed to run dotnet tool {subcommand}: {e}"))?;

        if output.status != Some(0) {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("dotnet tool {subcommand} failed: {stderr}"));
        }

        // Remove the legacy bicep-language-servers/ directory if present.
        if fs::metadata("bicep-language-servers").is_ok() {
            let _ = fs::remove_dir_all("bicep-language-servers");
        }

        self.bicep_ls_path = Some(bicep_ls.clone());
        Ok(bicep_ls)
    }
}

zed::register_extension!(BicepExtension);
