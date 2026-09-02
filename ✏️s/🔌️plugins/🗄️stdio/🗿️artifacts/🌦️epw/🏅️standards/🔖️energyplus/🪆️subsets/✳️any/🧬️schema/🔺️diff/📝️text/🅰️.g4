// 🅰️ ANTLR grammar for the hand-rolled `stdio.epw.diff` wire text (see 🦀️.rs
// `print_epw_diff`/`parse_epw_diff` — the real, authoritative implementation).
grammar Stdio_epw_diff;

diff    : token (SP token)* EOF ;
token   : 'location=' hex
        | 'design-conditions=' hex
        | 'typical-extreme-periods=' hex
        | 'ground-temperatures=' hex
        | 'ground-holidays-dst=' hex
        | 'comments-1=' hex
        | 'comments-2=' hex
        | 'data-periods=' hex
        | 'records{' recordsBody '}' ;
recordsBody : '[' INT? (',' INT)* '];[' modified? (',' modified)* '];[' added? (',' added)* ']' ;
modified    : INT ':' '[' option? (',' option)* ']' ;
added       : INT ':' '[' hex? (',' hex)* ']' ;
option      : '[0]' | '[1,' hex ']' ;
hex         : HEXPAIR* ;

HEXPAIR : [0-9a-f] [0-9a-f] ;
INT     : [0-9]+ ;
SP      : ' ' ;
