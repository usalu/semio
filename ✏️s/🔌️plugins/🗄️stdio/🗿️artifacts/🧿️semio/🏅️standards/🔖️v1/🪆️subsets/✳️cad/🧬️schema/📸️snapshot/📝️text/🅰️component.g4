// 🅰️ ANTLR grammar for `s.stdio.semio.cad`'s DSL text representation (store::ArtifactDsl::parse_dsl
// / print_dsl). This subset's snapshot is a NEUTRAL semio type -- the DSL text IS a whitespace-
// tolerant ASCII hex dump of `serde_json::to_vec(SemioCadSnapshot)` (see ../🔣️component.json for
// the decoded JSON's real schema, and ../💾️binary/🥋️component.ksy for the paired binary envelope).
// The `semio stdio.semio.cad.dsl v1` preamble line is stripped by
// store::semio_format::split_text_preamble before this grammar's `document` production runs.
grammar Stdio_semio_cad_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
