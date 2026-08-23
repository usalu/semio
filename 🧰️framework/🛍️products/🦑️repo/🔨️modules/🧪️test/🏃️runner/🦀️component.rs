//! 🏃️ Rust native host runner. A generated cache-local crate links the committed
//! `🦀️component.rs` adapter and calls [`run_main`]; nothing else about the Rust host is generated.

use crate::protocol::{digest, parse_json, Json, Outcome, Plan, Scenario};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

//#region 🔖️Adapter
/// 🧭️ Everything one scenario handler is given. Fixtures are immutable; mutation happens on copies.
pub struct Context<'a> {
    pub plan: &'a Plan,
    pub scenario: &'a Scenario,
    pub role: &'a str,
    pub repo_root: PathBuf,
    pub work_dir: PathBuf,
}

impl<'a> Context<'a> {
    /// 🧫️ Absolute path of a resolved fixture.
    pub fn fixture(&self, uri: &str) -> Result<PathBuf, String> {
        self.plan.fixture(uri).map(|relative| self.repo_root.join(relative))
    }

    /// 🧫️ Bytes of a resolved fixture.
    pub fn fixture_bytes(&self, uri: &str) -> Result<Vec<u8>, String> {
        std::fs::read(self.fixture(uri)?).map_err(|error| error.to_string())
    }

    /// 🧫️ Copies an immutable fixture into the work directory and returns the mutable copy's path.
    pub fn copy_fixture(&self, uri: &str, as_name: Option<&str>) -> Result<PathBuf, String> {
        let source = self.fixture(uri)?;
        let name = as_name.map(|value| value.to_string()).unwrap_or_else(|| source.file_name().map(|value| value.to_string_lossy().to_string()).unwrap_or_else(|| "fixture".to_string()));
        let target = self.work_dir.join(name);
        std::fs::create_dir_all(&self.work_dir).map_err(|error| error.to_string())?;
        std::fs::copy(&source, &target).map_err(|error| error.to_string())?;
        Ok(target)
    }

    /// 🎲️ Deterministic seed declared by the scenario's `@seed-…` tag.
    pub fn seed(&self) -> u64 {
        self.scenario.seed.parse().unwrap_or(0)
    }
}

type Handler = Box<dyn Fn(&Context) -> Result<Outcome, String>>;

/// 🧭️ One implementation's registration for a case: which scenarios it serves, in which roles.
pub struct Adapter {
    pub implementation: &'static str,
    handlers: BTreeMap<(String, String), Handler>,
}

impl Adapter {
    /// 🧭️ Starts a registration for the given implementation id (`rust`).
    pub fn new(implementation: &'static str) -> Adapter {
        Adapter { implementation, handlers: BTreeMap::new() }
    }

    /// 🔮️ Registers the reference-implementation handler for one scenario.
    pub fn oracle<F>(mut self, scenario: &str, handler: F) -> Adapter
    where
        F: Fn(&Context) -> Result<Outcome, String> + 'static,
    {
        self.handlers.insert((scenario.to_string(), "oracle".to_string()), Box::new(handler));
        self
    }

    /// 🎯️ Registers this repository's handler for one scenario.
    pub fn subject<F>(mut self, scenario: &str, handler: F) -> Adapter
    where
        F: Fn(&Context) -> Result<Outcome, String> + 'static,
    {
        self.handlers.insert((scenario.to_string(), "subject".to_string()), Box::new(handler));
        self
    }

    fn handler(&self, scenario: &str, role: &str) -> Option<&Handler> {
        self.handlers.get(&(scenario.to_string(), role.to_string()))
    }

    /// 🧾️ Scenario ids this adapter registered, for the coordinator's registration check.
    pub fn registered(&self, role: &str) -> Vec<String> {
        self.handlers.keys().filter(|(_, entry_role)| entry_role == role).map(|(id, _)| id.clone()).collect()
    }
}
//#endregion 🔖️Adapter

//#region 🔖️Runner
fn arg(argv: &[String], flag: &str) -> Option<String> {
    argv.iter().position(|value| value == flag).and_then(|index| argv.get(index + 1)).cloned()
}

