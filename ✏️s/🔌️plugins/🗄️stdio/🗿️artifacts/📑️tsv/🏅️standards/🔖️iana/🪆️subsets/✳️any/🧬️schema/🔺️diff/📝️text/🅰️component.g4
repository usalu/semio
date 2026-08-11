// 🅰️ ANTLR grammar for the hand-rolled `stdio.tsv.diff` wire text (see 🦀️component.rs
// `print_tsv_diff`/`parse_tsv_diff` — the real, authoritative implementation).
grammar Stdio_tsv_diff;

diff    : token (SP token)* EOF ;
token   : 'trailing-newline=' BIT
        | 'line-ending=' ('lf' | 'crlf')
        | 'records{' recordsBody '}' ;
recordsBody : '[' INT? (',' INT)* '];[' modified? (',' modified)* '];[' added? (',' added)* ']' ;
modified    : INT ':' rowdiff ;
rowdiff     : '[0]' | '[1,[' option? (',' option)* ']]' ;
added       : INT ':' '[' hex? (',' hex)* ']' ;
option      : '[0]' | '[1,' hex ']' ;
hex         : HEXPAIR* ;

HEXPAIR : [0-9a-f] [0-9a-f] ;
INT     : [0-9]+ ;
BIT     : [01] ;
SP      : ' ' ;
