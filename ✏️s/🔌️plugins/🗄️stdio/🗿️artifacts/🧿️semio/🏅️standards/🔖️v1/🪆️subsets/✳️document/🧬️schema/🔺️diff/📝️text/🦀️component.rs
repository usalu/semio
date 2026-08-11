//! 📄️ Text representation marker for `s.stdio.semio.document.diff`: space-separated
//! `key=value` tokens (`styles=`/`images=`/`blocks=`), each value a bracket-delimited
//! `[removed];[modified];[added]` triple (see `print_document_diff`/`parse_document_diff` in the
//! sibling `🦀️component.rs` for the real hand-rolled grammar).
pub const TEXT_MARKER: &str = "s.stdio.semio.document.diff";
