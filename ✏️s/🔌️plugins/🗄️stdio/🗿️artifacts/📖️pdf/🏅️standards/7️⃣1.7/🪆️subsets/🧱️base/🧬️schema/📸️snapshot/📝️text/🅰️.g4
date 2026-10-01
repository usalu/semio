grammar Stdio_pdf_1_7_snapshot;
document: 'stdio.pdf.1.7' field* EOF;
field: IDENT '=' value;
value: STRING | NUMBER | IDENT | '[' value* ']' | '{' member* '}';
member: (IDENT | STRING) '=' value;
NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
IDENT: [a-zA-Z_] [a-zA-Z_0-9.]*;
STRING: '"' ('\\' . | ~["\\])* '"';
WS: [ \t\r\n]+ -> skip;
