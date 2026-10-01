grammar Stdio_pdf_snapshot;
document: 'schema' '=' text 'pages' '{' '[' page* ']' '}' EOF;
page: 'width' '=' number 'height' '=' number 'text' '=' text;
text: STRING | IDENT;
number: NUMBER | IDENT;
NUMBER: '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
IDENT: [a-zA-Z_] [a-zA-Z_0-9]*;
STRING: '"' ('\\' . | ~["\\])* '"';
WS: [ \t\r\n]+ -> skip;
