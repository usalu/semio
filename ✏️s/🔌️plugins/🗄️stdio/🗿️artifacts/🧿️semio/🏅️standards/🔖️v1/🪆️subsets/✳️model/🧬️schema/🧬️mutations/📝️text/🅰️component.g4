// ANTLR4 grammar for `SemioModelMutation`'s hand-rolled `OpText` -- one line of compact JSON
// tagged on the `mutation` field.
grammar Stdio_semio_model_mutation;

line: value EOF;
value: object | array | STRING | NUMBER | 'true' | 'false' | 'null';
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
