//! 💾️ Binary representation marker for `s.stdio.semio.document.snapshot`: the pack binary form is
//! `wrap_binary(envelope, json(SemioDocumentSnapshot))` (see `store::ArtifactPack` impl on
//! `SemioDocumentSnapshot`) — a `store::semio_format::SemioEnvelope` header followed by the raw
//! JSON payload bytes.
pub const BINARY_MAGIC: &str = "s.stdio.semio.document";
