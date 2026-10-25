//! 🔧️ Execution and parsing: the process port with its replay runner, plan execution and the per-runner transcript parsers.
//!
//! Holds the `⚙️Execution`, `📊️Parsing` regions of the crate root; the root re-exports its items.

use super::*;

//#region ⚙️Execution
// 🚚️ `ProcessRequest` / `ProcessOutput` / `ProcessRunner` are the shape `🧩️providers` will own for
// every process this repository shells out to (git, gh, devcontainer, the test runners). They are
// declared here because `🧩️providers` does not exist yet; consolidating means deleting this block
// and importing the identical trait, with no call-site change.

/// 🎬️ One process to run.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProcessRequest {
    /// 🏷️ Executable name.
    pub program: String,
    /// 📜️ Arguments after the program name.
    pub args: Vec<String>,
    /// 📁️ Working directory.
    pub cwd: String,
    /// 🌱️ Environment overlay.
    #[serde(default)]
    pub env: BTreeMap<String, String>,
}

/// 📤️ What a process answered.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct ProcessOutput {
    /// 🚥️ Exit status.
    pub status: i32,
    /// 📝️ Standard output.
    #[serde(default)]
    pub stdout: String,
    /// 📕️ Standard error.
    #[serde(default)]
    pub stderr: String,
}

/// ⚙️ The only door from a plan to an operating system.
pub trait ProcessRunner {
    /// 🏃️ Runs one process to completion.
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String>;
}

/// 📹️ One recorded `(argv, cwd) → output` pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct RecordedTranscript {
    /// ⌨️ The argv this transcript answers, program name first.
    pub argv: Vec<String>,
    /// 💼️ The working directory it answers for; empty matches any.
    #[serde(default)]
    pub cwd: String,
    /// 🖨️ The recorded answer.
    pub output: ProcessOutput,
}

/// 🎞️ A `ProcessRunner` that replays committed transcripts and never touches the machine.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct RecordedProcessRunner {
    /// 🔃️ Transcripts, matched in order.
    #[serde(default)]
    pub transcripts: Vec<RecordedTranscript>,
}

impl RecordedProcessRunner {
    /// 🆕️ Builds a replaying runner.
    pub fn new(transcripts: Vec<RecordedTranscript>) -> Self {
        Self { transcripts }
    }
}

impl ProcessRunner for RecordedProcessRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        let mut argv = vec![request.program.clone()];
        argv.extend(request.args.clone());
        self.transcripts
            .iter()
            .find(|transcript| transcript.argv == argv && (transcript.cwd.is_empty() || clean_path(&transcript.cwd) == clean_path(&request.cwd)))
            .map(|transcript| transcript.output.clone())
            .ok_or_else(|| format!("no recorded transcript for {}", argv.join(" ")))
    }
}

/// 🚩️ A cooperative cancellation flag shared with whoever drives the run.
#[derive(Clone, Debug, Default)]
pub struct CancellationToken {
    flag: Arc<AtomicBool>,
}

impl CancellationToken {
    /// 🔑️ A token that is not cancelled.
    pub fn new() -> Self {
        Self::default()
    }
    /// 🛑️ Requests cancellation; every later invocation is skipped.
    pub fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
    }
    /// ❓️ Whether cancellation was requested.
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }
}

/// 📣️ What the caller is told while a plan runs.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(tag = "event", rename_all = "kebab-case")]
pub enum ProgressEvent {
    /// 🏠️ The plan is about to start.
    PlanStarted {
        /// 🗓️ Invocations in the plan.
        total: usize,
    },
    /// 🔰️ One invocation is starting.
    InvocationStarted {
        /// 🔢️ Zero-based position in the plan.
        index: usize,
        /// 📟️ Its argv.
        argv: Vec<String>,
    },
    /// ⏹️ One invocation finished.
    InvocationFinished {
        /// 📍️ Zero-based position in the plan.
        index: usize,
        /// 🚦️ How it ended.
        status: RunStatus,
        /// 🚨️ Failing tests it reported.
        failed: usize,
    },
    /// ✖️ Cancellation was observed before the invocation at `index` started.
    Cancelled {
        /// ✔️ Invocations that had already completed.
        completed: usize,
    },
    /// 🏁️ The plan is done.
    PlanFinished {
        /// 🎉️ Invocations that completed.
        completed: usize,
    },
}

