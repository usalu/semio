//! 💾️ Binary representation marker for `stdio.semio.mesh.snapshot` — the real codec is
//! `store::ArtifactPack for SemioMeshSnapshot` (`../../🦀️component.rs`): the shared
//! `store::semio_format::wrap_binary` envelope (8-byte magic + u32 LE token length + token)
//! wrapping the snapshot's own `serde_json::to_vec` bytes. See sibling `.ksy`/`.abnf`/
//! `.protocol.semio` leaves for the byte-level layout this marker documents.
pub const BINARY_MAGIC: &str = "stdio.semio.mesh.snapshot";
