mod error;
mod info;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "linux")]
mod linux;

pub use error::Error;
pub use info::{OsInfo, OsType};

/// Get the current operating system information
pub fn get() -> Result<OsInfo, Error> {
    OsInfo::get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_os_info() {
        let info = get().expect("Failed to get OS info");

        println!("OS Type: {}", info.os_type);
        println!("Version: {}", info.version);
        println!("Architecture: {}", info.architecture);
        println!("Description: {}", info.description());

        assert!(!info.version.is_empty());
        assert!(!info.architecture.is_empty());
    }

    #[test]
    fn test_os_type_display() {
        assert_eq!(OsType::Windows.to_string(), "Windows");
        assert_eq!(OsType::MacOS.to_string(), "macOS");
        assert_eq!(OsType::Linux.to_string(), "Linux");
    }

    #[test]
    fn test_os_info_description() {
        let info = OsInfo {
            os_type: OsType::Linux,
            version: "22.04".to_string(),
            release: Some("5.15.0".to_string()),
            edition: Some("Ubuntu".to_string()),
            architecture: "x86_64".to_string(),
        };

        let desc = info.description();
        assert!(desc.contains("Linux"));
        assert!(desc.contains("22.04"));
        assert!(desc.contains("Ubuntu"));
        assert!(desc.contains("x86_64"));
    }
}
