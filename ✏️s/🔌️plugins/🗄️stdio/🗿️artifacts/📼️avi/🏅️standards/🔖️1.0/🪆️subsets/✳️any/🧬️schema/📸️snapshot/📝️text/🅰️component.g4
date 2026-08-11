// 🅰️ ANTLR grammar for `stdio.avi`'s DSL text representation. RIFF/AVI has no textual syntax
// of its own — the DSL text IS a whitespace-tolerant ASCII hex dump of the REAL binary RIFF
// bytes `⚙️engine::{decode_avi,encode_avi}` produce/consume (see ../💾️binary/🥋️component.ksy).
grammar Stdio_avi_snapshot;
document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;
HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
