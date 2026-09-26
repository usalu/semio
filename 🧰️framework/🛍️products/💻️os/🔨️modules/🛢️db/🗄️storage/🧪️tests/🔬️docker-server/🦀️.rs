//! 🐳️ The disposable Docker servers of the live storage laws (the WAL writer fence lanes, the
//! PostgreSQL round-trip law): one `docker` call, one free loopback port, one container removed when
//! its law ends.
pub(crate) fn docker(args: &[&str]) -> String {
    let output = std::process::Command::new("docker").args(args).output().expect("docker CLI");
    assert!(output.status.success(), "docker {args:?}: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

pub(crate) fn free_port() -> u16 {
    std::net::TcpListener::bind(("127.0.0.1", 0)).unwrap().local_addr().unwrap().port()
}

/// 🐳️ A disposable server container, removed when the law ends.
pub(crate) struct Container {
    pub(crate) name: String,
}

impl Drop for Container {
    fn drop(&mut self) {
        let _ = std::process::Command::new("docker").args(["rm", "--force", &self.name]).output();
    }
}
