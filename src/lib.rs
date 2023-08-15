use std::os::windows::process::CommandExt;

pub fn run(cmd: &str, param: &str) -> Result<String, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("powershell")
    .args(&["/C", "start", "-wait", &format!("\"{}\"", cmd), param])
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);
    Ok(res.to_string())
}