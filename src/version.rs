use regex::Regex;
use std::{fmt::Display, process::Command};

#[derive(Debug)]
pub struct NginxVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Display for NginxVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn get_nginx_version() -> Option<NginxVersion> {
    let output_result = Command::new("nginx").arg("-v").output();

    let output = match output_result {
        Ok(output) => output,
        Err(_) => {
            eprintln!("Error: nginx not found");
            return None;
        }
    };

    let version_info = String::from_utf8(output.stderr).unwrap();
    let re = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();

    re.captures(&version_info).and_then(|cap| {
        let major = cap[1].parse().ok()?;
        let minor = cap[2].parse().ok()?;
        let patch = cap[3].parse().ok()?;
        Some(NginxVersion {
            major,
            minor,
            patch,
        })
    })
}
