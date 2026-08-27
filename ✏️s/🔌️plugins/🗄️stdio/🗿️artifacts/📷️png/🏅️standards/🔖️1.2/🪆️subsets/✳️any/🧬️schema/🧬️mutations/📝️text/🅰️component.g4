grammar Stdio_png_mutation;
// Leaf payload grammars are authoritative in the direct text facets.
mutation: opcode argument* EOF;
opcode: 'change-header' | 'replace-palette' | 'change-transparency' | 'change-gamma' | 'change-chromaticities' | 'change-srgb-intent' | 'change-physical-dims' | 'change-timestamp' | 'change-background' | 'insert-text-chunk' | 'remove-text-chunk' | 'replace-text-chunk' | 'replace-pixels' | 'insert-unknown-chunk' | 'remove-unknown-chunk';
argument: WORD '=' VALUE;
WORD: [a-zA-Z][a-zA-Z0-9_-]*;
VALUE: ~[ \t\r\n]+;
WS: [ \t\r\n]+ -> skip;
