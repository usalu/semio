grammar Stdio_jpg_mutation;
mutation: opcode argument* EOF;
opcode: 'change-jfif-header' | 'insert-other-segment' | 'remove-other-segment' | 'replace-pixels' | 'replace-image';
argument: WORD '=' VALUE;
WORD: [a-zA-Z][a-zA-Z0-9_-]*;
VALUE: ~[ \t\r\n]+;
WS: [ \t\r\n]+ -> skip;
