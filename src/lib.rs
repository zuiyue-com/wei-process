#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub fn run(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let path = std::path::Path::new("./").join(cmd).join(".exe");
    #[cfg(not(target_os = "windows"))]
    let path = std::path::Path::new("./").join(cmd);

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

    let path = wei_env::read(&wei_env::dir_bin(),cmd)?;
    command(path.as_str(), param)
}

pub fn command(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    //let output = std::process::Command::new("powershell")
    // .args(&["/C", "start", "-wait", &format!("\"{}\"", cmd), param])

    #[cfg(target_os = "windows")]
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    #[cfg(not(target_os = "windows"))]
    let output = std::process::Command::new(cmd)
    .args(param)
    .output()?;

    // let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);
    //Ok(&output.stdout)

    match std::str::from_utf8(&output.stdout) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(Box::new(e))
    }    
}
