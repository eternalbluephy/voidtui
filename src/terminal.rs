pub struct WindowsTerminalSupports {
    pub virtual_terminal_processing: bool,
    pub truecolor: bool
}

/// Get windows terminal vt and truecolor support.
/// This function would return wrong infomation if you run by 'cargo run' or 'cargo test'.
/// Please compile and run the executable file in your terminal.
#[cfg(windows)]
pub fn get_windows_terminal_supports() -> WindowsTerminalSupports {
    use winapi::{shared::{minwindef::DWORD, ntdef::NULL}, um::{consoleapi::GetConsoleMode, processenv::GetStdHandle, winbase::STD_OUTPUT_HANDLE, wincon::ENABLE_VIRTUAL_TERMINAL_PROCESSING}};
    use winver::WindowsVersion;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle == NULL {
            panic!("Failed to get stdout handle.")
        }
        let mut mode: DWORD = 0;
        let success = GetConsoleMode(handle, &mut mode);
        let vt = success != 0 && (mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING) != 0;
        let mut truecolor = false;
        if vt {
            let version = WindowsVersion::detect().unwrap();
            truecolor = version > WindowsVersion::new(10, 0, 15063);
        }
        WindowsTerminalSupports {
            virtual_terminal_processing: vt,
            truecolor
        }
    }
}


#[cfg(test)]
mod tests {
    use super::get_windows_terminal_supports;

    #[test]
    fn test_get_windows_terminal_supports() {
        let supports = get_windows_terminal_supports();
        println!("Support virtual terminal processing: {}", supports.virtual_terminal_processing);
        println!("Support truecolor: {}", supports.truecolor);
    }
    
}