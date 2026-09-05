grammar Stdio_tiff_mutation;
// Leaf payload grammars are authoritative in the direct text facets.
mutation: opcode argument* EOF;
opcode: 'change-byte-order' | 'insert-ifd' | 'remove-ifd' | 'replace-tag' | 'remove-tag' | 'replace-pixels';
argument: WORD '=' VALUE;
WORD: [a-zA-Z][a-zA-Z0-9_-]*;
VALUE: ~[ \t\r\n]+;
WS: [ \t\r\n]+ -> skip;
