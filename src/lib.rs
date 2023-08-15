pub fn run(cmd: &str) -> String {
    let output = std::process::Command::new("powershell")
    .args(&["/C", "start", "-wait", &format!("\"{}\"", cmd)]).output().unwrap();

    let (res, _, _) = encoding_rs::UTF_16LE.decode(&output.stdout);
    res.to_string()
}