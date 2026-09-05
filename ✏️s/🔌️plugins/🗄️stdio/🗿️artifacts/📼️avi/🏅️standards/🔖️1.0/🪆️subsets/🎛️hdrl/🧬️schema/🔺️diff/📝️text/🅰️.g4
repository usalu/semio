// 🅰️ ANTLR grammar for stdio.avi's diff text form — op codecs are the handcrafted
// OpText/OpBinary JSON round-trip in 🧬️mutations/🦀️.rs.
grammar Stdio_avi_diff;
document : hexByte (WS? hexByte)* EOF ;
hexByte  : HEXDIGIT HEXDIGIT ;
HEXDIGIT : [0-9a-fA-F] ;
WS       : [ \t\r\n]+ ;
