//! 📄️ Text representation marker for `s.stdio.semio.document.snapshot`: the DSL text form is
//! `stdio.semio.document.dsl.v1\n<hex(json(SemioDocumentSnapshot))>` (see `store::ArtifactDsl`
//! impl on `SemioDocumentSnapshot` — envelope preamble + hex-encoded JSON body, honestly documented
//! rather than a `*OCTET` catch-all).
pub const TEXT_MARKER: &str = "s.stdio.semio.document";
