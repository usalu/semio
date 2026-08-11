// ANTLR4 grammar for a `stdio.semio.model` DSL (text) document: the shared `semio_format`
// preamble line, then the compact-JSON snapshot body hex-encoded.
grammar Stdio_semio_model_snapshot;

document: preamble NL body EOF;
preamble: 'semio' WS 'stdio.semio.model.dsl' WS 'v' VERSION;
body: (HEXBYTE)*;

VERSION: DIGIT+;
HEXBYTE: HEXDIGIT HEXDIGIT;
fragment HEXDIGIT: DIGIT | [a-f];
fragment DIGIT: [0-9];

WS: [ \t]+ -> skip;
NL: '\r'? '\n';
