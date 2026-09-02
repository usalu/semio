// 🅰️ ANTLR grammar for `stdio.obj` (snapshot text) — the real, commonly-implemented
// Wavefront OBJ 3.0 statement grammar (statements may appear in any order).
grammar Stdio_obj_snapshot;

document    : statement* EOF ;
statement   : comment | vertex | texcoord | normal | face | objectStmt | groupStmt
            | usemtlStmt | mtllibStmt | smoothingStmt | unknownStmt ;

comment     : COMMENT ;
vertex      : 'v' FLOAT FLOAT FLOAT FLOAT? ;
texcoord    : 'vt' FLOAT FLOAT? FLOAT? ;
normal      : 'vn' FLOAT FLOAT FLOAT ;
face        : 'f' faceVertex faceVertex faceVertex+ ;
faceVertex  : INDEX ('/' INDEX? '/' INDEX | '/' INDEX)? ;
objectStmt  : 'o' NAME? ;
groupStmt   : 'g' NAME* ;
usemtlStmt  : 'usemtl' NAME? ;
mtllibStmt  : 'mtllib' NAME+ ;
smoothingStmt : 's' ('off' | INDEX) ;
unknownStmt : UNKNOWN_LINE ;

FLOAT       : '-'? DIGIT+ ('.' DIGIT+)? (('e'|'E') ('-'|'+')? DIGIT+)? ;
INDEX       : '-'? DIGIT+ ;
NAME        : (~[ \t\r\n])+ ;
COMMENT     : '#' ~[\r\n]* ;
UNKNOWN_LINE: ~[\r\n]+ ;
fragment DIGIT : [0-9] ;
