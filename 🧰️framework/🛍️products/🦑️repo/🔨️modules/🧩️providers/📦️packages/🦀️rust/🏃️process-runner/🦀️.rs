//! 🏃️ Process runner: the process port with its system and recorded runners.
//!
//! Holds the `🏃️ProcessRunner` region of the crate root; the root re-exports its items.

use super::*;

// #region 🏃️ProcessRunner
/// 📨️ One external process invocation: `argv[0]` is the program, the rest are its arguments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessRequest {
    pub argv: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
}

impl ProcessRequest {
    /// 🆕️ Builds a request from a program and its arguments.
    pub fn new(program: &str, args: &[String], cwd: &str) -> ProcessRequest {
        let mut argv = Vec::with_capacity(args.len() + 1);
        argv.push(program.to_string());
        argv.extend(args.iter().cloned());
        ProcessRequest { argv, cwd: if cwd.is_empty() { None } else { Some(cwd.to_string()) }, stdin: None }
    }
}

/// 📬️ What a process left behind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessOutcome {
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    pub status: i32,
}

impl ProcessOutcome {
    /// 🆕️ A successful outcome carrying only standard output.
    pub fn ok(stdout: impl Into<String>) -> ProcessOutcome {
        ProcessOutcome { stdout: stdout.into(), stderr: String::new(), status: 0 }
    }
}

/// 🎞️ One recorded exchange of a process transcript fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessExchange {
    pub argv: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stdin: Option<String>,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    pub status: i32,
}

/// 🎬️ A whole transcript: the exchanges a recorded runner replays.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProcessTranscript {
    pub exchanges: Vec<ProcessExchange>,
}

/// 🏃️ The single seam through which a provider touches the operating system. Everything a provider
/// does to `gh` or `git` is one call on this port, so a transcript replaces the machine.
#[dyn_enum]
pub trait ProcessRunner {
    /// ▶️ Runs one process and reports what it left behind.
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome;
    /// 📜️ Every argv issued so far, in order.
    fn issued(&self) -> Vec<Vec<String>>;
}

/// 🚫️ Go `exec.Error`'s wording for a program `PATH` does not answer to.
fn not_found_message(program: &str) -> String {
    let variable = if cfg!(windows) { "%PATH%" } else { "$PATH" };
    format!("exec: {:?}: executable file not found in {variable}", program)
}

/// 💻️ Runs the real thing through `std::process`.
#[derive(Debug, Default)]
pub struct SystemProcessRunner {
    issued: RefCell<Vec<Vec<String>>>,
}

impl SystemProcessRunner {
    /// 🆕️ A fresh runner with an empty log.
    pub fn new() -> SystemProcessRunner {
        SystemProcessRunner { issued: RefCell::new(Vec::new()) }
    }
}

impl ProcessRunner for SystemProcessRunner {
    /// ▶️ Runs the request and answers what it left behind. A program `PATH` does not answer to is
    /// reported the way Go's `exec.Error` words it, so a provider's refusal reads the same in both
    /// implementations.
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome {
        self.issued.borrow_mut().push(request.argv.clone());
        let Some((program, args)) = request.argv.split_first() else {
            return ProcessOutcome { stdout: String::new(), stderr: "empty argv".to_string(), status: 127 };
        };
        if semio_framework_repo_workspace::look_path(program).is_none() {
            return ProcessOutcome { stdout: String::new(), stderr: not_found_message(program), status: 127 };
        }
        let mut command = semio_framework_repo_workspace::spawn_command(program);
        command.args(args);
        if let Some(cwd) = &request.cwd {
            command.current_dir(cwd);
        }
        match command.output() {
            Ok(output) => ProcessOutcome {
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
                status: output.status.code().unwrap_or(-1),
            },
            Err(error) => ProcessOutcome { stdout: String::new(), stderr: error.to_string(), status: 127 },
        }
    }

    fn issued(&self) -> Vec<Vec<String>> {
        self.issued.borrow().clone()
    }
}

/// 🎞️ Replays a recorded transcript: the first not-yet-consumed exchange whose argv matches wins, so
/// a provider's optional calls may be omitted from the fixture without reordering it.
#[derive(Debug)]
pub struct RecordedProcessRunner {
    transcript: ProcessTranscript,
    consumed: RefCell<Vec<bool>>,
    pub(super) issued: RefCell<Vec<Vec<String>>>,
}

impl RecordedProcessRunner {
    /// 🆕️ Wraps a transcript.
    pub fn new(transcript: ProcessTranscript) -> RecordedProcessRunner {
        let consumed = vec![false; transcript.exchanges.len()];
        RecordedProcessRunner { transcript, consumed: RefCell::new(consumed), issued: RefCell::new(Vec::new()) }
    }

    /// 📥️ Parses a transcript fixture.
    pub fn from_json(source: &str) -> ProviderResult<RecordedProcessRunner> {
        let transcript: ProcessTranscript = serde_json::from_str(source).map_err(|error| ProviderError::new(format!("failed to parse process transcript: {error}")))?;
        Ok(RecordedProcessRunner::new(transcript))
    }
}

impl ProcessRunner for RecordedProcessRunner {
    fn run(&self, request: &ProcessRequest) -> ProcessOutcome {
        self.issued.borrow_mut().push(request.argv.clone());
        let mut consumed = self.consumed.borrow_mut();
        for (index, exchange) in self.transcript.exchanges.iter().enumerate() {
            if consumed[index] || exchange.argv != request.argv {
                continue;
            }
            consumed[index] = true;
            return ProcessOutcome { stdout: exchange.stdout.clone(), stderr: exchange.stderr.clone(), status: exchange.status };
        }
        ProcessOutcome { stdout: String::new(), stderr: format!("no recorded exchange for {:?}", request.argv), status: 127 }
    }

    fn issued(&self) -> Vec<Vec<String>> {
        self.issued.borrow().clone()
    }
}

dyn_enum_close! {
    pub enum ProcessRunners: ProcessRunner {
        System(SystemProcessRunner),
        Recorded(RecordedProcessRunner),
    }
}
// #endregion 🏃️ProcessRunner
