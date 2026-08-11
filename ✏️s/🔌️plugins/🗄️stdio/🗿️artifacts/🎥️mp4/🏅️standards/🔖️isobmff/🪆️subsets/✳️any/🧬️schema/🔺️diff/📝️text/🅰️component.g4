// 🅰️ ANTLR grammar for stdio.mp4's diff text form — mirrors ../../📸️snapshot/📝️text/🅰️component.g4:
// a whitespace-tolerant hex dump, this time of the JSON-serialized Mp4Diff (op codecs are the
// handcrafted `OpText`/`OpBinary` JSON round-trip in 🧬️mutations/🦀️component.rs, not a bespoke
// diff-text grammar — the mutation vocabulary IS the diff's textual protocol).
grammar Stdio_mp4_diff;
document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;
HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
