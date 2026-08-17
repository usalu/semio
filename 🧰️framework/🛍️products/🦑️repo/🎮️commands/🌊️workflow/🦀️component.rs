use crate::args::ParsedArgs;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// #region 🔖️Command
/// 🌊️ Executes a ticket-scoped dependency workflow through locally available coding-agent runners.
pub fn run(root: &Path, parsed: &ParsedArgs) -> i32 {
    let sub = parsed.segments.first().map(String::as_str).unwrap_or("status");
    let ticket = parsed.flag("ticket").map(PathBuf::from).unwrap_or_else(|| root.join(".🧬semio/🦑️repo/🎫️tickets"));
    match sub {
        "status" => {
            let path = workflow_path(&ticket);
            if !path.exists() {
                eprintln!("no workflow at {}", path.display());
                return 1;
            }
            match load_workflow(&ticket) {
                Ok(spec) => {
                    println!("workflow {} waves={} concurrency={}", spec.id, spec.waves.len(), spec.concurrency);
                    0
                }
                Err(error) => {
                    eprintln!("{error}");
                    1
                }
            }
        }
        "run" => {
            let Ok(spec) = load_workflow(&ticket) else {
                eprintln!("missing 🌊️workflow.json under {}", ticket.display());
                return 1;
            };
            let tasks: Vec<_> = spec.waves.iter().flat_map(|wave| wave.tasks.clone()).collect();
            let mut scheduler = Scheduler::new(spec);
            while !scheduler.done() {
                let batch = scheduler.poll_ready();
                if batch.is_empty() {
                    break;
                }
                for id in batch {
                    let task = tasks.iter().find(|task| task.id == id);
                    let (model, prompt) = task.map(|task| (task.model.as_str(), task.prompt.as_str())).unwrap_or(("claude", ""));
                    let ok = if let Some(runner) = select_runner(model) {
                        match runner.spawn(prompt, root) {
                            Ok(mut child) => child.wait().map(|status| status.success()).unwrap_or(false),
                            Err(error) => {
                                eprintln!("agent spawn failed: {error}");
                                false
                            }
                        }
                    } else {
                        eprintln!("no agent runner for model {model}");
                        false
                    };
                    scheduler.complete(&id, ok, None);
                }
            }
            0
        }
        _ => {
            eprintln!("usage: semio workflow status|run --ticket <dir>");
            1
        }
    }
}
// #endregion 🔖️Command

