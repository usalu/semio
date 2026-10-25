//! 💽️ Real world: the only layer that touches the machine: the filesystem walk and the spawning process runner.
//!
//! Holds the `💽️RealWorld` region of the crate root; the root re-exports its items.

use super::*;

//#region 💽️RealWorld
// 💽️ The only block in this crate that touches the machine: the walk that turns a real repository
// into a `FilesystemSnapshot`, the `ProcessRunner` that actually spawns, and the streaming spawn
// the CLI's `test` verb uses so a child's output reaches the terminal unbuffered.

/// 📖️ The manifest files planning probes for contents, by base name.
const SNAPSHOT_READ_NAMES: &[&str] = &["package.json"];

/// 📏️ A file larger than this is recorded as a path without contents.
const SNAPSHOT_READ_LIMIT: u64 = 1 << 20;

impl FilesystemSnapshot {
    /// 🌍️ Walks a real repository into the snapshot planning reads.
    ///
    /// Every non-ignored directory and file below `root` is recorded as a repository-relative,
    /// forward-slashed path. Contents are recorded only for the manifests a planning rule inspects
    /// ([`SNAPSHOT_READ_NAMES`]), so the walk stays a directory traversal rather than a full read;
    /// [`FilesystemSnapshot::hydrate`] loads any further file a narrower scope needs.
    pub fn from_root(root: &Path, ignore: Option<&semio_framework_repo_workspace::GitIgnore>) -> Self {
        let mut snapshot = FilesystemSnapshot { root: normalize_separators(&root.to_string_lossy()), uv_available: program_on_path("uv"), ..FilesystemSnapshot::default() };
        walk_into(root, root, ignore, &mut snapshot.entries);
        snapshot.entries.sort_by(|left, right| left.path.cmp(&right.path));
        snapshot
    }

    /// 💧️ Loads the contents of one already-recorded file from disk, so a scope that inspects a
    /// source file (a section or a definition run) sees the same text the reference read.
    pub fn hydrate(&mut self, path: &str) {
        let absolute = self.absolute(path);
        let Ok(contents) = std::fs::read_to_string(absolute.replace('/', std::path::MAIN_SEPARATOR_STR)) else { return };
        let wanted = clean_path(&absolute);
        match self.entries.iter_mut().find(|entry| entry.kind == EntryKind::File && clean_path(&entry.path) == wanted) {
            Some(entry) => entry.contents = Some(contents),
            None => self.entries.push(SnapshotEntry { path: wanted, kind: EntryKind::File, contents: Some(contents) }),
        }
    }

    /// 📦️ Replaces the bundle table planning resolves names and roots against.
    pub fn with_bundles(mut self, bundles: Vec<SnapshotBundle>) -> Self {
        self.bundles = bundles;
        self
    }

    /// 📸️ The snapshot planning actually reads: the bundle table plus one directory listing per
    /// bundle root.
    ///
    /// Every planning rule that touches the world probes a bundle root's own manifests
    /// (`go.mod`, `Cargo.toml`, `*.csproj`, `*.sln`, `package.json`, `pyproject.toml`,
    /// `requirements.txt`) or one named source file, which [`FilesystemSnapshot::hydrate`] loads on
    /// demand. Nothing walks the tree, so this is what the CLI's `test` verb builds — the reference
    /// stats those same manifests and never walks either.
    pub fn for_bundles(root: &Path, bundles: Vec<SnapshotBundle>) -> Self {
        let mut snapshot = FilesystemSnapshot { root: normalize_separators(&root.to_string_lossy()), bundles, uv_available: program_on_path("uv"), entries: Vec::new() };
        let roots: Vec<String> = snapshot.bundles.iter().map(|bundle| snapshot.absolute(&bundle.root)).collect();
        for absolute in roots {
            snapshot.entries.push(SnapshotEntry { path: absolute.clone(), kind: EntryKind::Directory, contents: None });
            let Ok(children) = std::fs::read_dir(absolute.replace('/', std::path::MAIN_SEPARATOR_STR)) else { continue };
            for child in children.flatten() {
                let Ok(kind) = child.file_type() else { continue };
                let path = normalize_separators(&child.path().to_string_lossy());
                if kind.is_dir() {
                    snapshot.entries.push(SnapshotEntry { path, kind: EntryKind::Directory, contents: None });
                    continue;
                }
                let contents = if SNAPSHOT_READ_NAMES.contains(&base_name(&path).as_str()) { std::fs::read_to_string(child.path()).ok() } else { None };
                snapshot.entries.push(SnapshotEntry { path, kind: EntryKind::File, contents });
            }
        }
        snapshot.entries.sort_by(|left, right| left.path.cmp(&right.path));
        snapshot.entries.dedup_by(|left, right| left.path == right.path);
        snapshot
    }
}

