use crate::{OsInfo, OsType};
use std::ffi::CStr;
use std::fs;
use std::mem;

pub fn get_os_info() -> Result<OsInfo, crate::Error> {
    let mut info = unsafe {
        let mut info: libc::utsname = mem::zeroed();
        if libc::uname(&mut info) != 0 {
            return Err(crate::Error::SystemCall("uname failed".to_string()));
        }
        info
    };

    let release = unsafe { CStr::from_ptr(info.release.as_ptr()) }.to_string_lossy().to_string();
    let machine = unsafe { CStr::from_ptr(info.machine.as_ptr()) }.to_string_lossy().to_string();

    // Try to get distribution info from /etc/os-release
    let (distro_name, distro_version) = get_distro_info();

    Ok(OsInfo {
        os_type: OsType::Linux,
        version: distro_version.unwrap_or_else(|| release.clone()),
        release: Some(release),
        edition: distro_name,
        architecture: machine,
    })
}

fn get_distro_info() -> (Option<String>, Option<String>) {
    // Try /etc/os-release first (standard location)
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        return parse_os_release(&content);
    }

    // Fallback to /usr/lib/os-release
    if let Ok(content) = fs::read_to_string("/usr/lib/os-release") {
        return parse_os_release(&content);
    }

    (None, None)
}

fn parse_os_release(content: &str) -> (Option<String>, Option<String>) {
    let mut name = None;
    let mut version = None;

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("NAME=") {
            name = Some(value.trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("VERSION=") {
            version = Some(value.trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("VERSION_ID=") {
            if version.is_none() {
                version = Some(value.trim_matches('"').to_string());
            }
        }
    }

    (name, version)
}
