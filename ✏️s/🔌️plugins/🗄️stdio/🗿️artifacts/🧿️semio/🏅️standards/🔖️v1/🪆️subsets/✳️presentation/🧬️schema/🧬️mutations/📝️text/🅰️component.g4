// ANTLR4 grammar for `stdio.semio.presentation`'s real one-line `OpText` text form — descriptive
// mirror of the authoritative `📖️component.grammar.semio` (same production names, abridged: full
// per-keyword argument detail lives there).
grammar Semio_semio_presentation_mutations;

op: noMutation | keywordOp EOF;
noMutation: 'no-mutation';
keywordOp: KEYWORD arg*;
KEYWORD: 'set-snapshot' | 'insert-slide' | 'remove-slide' | 'set-slide-layout' | 'set-slide-notes'
       | 'insert-shape' | 'remove-shape' | 'set-shape-frame' | 'set-textbox-blocks' | 'insert-master'
       | 'remove-master' | 'insert-layout' | 'remove-layout' | 'set-layout-master';
arg: IDENT '=' value;
value: '[' .*? ']' | HEX | INT;

IDENT: [a-zA-Z] [a-zA-Z-]*;
HEX: [0-9a-f]*;
INT: [0-9]+;
WS: [ \t\r\n]+ -> skip;
