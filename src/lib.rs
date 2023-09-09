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

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run_async(cmd: &str, param: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let path = "./".to_owned() + cmd + ".exe";
    #[cfg(not(target_os = "windows"))]
    let path = "./".to_owned() + cmd;

    info!("path: {:?}", path);

    if let Ok(()) = command_async(&path, param.clone()) {
        return Ok(());
    };

    #[cfg(target_os = "windows")]
    let path = "./data/".to_owned() + cmd + ".exe";
    #[cfg(not(target_os = "windows"))]
    let path = "./data/".to_owned() + cmd;

    if let Ok(()) = command_async(&path, param.clone()) {
        return Ok(());
    };

    info!("{} dir: {:?}", cmd, wei_env::dir_bin());
    let path = wei_env::read(&wei_env::dir_bin(),cmd)?;
    command_async(path.as_str(), param)
}

pub fn command_async(cmd: &str, param: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .spawn()?;

    #[cfg(not(target_os = "windows"))]
    std::process::Command::new(cmd)
    .args(param)
    .spawn()?;

    Ok(())
}

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::ptr::null_mut;

#[cfg(target_os = "windows")]
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS};
#[cfg(target_os = "windows")]
use winapi::um::handleapi::CloseHandle;
#[cfg(target_os = "windows")]
use winapi::um::psapi::GetModuleFileNameExW;
#[cfg(target_os = "windows")]
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
#[cfg(target_os = "windows")]
use winapi::um::processthreadsapi::OpenProcess;
#[cfg(target_os = "windows")]
pub fn is_process_running(process_file_name: &str) -> bool {
    let process_file_name = process_file_name.to_owned() + ".exe";
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snapshot.is_null() { return false; }

        let mut process_entry = PROCESSENTRY32 {
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

        if Process32First(snapshot, &mut process_entry) == 0 {
            CloseHandle(snapshot);
            return false;
        }

        loop {
            let h_process = OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_entry.th32ProcessID);
            if !h_process.is_null() {
                let mut buffer = [0u16; 260];
                let result = GetModuleFileNameExW(h_process, null_mut(), buffer.as_mut_ptr(), buffer.len() as u32);
                if result != 0 {
                    let len = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
                    let wide_string: &[u16] = std::slice::from_raw_parts(buffer.as_ptr() as *const u16, len);
                    let current_proc_file_path = OsString::from_wide(wide_string).to_string_lossy().into_owned();
                    let current_proc_file_name = Path::new(&current_proc_file_path).file_name().unwrap().to_str().unwrap();

                    if current_proc_file_name == process_file_name {
                        CloseHandle(h_process);
                        CloseHandle(snapshot);
                        return true;
                    }
                }
                CloseHandle(h_process);
            }

            if Process32Next(snapshot, &mut process_entry) == 0 { break; }
        }

        CloseHandle(snapshot);
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
