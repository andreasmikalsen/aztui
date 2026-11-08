use std::process::Command;

pub fn run_az(args: &[&str]) -> Result<String, Box<dyn std::error::Error>> {
    let program = if cfg!(windows) { "az.cmd" } else { "az" };
    let output = Command::new(program).args(args).output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.to_string().into());
    }
    Ok(String::from_utf8(output.stdout)?)
}
