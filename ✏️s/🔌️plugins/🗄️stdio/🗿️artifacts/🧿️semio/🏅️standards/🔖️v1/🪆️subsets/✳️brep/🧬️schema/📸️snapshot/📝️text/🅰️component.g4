// 🅰️ ANTLR grammar for `stdio.semio.brep`'s DSL text representation
// (store::ArtifactDsl::parse_dsl / print_dsl). The snapshot has no textual syntax of its own —
// the DSL text IS a whitespace-tolerant ASCII hex dump of the REAL JSON-pack bytes of a
// `SemioBrepSnapshot` (see ../🔣️component.json for that payload's own structure). The
// `semio stdio.semio.brep.dsl v1` preamble line is stripped by
// store::semio_format::split_text_preamble before this grammar's `document` production runs.
grammar Stdio_semio_brep_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
