// 🅰️ ANTLR grammar for `stdio.tiff`'s DSL text representation (store::ArtifactDsl::parse_dsl
// / print_dsl). TIFF itself has no textual syntax — the DSL text IS a whitespace-tolerant
// ASCII hex dump of the REAL binary TIFF bytes `crate::artifacts::tiff::engine::{encode_tiff,
// decode_tiff}` produce/consume (see ../💾️binary/🥋️.ksy for that binary's own real
// byte-order/IFD-chain/tag grammar). The `semio stdio.tiff.dsl v1` preamble line is stripped
// by store::semio_format::split_text_preamble before this grammar's `document` production runs.
grammar Stdio_tiff_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