// #region 🔖️Workflow
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct WorkflowTask {
    id: String,
    model: String,
    prompt: String,
    path_scope: Vec<String>,
    verify: Option<String>,
    retries: u32,
    depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct WorkflowWave {
    id: String,
    tasks: Vec<WorkflowTask>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct WorkflowSpec {
    id: String,
    concurrency: usize,
    waves: Vec<WorkflowWave>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TaskState {
    Pending,
    Running,
    Succeeded,
    Failed,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TaskStatus {
    state: TaskState,
    attempts: u32,
    message: Option<String>,
}

/// 📁 Resolves the canonical workflow specification beneath a ticket directory.
fn workflow_path(ticket_dir: &Path) -> PathBuf {
    ticket_dir.join("🌊️workflow.json")
}

fn load_workflow(ticket_dir: &Path) -> std::io::Result<WorkflowSpec> {
    let text = std::fs::read_to_string(workflow_path(ticket_dir))?;
    serde_json::from_str(&text).map_err(|error| std::io::Error::new(std::io::ErrorKind::InvalidData, error))
}
// #endregion 🔖️Workflow

// #region 🔖️Scheduler
/// 🧭 Wave DAG scheduler with bounded concurrency and exclusive path-scope claims.
struct Scheduler {
    spec: WorkflowSpec,
    statuses: HashMap<String, TaskStatus>,
    claims: HashSet<String>,
    ready: VecDeque<String>,
    running: HashSet<String>,
}

impl Scheduler {
    fn new(spec: WorkflowSpec) -> Self {
        let mut statuses = HashMap::new();
        let mut ready = VecDeque::new();
        for wave in &spec.waves {
            for task in &wave.tasks {
                let blocked = !task.depends_on.is_empty();
                statuses.insert(
                    task.id.clone(),
                    TaskStatus { state: if blocked { TaskState::Blocked } else { TaskState::Pending }, attempts: 0, message: None },
                );
                if !blocked {
                    ready.push_back(task.id.clone());
                }
            }
        }
        Self { spec, statuses, claims: HashSet::new(), ready, running: HashSet::new() }
    }

    fn task(&self, id: &str) -> Option<&WorkflowTask> {
        self.spec.waves.iter().flat_map(|wave| wave.tasks.iter()).find(|task| task.id == id)
    }

    fn scope_free(&self, task: &WorkflowTask) -> bool {
        task.path_scope.iter().all(|path| !self.claims.contains(path))
    }

    /// ▶️ Returns task IDs that may start without exceeding concurrency or path claims.
    fn poll_ready(&mut self) -> Vec<String> {
        let capacity = self.spec.concurrency.max(1);
        let mut ready = Vec::new();
        let mut deferred = VecDeque::new();
        while ready.len() + self.running.len() < capacity {
            let Some(id) = self.ready.pop_front() else { break };
            let Some(task) = self.task(&id).cloned() else { continue };
            if !self.scope_free(&task) {
                deferred.push_back(id);
                continue;
            }
            for path in &task.path_scope {
                self.claims.insert(path.clone());
            }
            self.running.insert(id.clone());
            if let Some(status) = self.statuses.get_mut(&id) {
                status.state = TaskState::Running;
                status.attempts += 1;
            }
            ready.push(id);
        }
        self.ready.append(&mut deferred);
        ready
    }

    fn complete(&mut self, id: &str, ok: bool, message: Option<String>) {
        if let Some(task) = self.task(id).cloned() {
            for path in &task.path_scope {
                self.claims.remove(path);
            }
        }
        self.running.remove(id);
        if let Some(status) = self.statuses.get_mut(id) {
            status.state = if ok { TaskState::Succeeded } else { TaskState::Failed };
            status.message = message;
        }
        let succeeded: HashSet<String> = self
            .statuses
            .iter()
            .filter(|(_, status)| status.state == TaskState::Succeeded)
            .map(|(id, _)| id.clone())
            .collect();
        for wave in &self.spec.waves {
            for task in &wave.tasks {
                if self.statuses.get(&task.id).map(|status| status.state) == Some(TaskState::Blocked) && task.depends_on.iter().all(|dependency| succeeded.contains(dependency)) {
                    if let Some(status) = self.statuses.get_mut(&task.id) {
                        status.state = TaskState::Pending;
                    }
                    if !self.ready.contains(&task.id) {
                        self.ready.push_back(task.id.clone());
                    }
                }
            }
        }
    }

    fn done(&self) -> bool {
        self.statuses.values().all(|status| matches!(status.state, TaskState::Succeeded | TaskState::Failed))
    }
}
// #endregion 🔖️Scheduler

// #region 🔖️AgentRunner
trait AgentRunner: Send {
    fn id(&self) -> &str;
    fn available(&self) -> bool;
    fn spawn(&self, prompt: &str, cwd: &Path) -> std::io::Result<std::process::Child>;
}

fn probe_command() -> &'static str {
    if cfg!(windows) { "where" } else { "which" }
}

fn executable_on_path(binary: &str) -> bool {
    Command::new(probe_command()).arg(binary).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|status| status.success()).unwrap_or(false)
}

struct CursorAgent;

impl AgentRunner for CursorAgent {
    fn id(&self) -> &str { "cursor-agent" }
    fn available(&self) -> bool { executable_on_path("cursor-agent") }
    fn spawn(&self, prompt: &str, cwd: &Path) -> std::io::Result<std::process::Child> {
        Command::new("cursor-agent").arg("-p").arg(prompt).current_dir(cwd).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
    }
}

struct ClaudeAgent;

impl AgentRunner for ClaudeAgent {
    fn id(&self) -> &str { "claude" }
    fn available(&self) -> bool { executable_on_path("claude") }
    fn spawn(&self, prompt: &str, cwd: &Path) -> std::io::Result<std::process::Child> {
        Command::new("claude").args(["-p", prompt]).current_dir(cwd).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
    }
}

struct CodexAgent;

impl AgentRunner for CodexAgent {
    fn id(&self) -> &str { "codex" }
    fn available(&self) -> bool { executable_on_path("codex") }
    fn spawn(&self, prompt: &str, cwd: &Path) -> std::io::Result<std::process::Child> {
        Command::new("codex").args(["exec", prompt]).current_dir(cwd).stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()
    }
}

fn available_runners(candidates: Vec<Box<dyn AgentRunner>>) -> Vec<Box<dyn AgentRunner>> {
    candidates.into_iter().filter(|runner| runner.available()).collect()
}

fn select_runner(model: &str) -> Option<Box<dyn AgentRunner>> {
    available_runners(vec![Box::new(CursorAgent), Box::new(ClaudeAgent), Box::new(CodexAgent)]).into_iter().find(|runner| runner.id() == model)
}
// #endregion 🔖️AgentRunner

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scheduler_respects_scope_and_dependencies() {
        let spec = WorkflowSpec {
            id: "demo".into(),
            concurrency: 1,
            waves: vec![WorkflowWave {
                id: "w0".into(),
                tasks: vec![
                    WorkflowTask { id: "a".into(), model: "claude".into(), prompt: "one".into(), path_scope: vec!["x".into()], verify: None, retries: 0, depends_on: vec![] },
                    WorkflowTask { id: "b".into(), model: "claude".into(), prompt: "two".into(), path_scope: vec!["x".into()], verify: None, retries: 0, depends_on: vec!["a".into()] },
                ],
            }],
        };
        let mut scheduler = Scheduler::new(spec);
        assert_eq!(scheduler.poll_ready(), vec!["a".to_string()]);
        assert!(scheduler.poll_ready().is_empty(), "scope claimed / concurrency full");
        scheduler.complete("a", true, None);
        assert_eq!(scheduler.poll_ready(), vec!["b".to_string()]);
        assert_eq!(scheduler.statuses["b"].state, TaskState::Running);
    }

    struct UnavailableRunner;

    impl AgentRunner for UnavailableRunner {
        fn id(&self) -> &str { "unavailable" }
        fn available(&self) -> bool { false }
        fn spawn(&self, _: &str, _: &Path) -> std::io::Result<std::process::Child> { unreachable!() }
    }

    #[test]
    fn unavailable_runners_are_filtered() {
        assert!(available_runners(vec![Box::new(UnavailableRunner)]).is_empty());
    }

    #[test]
    fn platform_probe_command_matches_host() {
        assert_eq!(probe_command(), if cfg!(windows) { "where" } else { "which" });
    }
}
// #endregion 🔖️Tests
