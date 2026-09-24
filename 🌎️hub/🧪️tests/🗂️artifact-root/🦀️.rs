//! 🗂️ The writable root every descriptor-rooted hub law stages its fixtures under.
//!
//! The `os-hub:test` verb exports `SEMIO_TEST_ARTIFACT_DIR` (`📜️script.ts`'s `hubTestArtifactRoot`)
//! so fixtures land in the crate's own `🗑️generated/test-artifacts`. A bare `cargo test` or
//! `cargo nextest run -p semio-hub` exports nothing, and the laws that read the variable used to
//! panic in their fixture setup — 25 at once — which reads like a product regression and is not
//! one. The default below is the very directory the verb computes, derived from the crate manifest,
//! so every runner observes the same law.

use std::path::PathBuf;

/// @emoji 🗂️ The artifact root a fixture stages under: `SEMIO_TEST_ARTIFACT_DIR` when a runner named
/// one, otherwise the crate's own `🗑️generated/test-artifacts`. The directory exists on return and is
/// canonical: server-owned roots are opened component by component with `O_NOFOLLOW`, so a root
/// reached through a symlink (a ticket alias, macOS `/tmp`) must never reach a fixture.
pub fn test_artifact_root() -> PathBuf {
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(|| PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/🗑️generated/test-artifacts")), PathBuf::from);
    std::fs::create_dir_all(&root).expect("hub test artifact root");
    std::fs::canonicalize(&root).expect("canonical hub test artifact root")
}
