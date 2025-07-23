use std::ffi::c_void;
use std::fmt::Display;
use std::path::Path;
use windows::Win32::Foundation::*;
use windows::Win32::System::Diagnostics::Debug::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::System::Memory::*;
use windows::Win32::System::Threading::*;
use windows::core::{PCSTR, PCWSTR, PWSTR};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameLauncherError {
    CreateProcessWFailed(windows::core::Error),
    VirtualAllocExFailed,
    WriteProcessMemoryFailed,
    GetModuleHandleFailed,
    GetProcAddressFailed,
    CreateRemoteThreadFailed,
    WaitForSingleObjectFailed(WAIT_EVENT),
    GetExitCodeThreadFailed,
    LauncherFailed(u32),
}

pub fn get_last_error() -> WIN32_ERROR {
    unsafe { GetLastError() }
}

impl Display for GameLauncherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameLauncherError::CreateProcessWFailed(err) => {
                write!(f, "CreateProcessWFailed: {}", err)
            }
            GameLauncherError::VirtualAllocExFailed => {
                write!(f, "VirtualAllocExFailed {:?}", get_last_error())
            }
            GameLauncherError::WriteProcessMemoryFailed => {
                write!(f, "WriteProcessMemoryFailed {:?}", get_last_error())
            }
            GameLauncherError::GetModuleHandleFailed => {
                write!(f, "GetModuleHandleFailed {:?}", get_last_error())
            }
            GameLauncherError::GetProcAddressFailed => {
                write!(f, "GetProcAddressFailed {:?}", get_last_error())
            }
            GameLauncherError::CreateRemoteThreadFailed => {
                write!(f, "CreateRemoteThreadFailed {:?}", get_last_error())
            }
            GameLauncherError::WaitForSingleObjectFailed(event) => {
                write!(f, "WaitForSingleObjectFailed {:?}", event)
            }
            GameLauncherError::GetExitCodeThreadFailed => {
                write!(f, "GetExitCodeThreadFailed {:?}", get_last_error())
            }
            GameLauncherError::LauncherFailed(code) => write!(f, "LauncherFailed: {}", code),
        }
    }
}

fn get_wchar_t(content: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    OsStr::new(content)
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<u16>>()
}

pub type GameLauncherResult<T> = Result<T, GameLauncherError>;

/*pub fn launcher_game(
    gta_dir: &Path,
    dll_dir: &Path,
    command_line: String,
) -> GameLauncherResult<()> {
    // first find testapp.exe is steam
    let is_steam = {
        let test_app = gta_dir.join("testapp.exe");
        test_app.exists() && test_app.is_file()
    };

    let gta_exe = gta_dir.join("gta-vc.exe");
    let dll_file = dll_dir.join("vcmp-game.dll");

    let mut pi = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessW(
            PCWSTR(get_wchar_t(gta_exe.to_str().unwrap()).as_ptr()),
            Some(PWSTR(get_wchar_t(&command_line).as_mut_ptr())),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            PCWSTR(get_wchar_t(gta_dir.to_str().unwrap()).as_ptr()),
            &STARTUPINFOW::default(),
            &mut pi,
        )
        .map_err(GameLauncherError::CreateProcessWFailed)?;
    }

    let dll_wide = get_wchar_t(dll_file.to_str().unwrap());
    let byte_len = dll_wide.len() * 2 + 1 + (
        if is_steam {
            19
        } else {
            0
        }
    ); // 字节数

    let remote_buf = unsafe {
        VirtualAllocEx(
            pi.hProcess,
            None,
            byte_len,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };
    if remote_buf.is_null() {
        return Err(GameLauncherError::VirtualAllocExFailed);
    }

    if is_steam {
        // get module
        let load_lib = unsafe {
            let kernel = GetModuleHandleA(PCSTR("kernel32.dll\0".as_ptr()))
                .map_err(|_| GameLauncherError::GetModuleHandleFailed)?;
            GetProcAddress(kernel, PCSTR("LoadLibraryW\0".as_ptr()))
                .ok_or(GameLauncherError::GetProcAddressFailed)?
        };
        let mut code = [0u8; 19];

        // push lpMem + 19
        code[0] = 0x68;
        let push_addr = (lpMem as usize + code.len()) as u32;
        code[1..5].copy_from_slice(&push_addr.to_le_bytes());

        // call kernel32.LoadLibraryW (relative offset calculation)
        code[5] = 0xE8;
        let call_offset = (fnLoadLibraryW as usize).wrapping_sub(lpMem as usize + 10) as u32;
        code[6..10].copy_from_slice(&call_offset.to_le_bytes());

        // pop eax (get OEP)
        code[10] = 0x58;

        // restore registers
        code[11] = 0x5D; // pop ebp
        code[12] = 0x5F; // pop edi
        code[13] = 0x5E; // pop esi
        code[14] = 0x5A; // pop edx
        code[15] = 0x59; // pop ecx
        code[16] = 0x5B; // pop ebx

        // jmp eax (jump to OEP)
        code[17] = 0xFF;
        code[18] = 0xE0;

        // Wirte mechine code to GTA process.
        unsafe {
            WriteProcessMemory(
                pi.hProcess,
                remote_buf,
                code,
                code.len(),
                Some(&mut written),
            )
        }
        .map_err(|_| GameLauncherError::WriteProcessMemoryFailed)?;

        // Wirte VCMP dll path to GTA process.
        unsafe {
            WriteProcessMemory(
                pi.hProcess,
                remote_buf,
                code,
                code.len(),
                Some(&mut written),
            )
        }
        .map_err(|_| GameLauncherError::WriteProcessMemoryFailed)?;

    }


}*/