/// 📊️ What a whole plan produced.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct ExecutionReport {
    /// 📋️ One outcome per completed invocation, in plan order.
    #[serde(default)]
    pub outcomes: Vec<TestOutcome>,
    /// 📬️ Whether the run stopped early because cancellation was requested.
    #[serde(default)]
    pub cancelled: bool,
    /// 🥇️ Invocations that completed.
    #[serde(default)]
    pub completed: usize,
    /// 🗺️ Invocations the plan contained.
    #[serde(default)]
    pub total: usize,
    /// 🚫️ Planning refusals plus any runner that could not be started.
    #[serde(default)]
    pub problems: Vec<String>,
}

/// ▶️ Runs a plan through the port, reporting progress and honouring cancellation between invocations.
pub fn execute_plan(plan: &InvocationPlan, runner: &dyn ProcessRunner, cancellation: &CancellationToken, progress: &mut dyn FnMut(ProgressEvent)) -> ExecutionReport {
    let total = plan.invocations.len();
    let mut report = ExecutionReport { total, problems: plan.problems.clone(), ..ExecutionReport::default() };
    progress(ProgressEvent::PlanStarted { total });
    for (index, invocation) in plan.invocations.iter().enumerate() {
        if cancellation.is_cancelled() {
            report.cancelled = true;
            progress(ProgressEvent::Cancelled { completed: report.completed });
            return report;
        }
        progress(ProgressEvent::InvocationStarted { index, argv: invocation.argv.clone() });
        let request = ProcessRequest {
            program: invocation.argv.first().cloned().unwrap_or_default(),
            args: invocation.argv.iter().skip(1).cloned().collect(),
            cwd: invocation.cwd.clone(),
            env: invocation.env.clone(),
        };
        let outcome = match runner.run(&request) {
            Ok(output) => parse_outcome(invocation.runner, &output),
            Err(message) => {
                report.problems.push(message);
                TestOutcome { runner: invocation.runner, status: RunStatus::NotRun, ..TestOutcome::default() }
            }
        };
        progress(ProgressEvent::InvocationFinished { index, status: outcome.status, failed: outcome.totals.failed });
        report.outcomes.push(outcome);
        report.completed += 1;
    }
    progress(ProgressEvent::PlanFinished { completed: report.completed });
    report
}
//#endregion ⚙️Execution

//#region 📊️Parsing
/// ↕️ How one test ended.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum TestStatus {
    /// 🟢️ Passed.
    #[default]
    Passed,
    /// 💥️ Failed.
    Failed,
    /// ⏭️ Skipped, ignored or pending.
    Skipped,
}

/// 🔚️ How one invocation ended.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum RunStatus {
    /// 🎟️ Every test passed.
    Passed,
    /// 🔴️ At least one test failed, or the runner exited non-zero.
    Failed,
    /// ⛔️ Cancelled before it produced a verdict.
    Cancelled,
    /// ⚪️ Never started.
    NotRun,
}

/// 🧪️ One test, normalised across every runner dialect.
#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]
pub struct TestCaseOutcome {
    /// 🔖️ Test name as the runner reported it.
    pub name: String,
    /// 📦️ Package, file or suite the runner attributed it to.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub suite: String,
    /// 🎚️ Verdict.
    pub status: TestStatus,
    /// ⏱️ Duration in milliseconds where the runner reports one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<f64>,
    /// 💬️ Failure text where the runner reports one.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

/// 💯️ Counts, always recomputed from the cases so a transcript's own summary can never drift.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TestTotals {
    /// ✅️ Passing tests.
    pub passed: usize,
    /// ❌️ Failing tests.
    pub failed: usize,
    /// 🚷️ Skipped tests.
    pub skipped: usize,
    /// 🧾️ All reported tests.
    pub total: usize,
}

