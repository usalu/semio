//! 📝️ Text representation marker for `stdio.semio.mesh.snapshot` — the real grammar is
//! `store::ArtifactDsl for SemioMeshSnapshot` (`../../🦀️component.rs`): a
//! `semio stdio.semio.mesh.dsl v1` preamble line followed by a whitespace-tolerant ASCII hex
//! dump of the snapshot's own `serde_json::to_vec` bytes (this subset's snapshot is a neutral
//! semio type, not an on-disk file format, so there is no richer textual syntax to grammar
//! beyond the hex envelope — see sibling `.g4`/`.ebnf`/`.grammar.semio` leaves).
pub const TEXT_MARKER: &str = "stdio.semio.mesh.snapshot";
