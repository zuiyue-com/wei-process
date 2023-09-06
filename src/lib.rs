#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

/// Run wei command, If the program does not exist/ Under the data/directory, search for the program's configuration file
/// # Arguments
/// * `cmd` - Command name
/// * `param` - Command parameters
pub fn run(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let path = std::path::Path::new("./").join(cmd).join(".exe");
    #[cfg(not(target_os = "windows"))]
    let path = std::path::Path::new("./").join(cmd);

    info!("path: {:?}", path);

    if path.exists() {
        return command(&path.display().to_string(), param);
    }

    #[cfg(target_os = "windows")]
    let path = std::path::Path::new("./data/").join(cmd).join(".exe");
    #[cfg(not(target_os = "windows"))]
    let path = std::path::Path::new("./data/").join(cmd);

    if path.exists() {
        return command(&path.display().to_string(), param);
    }

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
