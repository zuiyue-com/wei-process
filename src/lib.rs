#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

pub fn run(cmd: &str, param: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let path = std::path::Path::new("./").join(cmd).join(".exe");
    if path.exists() {
        return command(&path.display().to_string(), param);
    }

    let path = std::path::Path::new("./bin/").join(cmd).join(".exe");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() -> Result<(), Box<dyn std::error::Error>> {
        // 使用 cmd 和 echo 命令来测试
        let output = run("cmd", vec!["/C".to_string(), "echo Hello, world!".to_string()])?;
        // 检查输出是否为期望的值
        assert_eq!(output.trim(), "Hello, world!"); 
        Ok(())
    }

    #[test]
    fn test_run_wei_sd_exe() -> Result<(), Box<dyn std::error::Error>> {
        let cmd_path = "C:\\Users\\Wei\\Desktop\\work\\wei-sd\\target\\debug\\wei-sd.exe";

        let output = run(cmd_path, vec![])?;

        // 打印输出的数据
        println!("Output from {}: {}", cmd_path, output.trim());

        // 这里你可以检查输出是否为期望的值。但在此例子中，我们只检查输出是否为空。
        assert!(!output.trim().is_empty(), "Output is empty!");

        Ok(())
    }
}