/// 📈️ One invocation's result in the one model every runner folds into.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TestOutcome {
    /// 🏭️ Which runner produced it.
    pub runner: Runner,
    /// ⚖️ Overall verdict.
    pub status: RunStatus,
    /// 🔬️ Every reported test, in report order.
    #[serde(default)]
    pub tests: Vec<TestCaseOutcome>,
    /// 🧮️ Counts.
    pub totals: TestTotals,
    /// 🚏️ Runner exit status where one was observed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_status: Option<i32>,
}

impl Default for TestOutcome {
    fn default() -> Self {
        Self { runner: Runner::Go, status: RunStatus::NotRun, tests: Vec::new(), totals: TestTotals::default(), exit_status: None }
    }
}

impl TestOutcome {
    fn finish(runner: Runner, tests: Vec<TestCaseOutcome>, output: &ProcessOutput) -> Self {
        let mut totals = TestTotals { total: tests.len(), ..TestTotals::default() };
        for case in &tests {
            match case.status {
                TestStatus::Passed => totals.passed += 1,
                TestStatus::Failed => totals.failed += 1,
                TestStatus::Skipped => totals.skipped += 1,
            }
        }
        let status = if totals.failed > 0 || output.status != 0 { RunStatus::Failed } else { RunStatus::Passed };
        Self { runner, status, tests, totals, exit_status: Some(output.status) }
    }
}

/// 🧲️ Dispatches a runner's native report to its parser.
pub fn parse_outcome(runner: Runner, output: &ProcessOutput) -> TestOutcome {
    let outcome = match runner {
        Runner::Go => parse_go_test_json(output),
        Runner::Cargo => parse_cargo_test(output),
        Runner::CargoNextest => parse_cargo_nextest(output),
        Runner::Dotnet => parse_dotnet_test(output),
        Runner::Npx | Runner::Npm => parse_vitest_json(output),
        Runner::Uv | Runner::Pytest => parse_pytest(output),
        Runner::Rspec => parse_rspec_json(output),
    };
    TestOutcome { runner, ..outcome }
}

/// 🐹️ `go test -json`: one JSON object per line, verdicts on `pass`/`fail`/`skip` with a `Test`.
pub fn parse_go_test_json(output: &ProcessOutput) -> TestOutcome {
    let mut messages: BTreeMap<String, String> = BTreeMap::new();
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let Ok(Value::Object(event)) = serde_json::from_str::<Value>(line.trim()) else { continue };
        let action = event.get("Action").and_then(Value::as_str).unwrap_or("");
        let package = event.get("Package").and_then(Value::as_str).unwrap_or("").to_string();
        let Some(name) = event.get("Test").and_then(Value::as_str) else { continue };
        let key = format!("{package}::{name}");
        if action == "output" {
            let text = event.get("Output").and_then(Value::as_str).unwrap_or("");
            messages.entry(key).or_default().push_str(text);
            continue;
        }
        let status = match action {
            "pass" => TestStatus::Passed,
            "fail" => TestStatus::Failed,
            "skip" => TestStatus::Skipped,
            _ => continue,
        };
        let duration_ms = event.get("Elapsed").and_then(Value::as_f64).map(|seconds| seconds * 1000.0);
        let message = if status == TestStatus::Failed { messages.get(&key).cloned().unwrap_or_default().trim_end().to_string() } else { String::new() };
        tests.push(TestCaseOutcome { name: name.to_string(), suite: package, status, duration_ms, message });
    }
    TestOutcome::finish(Runner::Go, tests, output)
}

