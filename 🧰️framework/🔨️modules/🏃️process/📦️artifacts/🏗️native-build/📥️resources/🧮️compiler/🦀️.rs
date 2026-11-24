//! 🧮️ Captures original proc-macro filesystem inputs and registers their genuine compiler dependencies.

use semio_framework_hash as content_hash;

use std::{cell::RefCell, fs, io, panic::{AssertUnwindSafe, catch_unwind, resume_unwind, Location}, path::{Path, PathBuf}, sync::atomic::{AtomicU64, Ordering}, time::{SystemTime, UNIX_EPOCH}};

struct Capture {
    directory: Option<PathBuf>,
    producer_manifest: PathBuf,
    producer_source: PathBuf,
    caller_manifest: PathBuf,
    caller_crate: String,
    caller_source: Option<PathBuf>,
    built_at: u128,
    complete: bool,
    rows: Vec<String>,
    track: fn(&Path),
}

thread_local! { static CURRENT: RefCell<Option<Capture>> = const { RefCell::new(None) }; }
static NEXT: AtomicU64 = AtomicU64::new(0);

fn quoted(value: &str) -> String {
    let mut output = String::from("\"");
    for character in value.chars() {
        match character {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            character if character < '\u{20}' => output.push_str(&format!("\\u{:04x}", character as u32)),
            character => output.push(character),
        }
    }
    output.push('"');
    output
}

