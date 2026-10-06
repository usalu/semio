grammar Stdio_png_mutation;
// Leaf payload grammars are authoritative in the direct text facets.
mutation: opcode argument* EOF;
opcode: 'change-gamma' | 'set-snapshot' | 'patch-pixels' | 'patch-snapshot' | 'paint-native-samples';
argument: WORD '=' VALUE;
WORD: [a-zA-Z][a-zA-Z0-9_-]*;
VALUE: ~[ \t\r\n]+;
WS: [ \t\r\n]+ -> skip;
