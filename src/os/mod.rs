pub mod macos;

use std::path::PathBuf;

use crate::{Error, WolframApp};

pub fn discover_all() -> Vec<WolframApp> {
    #[cfg(target_os = "macos")]
    return macos::discover_all();

    #[cfg(not(target_os = "macos"))]
    return crate::platform_unsupported_error(
        "discover all installed Wolfram applications",
    );
}

pub fn from_app_directory(dir: &PathBuf) -> Result<WolframApp, Error> {
    #[cfg(target_os = "macos")]
    return macos::from_app_directory(dir);

    #[cfg(not(target_os = "macos"))]
    crate::platform_unsupported_error("WolframApp::from_app_directory()")
}