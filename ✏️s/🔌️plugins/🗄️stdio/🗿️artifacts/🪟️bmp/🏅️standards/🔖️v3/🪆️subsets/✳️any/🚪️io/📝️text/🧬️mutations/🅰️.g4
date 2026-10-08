grammar Stdio_bmp_mutation;
// Leaf payload grammars are authoritative in the direct text facets.
mutation: opcode argument* EOF;
opcode: 'paint-indexed-region' | 'paint-direct-region';
argument: WORD '=' VALUE;
WORD: [a-zA-Z][a-zA-Z0-9_-]*;
VALUE: ~[ \t\r\n]+;
WS: [ \t\r\n]+ -> skip;
