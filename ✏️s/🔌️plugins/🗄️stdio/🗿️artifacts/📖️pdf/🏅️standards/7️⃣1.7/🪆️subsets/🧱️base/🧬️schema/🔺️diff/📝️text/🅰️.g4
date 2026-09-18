// ANTLR4 grammar for PdfDiff's one-line derive-owned text record: the sparse typed diff value
// printed by the shared DSL value printer.
grammar Stdio_pdf_1_7_Diff;

pdfDiff: 'value=' value EOF;
value: record | list | STRING | NUMBER | 'true' | 'false' | 'null';
record: '{' (field (',' field)*)? '}';
field: KEY ':' value;
list: '[' (value (',' value)*)? ']';

KEY: [A-Za-z_] [A-Za-z0-9_]*;
STRING: '"' (~["\\] | '\\' .)* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
WS: [ \t]+ -> skip;
