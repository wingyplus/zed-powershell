use zed_extension_api::{self as zed, Result};

pub struct PowerShellEditorServices;

impl PowerShellEditorServices {
    /// Install the PowerShellEditorServices.
    ///
    /// Returns the bundle path after installed.
    pub fn install(language_server_id: &zed_extension_api::LanguageServerId) -> Result<String> {
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

        let path = format!("powershell-es-{}", release.version);

        // TODO: do download when no bundle path found.
        // TODO: cache the bundle path.

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(&asset.download_url, &path, zed::DownloadedFileType::Zip)
            .map_err(|err| format!("download error {}", err))?;

        Ok(path.to_string())
    }
}
