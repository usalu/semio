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
    let read_operation = operation(file!(), line!(), "read"); let bytes = match fs::read(&path) { Ok(bytes) => bytes, Err(error) => { CURRENT.with(|current| { if let Some(capture) = current.borrow_mut().as_mut() { (capture.track)(&path); capture.complete = false; } }); return Err(error); } };
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

/// 🗂️ Retains one real directory enumeration while keeping entry failures visible to its caller.
#[track_caller]
pub fn read_dir(path: impl AsRef<Path>) -> io::Result<std::vec::IntoIter<io::Result<fs::DirEntry>>> {
    let path = absolute(path);
    require_capture();
    let read_operation = operation(file!(), line!(), "read_dir"); let entries = match fs::read_dir(&path) { Ok(entries) => entries.collect::<Vec<_>>(), Err(error) => { CURRENT.with(|current| { if let Some(capture) = current.borrow_mut().as_mut() { (capture.track)(&path); capture.complete = false; } }); return Err(error); } };
    let location = Location::caller();
    CURRENT.with(|current| {
        if let Some(capture) = current.borrow_mut().as_mut() {
            (capture.track)(&path);
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
