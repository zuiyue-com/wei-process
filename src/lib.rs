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
    let path = "./".to_owned() + cmd;

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

use std::process::Command;
pub fn is_process_running(process_name: &str) -> bool {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .arg("-Command")
            .arg(format!("Get-Process -Name {} -ErrorAction SilentlyContinue", process_name))
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("bash")
            .arg("-c")
            .arg(format!("pgrep -f {}", process_name))
            .output()
            .expect("Failed to execute command")
    };

    !output.stdout.is_empty()
}