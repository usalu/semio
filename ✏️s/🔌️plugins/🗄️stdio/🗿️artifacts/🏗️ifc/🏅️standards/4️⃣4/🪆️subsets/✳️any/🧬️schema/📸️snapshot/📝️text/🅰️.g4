grammar Stdio_ifc_snapshot;
// Real ISO 10303-21 (Part-21) exchange-structure grammar (ANTLR4).

exchangeFile: 'ISO-10303-21;' header data 'END-ISO-10303-21;' ;
header: 'HEADER;' fileDescription fileName fileSchema 'ENDSEC;' ;
fileDescription: 'FILE_DESCRIPTION' '(' valueList ')' ';' ;
fileName: 'FILE_NAME' '(' valueList ')' ';' ;
fileSchema: 'FILE_SCHEMA' '(' valueList ')' ';' ;
data: 'DATA;' instance* 'ENDSEC;' ;
instance: '#' ID '=' instanceBody ';' ;
instanceBody: simpleRecord | complexRecord ;
simpleRecord: KEYWORD '(' valueList? ')' ;
complexRecord: '(' simpleRecord+ ')' ;
valueList: value (',' value)* ;
value: UNSET | DERIVED | reference | STRING | ENUM | REAL | INT | aggregate | typedValue ;
reference: '#' ID ;
aggregate: '(' valueList? ')' ;
typedValue: KEYWORD '(' valueList? ')' ;

UNSET: '$' ;
DERIVED: '*' ;
KEYWORD: [A-Z] [A-Z0-9_]* ;
ID: [0-9]+ ;
STRING: '\'' ( '\'\'' | ~['] )* '\'' ;
ENUM: '.' [A-Z0-9_]* '.' ;
REAL: '-'? [0-9]+ '.' [0-9]* ( [Ee] [+-]? [0-9]+ )? ;
INT: '-'? [0-9]+ ;
WS: [ \t\r\n]+ -> skip ;
COMMENT: '/*' .*? '*/' -> skip ;
