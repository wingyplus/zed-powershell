mod language_server;

use zed_extension_api::{self as zed, settings::LspSettings};

use crate::language_server::PowerShellEditorServices as EditorServices;

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
            powershell_bin: Some("pwsh".to_string()),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let pwsh_bin = worktree
            .which(self.powershell_bin.clone().unwrap().as_str())
            .ok_or_else(|| "No PowerShell command found")?;

        let bundle_path = LspSettings::for_worktree("powershell-es", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary)
            .and_then(|binary| binary.path)
            .unwrap();

        // TODO: make remote install works.
        // let bundle_path = EditorServices::install(language_server_id)
        //     .map_err(|err| format!("failed to get editor services: {}", err))?;

        let command = format!("{bundle_path}/PowerShellEditorServices/Start-EditorServices.ps1 -BundledModulesPath {bundle_path} -Stdio -SessionDetailsPath {bundle_path}/powershell-es.session.json  -LogPath {bundle_path}/logs -FeatureFlags @() -AdditionalModules @() -HostName zed -HostProfileId 0 -HostVersion 1.0.0 -LogLevel Diagnostic");

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

zed::register_extension!(PowerShellExtension);
