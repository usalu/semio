// 🅰️ ANTLR grammar for `s.stdio.semio.mesh`'s DSL text representation
// (store::ArtifactDsl::parse_dsl/print_dsl). This subset's snapshot is a NEUTRAL semio type (not
// an on-disk file format), so the DSL text IS a whitespace-tolerant ASCII hex dump of the
// snapshot's own real JSON serialization (serde_json::to_vec/from_slice — see
// ../🦀️component.rs's `ArtifactDsl` impl). The `semio stdio.semio.mesh.dsl v1` preamble line
// is stripped by store::semio_format::split_text_preamble before this grammar's `document`
// production runs.
grammar Stdio_semio_mesh_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
