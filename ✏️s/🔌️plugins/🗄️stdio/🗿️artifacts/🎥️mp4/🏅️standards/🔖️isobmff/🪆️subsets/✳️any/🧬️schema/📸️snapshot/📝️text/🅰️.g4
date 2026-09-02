// 🅰️ Structured logical MP4 snapshot records emitted by dsl::DslRecord.
grammar Stdio_mp4_snapshot;

document : record EOF ;
record : IDENT LBRACE field* RBRACE ;
field : IDENT EQUAL value ;
value : IDENT | STRING | INT | record | list | object ;
list : LBRACK value* RBRACK ;
object : LBRACE field* RBRACE ;
IDENT : [A-Za-z_][A-Za-z0-9_-]* ;
STRING : '"' ('\\' . | ~["\\])* '"' ;
INT : '-'? [0-9]+ ;
LBRACE : '{' ; RBRACE : '}' ; LBRACK : '[' ; RBRACK : ']' ; EQUAL : '=' ;
WS : [ \t\r\n]+ -> skip ;
