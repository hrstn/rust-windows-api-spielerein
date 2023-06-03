use std::any::Any;
use windows::{
    core::*, Data::Xml::Dom::*, Win32::Foundation::*, Win32::System::Threading::*,
    Win32::UI::WindowsAndMessaging::*,
};
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::ptr::{null, null_mut};

fn main() {
    unsafe {
       println!("[+] spawning ... ");
        let mut si = STARTUPINFOW::default() ;
        let mut pi = PROCESS_INFORMATION::default();
        let command = "C:\\Windows\\System32\\notepad.exe";
        let wide_command: Vec<u16> = OsStr::new(&command)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();
        let command_ptr = wide_command.as_ptr();
       let process = CreateProcessW(PCWSTR::from_raw(command_ptr),PWSTR::null(),
                                    Option::default(), Option::default(), FALSE,
                                    CREATE_UNICODE_ENVIRONMENT, Option::default(),
                                    Option::default(), &mut si, &mut pi );
        if process != false {
            println!("[+] spawned it nice");
        } else {
            println!("Failed to spawn process" );
        }

        println!("(+) process started! pid: {}\n", pi.dwProcessId);
        println!("\t(+) pid: {}, handle: {:#02x?}\n", pi.dwProcessId, pi.hProcess);
        println!("\t(+) tid: {}, handle: {:#02x?}\n\r", pi.dwThreadId, pi.hThread);
    }
}
