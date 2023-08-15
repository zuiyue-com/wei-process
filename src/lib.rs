use std::os::windows::process::CommandExt;

pub fn run(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    //let output = std::process::Command::new("powershell")
    // .args(&["/C", "start", "-wait", &format!("\"{}\"", cmd), param])
    let output = std::process::Command::new(cmd)
    .args(param)
    .creation_flags(winapi::um::winbase::CREATE_NO_WINDOW)
    .output()?;

    // let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);
    //Ok(&output.stdout)

    match std::str::from_utf8(&output.stdout) {
        Ok(v) => Ok(v.to_string()),
        Err(e) => Err(Box::new(e))
    }    
}