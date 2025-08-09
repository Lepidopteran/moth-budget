use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=frontend");
    println!("cargo:rerun-if-changed=migrations");

    if let Err(err) = build_frontend() {
        panic!("Failed to build frontend: {err}");
    };
}

fn build_frontend() -> Result<(), String> {

    let status = Command::new("bun")
        .arg("run")
        .arg("build")
        .current_dir("frontend")
        .status()
        .map_err(|err| format!("Failed to execute build command: {err}"))?;

    if !status.success() {

        let status = Command::new("bun")
            .arg("install")
            .current_dir("frontend")
            .status()
            .map_err(|err| format!("Failed to execute install command: {err}"))?;

        if !status.success() {
            return Err("Failed to install dependencies".into());
        }

        let status = Command::new("bun")
            .arg("run")
            .arg("build")
            .current_dir("frontend")
            .status()
            .map_err(|err| format!("Failed to execute build command: {err}"))?;

        if !status.success() {
            return Err("Failed to build frontend".into());
        }

    }

    Ok(())
}
