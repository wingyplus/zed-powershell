use std::fs;
use std::path;
use zed_extension_api::{self as zed, Result};

fn normalize_bundle_path(path: String) -> String {
    if let Some(stripped_path) = path.strip_prefix('/') {
        if stripped_path.len() > 2
            && stripped_path.as_bytes()[0].is_ascii_alphabetic()
            && stripped_path.as_bytes()[1] == b':'
        {
            return stripped_path.to_string();
        }
    }
    path
}

struct PowerShellExtension {
    /// The PowerShell binary, default to `pwsh`.
    // TODO: allow to configure via Zed settings.
    powershell_bin: Option<String>,
}

impl zed::Extension for PowerShellExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            powershell_bin: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let pwsh_bin = PowerShellExtension::powershell_binary_path(self, worktree)?;

        let bundle_path = self
            .language_server_path(language_server_id)
            .map_err(|err| format!("failed to get editor services: {}", err))?;
        let bundle_path = normalize_bundle_path(bundle_path);

        let command = format!(
            r#"
                $Module = Join-Path "{bundle_path}" "PowerShellEditorServices" "PowerShellEditorServices.psd1"
                $SessionDetails = Join-Path "{bundle_path}" "powershell-es.session.json"
                $Log = Join-Path "{bundle_path}" "logs"

                Import-Module $Module

                Start-EditorServices `
                    -Stdio `
                    -SessionDetailsPath $SessionDetails `
                    -LogPath $Log `
                    -FeatureFlags @() `
                    -AdditionalModules @() `
                    -HostName "zed" `
                    -HostProfileId 0 `
                    -HostVersion "1.0.0" `
                    -LogLevel "Trace"
            "#,
        );

        Ok(zed::Command {
            command: pwsh_bin,
            args: vec![
                "-NoLogo".to_string(),
                "-NoProfile".to_string(),
                "-Command".to_string(),
                command.to_string(),
            ],
            env: Default::default(),
        })
    }
}

impl PowerShellExtension {
    fn powershell_binary_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let pwsh_path = match &self.powershell_bin {
            Some(path) if fs::metadata(path).map_or(false, |stat| stat.is_file()) => path.clone(),
            Some(path) => worktree
                .which(path.clone().as_str())
                .ok_or_else(|| "PowerShell must be installed for PowerShell Extension")?,
            None => worktree
                .which("pwsh")
                .ok_or_else(|| "PowerShell must be installed for PowerShell Extension")?,
        };
        self.powershell_bin = Some(pwsh_path.clone());
        Ok(pwsh_path)
    }

    fn language_server_path(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
    ) -> Result<String> {
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "PowerShell/PowerShellEditorServices",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == "PowerShellEditorServices.zip")
            .ok_or_else(|| format!("no PowerShellEditorServices.zip found"))?;

        let version_dir = format!("powershell-es-{}", release.version);
        let lsp_path = format!("{version_dir}/PowerShellEditorServices/Start-EditorServices.ps1");

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

        let abs_path =
            path::absolute(&version_dir).map_err(|e| format!("failed to get absolute path {e}"))?;
        Ok(abs_path.display().to_string())
    }
}

zed::register_extension!(PowerShellExtension);
