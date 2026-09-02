// 🅰️ ANTLR grammar for `stdio.png`'s DSL text representation (store::ArtifactDsl::parse_dsl
// / print_dsl). PNG itself has no textual syntax — the DSL text IS a whitespace-tolerant ASCII
// hex dump of the REAL binary PNG bytes `crate::artifacts::png::engine::{encode_png,decode_png}`
// produce/consume (see ../💾️binary/🥋️.ksy for that binary's own real chunk grammar).
// The `semio stdio.png.dsl v1` preamble line is stripped by store::semio_format::split_text_preamble
// before this grammar's `document` production runs.
grammar Stdio_png_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
