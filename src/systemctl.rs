use std::process::Command;

pub enum SystemctlCommand {
    Start,
    Stop,
    Restart,
    Reload,
    Status,
    Enable,
    Disable,
}

impl SystemctlCommand {
    fn as_str(&self) -> &str {
        match *self {
            SystemctlCommand::Start => "start",
            SystemctlCommand::Stop => "stop",
            SystemctlCommand::Restart => "restart",
            SystemctlCommand::Reload => "reload",
            SystemctlCommand::Status => "status",
            SystemctlCommand::Enable => "enable",
            SystemctlCommand::Disable => "disable",
        }
    }

    pub fn execute(&self) {
        let output = Command::new("systemctl")
            .arg(self.as_str())
            .arg("nginx")
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let result = String::from_utf8(output.stdout).unwrap();
            println!("{}", result);
        } else {
            let error_message = String::from_utf8(output.stderr).unwrap();
            eprintln!("Error: {}", error_message);
        }
    }
}
