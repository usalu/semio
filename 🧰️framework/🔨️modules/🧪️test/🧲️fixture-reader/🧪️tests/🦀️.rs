//! 🧪️ Laws of the shared run-time fixture reader, checked against the language-neutral vectors that the TypeScript test of this folder replays with `node:fs` and `node:crypto`.
//!
//! @see <https://doc.rust-lang.org/std/panic/fn.catch_unwind.html>

use crate::{bytes_in, path_in, repo_bytes_in, repo_root_in, repo_text_in, text_in};

struct Vector {
    kind: &'static str,
    id: &'static str,
    relative: &'static str,
    bytes_hex: &'static str,
    message_contains: Vec<&'static str>,
}

fn vectors() -> Vec<Vector> {
    fixture_text!("../../🧪️tests/🧫️fixtures/📊️.csv")
        .lines()
        .skip(1)
        .map(|line| {
            let cells: Vec<&'static str> = line.split(',').collect();
            Vector { kind: cells[0], id: cells[1], relative: cells[2], bytes_hex: cells[3], message_contains: cells[5].split('|').filter(|needle| !needle.is_empty()).collect() }
        })
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn refusal(operation: impl FnOnce() + std::panic::UnwindSafe) -> String {
    let payload = std::panic::catch_unwind(operation).expect_err("the read is refused");
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => payload.downcast_ref::<&str>().map(|message| (*message).to_owned()).expect("the panic carries a message"),
    }
}

#[test]
fn the_vectors_cover_text_binary_and_refusals() {
    let kinds: Vec<&str> = vectors().iter().map(|vector| vector.kind).collect();
    assert_eq!(kinds, ["text", "binary", "refuse"]);
}

#[test]
fn reads_equal_the_vector_bytes() {
    for vector in vectors().iter().filter(|vector| vector.kind != "refuse") {
        assert_eq!(hex(bytes_in(env!("CARGO_MANIFEST_DIR"), vector.relative)), vector.bytes_hex, "{}", vector.id);
        assert_eq!(hex(&std::fs::read(fixture_path!(vector.relative)).expect("reads")), vector.bytes_hex, "{}", vector.id);
        assert_eq!(hex(fixture_bytes!(vector.relative)), vector.bytes_hex, "{}", vector.id);
    }
}

#[test]
fn text_reads_equal_the_decoded_vector_bytes() {
    for vector in vectors().iter().filter(|vector| vector.kind == "text") {
        let bytes: Vec<u8> = (0..vector.bytes_hex.len()).step_by(2).map(|at| u8::from_str_radix(&vector.bytes_hex[at..at + 2], 16).expect("hex")).collect();
        let decoded = String::from_utf8(bytes).expect("the text vector is UTF-8");
        assert_eq!(text_in(env!("CARGO_MANIFEST_DIR"), vector.relative), decoded, "{}", vector.id);
        assert_eq!(fixture_text!(vector.relative), decoded, "{}", vector.id);
    }
}

#[test]
fn refused_reads_name_the_resolved_path_and_both_inputs() {
    for vector in vectors().iter().filter(|vector| vector.kind != "text") {
        let relative = vector.relative;
        let message = refusal(move || {
            let _ = text_in(env!("CARGO_MANIFEST_DIR"), relative);
        });
        assert!(message.contains("fixture"), "{}: {message}", vector.id);
        assert!(message.contains(relative), "{}: {message}", vector.id);
        for needle in &vector.message_contains {
            assert!(message.contains(needle), "{}: {message}", vector.id);
        }
    }
}

#[test]
fn missing_bytes_name_the_resolved_path() {
    let message = refusal(|| {
        let _ = bytes_in(env!("CARGO_MANIFEST_DIR"), "missing.bin");
    });
    assert!(message.contains("missing.bin") && message.contains("CARGO_MANIFEST_DIR"), "{message}");
}

#[test]
fn paths_resolve_below_the_manifest_directory_of_the_caller() {
    assert_eq!(manifest_dir!(), std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    assert_eq!(fixture_path!("Cargo.toml"), path_in(env!("CARGO_MANIFEST_DIR"), "Cargo.toml"));
    assert!(fixture_path!("Cargo.toml").is_file());
    assert!(fixture_path!("../../🧪️tests").is_dir());
}

#[test]
fn the_repository_root_is_the_declared_package_workspace() {
    let root = repo_root!();
    assert!(root.join("📋️project.json").is_file());
    assert!(root.join("package.json").is_file());
    assert_eq!(root, repo_root_in(env!("CARGO_MANIFEST_DIR")));
    assert!(manifest_dir!().starts_with(&root));
}

#[test]
fn a_missing_repository_root_is_refused() {
    let message = refusal(|| {
        let _ = repo_root_in("/");
    });
    assert!(message.contains("workspace root"), "{message}");
}

#[test]
fn repository_reads_equal_std_and_are_shared_between_calls() {
    let manifest = env!("CARGO_MANIFEST_DIR");
    let root = repo_root_in(manifest);
    assert_eq!(repo_text_in(manifest, "package.json"), std::fs::read_to_string(root.join("package.json")).expect("reads"));
    assert_eq!(repo_bytes_in(manifest, "package.json"), std::fs::read(root.join("package.json")).expect("reads").as_slice());
    assert!(std::ptr::eq(repo_bytes_in(manifest, "package.json"), repo_bytes_in(manifest, "package.json")));
    assert_eq!(repo_text!("package.json"), repo_text_in(manifest, "package.json"));
    assert_eq!(repo_bytes!("package.json"), repo_bytes_in(manifest, "package.json"));
}

#[test]
fn missing_repository_files_are_refused_with_their_path() {
    let message = refusal(|| {
        let _ = repo_bytes_in(env!("CARGO_MANIFEST_DIR"), "missing-in-the-repository.bin");
    });
    assert!(message.contains("missing-in-the-repository.bin") && message.contains("CARGO_MANIFEST_DIR"), "{message}");
}