fn repo_root_from(work_dir: &str) -> PathBuf {
    let mut dir = Path::new(work_dir).to_path_buf();
    for _ in 0..32 {
        if dir.join("nx.json").exists() && dir.join("package.json").exists() {
            return dir;
        }
        match dir.parent() {
            Some(parent) => dir = parent.to_path_buf(),
            None => break,
        }
    }
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn result_json(plan: &Plan, scenario: &Scenario, status: &str, duration_ms: u128, outcome: Option<&Outcome>, diagnostics: Vec<(String, String)>, raw_path: Option<String>, projection_path: Option<String>) -> Json {
    let projection = outcome.map(|value| value.projection.clone()).unwrap_or(Json::Null);
    let raw_hash = outcome.and_then(|value| value.raw.as_ref()).map(|bytes| digest(bytes)).unwrap_or_else(|| digest(b""));
    let mut output = vec![("rawHash".to_string(), Json::String(raw_hash)), ("projectionHash".to_string(), Json::String(digest(projection.to_string().as_bytes()))), ("projection".to_string(), projection)];
    if let Some(path) = raw_path {
        output.push(("rawPath".to_string(), Json::String(path)));
    }
    if let Some(path) = projection_path {
        output.push(("projectionPath".to_string(), Json::String(path)));
    }
    Json::Object(vec![
        ("testId".to_string(), Json::String(format!("{}::{}::{}::{}::{}", plan.owner, plan.case, scenario.id, plan.implementation, plan.role))),
        ("owner".to_string(), Json::String(plan.owner.clone())),
        ("case".to_string(), Json::String(plan.case.clone())),
        ("scenario".to_string(), Json::String(scenario.id.clone())),
        ("implementation".to_string(), Json::String(plan.implementation.clone())),
        ("role".to_string(), Json::String(plan.role.clone())),
        ("level".to_string(), Json::String(scenario.level.clone())),
        ("status".to_string(), Json::String(status.to_string())),
        ("durationMs".to_string(), Json::Number(duration_ms as f64)),
        ("seed".to_string(), Json::String(scenario.seed.clone())),
        ("featureHash".to_string(), Json::String(plan.feature_hash.clone())),
        ("output".to_string(), Json::Object(output)),
        ("diagnostics".to_string(), Json::Array(diagnostics.into_iter().map(|(severity, message)| Json::Object(vec![("severity".to_string(), Json::String(severity)), ("message".to_string(), Json::String(message))])).collect())),
    ])
}

/// 🚪️ Rust host entry: load the plan, execute every planned scenario against the adapter, emit JSONL.
/// A missing registration, a panic and an error are all *results* — never a silent skip.
pub fn run_main(adapter: Adapter) -> std::process::ExitCode {
    let argv: Vec<String> = std::env::args().collect();
    let plan_path = match arg(&argv, "--plan") {
        Some(value) => value,
        None => {
            eprintln!("usage: host --plan <plan.json> --out <results.jsonl>");
            return std::process::ExitCode::from(2);
        }
    };
    let out_path = match arg(&argv, "--out") {
        Some(value) => value,
        None => {
            eprintln!("usage: host --plan <plan.json> --out <results.jsonl>");
            return std::process::ExitCode::from(2);
        }
    };
    let source = match std::fs::read_to_string(&plan_path) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("cannot read plan {}: {}", plan_path, error);
            return std::process::ExitCode::from(2);
        }
    };
    let plan = match parse_json(&source) {
        Ok(value) => Plan::from_json(&value),
        Err(error) => {
            eprintln!("malformed plan {}: {}", plan_path, error);
            return std::process::ExitCode::from(2);
        }
    };
    let repo_root = repo_root_from(&plan.work_dir);
    let work_dir = PathBuf::from(&plan.work_dir);
    let _ = std::fs::create_dir_all(&work_dir);
    let _ = std::fs::create_dir_all(&plan.output_dir);

    let mut lines: Vec<String> = Vec::new();
    let mut failed = false;
    for scenario in &plan.scenarios {
        let started = std::time::Instant::now();
        let context = Context { plan: &plan, scenario, role: &plan.role, repo_root: repo_root.clone(), work_dir: work_dir.clone() };
        match adapter.handler(&scenario.id, &plan.role) {
            None => {
                failed = true;
                lines.push(result_json(&plan, scenario, "errored", started.elapsed().as_millis(), None, vec![("error".to_string(), format!("adapter has no {} registration for scenario {}", plan.role, scenario.id))], None, None).to_string());
            }
            Some(handler) => match handler(&context) {
                Ok(outcome) => {
                    let raw_path = outcome.raw.as_ref().map(|bytes| {
                        let path = Path::new(&plan.output_dir).join(format!("{}.{}.raw", scenario.id, plan.role));
                        let _ = std::fs::write(&path, bytes);
                        path.to_string_lossy().to_string()
                    });
                    let projection_path = {
                        let path = Path::new(&plan.output_dir).join(format!("{}.{}.projection.json", scenario.id, plan.role));
                        let _ = std::fs::write(&path, outcome.projection.to_string());
                        Some(path.to_string_lossy().to_string())
                    };
                    let diagnostics = outcome.diagnostics.clone();
                    lines.push(result_json(&plan, scenario, "passed", started.elapsed().as_millis(), Some(&outcome), diagnostics, raw_path, projection_path).to_string());
                }
                Err(message) => {
                    failed = true;
                    lines.push(result_json(&plan, scenario, "failed", started.elapsed().as_millis(), None, vec![("error".to_string(), message)], None, None).to_string());
                }
            },
        }
    }

    let body = if lines.is_empty() { String::new() } else { format!("{}\n", lines.join("\n")) };
    if let Some(parent) = Path::new(&out_path).parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(error) = std::fs::write(&out_path, body) {
        eprintln!("cannot write results {}: {}", out_path, error);
        return std::process::ExitCode::from(2);
    }
    if failed {
        std::process::ExitCode::from(1)
    } else {
        std::process::ExitCode::SUCCESS
    }
}
//#endregion 🔖️Runner
