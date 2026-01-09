use crate::{OsInfo, OsType};
use std::ffi::CStr;
use std::mem;

pub fn get_os_info() -> Result<OsInfo, crate::Error> {
    let mut info = unsafe {
        let mut info: libc::utsname = mem::zeroed();
        if libc::uname(&mut info) != 0 {
            return Err(crate::Error::SystemCall("uname failed".to_string()));
        }
        info
    };

    let sysname = unsafe { CStr::from_ptr(info.sysname.as_ptr()) }.to_string_lossy().to_string();
    let release = unsafe { CStr::from_ptr(info.release.as_ptr()) }.to_string_lossy().to_string();
    let version = unsafe { CStr::from_ptr(info.version.as_ptr()) }.to_string_lossy().to_string();
    let machine = unsafe { CStr::from_ptr(info.machine.as_ptr()) }.to_string_lossy().to_string();

    // Get macOS version from system_profiler or sw_vers
    let macos_version = get_macos_version().unwrap_or_else(|| release.clone());

    Ok(OsInfo {
        os_type: OsType::MacOS,
        version: macos_version,
        release: Some(release),
        edition: None,
        architecture: machine,
    })
}

fn get_macos_version() -> Option<String> {
    use std::process::Command;

    // Try sw_vers first (faster)
    if let Ok(output) = Command::new("sw_vers").arg("-productVersion").output() {
        if output.status.success() {
            if let Ok(version) = String::from_utf8(output.stdout) {
                return Some(version.trim().to_string());
            }
        }
    }

    None
}
