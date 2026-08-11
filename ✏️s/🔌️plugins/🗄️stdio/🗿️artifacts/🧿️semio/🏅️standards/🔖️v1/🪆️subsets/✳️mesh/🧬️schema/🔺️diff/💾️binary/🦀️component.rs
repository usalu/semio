//! 💾️ Binary representation marker for `stdio.semio.mesh.diff` — `encode_diff`/`decode_diff`
//! (`../../🦀️component.rs`'s `impl protocol::DiffCodec for SemioMeshDiff`) are the TEXT bytes
//! verbatim (`self.print_diff().into_bytes()`), no additional framing — same simplification
//! gif/svg/bcf/docx's own hand-rolled `DiffCodec` impls use.
pub const BINARY_MAGIC: &str = "stdio.semio.mesh.diff";
