//! ⚖️ Law: the wgpu unit-test case directories are wired, and wired once.
//!
//! The wgpu suites are not `[[test]]` integration binaries — every `🧪️tests/🔬️*wgpu*` case
//! directory is mounted into a production source file with `#[cfg(test)] #[path = "…"] mod tests;`
//! or an `include!("…")` inside an existing `mod tests { … }`. That wiring is invisible to Cargo,
//! to the taxonomy-driven Nx test plugin, and to every reviewer who does not already know it
//! exists, which makes both of its failure modes silent:
//!
//! - a case directory nobody mounts is a test file that never compiles and never runs, and
//! - a mount whose target does not exist fails the WHOLE crate's `--tests` build with
//!   `couldn't read …: No such file or directory` — not a warning, not one lane, the whole crate.
//!   That is not hypothetical: it happened on 26/09/09 to `semio-framework-os-renderer-wgpu`, and
//!   was patched by committing an empty placeholder rather than by removing the dangling mount.
//!
//! Both are structural, so they are checked here instead of being rediscovered by a wedged build.

use std::fs;
use std::path::{Path, PathBuf};

/// 📂️ Repo root, five levels above `semio-framework-ui`'s manifest
/// (`🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust`).
fn repo_root() -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for _ in 0..5 {
        root = root.parent().expect("manifest dir has five ancestors up to the repo root").to_path_buf();
    }
    root
}

/// 🔎️ The two trees that own wgpu case directories: the ui module (its own `🎯️targets/🧊️wgpu` plus
/// every `🧱️elements/*/🎯️targets/🧊️wgpu`) and the os renderer engine.
const SCAN_ROOTS: &[&str] = &["🧰️framework/🔨️modules/🖱️ui", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer"];

/// 🚫️ Build output and vendored trees — never sources, and large enough to matter when walking.
const SKIP_DIRS: &[&str] = &["node_modules", "dist", "target", "🗑️generated"];

/// 🪧️ The two ways one Rust file is mounted into another: a `path` attribute, or a textual include.
const MOUNT_MARKERS: &[&str] = &["#[path", "include!"];

/// 🧪️ `🧪️tests/🔬️<something-wgpu-something>/🦀️.rs` — the case-file shape this law owns.
fn is_wgpu_case_dir(dir: &Path) -> bool {
    let name = dir.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    let parent = dir.parent().and_then(Path::file_name).and_then(|name| name.to_str()).unwrap_or_default();
    parent == "🧪️tests" && name.starts_with("🔬️") && name.contains("wgpu")
}

/// 🚶️ Collects every `.rs` source and every wgpu case file under one root in a single walk.
fn walk(dir: &Path, sources: &mut Vec<PathBuf>, cases: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_str().unwrap_or_default();
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            if !SKIP_DIRS.contains(&name) {
                walk(&path, sources, cases);
            }
        } else if name.ends_with(".rs") {
            if is_wgpu_case_dir(dir) && name == "🦀️.rs" {
                cases.push(path.clone());
            }
            sources.push(path);
        }
    }
}

/// 🔤️ The file literal of every module mount on one source line. Only literals naming a Rust file
/// count: the directory form (`path = "."`, which only re-roots an inline module's own children)
/// mounts nothing, and a first quote that belongs to `concat!`/`env!` rather than to the mount —
/// or to prose about mounts inside a string — names no file either.
fn mount_literals(line: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let mut cursor = 0usize;
    while cursor < line.len() {
        let tail = &line[cursor..];
        let Some((offset, marker)) = MOUNT_MARKERS.iter().filter_map(|marker| tail.find(marker).map(|offset| (offset, marker.len()))).min() else { break };
        let after = cursor + offset + marker;
        let Some(open) = line[after..].find('"').map(|index| after + index + 1) else { break };
        let Some(close) = line[open..].find('"').map(|index| open + index) else { break };
        if line[open..close].ends_with(".rs") {
            literals.push(line[open..close].to_string());
        }
        cursor = close + 1;
    }
    literals
}

/// 🧭️ Resolves one mount literal the way rustc does: relative to the source file's own directory
/// for a top-level `mod`, or relative to `<dir>/<file stem>` for one inside an inline `mod` block.
/// `None` means the mount points at nothing — the wedge shape.
fn resolve_mount(source: &Path, literal: &str) -> Option<PathBuf> {
    let dir = source.parent()?;
    let stem = source.file_stem().map(PathBuf::from).unwrap_or_default();
    [dir.join(literal), dir.join(stem).join(literal)].into_iter().find_map(|candidate| fs::canonicalize(candidate).ok())
}

/// 📇️ Every mount in the scan roots, as `(source file, literal, resolved target)`.
fn mounts(root: &Path, sources: &[PathBuf]) -> Vec<(String, String, Option<PathBuf>)> {
    let mut mounts = Vec::new();
    for source in sources {
        let Ok(text) = fs::read_to_string(source) else { continue };
        for line in text.lines() {
            // 💬️ Doc comments quote `#[path = "…"]` while explaining it; only real items count.
            if line.trim_start().starts_with("//") {
                continue;
            }
            for literal in mount_literals(line) {
                let relative = source.strip_prefix(root).unwrap_or(source).display().to_string();
                let resolved = resolve_mount(source, &literal);
                mounts.push((relative, literal, resolved));
            }
        }
    }
    mounts
}

/// 🗂️ One walk of both scan roots, shared by the two laws below.
fn scan() -> (PathBuf, Vec<PathBuf>, Vec<PathBuf>) {
    let root = repo_root();
    let (mut sources, mut cases) = (Vec::new(), Vec::new());
    for scan_root in SCAN_ROOTS {
        walk(&root.join(scan_root), &mut sources, &mut cases);
    }
    assert!(!cases.is_empty(), "no wgpu test case directories found under {SCAN_ROOTS:?} — the walk, not the repo, is wrong");
    (root, sources, cases)
}

/// ⚖️ Exactly one production mount per case directory: none means dead test code, two means the
/// same assertions run twice under two module paths.
#[test]
fn every_wgpu_case_directory_is_mounted_exactly_once() {
    let (root, sources, cases) = scan();
    let mounts = mounts(&root, &sources);
    let mut violations = Vec::new();
    for case in &cases {
        let canonical = fs::canonicalize(case).expect("case file exists");
        let mounted: Vec<&str> = mounts.iter().filter(|(_, _, resolved)| resolved.as_deref() == Some(canonical.as_path())).map(|(source, _, _)| source.as_str()).collect();
        if mounted.len() != 1 {
            violations.push(format!("{} is mounted {} times {:?}", case.strip_prefix(&root).unwrap_or(case).display(), mounted.len(), mounted));
        }
    }
    assert!(violations.is_empty(), "wgpu test case directories must be mounted exactly once by a `#[path]`/`include!` in a production source:\n{}", violations.join("\n"));
}

/// ⚖️ No mount points at a missing file. One dangling `#[path]` fails the crate's whole `--tests`
/// build, so it must fail here — where the message names the offender — instead of there.
#[test]
fn no_module_mount_in_the_wgpu_trees_is_dangling() {
    let (root, sources, _) = scan();
    let dangling: Vec<String> = mounts(&root, &sources).into_iter().filter(|(_, _, resolved)| resolved.is_none()).map(|(source, literal, _)| format!("{source} → {literal}")).collect();
    assert!(dangling.is_empty(), "a `#[path]`/`include!` whose target does not exist wedges the whole crate's test build:\n{}", dangling.join("\n"));
}
