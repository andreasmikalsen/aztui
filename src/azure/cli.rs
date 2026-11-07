use std::process::Command;

pub fn run_az(
    args: &[&str],
) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("az")
        .args(args)
        .output()?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(error.to_string().into());
    }
    Ok(String::from_utf8(output.stdout)?)
}