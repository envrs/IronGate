use std::fmt;

/// Operating system type
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OsType {
    Windows,
    MacOS,
    Linux,
    FreeBSD,
    OpenBSD,
    NetBSD,
    Android,
    #[allow(clippy::upper_case_acronyms)]
    IOS,
    Unknown,
}

impl fmt::Display for OsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsType::Windows => write!(f, "Windows"),
            OsType::MacOS => write!(f, "macOS"),
            OsType::Linux => write!(f, "Linux"),
            OsType::FreeBSD => write!(f, "FreeBSD"),
            OsType::OpenBSD => write!(f, "OpenBSD"),
            OsType::NetBSD => write!(f, "NetBSD"),
            OsType::Android => write!(f, "Android"),
            OsType::IOS => write!(f, "iOS"),
            OsType::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Operating system information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OsInfo {
    /// OS type
    pub os_type: OsType,
    /// OS version string
    pub version: String,
    /// OS release/build number (if available)
    pub release: Option<String>,
    /// OS edition (e.g., "Home", "Pro" for Windows)
    pub edition: Option<String>,
    /// Architecture (e.g., "x86_64", "aarch64")
    pub architecture: String,
}

impl OsInfo {
    /// Get the current operating system information
    pub fn get() -> Result<Self, crate::Error> {
        #[cfg(target_os = "windows")]
        return crate::windows::get_os_info();

        #[cfg(target_os = "macos")]
        return crate::macos::get_os_info();

        #[cfg(target_os = "linux")]
        return crate::linux::get_os_info();

        #[cfg(target_os = "freebsd")]
        return Ok(Self {
            os_type: OsType::FreeBSD,
            version: std::env::consts::OS.to_string(),
            release: None,
            edition: None,
            architecture: std::env::consts::ARCH.to_string(),
        });

        #[cfg(target_os = "android")]
        return Ok(Self {
            os_type: OsType::Android,
            version: std::env::consts::OS.to_string(),
            release: None,
            edition: None,
            architecture: std::env::consts::ARCH.to_string(),
        });

        #[cfg(target_os = "ios")]
        return Ok(Self {
            os_type: OsType::IOS,
            version: std::env::consts::OS.to_string(),
            release: None,
            edition: None,
            architecture: std::env::consts::ARCH.to_string(),
        });

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "linux",
            target_os = "freebsd",
            target_os = "android",
            target_os = "ios"
        )))]
        return Ok(Self {
            os_type: OsType::Unknown,
            version: "unknown".to_string(),
            release: None,
            edition: None,
            architecture: std::env::consts::ARCH.to_string(),
        });
    }

    /// Get a human-readable description of the OS
    pub fn description(&self) -> String {
        let mut parts = vec![self.os_type.to_string()];

        if !self.version.is_empty() && self.version != "unknown" {
            parts.push(self.version.clone());
        }

        if let Some(ref edition) = self.edition {
            parts.push(edition.clone());
        }

        if let Some(ref release) = self.release {
            parts.push(format!("({})", release));
        }

        parts.push(self.architecture.clone());

        parts.join(" ")
    }
}

impl fmt::Display for OsInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
