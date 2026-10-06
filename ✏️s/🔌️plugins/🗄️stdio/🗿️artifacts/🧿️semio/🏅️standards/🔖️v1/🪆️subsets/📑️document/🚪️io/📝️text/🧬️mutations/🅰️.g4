// ANTLR4 grammar for `stdio.semio.document`'s real one-line `OpText` text form — descriptive
// mirror of the authoritative `📖️.grammar.semio` (same production names, abridged: full
// per-keyword argument detail lives there).
grammar Semio_document_mutations;

op: keywordOp EOF;
keywordOp: KEYWORD arg*;
KEYWORD: 'set-snapshot' | 'insert-block' | 'remove-block' | 'set-block-content' | 'set-paragraph-style'
       | 'set-heading-level' | 'set-list-ordered' | 'set-run-text' | 'set-run-style' | 'set-image-block'
       | 'insert-style' | 'remove-style' | 'set-style-name' | 'set-style-based-on' | 'insert-image'
       | 'remove-image' | 'set-image-bytes';
arg: IDENT '=' value;
value: '[' .*? ']' | HEX | INT;

IDENT: [a-zA-Z] [a-zA-Z-]*;
HEX: [0-9a-f]*;
INT: [0-9]+;
WS: [ \t\r\n]+ -> skip;
