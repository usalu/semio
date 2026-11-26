//! 🧪️ Cold compiler admission across OS processes.

use super::try_native_compile;
use std::path::Path;
use std::process::Command;

fn observe(cache_root: &Path) -> &'static str {
    match try_native_compile(cache_root) {
        Ok(_) => "admitted",
        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => "busy",
        Err(_) => "io-error",
    }
}

#[test]
fn other_process_checks_native_compile_slot() {
    let Some(root) = std::env::var_os("SEMIO_TEST_COLD_COMPILE_CACHE") else { return };
    let expected = std::env::var("SEMIO_TEST_COLD_COMPILE_EXPECTED").unwrap();
    assert_eq!(observe(Path::new(&root)), expected);
}

#[test]
fn cold_compiler_slot_is_shared_by_processes_and_released_by_its_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let root = std::env::temp_dir().join(format!("semio-native-compile-admission-{}", std::process::id()));
    let cache = root.join("shared");
    let independent = root.join("independent");
    let invalid = root.join("file");
    let child = |cache_root: &Path, expected: &str| {
        let module = module_path!().split_once("::").unwrap().1;
        let test = format!("{module}::other_process_checks_native_compile_slot");
        let output = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", &test, "--nocapture"])
            .env("SEMIO_TEST_COLD_COMPILE_CACHE", cache_root)
            .env("SEMIO_TEST_COLD_COMPILE_EXPECTED", expected)
            .output()
            .unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success() && stdout.contains(&format!("test {test} ... ok")), "{stdout}\n{stderr}");
    };

    let admission = try_native_compile(&cache).unwrap();
    child(&cache, fixture["sharedCacheWhileHeld"].as_str().unwrap());
    child(&independent, fixture["independentCacheWhileHeld"].as_str().unwrap());
    std::fs::write(&invalid, b"file is not a cache directory").unwrap();
    child(&invalid, fixture["invalidCacheRoot"].as_str().unwrap());
    drop(admission);
    child(&cache, fixture["sharedCacheAfterRelease"].as_str().unwrap());
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn cold_compiler_requires_current_memory_headroom_and_desktop_reserve() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture["memoryCases"].as_array().unwrap() {
        let component_bytes = usize::try_from(case["componentBytes"].as_u64().unwrap()).unwrap();
        let total = case["totalBytes"].as_u64().unwrap();
        let sample = case["availableBytes"].as_u64().map(|available| (total, available));
        let result = super::memory::check_compile_memory(component_bytes, sample);
        match case["result"].as_str().unwrap() {
            "admitted" => assert!(result.is_ok(), "{case}"),
            "busy" => assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::WouldBlock, "{case}"),
            unexpected => panic!("unknown fixture result {unexpected}"),
        }
    }
}

#[test]
fn worker_reads_at_most_the_admitted_size_and_one_refusal_byte() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    for case in fixture["streamCases"].as_array().unwrap() {
        let declared = usize::try_from(case["declaredBytes"].as_u64().unwrap()).unwrap();
        let payload = case["payload"].as_str().unwrap().as_bytes();
        let mut cursor = std::io::Cursor::new(payload);
        let result = super::read_compile_input(&mut cursor, declared);
        match case["result"].as_str().unwrap() {
            "admitted" => assert_eq!(result.unwrap(), payload, "{case}"),
            "io-error" => assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::InvalidData, "{case}"),
            unexpected => panic!("unknown fixture result {unexpected}"),
        }
        assert_eq!(cursor.position() as usize, payload.len().min(declared + 1));
    }
}
