//! 💾️ Binary representation marker for `stdio.semio.mesh.mutations` — `encode_op`/`decode_op`
//! (`../../🦀️component.rs`'s `impl protocol::OpBinary for SemioMeshMutation`) are the TEXT bytes
//! verbatim (`self.print_op().into_bytes()`), no additional framing.
pub const BINARY_MAGIC: &str = "stdio.semio.mesh.mutations";