/// 🚶️ Records one directory level, descending into every directory the ignore rules keep.
fn walk_into(root: &Path, directory: &Path, ignore: Option<&semio_framework_repo_workspace::GitIgnore>, entries: &mut Vec<SnapshotEntry>) {
    let Ok(children) = std::fs::read_dir(directory) else { return };
    for child in children.flatten() {
        let path = child.path();
        let Ok(relative) = path.strip_prefix(root) else { continue };
        let relative = normalize_separators(&relative.to_string_lossy());
        if relative == ".git" || relative.ends_with("/.git") {
            continue;
        }
        if ignore.is_some_and(|rules| rules.matches_path(&relative)) {
            continue;
        }
        let absolute = normalize_separators(&path.to_string_lossy());
        let Ok(kind) = child.file_type() else { continue };
        if kind.is_symlink() {
            continue;
        }
        if kind.is_dir() {
            entries.push(SnapshotEntry { path: absolute, kind: EntryKind::Directory, contents: None });
            walk_into(root, &path, ignore, entries);
            continue;
        }
        let contents = if SNAPSHOT_READ_NAMES.contains(&base_name(&absolute).as_str()) && child.metadata().is_ok_and(|meta| meta.len() <= SNAPSHOT_READ_LIMIT) {
            std::fs::read_to_string(&path).ok()
        } else {
            None
        };
        entries.push(SnapshotEntry { path: absolute, kind: EntryKind::File, contents });
    }
}

/// 🔎️ Whether an executable of that name is reachable through `PATH` — Go `exec.LookPath`.
pub fn program_on_path(program: &str) -> bool {
    semio_framework_repo_workspace::program_exists(program)
}



/// 🏭️ The [`ProcessRunner`] that actually spawns, capturing both streams.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemProcessRunner;

impl ProcessRunner for SystemProcessRunner {
    fn run(&self, request: &ProcessRequest) -> Result<ProcessOutput, String> {
        let mut command = semio_framework_repo_workspace::spawn_command(&request.program);
        command.args(&request.args);
        if !request.cwd.is_empty() {
            command.current_dir(request.cwd.replace('/', std::path::MAIN_SEPARATOR_STR));
        }
        for (name, value) in &request.env {
            command.env(name, value);
        }
        let output = command.output().map_err(|error| format!("{}: {error}", request.program))?;
        Ok(ProcessOutput {
            status: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

/// ▶️ Spawns one planned invocation with the parent's streams, answering its exit code.
///
/// This is what the reference's `runExternalCommand` does: the child writes straight to the
/// terminal, so a long test run shows progress instead of arriving in one block at the end.
pub fn run_invocation_inherited(invocation: &RunnerInvocation) -> Result<i32, String> {
    let program = invocation.argv.first().cloned().unwrap_or_default();
    let mut command = semio_framework_repo_workspace::spawn_command(&program);
    command.args(invocation.argv.iter().skip(1));
    if !invocation.cwd.is_empty() {
        command.current_dir(invocation.cwd.replace('/', std::path::MAIN_SEPARATOR_STR));
    }
    for (name, value) in &invocation.env {
        command.env(name, value);
    }
    let status = command.status().map_err(|error| format!("{program}: {error}"))?;
    Ok(status.code().unwrap_or(-1))
}

/// 🏷️ The `Running: <argv> (in <dir>)` line the reference prints before every invocation.
pub fn running_line(invocation: &RunnerInvocation) -> String {
    format!("Running: {} (in {})\n", invocation.argv.join(" "), invocation.cwd.replace('/', std::path::MAIN_SEPARATOR_STR))
}
//#endregion 💽️RealWorld
