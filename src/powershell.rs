use zed_extension_api::{self as zed};

struct PowerShellExtension;

impl zed::Extension for PowerShellExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        todo!()
    }
}

zed::register_extension!(PowerShellExtension);
