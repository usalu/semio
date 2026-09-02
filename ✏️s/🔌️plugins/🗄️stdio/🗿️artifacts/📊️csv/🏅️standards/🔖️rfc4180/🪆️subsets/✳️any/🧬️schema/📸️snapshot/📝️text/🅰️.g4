// 🅰️ ANTLR grammar for the RFC 4180 CSV wire format itself (not a serialization of the
// Rust snapshot struct — the snapshot's `records`/`hasHeader` ARE this grammar's parse).
// https://www.rfc-editor.org/rfc/rfc4180
grammar Stdio_csv_snapshot;

file    : record (CRLF record)* CRLF? EOF ;
record  : field (COMMA field)* ;
field   : ESCAPED | NON_ESCAPED ;

COMMA   : ',' ;
CRLF    : '\r\n' | '\n' ;
ESCAPED : '"' ( ~["] | '""' )* '"' ;
NON_ESCAPED : ~[",\r\n]* ;
