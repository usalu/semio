// 🅰️ Schema-owned AVI snapshot text; complete semantic field domains live in the adjacent schema.
grammar Stdio_avi_snapshot;
document : element* EOF ;
element : field | block | list | IDENT | STRING | INT ;
field : IDENT EQUAL element ;
block : IDENT LBRACE element* RBRACE ;
list : LBRACK element* RBRACK ;
IDENT : [A-Za-z_][A-Za-z0-9_-]* ;
STRING : '"' ('\\' . | ~["\\])* '"' ;
INT : '-'? [0-9]+ ;
LBRACE : '{' ; RBRACE : '}' ; LBRACK : '[' ; RBRACK : ']' ; EQUAL : '=' ;
WS : [ \t\r\n]+ -> skip ;
