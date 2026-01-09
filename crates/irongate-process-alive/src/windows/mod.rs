use std::mem::MaybeUninit;

use windows_sys::Win32::Foundation::{FALSE, STILL_ACTIVE};
use windows_sys::Win32::System::Threading::GetExitCodeProcess;

use self::handle::Handle;
use crate::{Pid, State};

mod handle;

pub fn state(pid: Pid) -> State {
    let handle = match Handle::open(pid) {
        Some(handle) => handle,
        None => return State::Unknown,
    };

    let mut status = MaybeUninit::uninit();
    unsafe {
        if GetExitCodeProcess(*handle, status.as_mut_ptr()) == FALSE {
            return State::Unknown;
        }

        if status.assume_init() == STILL_ACTIVE as u32 {
            State::Alive
        } else {
            State::Dead
        }
    }
}
