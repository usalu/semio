//! 🧹️ The purge-generated law, Rust side: real files in a throwaway git repository reproduce the plan
//! that the independent Python model froze for every case, a dry run changes nothing and an apply
//! removes exactly the planned paths.
//!
//! @see ../🧫️fixtures/🧹️purge-generated/🧹️cases.json
//! @see ./🐍️.py

use super::*;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    path: String,
    kind: String,
    #[serde(default)]
    bytes: u64,
    #[serde(default)]
    age_seconds: i64,
    #[serde(default)]
    tracked: bool,
    #[serde(default)]
    target: String,
}

#[derive(Deserialize)]
struct FixtureTicket {
    id: String,
    manifest: String,
    entries: Vec<Entry>,
}

#[derive(Deserialize)]
struct Request {
    scope: String,
    #[serde(default)]
    ticket: String,
    selection: String,
}

#[derive(Deserialize)]
struct Case {
    id: String,
    git: bool,
    request: Request,
    tickets: Vec<FixtureTicket>,
    expect: PurgePlan,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Policy {
    generated_directory: String,
    oversized_file_bytes: u64,
    oversized_folder_bytes: u64,
    live_writer_seconds: u64,
}

#[derive(Deserialize)]
struct Fixture {
    policy: Policy,
    cases: Vec<Case>,
}

fn slash_relative(base: &Path, path: &Path) -> String {
    path.strip_prefix(base).unwrap_or(path).components().map(|component| component.as_os_str().to_string_lossy().into_owned()).collect::<Vec<_>>().join("/")
}

struct World {
    root: PathBuf,
    tickets_dir: PathBuf,
    external: PathBuf,
}

fn fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🧪️tests/🧫️fixtures/🧹️purge-generated/🧹️cases.json");
    serde_json::from_str(&std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))).expect("the purge fixture parses")
}

fn scratch() -> PathBuf {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).map(|elapsed| elapsed.as_nanos()).unwrap_or_default();
    let directory = std::env::temp_dir().join(format!("purge-generated-{}-{nanos}-{}", std::process::id(), NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)));
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

fn git(root: &Path, stdin: &[u8], arguments: &[&str]) {
    use std::io::Write;
    let mut child = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .env_clear()
        .envs(std::env::vars_os().filter(|(name, _)| !name.to_string_lossy().to_ascii_uppercase().starts_with("GIT_")))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("git starts");
    child.stdin.take().unwrap().write_all(stdin).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "git {arguments:?}: {}", String::from_utf8_lossy(&output.stderr));
}

fn write_file(path: &Path, size: u64, modified: SystemTime) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let file = std::fs::File::create(path).unwrap();
    file.set_len(size).unwrap();
    file.set_modified(modified).unwrap();
}

fn shifted(now: SystemTime, age_seconds: i64) -> SystemTime {
    if age_seconds >= 0 { now - Duration::from_secs(age_seconds as u64) } else { now + Duration::from_secs(age_seconds.unsigned_abs()) }
}

#[cfg(unix)]
fn link(target: &Path, path: &Path) -> bool {
    std::os::unix::fs::symlink(target, path).is_ok()
}

#[cfg(not(unix))]
fn link(_target: &Path, _path: &Path) -> bool {
    false
}

fn materialize(scratch: &Path, case: &Case, now: SystemTime) -> Option<World> {
    let root = scratch.join(format!("repo-{}", case.id));
    let external = scratch.join(format!("external-{}", case.id));
    std::fs::create_dir_all(&root).unwrap();
    write_file(&external.join("blob.bin"), 11 << 20, shifted(now, 86_400));
    if case.git {
        git(&root, b"", &["init", "-q"]);
    }
    let tickets_dir = root.join(".🧬semio").join("🦑️repo").join("🎫️tickets");
    let mut tracked: Vec<String> = Vec::new();
    for ticket in &case.tickets {
        let directory = ticket.id.split('/').fold(tickets_dir.clone(), |path, part| path.join(part));
        std::fs::create_dir_all(&directory).unwrap();
        let manifest = match ticket.manifest.as_str() {
            "open" => Some(r#"{"status":"open"}"#),
            "closed" => Some(r#"{"status":"closed"}"#),
            "closing" => Some(r#"{"status":"closing"}"#),
            "invalid" => Some("{"),
            _ => None,
        };
        if let Some(text) = manifest {
            std::fs::write(directory.join(TICKET_DOCUMENT_NAME), text).unwrap();
        }
        for entry in &ticket.entries {
            let path = entry.path.split('/').fold(directory.clone(), |path, part| path.join(part));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            match entry.kind.as_str() {
                "directory" => std::fs::create_dir_all(&path).unwrap(),
                "symlink" => {
                    let target = entry.target.strip_prefix("external").unwrap_or_default().trim_start_matches('/');
                    if !link(&external.join(target), &path) {
                        return None;
                    }
                }
                _ => {
                    write_file(&path, entry.bytes, shifted(now, entry.age_seconds));
                    if entry.tracked {
                        tracked.push(slash_relative(&root, &path));
                    }
                }
            }
        }
    }
    if !tracked.is_empty() {
        git(&root, tracked.join("\0").as_bytes(), &["add", "-f", "--pathspec-from-file=-", "--pathspec-file-nul"]);
    }
    Some(World { root, tickets_dir, external })
}

fn tree(root: &Path) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory).unwrap().flatten() {
            let path = entry.path();
            let relative = slash_relative(root, &path);
            if relative == ".git" {
                continue;
            }
            let metadata = std::fs::symlink_metadata(&path).unwrap();
            let state = if metadata.is_file() { format!("file|{}|{:?}", metadata.len(), metadata.modified().ok()) } else { format!("{:?}", metadata.file_type()) };
            found.insert(relative, state);
            if metadata.is_dir() {
                pending.push(path);
            }
        }
    }
    found
}

