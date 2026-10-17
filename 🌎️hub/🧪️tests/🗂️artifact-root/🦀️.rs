//! 🗂️ The writable root every descriptor-rooted hub law stages its fixtures under.
//!
//! The `os-hub:test` verb exports `SEMIO_TEST_ARTIFACT_DIR` (`📜️script.ts`'s `hubTestArtifactRoot`)
//! so fixtures land in the crate's own `🗑️generated/test-artifacts`. A bare `cargo test` or
//! `cargo nextest run -p semio-hub` exports nothing, and the laws that read the variable used to
//! panic in their fixture setup — 25 at once — which reads like a product regression and is not
//! one. The default below is the very directory the verb computes, derived from the crate manifest,
//! so every runner observes the same law.

use semio_framework_test_fixtures::fixture_path;
use std::path::PathBuf;

/// 🗂️ The artifact root a fixture stages under: `SEMIO_TEST_ARTIFACT_DIR` when a runner named
/// one, otherwise the crate's own `🗑️generated/test-artifacts`. The directory exists on return and is
/// canonical: server-owned roots are opened component by component with `O_NOFOLLOW`, so a root
/// reached through a symlink (a ticket alias, macOS `/tmp`) must never reach a fixture.
pub fn test_artifact_root() -> PathBuf {
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/🗑️generated/test-artifacts")), PathBuf::from);
    std::fs::create_dir_all(&root).expect("hub test artifact root");
    std::fs::canonicalize(&root).expect("canonical hub test artifact root")
}

/// 📖️ A file of the hub workspace read when the law runs instead of being embedded into the test binary; `relative` is relative to the hub workspace root. The text lives as long as the process, like an embedded constant.
pub fn hub_source(relative: &str) -> &'static str {
    let path = fixture_path!("../../../..").join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("hub source {relative}: {error}")).into_boxed_str())
}

/// 🌳️ A hub Rust source together with every module it mounts through `#[path = "…"]` and every fragment it splices through `include!("…");`, parent first and then each child in declaration order, with the `🧪️tests` mounts left out; `relative` is relative to the hub workspace root. Source-text laws read a split file through this so they keep seeing what the unsplit file held.
pub fn hub_module_tree(relative: &str) -> &'static str {
    fn walk(path: &std::path::Path, text: &mut String) {
        let source = std::fs::read_to_string(path).unwrap_or_else(|error| panic!("hub source {}: {error}", path.display()));
        text.push_str(&source);
        text.push('\n');
        let directory = path.parent().expect("hub source directory");
        for line in source.lines() {
            let mounted = line.trim().strip_prefix("#[path = \"").and_then(|rest| rest.strip_suffix("\"]")).or_else(|| line.trim().strip_prefix("include!(\"").and_then(|rest| rest.strip_suffix("\");")));
            if let Some(child) = mounted {
                if !child.contains("🧪️tests") {
                    walk(&directory.join(child), text);
                }
            }
        }
    }
    let mut text = String::new();
    walk(&fixture_path!("../../../..").join(relative), &mut text);
    Box::leak(text.into_boxed_str())
}
