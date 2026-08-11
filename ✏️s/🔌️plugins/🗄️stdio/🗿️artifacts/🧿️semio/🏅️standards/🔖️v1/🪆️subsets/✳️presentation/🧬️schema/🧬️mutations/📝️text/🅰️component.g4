grammar Semio_semio_presentation_mutations;
line: NO_MUTATION | KEYWORD (WS arg)*;
arg: NAME '=' VALUE;
NO_MUTATION: 'no-mutation';
KEYWORD: [a-z-]+;
NAME: [a-z-]+;
VALUE: (~[ ])*;
WS: ' ';
