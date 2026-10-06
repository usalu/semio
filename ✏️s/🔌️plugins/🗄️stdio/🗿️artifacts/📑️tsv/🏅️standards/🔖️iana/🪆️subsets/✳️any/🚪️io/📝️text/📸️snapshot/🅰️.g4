// 🅰️ ANTLR grammar for the real IANA text/tab-separated-values wire format itself (not a
// serialization of the Rust snapshot struct — the snapshot's `records` IS this grammar's parse).
// https://www.iana.org/assignments/media-types/text/tab-separated-values
// NO quoting/escaping exists in this format: a field can never contain a literal TAB/CR/LF byte.
grammar Stdio_tsv_snapshot;

file    : (record (LE record)*)? LE? EOF ;
record  : field (TAB field)* ;
field   : FIELDDATA? ;

TAB   : '	' ;
LE    : '\r\n' | '\n' ;
FIELDDATA : ~[\t\r\n]+ ;
