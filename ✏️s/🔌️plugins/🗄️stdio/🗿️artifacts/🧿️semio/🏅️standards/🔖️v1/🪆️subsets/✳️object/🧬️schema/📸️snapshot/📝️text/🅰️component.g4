// ANTLR4 grammar for `stdio.semio.object`'s DSL text form: the semio envelope preamble wrapping a
// hex-encoded compact-JSON `SemioObjectSnapshot` (see the sibling 📖️component.grammar.semio for the
// authoritative production set).
grammar Stdio_semio_object_snapshot;

document: header body EOF;
header: 'schema' WS 'stdio.semio.object' NL;
body: HEXDIG*;

snapshotJson: '{' '"schema"' ':' jsonString ',' '"root"' ':' semioValue ',' '"objects"' ':' '[' (objectNode (',' objectNode)*)? ']' '}';
objectNode: '{' '"id"' ':' objectId ',' '"value"' ':' semioValue '}';
objectId: '{' '"value"' ':' jsonString '}';

semioValue
    : '{' '"kind"' ':' '"null"' '}'
    | '{' '"kind"' ':' '"bool"' ',' '"value"' ':' ('true' | 'false') '}'
    | '{' '"kind"' ':' '"int"' ',' '"lexeme"' ':' jsonString '}'
    | '{' '"kind"' ':' '"float"' ',' '"lexeme"' ':' jsonString '}'
    | '{' '"kind"' ':' '"str"' ',' '"value"' ':' jsonString '}'
    | '{' '"kind"' ':' '"bytes"' ',' '"value"' ':' '[' (INT (',' INT)*)? ']' '}'
    | '{' '"kind"' ':' '"list"' ',' '"items"' ':' '[' (semioValue (',' semioValue)*)? ']' '}'
    | '{' '"kind"' ':' '"map"' ',' '"entries"' ':' '[' (mapEntry (',' mapEntry)*)? ']' '}'
    | '{' '"kind"' ':' '"ref"' ',' '"id"' ':' objectId '}'
    ;
mapEntry: '{' '"key"' ':' jsonString ',' '"value"' ':' semioValue '}';

jsonString: '"' (ESC | ~["\\])* '"';
fragment ESC: '\\' (["\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEXDIG HEXDIG HEXDIG HEXDIG;
fragment HEXDIG: [0-9a-fA-F];
INT: '-'? [0-9]+;

WS: [ \t]+ -> skip;
NL: '\r'? '\n';
