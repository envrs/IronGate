use crate::{OsInfo, OsType};
use std::mem;
use windows_sys::Win32::System::SystemInformation::{
    GetVersionExW, OSVERSIONINFOEXW, VER_NT_WORKSTATION,
};

pub fn get_os_info() -> Result<OsInfo, crate::Error> {
    unsafe {
        let mut version_info: OSVERSIONINFOEXW = mem::zeroed();
        version_info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEXW>() as u32;

        #[allow(deprecated)]
        if GetVersionExW(&mut version_info as *mut _ as *mut _) == 0 {
            // Fallback to environment variables if GetVersionEx fails
            return get_os_info_fallback();
        }

        let version = format!(
            "{}.{}.{}",
            version_info.dwMajorVersion, version_info.dwMinorVersion, version_info.dwBuildNumber
        );

        let edition = get_windows_edition(&version_info);

        Ok(OsInfo {
            os_type: OsType::Windows,
            version,
            release: Some(format!("Build {}", version_info.dwBuildNumber)),
            edition: Some(edition),
            architecture: std::env::consts::ARCH.to_string(),
        })
    }
}

fn get_windows_edition(info: &OSVERSIONINFOEXW) -> String {
    // Determine Windows edition based on version numbers
    let major = info.dwMajorVersion;
    let minor = info.dwMinorVersion;
    let build = info.dwBuildNumber;

    let base_name = match (major, minor) {
        (10, 0) if build >= 22000 => "Windows 11",
        (10, 0) => "Windows 10",
        (6, 3) => "Windows 8.1",
        (6, 2) => "Windows 8",
        (6, 1) => "Windows 7",
        (6, 0) => "Windows Vista",
        (5, 2) => "Windows Server 2003",
        (5, 1) => "Windows XP",
        _ => "Windows",
    };

    let product_type = if info.wProductType == VER_NT_WORKSTATION { "" } else { " Server" };

    format!("{}{}", base_name, product_type)
}

fn get_os_info_fallback() -> Result<OsInfo, crate::Error> {
    // Use std::env as fallback
    Ok(OsInfo {
        os_type: OsType::Windows,
        version: "Unknown".to_string(),
        release: None,
        edition: Some("Windows".to_string()),
        architecture: std::env::consts::ARCH.to_string(),
    })
}