fn run(case: &Case, world: &World, now: SystemTime, apply: bool) -> PurgePlan {
    let options = PurgeOptions {
        root: world.root.clone(),
        tickets_dir: world.tickets_dir.clone(),
        selection: if case.request.selection == "generated" { PurgeSelection::Generated } else { PurgeSelection::Oversized },
        apply,
        now,
        tracked: &GitTrackedFiles,
    };
    if case.request.scope == "ticket" {
        let directory = case.request.ticket.split('/').fold(world.tickets_dir.clone(), |path, part| path.join(part));
        return purge_ticket_generated_output(&options, &[directory]).unwrap();
    }
    purge_closed_ticket_generated_output(&options).unwrap()
}

#[test]
fn the_policy_matches_the_schema_fixture() {
    let fixture = fixture();
    assert_eq!(fixture.policy.generated_directory, TICKET_GENERATED_DIRECTORY);
    assert_eq!(fixture.policy.oversized_file_bytes, OVERSIZED_FILE_BYTES);
    assert_eq!(fixture.policy.oversized_folder_bytes, OVERSIZED_FOLDER_BYTES);
    assert_eq!(Duration::from_secs(fixture.policy.live_writer_seconds), LIVE_WRITER_WINDOW);
    assert!(!fixture.cases.is_empty());
}

#[test]
fn every_frozen_plan_is_reproduced_from_real_files() {
    let fixture = fixture();
    let scratch = scratch();
    let now = SystemTime::UNIX_EPOCH + Duration::from_secs(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs());
    for case in &fixture.cases {
        let Some(world) = materialize(&scratch, case, now) else { continue };
        let before = tree(&world.root);
        let external_before = tree(&world.external);
        assert_eq!(run(case, &world, now, false), case.expect, "dry-run plan of {}", case.id);
        assert_eq!(tree(&world.root), before, "a dry run changed the repository in {}", case.id);
        assert_eq!(run(case, &world, now, true), case.expect, "applied plan of {}", case.id);
        let mut remaining = before.clone();
        for removal in &case.expect.removals {
            let prefix = slash_relative(&world.root, &removal.path.split('/').fold(world.tickets_dir.clone(), |path, part| path.join(part)));
            remaining.retain(|path, _| *path != prefix && !path.starts_with(&format!("{prefix}/")));
        }
        assert_eq!(tree(&world.root), remaining, "apply removed or kept the wrong paths in {}", case.id);
        assert_eq!(tree(&world.external), external_before, "a purge reached through a link in {}", case.id);
    }
    let _ = std::fs::remove_dir_all(&scratch);
}

#[test]
fn the_default_selection_of_a_dry_run_changes_nothing() {
    let fixture = fixture();
    let scratch = scratch();
    let now = SystemTime::now();
    let case = &fixture.cases[2];
    let world = materialize(&scratch, case, now).unwrap();
    let before = tree(&world.root);
    purge_closed_ticket_generated_output(&PurgeOptions { root: world.root.clone(), tickets_dir: world.tickets_dir.clone(), selection: PurgeSelection::Oversized, apply: false, now, tracked: &GitTrackedFiles }).unwrap();
    assert_eq!(tree(&world.root), before);
    let _ = std::fs::remove_dir_all(&scratch);
}
