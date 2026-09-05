// ANTLR4 grammar for one `stdio.json` mutation op line -- an RFC8259 JSON document (the tagged
// `JsonMutation` struct), same value grammar as ../../📸️snapshot/📝️text/🅰️.g4.
grammar Stdio_json_mutations;

document: header body EOF;
header: 'schema' WS 'stdio.json' NL;
body: value;

value
    : object
    | array
    | STRING
    | NUMBER
    | 'true'
    | 'false'
    | 'null'
    ;

object: '{' (member (',' member)*)? '}';
member: STRING ':' value;

array: '[' (value (',' value)*)? ']';

STRING: '"' (ESC | ~["\\ -])* '"';
fragment ESC: '\\' (["\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEX HEX HEX HEX;
fragment HEX: [0-9a-fA-F];

NUMBER: '-'? INT FRAC? EXP?;
fragment INT: '0' | [1-9] [0-9]*;
fragment FRAC: '.' [0-9]+;
fragment EXP: [eE] [+-]? [0-9]+;

WS: [ \t]+ -> skip;
NL: '\r'? '\n';
