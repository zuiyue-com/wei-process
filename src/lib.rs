#[macro_use]
extern crate wei_log;

#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let path = "./".to_owned() + cmd + ".exe";
    #[cfg(not(target_os = "windows"))]
    let path = "./".to_owned() + cmd;

    info!("path: {:?}", path);

    if let Ok(data) = command(&path, param.clone()) {
        return Ok(data);
    };

    #[cfg(target_os = "windows")]
    let path = "./data/".to_owned() + cmd + ".exe";
    #[cfg(not(target_os = "windows"))]
    let path = "./data/".to_owned() + cmd;

    if let Ok(data) = command(&path, param.clone()) {
        return Ok(data);
    };

    info!("{} dir: {:?}", cmd, wei_env::dir_bin());
    let path = wei_env::read(&wei_env::dir_bin(),cmd)?;
    command(path.as_str(), param)
}

/// Run command
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn command(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new(cmd)
    .args(param)
    .output()?;

    match std::str::from_utf8(&output.stdout) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(Box::new(e))
    }    
}

pub fn command_async(cmd: &str, param: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .spawn()?;

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new(cmd)
    .args(param)
    .spawn()?;

    Ok(())
}

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::shared::minwindef::FALSE;
#[cfg(target_os = "windows")]
pub fn is_process_running(target_process_name: &str) -> bool {
    unsafe {
        let h_process_snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if h_process_snap.is_null() {
            eprintln!("Failed to create snapshot.");
            return false;
        }

        let mut pe32 = PROCESSENTRY32 {
            dwSize: std::mem::size_of::<PROCESSENTRY32>() as u32,
            cntUsage: 0,
            th32ProcessID: 0,
            th32DefaultHeapID: 0,
            th32ModuleID: 0,
            cntThreads: 0,
            th32ParentProcessID: 0,
            pcPriClassBase: 0,
            dwFlags: 0,
            szExeFile: [0; 260],
        };

        if Process32First(h_process_snap, &mut pe32) == FALSE {
            CloseHandle(h_process_snap);
            eprintln!("Failed to gather information from the first process.");
            return false;
        }

        while Process32Next(h_process_snap, &mut pe32) != FALSE {
            let name = {
                let len = pe32.szExeFile.iter().position(|&x| x == 0).unwrap_or(pe32.szExeFile.len());
                let wide_string: &[u16] = std::slice::from_raw_parts(pe32.szExeFile.as_ptr() as *const u16, len);
                OsString::from_wide(wide_string)
                    .to_string_lossy()
                    .into_owned()
            };
            
            if name == target_process_name {
                CloseHandle(h_process_snap);
                return true;
            }
        }

        CloseHandle(h_process_snap);
    }

    false
}

#[cfg(target_os = "linux")]
use procfs::process::all_processes;
#[cfg(target_os = "linux")]
fn is_process_running(target_process_name: &str) -> bool {
    match all_processes() {
        Ok(processes) => {
            for proc in processes {
                if let Ok(proc) = proc {
                    if proc.comm == target_process_name {
                        return true;
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("Failed to get processes: {}", err);
        },
    }

    false
}

use std::process::Command;
pub fn kill(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C").arg(format!("taskkill /IM {}.exe /F", name));
        cmd.output()?;
    }

    #[cfg(target_os = "linux")]
    {
        let mut cmd = Command::new("bash");
        cmd.arg("-c").arg(format!("pkill {}", name));
        cmd.output()?;
    }
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn hide() -> Result<(), Box<dyn std::error::Error>> {
    if !is_debug()? {
        let window = unsafe { winapi::um::wincon::GetConsoleWindow() };
        if window != std::ptr::null_mut() {
            unsafe {
                winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_HIDE);
            }
        }
    }
    Ok(())
}

pub fn is_debug() -> Result<bool, Box<dyn std::error::Error>> {
    let home_dir = std::env::var("USERPROFILE")?;
    if std::path::Path::new(&home_dir).join("AppData\\Local\\wei\\debug.dat").exists() {
        return Ok(true);
    }

    return Ok(false);
}