/// 🟦️ Vitest's JSON reporter (the shape Jest's reporter also emits).
pub fn parse_vitest_json(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    if let Ok(Value::Object(report)) = serde_json::from_str::<Value>(json_body(&output.stdout)) {
        for file in report.get("testResults").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default() {
            let suite = file.get("name").and_then(Value::as_str).unwrap_or("").to_string();
            for assertion in file.get("assertionResults").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default() {
                let name = assertion
                    .get("fullName")
                    .and_then(Value::as_str)
                    .or_else(|| assertion.get("title").and_then(Value::as_str))
                    .unwrap_or("")
                    .to_string();
                let status = match assertion.get("status").and_then(Value::as_str).unwrap_or("") {
                    "passed" => TestStatus::Passed,
                    "failed" => TestStatus::Failed,
                    _ => TestStatus::Skipped,
                };
                let message = assertion
                    .get("failureMessages")
                    .and_then(Value::as_array)
                    .map(|values| values.iter().filter_map(Value::as_str).collect::<Vec<_>>().join("\n"))
                    .unwrap_or_default();
                tests.push(TestCaseOutcome { name, suite: suite.clone(), status, duration_ms: assertion.get("duration").and_then(Value::as_f64), message });
            }
        }
    }
    TestOutcome::finish(Runner::Npx, tests, output)
}

/// 🐍️ pytest's verbose terminal report: `<nodeid> <VERDICT>` lines plus its failure sections.
pub fn parse_pytest(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim_end();
        let Some((node, tail)) = trimmed.split_once(' ') else { continue };
        if !node.contains("::") {
            continue;
        }
        let verdict = tail.split_whitespace().next().unwrap_or("");
        let status = match verdict {
            "PASSED" | "XPASS" => TestStatus::Passed,
            "FAILED" | "ERROR" => TestStatus::Failed,
            "SKIPPED" | "XFAIL" => TestStatus::Skipped,
            _ => continue,
        };
        let (suite, name) = node.split_once("::").unwrap_or(("", node));
        tests.push(TestCaseOutcome { name: name.to_string(), suite: suite.to_string(), status, duration_ms: None, message: String::new() });
    }
    TestOutcome::finish(Runner::Pytest, tests, output)
}

/// 🦀️ libtest's text report: `test <path> ... ok|FAILED|ignored`, with `---- <path> stdout ----` bodies.
pub fn parse_cargo_test(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim();
        let Some(rest) = trimmed.strip_prefix("test ") else { continue };
        let Some((name, verdict)) = rest.split_once(" ... ") else { continue };
        let status = match verdict.trim() {
            "ok" => TestStatus::Passed,
            "FAILED" => TestStatus::Failed,
            value if value.starts_with("ignored") => TestStatus::Skipped,
            _ => continue,
        };
        let message = if status == TestStatus::Failed { cargo_failure_body(&output.stdout, name.trim()) } else { String::new() };
        tests.push(TestCaseOutcome { name: name.trim().to_string(), suite: String::new(), status, duration_ms: None, message });
    }
    TestOutcome::finish(Runner::Cargo, tests, output)
}

/// 📖️ cargo-nextest's report: `        PASS [   0.012s] <crate> <test>`.
pub fn parse_cargo_nextest(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines().chain(output.stderr.lines()) {
        let trimmed = line.trim();
        let (verdict, rest) = match trimmed.split_once(' ') {
            Some(parts) => parts,
            None => continue,
        };
        let status = match verdict {
            "PASS" => TestStatus::Passed,
            "FAIL" | "TRY" => TestStatus::Failed,
            "SKIP" => TestStatus::Skipped,
            _ => continue,
        };
        let rest = rest.trim();
        let Some(close) = rest.find(']') else { continue };
        let duration_ms = rest[..close].trim_start_matches('[').trim().trim_end_matches('s').trim().parse::<f64>().ok().map(|seconds| seconds * 1000.0);
        let identifier = rest[close + 1..].trim();
        let (suite, name) = identifier.split_once(' ').unwrap_or(("", identifier));
        tests.push(TestCaseOutcome { name: name.trim().to_string(), suite: suite.trim().to_string(), status, duration_ms, message: String::new() });
    }
    TestOutcome::finish(Runner::CargoNextest, tests, output)
}