fn absolute(path: impl AsRef<Path>) -> PathBuf { std::path::absolute(path).expect("absolute compiler resource path") }
fn path_text(path: &Path) -> String { quoted(path.to_str().expect("UTF-8 compiler resource path")) }
fn now() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).expect("compiler observation clock").as_millis() }
fn operation(source: &str, line: u32, name: &str) -> String { format!("{{\"source\":{},\"line\":{},\"name\":{}}}", path_text(&absolute(source)), line, quoted(name)) }
fn require_capture() { if !cfg!(test) && std::env::var_os("SEMIO_COMPILER_RESOURCE_ROOT").is_some() { CURRENT.with(|current| assert!(current.borrow().is_some(), "actual compiler read requires its entry capture")); } }
fn callsite(location: &Location<'_>) -> String { format!("{{\"source\":{},\"line\":{}}}", path_text(&absolute(location.file())), location.line()) }

/// 🚧️ The variable with which a remote worker names the real private input root of an offloaded compilation.
pub const INPUT_ROOT_ENV_V1: &str = "SEMIO_OFFLOAD_INPUT_ROOT";

/// 🚧️ The private input root of an offloaded compilation, or none when the compiler runs in the original worktree.
pub fn input_root_v1() -> Option<PathBuf> { std::env::var_os(INPUT_ROOT_ENV_V1).map(PathBuf::from) }

/// 🚧️ Whether a path may be read: always in the original worktree, inside the input root of an offloaded compilation. Everything above that root is another machine's tree or refused by the worker's sandbox (`EPERM`).
pub fn inside_input_root_v1(path: &Path) -> bool { inside_root_v1(path, input_root_v1().as_deref()) }

fn inside_root_v1(path: &Path, root: Option<&Path>) -> bool { root.is_none_or(|root| path.starts_with(root)) }

/// 🧭️ `path` as the `/`-separated path from the directory of `source` (for `include_str!` in the output of a proc-macro): relative, so the expansion is the same in every checkout root and offload sandbox.
pub fn include_path_v1(source: &Path, path: &Path) -> Result<String, String> {
    let from: Vec<_> = source.parent().ok_or("source has no directory")?.components().collect();
    let to: Vec<_> = path.components().collect();
    let common = from.iter().zip(&to).take_while(|(left, right)| left == right).count();
    Ok(from[common..].iter().map(|_| "..".into()).chain(to[common..].iter().map(|part| part.as_os_str().to_string_lossy().into_owned())).collect::<Vec<String>>().join("/"))
}

/// 🔁️ Runs a filesystem read again across the short window in which a replica daemon replaces a file (missing or refused for a moment), then fails with the path named. Other errors fail at once.
fn settled_v1<T>(path: &Path, mut operation: impl FnMut() -> io::Result<T>) -> io::Result<T> {
    for attempt in 1..=SETTLE_ATTEMPTS_V1 {
        match operation() {
            Err(error) if attempt < SETTLE_ATTEMPTS_V1 && matches!(error.kind(), io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied) => std::thread::sleep(std::time::Duration::from_millis(SETTLE_PAUSE_MILLISECONDS_V1 * u64::from(attempt))),
            Err(error) => return Err(io::Error::new(error.kind(), format!("{}: {error}", path.display()))),
            done => return done,
        }
    }
    unreachable!("the last attempt returns")
}

const SETTLE_ATTEMPTS_V1: u32 = 5;
const SETTLE_PAUSE_MILLISECONDS_V1: u64 = 40;

/// 🔎️ `symlink_metadata` that settles across an atomic replace and names the path on failure.
pub fn lstat_v1(path: impl AsRef<Path>) -> io::Result<fs::Metadata> { let path = path.as_ref(); settled_v1(path, || fs::symlink_metadata(path)) }

/// 🏭️ Records a completed actual proc-macro entry, including an explicit empty read roster.
pub fn with_compiler_resources_v1<R>(producer_manifest: &str, producer_source: &str, caller_source: Option<PathBuf>, root: Option<String>, track: fn(&Path), run: impl FnOnce() -> R) -> R {
    let built_at = now();
    let directory = root.map(|root| absolute(root).join(format!("{}-{}-{}", built_at, std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed))));
    if let Some(directory) = &directory { fs::create_dir_all(directory).expect("create compiler resource capture"); }
    let capture = Capture { directory, producer_manifest: absolute(producer_manifest).join("Cargo.toml"), producer_source: absolute(producer_source), caller_manifest: absolute(std::env::var("CARGO_MANIFEST_DIR").expect("actual caller manifest directory")).join("Cargo.toml"), caller_crate: std::env::var("CARGO_CRATE_NAME").unwrap_or_default(), caller_source: caller_source.map(absolute), built_at, complete: true, rows: Vec::new(), track };
    let previous = CURRENT.with(|current| current.replace(Some(capture)));
    let result = catch_unwind(AssertUnwindSafe(run));
    let capture = CURRENT.with(|current| current.replace(previous)).expect("active compiler resource capture");
    let value = match result { Ok(value) => value, Err(error) => resume_unwind(error) };
    if let Some(directory) = capture.directory {
        let complete = capture.complete && capture.caller_source.is_some() && !capture.caller_crate.is_empty();
        let text = format!("{{\"version\":1,\"kind\":\"compiler-resource\",\"producer\":{{\"manifest\":{},\"source\":{}}},\"caller\":{{\"manifest\":{},\"crate\":{},\"source\":{}}},\"builtAtMs\":{},\"observedAtMs\":{},\"completed\":{},\"resources\":[{}]}}\n", path_text(&capture.producer_manifest), path_text(&capture.producer_source), path_text(&capture.caller_manifest), quoted(&capture.caller_crate), capture.caller_source.as_ref().map(|path| path_text(path)).unwrap_or_else(|| "null".to_owned()), capture.built_at, now(), complete, capture.rows.join(","));
        let path = directory.join("observation.json");
        let temporary = directory.join("observation.tmp");
        fs::write(&temporary, text).expect("write compiler resource capture");
        fs::rename(&temporary, &path).expect("complete immutable compiler resource capture");
        (capture.track)(&path);
    }
    value
}

/// 📖️ Records the size and digest of original bytes consumed in place by the real proc-macro read.
#[track_caller]
pub fn read(path: impl AsRef<Path>) -> io::Result<Vec<u8>> {
    let path = absolute(path);
    require_capture();
    let read_operation = operation(file!(), line!(), "read"); let bytes = match settled_v1(&path, || fs::read(&path)) { Ok(bytes) => bytes, Err(error) => { CURRENT.with(|current| { if let Some(capture) = current.borrow_mut().as_mut() { (capture.track)(&path); capture.complete = false; } }); return Err(error); } };
    let location = Location::caller();
    CURRENT.with(|current| {
        if let Some(capture) = current.borrow_mut().as_mut() {
            (capture.track)(&path);
            if capture.directory.is_some() {
                capture.rows.push(format!("{{\"kind\":\"read\",\"path\":{},\"bytes\":{},\"sha256\":{},\"callsite\":{},\"operation\":{}}}", path_text(&path), bytes.len(), quoted(&content_hash::sha256_hex(&bytes)), callsite(location), read_operation));
            }
        }
    });
    Ok(bytes)
}

