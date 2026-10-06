grammar PngSnapshot;
document: 'stdio.png' 'schema' '=' text 'bytes' '=' STRING EOF;
text: STRING | ATOM;
STRING: '"' ('\\' . | ~["\\])* '"';
ATOM: ~[ \t\r\n"=]+;
WS: [ \t\r\n]+ -> skip;
