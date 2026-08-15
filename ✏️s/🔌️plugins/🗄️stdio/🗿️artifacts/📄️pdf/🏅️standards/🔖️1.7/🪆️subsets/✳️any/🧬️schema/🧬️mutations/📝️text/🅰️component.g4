grammar Stdio_pdf_1_7_Mutations;
pdfMutation: 'no-mutation' | KEYWORD (' ' field)* EOF;
field: NAME '=' value;
value: INTEGER | HEX | record | pdfObject;
record: '[' (value (',' value)*)? ']';
pdfObject: 'Z' | OBJECT_TAG '[' (value (',' value)*)? ']';
KEYWORD: 'set-snapshot' | 'insert-page' | 'remove-page' | 'set-page-media-box'
       | 'set-page-crop-box' | 'append-page-content' | 'set-info' | 'insert-object'
       | 'remove-object' | 'set-object-value' | 'set-dict-entry' | 'remove-dict-entry'
       | 'set-trailer-entry' | 'remove-trailer-entry';
OBJECT_TAG: [BIRSNADFT];
NAME: [a-z] [a-z-]*;
INTEGER: '-'? [0-9]+;
HEX: [0-9a-f]*;
