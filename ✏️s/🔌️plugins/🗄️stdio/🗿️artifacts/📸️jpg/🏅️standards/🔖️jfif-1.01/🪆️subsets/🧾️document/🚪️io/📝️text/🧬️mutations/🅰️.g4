grammar Stdio_jpg_mutation;
// Leaf payload grammars are authoritative in the direct text facets.
mutation: opcode argument* EOF;
opcode: 'change-jfif-header' | 'replace-quant-table' | 'remove-quant-table' | 'replace-huffman-table' | 'remove-huffman-table' | 'change-restart-interval' | 'insert-other-segment' | 'remove-other-segment' | 'replace-pixels' | 'change-re-encode-quality';
argument: WORD '=' VALUE;
WORD: [a-zA-Z][a-zA-Z0-9_-]*;
VALUE: ~[ \t\r\n]+;
WS: [ \t\r\n]+ -> skip;
