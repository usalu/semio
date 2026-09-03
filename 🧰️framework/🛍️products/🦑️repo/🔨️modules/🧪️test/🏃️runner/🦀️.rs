//! 🏃️ Rust native host runner. A generated cache-local crate links the committed
//! `../🏃️🪻️runner/🦀️.rs` adapter and calls [`run_main`]; nothing else about the Rust host is generated.

use crate::protocol::{digest, parse_json, sha256_hex, Json, Outcome, Plan, Scenario};
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
    /// 📦️ Where this handler writes its produced artifact bundle.
    pub artifact_dir: PathBuf,
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

    /// 📥️ Bytes a subject host produced for an oracle that declares `@oracle-input-subject-raw`.
    pub fn subject_raw_bytes(&self, implementation: &str) -> Result<Vec<u8>, String> {
        let path = self
            .plan
            .subject_raw_inputs
            .iter()
            .find(|(id, _)| id == implementation)
            .map(|(_, path)| path)
            .ok_or_else(|| format!("scenario {} has no raw subject output from {implementation}; run its subject phase before this byte-decoding oracle", self.scenario.id))?;
        std::fs::read(path).map_err(|error| format!("cannot read raw subject output {path}: {error}"))
    }

    /// 🧫️ A resolved fixture parsed as JSON. Specification-vector cases — the ones resting on a
    /// recorded no-oracle decision — carry their vectors as committed JSON beside the implementation,
    /// and without this they had to be hand-transcribed into Rust literals, which is both laborious
    /// and a place for the transcription to drift away from the fixture it claims to mirror.
    pub fn fixture_json(&self, uri: &str) -> Result<crate::protocol::Json, String> {
        let bytes = self.fixture_bytes(uri)?;
        let text = String::from_utf8(bytes).map_err(|error| format!("fixture {} is not UTF-8: {}", uri, error))?;
        crate::protocol::parse_json(&text).map_err(|error| format!("fixture {} is not valid JSON: {}", uri, error))
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

    /// 📜️ The scenario's first doc string — the feature-owned input vector.
    pub fn doc_string(&self) -> Result<&str, String> {
        self.scenario.doc_strings.first().map(String::as_str).ok_or_else(|| format!("scenario {} carries no doc string", self.scenario.id))
    }

    /// 📜️ The scenario's first doc string, parsed as the owned JSON value.
    pub fn doc_json(&self) -> Result<crate::protocol::Json, String> {
        crate::protocol::parse_json(self.doc_string()?)
    }

    /// 📊️ The scenario's first data table — header row first.
    pub fn data_table(&self) -> Result<&Vec<Vec<String>>, String> {
        self.scenario.data_tables.first().ok_or_else(|| format!("scenario {} carries no data table", self.scenario.id))
    }

    /// 📦️ Absolute path to write one named result artifact to, creating parent directories.
    pub fn artifact(&self, role: &str, filename: &str) -> Result<PathBuf, String> {
        let dir = self.artifact_dir.join(role);
        std::fs::create_dir_all(&dir).map_err(|error| format!("cannot create artifact directory {}: {error}", dir.display()))?;
        Ok(dir.join(filename))
    }

    /// 🪆️ The smallest owning subset this case is scoped to, or an error when it has none — a handler
    /// that needs a scope must not invent one.
    pub fn target(&self) -> Result<&crate::protocol::SubsetTarget, String> {
        self.plan.target.as_ref().ok_or_else(|| format!("case {} declares no subset target — Protocol v2 scopes every mutation case to its smallest owning subset", self.plan.case))
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
    // 📦️Every produced file is re-hashed HERE rather than trusted from the handler: the digest a
    // comparison stage keys on must describe the bytes that actually reached disk.
    let artifacts: Vec<Json> = outcome
        .map(|value| {
            value
                .artifacts
                .iter()
                .map(|artifact| {
                    let bytes = std::fs::read(&artifact.path).unwrap_or_default();
                    Json::Object(vec![
                        ("role".to_string(), Json::String(artifact.role.clone())),
                        ("path".to_string(), Json::String(artifact.path.clone())),
                        ("mediaType".to_string(), Json::String(artifact.media_type.clone())),
                        ("sha256".to_string(), Json::String(format!("sha256:{}", sha256_hex(&bytes)))),
                        ("bytes".to_string(), Json::Number(bytes.len() as f64)),
                    ])
                })
                .collect()
        })
        .unwrap_or_default();
    if let Some(path) = raw_path {
        output.push(("rawPath".to_string(), Json::String(path)));
    }
    if let Some(path) = projection_path {
        output.push(("projectionPath".to_string(), Json::String(path)));
    }
    let mut record = vec![
        ("schemaVersion".to_string(), Json::Number(2.0)),
        ("testId".to_string(), Json::String(format!("{}::{}::{}::{}::{}", plan.owner, plan.case, scenario.id, plan.implementation, plan.role))),
        ("baselineSha".to_string(), Json::String(plan.baseline_sha.clone())),
        ("owner".to_string(), Json::String(plan.owner.clone())),
        ("case".to_string(), Json::String(plan.case.clone())),
        ("scenario".to_string(), Json::String(scenario.id.clone())),
        ("implementation".to_string(), Json::String(plan.implementation.clone())),
        ("role".to_string(), Json::String(plan.role.clone())),
        ("level".to_string(), Json::String(scenario.level.clone())),
        ("platform".to_string(), Json::String(plan.platform.clone())),
        ("status".to_string(), Json::String(status.to_string())),
        ("durationMs".to_string(), Json::Number(duration_ms as f64)),
        ("seed".to_string(), Json::String(scenario.seed.clone())),
        ("featureHash".to_string(), Json::String(plan.feature_hash.clone())),
        ("artifacts".to_string(), Json::Array(artifacts)),
        ("output".to_string(), Json::Object(output)),
        ("diagnostics".to_string(), Json::Array(diagnostics.into_iter().map(|(severity, message)| Json::Object(vec![("severity".to_string(), Json::String(severity)), ("message".to_string(), Json::String(message))])).collect())),
    ];
    // 🏭️Only a handler that actually reached production dispatch gets this field. Emitting it
    // unconditionally would make every replaying adapter look like a real subject.
    if let Some(dispatch) = outcome.and_then(|value| value.production_dispatch.as_ref()) {
        record.push((
            "productionDispatch".to_string(),
            Json::Object(vec![("invoked".to_string(), Json::Bool(true)), ("operation".to_string(), Json::String(dispatch.operation.clone())), ("bridgeVersion".to_string(), Json::Number(dispatch.bridge_version as f64))]),
        ));
    }
    Json::Object(record)
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
    let artifact_dir = if plan.artifact_dir.is_empty() { work_dir.join("📦️artifacts") } else { PathBuf::from(&plan.artifact_dir) };
    let _ = std::fs::create_dir_all(&work_dir);
    let _ = std::fs::create_dir_all(&plan.output_dir);
    let _ = std::fs::create_dir_all(&artifact_dir);

    let mut lines: Vec<String> = Vec::new();
    let mut failed = false;
    for scenario in &plan.scenarios {
        let started = std::time::Instant::now();
        let context = Context { plan: &plan, scenario, role: &plan.role, repo_root: repo_root.clone(), work_dir: work_dir.clone(), artifact_dir: artifact_dir.clone() };
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