/// 🔷️ `dotnet test`'s console logger: `  Passed <name> [1 ms]`.
pub fn parse_dotnet_test(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    for line in output.stdout.lines() {
        let trimmed = line.trim();
        let (verdict, rest) = match trimmed.split_once(' ') {
            Some(parts) => parts,
            None => continue,
        };
        let status = match verdict {
            "Passed" => TestStatus::Passed,
            "Failed" => TestStatus::Failed,
            "Skipped" => TestStatus::Skipped,
            _ => continue,
        };
        let rest = rest.trim();
        if rest.starts_with('!') || rest.starts_with('-') || rest.is_empty() {
            continue;
        }
        let (name, duration_ms) = match rest.rsplit_once(" [") {
            Some((name, tail)) => (name.trim().to_string(), parse_dotnet_duration(tail.trim_end_matches(']'))),
            None => (rest.to_string(), None),
        };
        tests.push(TestCaseOutcome { name, suite: String::new(), status, duration_ms, message: String::new() });
    }
    TestOutcome::finish(Runner::Dotnet, tests, output)
}

/// 💎️ RSpec's `--format json` document.
pub fn parse_rspec_json(output: &ProcessOutput) -> TestOutcome {
    let mut tests: Vec<TestCaseOutcome> = Vec::new();
    if let Ok(Value::Object(report)) = serde_json::from_str::<Value>(json_body(&output.stdout)) {
        for example in report.get("examples").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default() {
            let name = example
                .get("full_description")
                .and_then(Value::as_str)
                .or_else(|| example.get("description").and_then(Value::as_str))
                .unwrap_or("")
                .to_string();
            let status = match example.get("status").and_then(Value::as_str).unwrap_or("") {
                "passed" => TestStatus::Passed,
                "failed" => TestStatus::Failed,
                _ => TestStatus::Skipped,
            };
            let message = example.get("exception").and_then(|value| value.get("message")).and_then(Value::as_str).unwrap_or("").to_string();
            tests.push(TestCaseOutcome {
                name,
                suite: example.get("file_path").and_then(Value::as_str).unwrap_or("").to_string(),
                status,
                duration_ms: example.get("run_time").and_then(Value::as_f64).map(|seconds| seconds * 1000.0),
                message,
            });
        }
    }
    TestOutcome::finish(Runner::Rspec, tests, output)
}

/// 🧹️ The JSON document inside output that may carry leading runner chatter.
fn json_body(text: &str) -> &str {
    let start = text.find('{').unwrap_or(0);
    let end = text.rfind('}').map_or(text.len(), |index| index + 1);
    if start < end {
        &text[start..end]
    } else {
        text
    }
}

fn parse_dotnet_duration(value: &str) -> Option<f64> {
    let value = value.trim();
    let (number, unit) = value.split_once(' ')?;
    let number: f64 = number.parse().ok()?;
    match unit {
        "ms" => Some(number),
        "s" => Some(number * 1000.0),
        "us" => Some(number / 1000.0),
        _ => None,
    }
}

fn cargo_failure_body(stdout: &str, name: &str) -> String {
    let header = format!("---- {name} stdout ----");
    let Some(index) = stdout.find(&header) else { return String::new() };
    let rest = &stdout[index + header.len()..];
    let end = [rest.find("\n----"), rest.find("\nfailures:\n")].into_iter().flatten().min().unwrap_or(rest.len());
    rest[..end].trim().to_string()
}
//#endregion 📊️Parsing
