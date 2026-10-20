//! 🏗️ The journey workspace: isolated fixture metadata with free ports and immutable fixture code,
//! its own daemon (the daemon of a workspace is named after the workspace path) and the real `semio`.
//!
//! @see ../../../🧫️fixtures/🧭️journeys/🏗️workspace/📋️project.json

use semio_framework_test_fixtures::fixture_path;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};

static WORKSPACES: AtomicUsize = AtomicUsize::new(0);

/// 🧾️ The portable human workflow vectors consumed by the terminal journeys.
pub fn journey_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../🧫️fixtures/🧭️journeys/🔣️.json"))).unwrap()
}

/// 📜️ Immutable fixture behavior, executed with the isolated workspace as its working directory.
pub fn fixture_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../🧫️fixtures/🧭️journeys/🏗️workspace/📜️script.ts").canonicalize().unwrap()
}

/// 🔢️ The sentinel ports of the fixture manifest that every copy replaces by free ones.
const PORT_A: u16 = 47101;
const PORT_B: u16 = 47102;

/// 🔎️ The first program called `name` on `PATH`.
pub fn which(name: &str) -> PathBuf {
    let paths = std::env::var_os("PATH").unwrap_or_default();
    let exe = if cfg!(windows) { format!("{name}.exe") } else { name.to_string() };
    std::env::split_paths(&paths).map(|folder| folder.join(&exe)).find(|candidate| candidate.is_file()).unwrap_or_else(|| panic!("{name} is not on PATH"))
}

/// 🧪️ The explicit executable held by the owning journey target's installed-tool reader.
pub fn binary() -> PathBuf {
    PathBuf::from(std::env::var_os("SEMIO_TEST_CLI").filter(|path| !path.is_empty()).expect("SEMIO_TEST_CLI from the owning dashboard journey target is required"))
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
    /// 📂 Generates short-lived metadata under the test artifact root without copying source code.
    pub fn new(name: &str) -> Self {
        let source = fixture_path!("../../../🧫️fixtures/🧭️journeys/🏗️workspace");
        let artifacts = PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("owning journey artifact directory required"));
        let artifacts = if artifacts.is_absolute() { artifacts } else { std::env::current_dir().unwrap().join(artifacts) };
        let base = artifacts.join(format!("j{:x}.{:x}", std::process::id(), WORKSPACES.fetch_add(1, Ordering::Relaxed)));
        let root = base.join("w");
        assert!(root.to_string_lossy().encode_utf16().count() <= 152, "journey roots must leave room for native logs within 256 UTF-16 units");
        std::fs::create_dir_all(root.join(".git")).expect("create the workspace");
        let (port_a, port_b) = (free_port(), free_port());
        let text = std::fs::read_to_string(source.join("📋️project.json")).unwrap().replace(&PORT_A.to_string(), &port_a.to_string()).replace(&PORT_B.to_string(), &port_b.to_string());
        let mut manifest: serde_json::Value = serde_json::from_str(&text).unwrap();
        for tool in manifest["metadata"]["semio"]["dashboard"]["tools"].as_array_mut().unwrap() {
            for argument in tool["command"].as_array_mut().unwrap() {
                if argument.as_str() == Some("{workspace}/📜️script.ts") { *argument = fixture_script().display().to_string().into(); }
            }
        }
        std::fs::write(root.join("📋️project.json"), serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        let runtime = base.join("r");
        std::fs::create_dir_all(&runtime).unwrap();
        println!("[DEBUG] isolated journey {name} root={}", root.display());
        Self { root: std::fs::canonicalize(&root).unwrap(), port_a, port_b, runtime }
    }

    /// 🎚️ Adds the portable choice/flag/text fixture only to the journey that exercises its form.
    pub fn with_form_tool(name: &str) -> Self {
        Self::with_fixture_tool(name, "localizedForm")
    }

    /// 🐚️ Uses the immutable production developer operation with a test-owned original invocation.
    pub fn with_terminal_tool(name: &str) -> Self {
        Self::with_fixture_tool(name, "workspaceTerminal")
    }

    fn with_fixture_tool(name: &str, key: &str) -> Self {
        let workspace = Self::new(name);
        let path = workspace.root.join("📋️project.json");
        let mut manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        let fixture = journey_fixture();
        let mut tool = fixture[key]["tool"].clone();
        if key == "workspaceTerminal" { tool["parameters"][1]["default"] = fixture[key][if cfg!(windows) { "windowsShell" } else { "unixShell" }].clone(); }
        tool["command"][1] = fixture_script().display().to_string().into();
        manifest["metadata"]["semio"]["dashboard"]["tools"].as_array_mut().unwrap().push(tool);
        std::fs::write(path, serde_json::to_vec_pretty(&manifest).unwrap()).unwrap();
        workspace
    }

    /// 💾️ The bounded local preference journal of this isolated test workspace.
    pub fn preference_path(&self) -> PathBuf { self.runtime.join("p.jsonl") }

    /// 🌐️ Where the independent opener records the exact address the UI handed it.
    pub fn opened_url_path(&self) -> PathBuf { self.runtime.join("opened.json") }

    /// 🧾️ A durable independent receipt unaffected by native terminal hard wrapping.
    pub fn terminal_receipt_path(&self) -> PathBuf { self.runtime.join("t.json") }

    /// 📐️ Physical console cursor facts of the immutable fixture width probe.
    pub fn width_probe_path(&self) -> PathBuf { self.runtime.join("width.json") }

    /// 📏️ Verifies the actual native files remain within the repository's cross-platform path budget.
    pub fn assert_paths(&self) {
        fn maximum(path: &Path) -> usize {
            let length = path.to_string_lossy().encode_utf16().count();
            assert!(length <= 256, "journey path exceeds 256 UTF-16 units: {}", path.display());
            if path.is_dir() { std::fs::read_dir(path).unwrap().map(|entry| maximum(&entry.unwrap().path())).max().unwrap_or(length).max(length) } else { length }
        }
        println!("[DEBUG] native journey file paths maxUtf16={}", maximum(self.root.parent().unwrap()));
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
        vec![
            ("SEMIO_DASHBOARD_RUNTIME_DIR".into(), self.runtime.display().to_string()),
            ("SEMIO_DASHBOARD_INSTANCE".into(), String::new()),
            ("SEMIO_DASHBOARD_CONFIG".into(), self.preference_path().display().to_string()),
            ("JOURNEY_TERMINAL_RECORD".into(), self.terminal_receipt_path().display().to_string()),
            ("JOURNEY_WIDTH_RECORD".into(), self.width_probe_path().display().to_string()),
            ("SEMIO_LOCALE".into(), "en".into()),
            ("SEMIO_APPEARANCE".into(), String::new()),
            ("SEMIO_LOCKED_TERMINOLOGY".into(), String::new()),
            ("SEMIO_DASHBOARD_RENDERER".into(), String::new()),
            ("SEMIO_DASHBOARD_LAYOUT".into(), String::new()),
            ("SEMIO_DASHBOARD_PREFIX".into(), String::new()),
            ("SEMIO_REPO_IMPLEMENTATION".into(), "rust".into()),
        ]
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