pub fn launcher_common_game(
    gta_dir: &Path,
    dll_dir: &Path,
    command_line: String,
) -> GameLauncherResult<()> {
    let gta_exe = gta_dir.join("gta-vc.exe");
    let dll_file = dll_dir.join("vcmp-game.dll");

    let mut pi = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessW(
            PCWSTR(get_wchar_t(gta_exe.to_str().unwrap()).as_ptr()),
            Some(PWSTR(get_wchar_t(&command_line).as_mut_ptr())),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            PCWSTR(get_wchar_t(gta_dir.to_str().unwrap()).as_ptr()),
            &STARTUPINFOW::default(),
            &mut pi,
        )
        .map_err(GameLauncherError::CreateProcessWFailed)?;
    }

    let dll_wide = get_wchar_t(dll_file.to_str().unwrap());
    let byte_len = dll_wide.len() * 2 + 1; // 字节数


    let remote_buf = unsafe {
        VirtualAllocEx(
            pi.hProcess,
            None,
            byte_len,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        )
    };
    if remote_buf.is_null() {
        return Err(GameLauncherError::VirtualAllocExFailed);
    }

    let mut written = 0usize;
    unsafe {
        WriteProcessMemory(
            pi.hProcess,
            remote_buf,
            dll_wide.as_ptr() as *const c_void,
            byte_len,
            Some(&mut written),
        )
    }
    .map_err(|_| GameLauncherError::WriteProcessMemoryFailed)?;

    if written != byte_len {
        return Err(GameLauncherError::WriteProcessMemoryFailed);
    }

    let load_lib = unsafe {
        let kernel = GetModuleHandleA(PCSTR("kernel32.dll\0".as_ptr()))
            .map_err(|_| GameLauncherError::GetModuleHandleFailed)?;
        GetProcAddress(kernel, PCSTR("LoadLibraryW\0".as_ptr()))
            .ok_or(GameLauncherError::GetProcAddressFailed)?
    };

    let inject_thread = unsafe {
        CreateRemoteThread(
            pi.hProcess,
            None,
            0,
            Some(std::mem::transmute(load_lib)),
            Some(remote_buf),
            0,
            None,
        )
        .map_err(|_| GameLauncherError::CreateRemoteThreadFailed)?
    };

    unsafe {
        if WaitForSingleObject(inject_thread, 10000) != WAIT_OBJECT_0 {
            return Err(GameLauncherError::WaitForSingleObjectFailed(
                WaitForSingleObject(inject_thread, 0),
            ));
        }
    }

    // 7) 恢复主线程运行
    unsafe {
        ResumeThread(pi.hThread);
        CloseHandle(inject_thread).ok();
        CloseHandle(pi.hThread).ok();
        CloseHandle(pi.hProcess).ok();
    }

    Ok(())
}
