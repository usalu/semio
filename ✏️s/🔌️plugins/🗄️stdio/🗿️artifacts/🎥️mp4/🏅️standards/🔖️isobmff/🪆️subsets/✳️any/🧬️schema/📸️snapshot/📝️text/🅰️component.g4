// 🅰️ ANTLR grammar for `stdio.mp4`'s DSL text representation (store::ArtifactDsl::parse_dsl /
// print_dsl). MP4/ISO-BMFF has no textual syntax of its own — the DSL text IS a whitespace-
// tolerant ASCII hex dump of the REAL binary ISO-BMFF bytes `⚙️engine::{decode_mp4,encode_mp4}`
// produce/consume (see ../💾️binary/🥋️component.ksy for that binary's own real box grammar).
grammar Stdio_mp4_snapshot;

document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;

HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