/// 📝️ Preserves standard UTF-8 file-read semantics and the same actual source observation.
#[track_caller]
pub fn read_to_string(path: impl AsRef<Path>) -> io::Result<String> {
    String::from_utf8(read(path)?).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

/// 🗂️ Retains one real directory enumeration while keeping entry failures visible to its caller. The directory itself is not a compiler dependency; freshness comes from the files that are read.
#[allow(dead_code)]
#[track_caller]
pub fn read_dir(path: impl AsRef<Path>) -> io::Result<std::vec::IntoIter<io::Result<fs::DirEntry>>> {
    let path = absolute(path);
    require_capture();
    let read_operation = operation(file!(), line!(), "read_dir"); let entries = match fs::read_dir(&path) { Ok(entries) => entries.collect::<Vec<_>>(), Err(error) => { CURRENT.with(|current| { if let Some(capture) = current.borrow_mut().as_mut() { capture.complete = false; } }); return Err(error); } };
    let location = Location::caller();
    CURRENT.with(|current| {
        if let Some(capture) = current.borrow_mut().as_mut() {
            if capture.directory.is_some() {
                let mut observed = Vec::new();
                for entry in &entries {
                    let Ok(entry) = entry else { capture.complete = false; continue };
                    let metadata = match fs::symlink_metadata(entry.path()) { Ok(metadata) => metadata, Err(_) => { capture.complete = false; continue } };
                    let kind = if metadata.file_type().is_symlink() { "symlink" } else if metadata.is_dir() { "directory" } else if metadata.is_file() { "file" } else { "other" };
                    let target = if kind == "symlink" { match fs::read_link(entry.path()) { Ok(target) => path_text(&target), Err(_) => { capture.complete = false; continue } } } else { "null".to_owned() };
                    observed.push(format!("{{\"path\":{},\"kind\":{},\"symlinkTarget\":{}}}", path_text(&entry.path()), quoted(kind), target));
                }
                observed.sort();
                capture.rows.push(format!("{{\"kind\":\"directory\",\"path\":{},\"entries\":[{}],\"callsite\":{},\"operation\":{}}}", path_text(&path), observed.join(","), callsite(location), read_operation));
            }
        }
    });
    Ok(entries.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_above_the_offloaded_input_root_may_be_read() {
        let root = Path::new("/work/r");
        assert!(inside_root_v1(Path::new("/work/r/crate/src"), Some(root)));
        assert!(inside_root_v1(root, Some(root)));
        assert!(!inside_root_v1(Path::new("/work"), Some(root)));
        assert!(!inside_root_v1(Path::new("/Users/u/semio"), Some(root)));
        assert!(inside_root_v1(Path::new("/Users/u/semio"), None));
    }

    #[test]
    fn an_include_path_is_relative_to_the_source_directory_in_every_root() {
        for root in ["/a/ws", "/very/other/root/ws"] {
            let (source, target) = (Path::new(root).join("p/x/lib.rs"), Path::new(root).join("t/auth.json"));
            assert_eq!(include_path_v1(&source, &target).unwrap(), "../../t/auth.json");
            assert_eq!(include_path_v1(&source, &Path::new(root).join("p/x/s/d.json")).unwrap(), "s/d.json");
        }
    }

    #[test]
    fn a_read_settles_across_an_atomic_replace_window_and_names_the_path_when_it_fails() {
        let mut calls = 0;
        let value = settled_v1(Path::new("/x"), || { calls += 1; if calls < 3 { Err(io::ErrorKind::NotFound.into()) } else { Ok(7) } }).unwrap();
        assert_eq!((value, calls), (7, 3));
        let mut other = 0;
        let refused = settled_v1(Path::new("/x/y"), || -> io::Result<()> { other += 1; Err(io::ErrorKind::InvalidData.into()) }).unwrap_err();
        assert_eq!((other, refused.kind()), (1, io::ErrorKind::InvalidData));
        let missing = lstat_v1("/semio-no-such-root/descriptor.json").unwrap_err();
        assert!(missing.kind() == io::ErrorKind::NotFound && missing.to_string().starts_with("/semio-no-such-root/descriptor.json: "));
        let directory = std::env::temp_dir().join(format!("settle-{}", std::process::id()));
        fs::create_dir_all(&directory).unwrap();
        let file = directory.join("d.json");
        let target = file.clone();
        let writer = std::thread::spawn(move || { std::thread::sleep(std::time::Duration::from_millis(60)); fs::write(&target, b"ok").unwrap(); });
        assert_eq!(read(&file).unwrap(), b"ok");
        writer.join().unwrap();
        fs::remove_dir_all(&directory).unwrap();
    }
}
