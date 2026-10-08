//! 🏗️ The journey workspace: a copy of the fixture workspace in a temporary directory, with free ports,
//! its own daemon (the daemon of a workspace is named after the workspace path) and the real `semio`.
//!
//! @see ../../../🧫️fixtures/🧭️journeys/🏗️workspace/📋️project.json

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;

/// 🔢️ The sentinel ports of the fixture manifest that every copy replaces by free ones.
const PORT_A: u16 = 47101;
const PORT_B: u16 = 47102;

/// 🔎️ The first program called `name` on `PATH`.
pub fn which(name: &str) -> PathBuf {
    let paths = std::env::var_os("PATH").unwrap_or_default();
    let exe = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };
    std::env::split_paths(&paths).map(|folder| folder.join(&exe)).find(|candidate| candidate.is_file()).unwrap_or_else(|| panic!("{name} is not on PATH"))
}

/// 🧪 The `semio` binary under test: `SEMIO_TEST_CLI`, else the fleet debug build.
pub fn binary() -> PathBuf {
    if let Some(path) = std::env::var_os("SEMIO_TEST_CLI") { return PathBuf::from(path); }
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).ancestors().find(|folder| folder.join("nx.json").is_file()).expect("the repository root").to_path_buf();
    let exe = if cfg!(windows) { "semio.exe" } else { "semio" };
    repository.join(".🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-v1/debug").join(exe)
}

fn free_port() -> u16 {
    TcpListener::bind(("127.0.0.1", 0)).and_then(|listener| listener.local_addr()).map(|address| address.port()).expect("a free port")
}

/// 🧰️ One isolated workspace and the daemon the binary starts for it.
pub struct Workspace {
    pub root: PathBuf,
    pub port_a: u16,
    pub port_b: u16,
    runtime: PathBuf,
}

impl Workspace {
    /// 📂 Copies the fixture into a fresh directory outside the repository (a `.git` folder marks its root).
    pub fn new(name: &str) -> Self {
        let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../🧫️fixtures/🧭️journeys/🏗️workspace");
        let base = std::env::temp_dir().join(format!("semio-journeys-{}-{}", std::process::id(), name));
        let _ = std::fs::remove_dir_all(&base);
        let root = base.join("workspace");
        std::fs::create_dir_all(root.join(".git")).expect("create the workspace");
        let (port_a, port_b) = (free_port(), free_port());
        for entry in std::fs::read_dir(&source).expect("read the fixture workspace") {
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() || entry.file_name().to_string_lossy().ends_with(".rs") { continue; }
            let text = std::fs::read_to_string(entry.path()).unwrap().replace(&PORT_A.to_string(), &port_a.to_string()).replace(&PORT_B.to_string(), &port_b.to_string());
            std::fs::write(root.join(entry.file_name()), text).unwrap();
        }
        let runtime = base.join("runtime");
        std::fs::create_dir_all(&runtime).unwrap();
        Self { root: std::fs::canonicalize(&root).unwrap(), port_a, port_b, runtime }
    }

    /// 🧱 A workspace that also carries `count` generated tools in a second project (for the launcher search load).
    pub fn with_bulk_tools(name: &str, count: usize) -> Self {
        let workspace = Self::new(name);
        let tools: Vec<String> = (0..count).map(|index| format!(r#"{{"id":"bulk-{index:06}","verb":"{}","command":["bun","--version"]}}"#, ["dev", "build", "test", "check"][index % 4])).collect();
        std::fs::create_dir_all(workspace.root.join("bulk")).unwrap();
        std::fs::write(workspace.root.join("bulk/📋️project.json"), format!(r#"{{"name":"journey-bulk","metadata":{{"semio":{{"dashboard":{{"tools":[{}]}}}}}}}}"#, tools.join(","))).unwrap();
        workspace
    }

    /// 🌍️ The environment every process of this workspace gets: its own socket directory and no inherited nesting.
    pub fn env(&self) -> Vec<(String, String)> {
        vec![("SEMIO_DASHBOARD_RUNTIME_DIR".into(), self.runtime.display().to_string()), ("SEMIO_LOCALE".into(), "en".into())]
    }

    /// ⌨️ Runs `semio args…` in the workspace and waits for it.
    pub fn cli(&self, args: &[&str]) -> Output {
        let mut command = Command::new(binary());
        command.args(args).current_dir(&self.root).stdin(Stdio::null()).envs(self.env());
        command.output().expect("run semio")
    }

    /// 📄 `semio args…` as stdout text, failing the test with stderr when the exit code is not 0.
    pub fn ok(&self, args: &[&str]) -> String {
        let output = self.cli(args);
        assert!(output.status.success(), "semio {args:?} exited {:?}\nstdout:\n{}\nstderr:\n{}", output.status.code(), String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));
        String::from_utf8_lossy(&output.stdout).into_owned()
    }

    /// 🌐️ Fetches `http://127.0.0.1:port/path` with a plain socket (no HTTP library in the oracle's way).
    pub fn get(port: u16, path: &str) -> Result<(u16, String), String> {
        let mut stream = TcpStream::connect_timeout(&([127, 0, 0, 1], port).into(), Duration::from_secs(3)).map_err(|error| error.to_string())?;
        stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
        write!(stream, "GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\nConnection: close\r\n\r\n").map_err(|error| error.to_string())?;
        let mut response = String::new();
        stream.read_to_string(&mut response).map_err(|error| error.to_string())?;
        let status = response.split_whitespace().nth(1).and_then(|code| code.parse().ok()).ok_or_else(|| format!("no status in {response:?}"))?;
        Ok((status, response.split("\r\n\r\n").nth(1).unwrap_or_default().to_string()))
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = self.cli(&["daemon", "stop"]);
        std::thread::sleep(Duration::from_millis(300));
        if std::env::var_os("SEMIO_KEEP_WORKSPACES").is_some() { println!("[kept] {}", self.root.display()); return; }
        if let Some(base) = self.root.parent() { let _ = std::fs::remove_dir_all(base); }
    }
}
