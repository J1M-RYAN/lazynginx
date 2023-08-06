use std::process::Command;

pub fn get_nginx_status() -> String {
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("nginx")
        .output()
        .expect("Failed to execute command");

    let status = String::from_utf8(output.stdout).unwrap().trim().to_string();

    if status == "active" || status == "inactive" {
        status
    } else {
        "Error while getting status".to_string()
    }
}
