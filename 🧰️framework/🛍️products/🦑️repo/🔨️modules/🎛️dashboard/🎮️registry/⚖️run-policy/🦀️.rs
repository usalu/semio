//! ⚖️ Run policy of the registry: the graph reuse variable, the run policy, the file fingerprint and the graph basis. See [`super`].

use super::*;

// #region 🔖️RunPolicy
/// 🧵️ The variable that lets the build runner start a task from its published project graph instead of rebuilding it.
pub const GRAPH_REUSE: (&str, &str) = ("SEMIO_BUILD_REUSE_GRAPH", "true");

/// 🧱️ The workspace files besides project manifests that define the project graph.
pub const GRAPH_OWNERS: &[&str] = &["package.json"];

/// ⚙️ What the dashboard decides for every process it starts, in one place.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunPolicy {
    /// ♻️ Finite build tasks start from the published project graph.
    pub reuse_published_graph: bool,
    /// 🏭️ Which implementation answers the repo-domain commands.
    pub repo: RepoImplementation,
}

impl Default for RunPolicy {
    fn default() -> Self { Self { reuse_published_graph: false, repo: RepoImplementation::Rust } }
}

impl RunPolicy {
    /// 🌍️ The policy the environment states: `SEMIO_REPO_IMPLEMENTATION` picks the repo implementation.
    pub fn current() -> Self { Self { repo: dashboard_repo_domain::repo_implementation(), ..Self::default() } }

    pub(super) fn apply(self, env: &mut Vec<(String, String)>, build: bool, long_running: bool) {
        for (key, value) in RUNNER_ENV { set_env(env, key, value); }
        if build && !long_running && self.reuse_published_graph { set_env(env, GRAPH_REUSE.0, GRAPH_REUSE.1); }
    }
}

pub(super) fn set_env(env: &mut Vec<(String, String)>, key: &str, value: &str) {
    match env.iter_mut().find(|(name, _)| name == key) { Some(pair) => pair.1 = value.to_string(), None => env.push((key.to_string(), value.to_string())) }
}

/// 🔏️ The length and modification instant of a file: equal fingerprints mean an unchanged file,
/// whatever the clock did in between.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Fingerprint { pub len: u64, pub modified: u64 }

impl Fingerprint {
    /// 👆️ Reads the fingerprint of a path; a missing path has none.
    pub fn of(path: &Path) -> Option<Self> {
        let metadata = std::fs::metadata(path).ok()?;
        Some(Self { len: if metadata.is_dir() { 0 } else { metadata.len() }, modified: metadata.modified().ok()?.duration_since(std::time::UNIX_EPOCH).ok()?.as_nanos() as u64 })
    }
}

/// 📸️ A neutral graph publication and the content hashes of its manifest inputs.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphBasis { pub graph: Fingerprint, pub hashes: BTreeMap<String, u64> }

impl GraphBasis {
    /// 📖️ Captures inputs only when the graph publication is newer than every source.
    pub fn read(root: &Path, files: &[String]) -> Option<Self> {
        let graph = Fingerprint::of(&root.join(graph_file()))?;
        let mut hashes = BTreeMap::new();
        for file in files {
            let path = root.join(file);
            if Fingerprint::of(&path)?.modified > graph.modified { return None; }
            hashes.insert(file.clone(), xxh3_64(&std::fs::read(path).ok()?));
        }
        (Fingerprint::of(&root.join(graph_file())) == Some(graph)).then_some(Self { graph, hashes })
    }
}
// #endregion 🔖️RunPolicy
