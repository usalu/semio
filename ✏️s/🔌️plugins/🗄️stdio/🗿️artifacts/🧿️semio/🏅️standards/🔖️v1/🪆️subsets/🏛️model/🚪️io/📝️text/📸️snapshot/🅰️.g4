// ANTLR4 grammar for `stdio.semio.model`'s real text DSL body (the `SemioModelSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_model_snapshot;

document: artifactMark schemaLine spatialLine elementsLine relationsLine EOF;
artifactMark: 'stdio.semio.model';

schemaLine: 'schema' '=' HEX;

spatialLine: 'spatial' '=' '[' spatialList? ']';
spatialList: spatialNode (',' spatialNode)*;
spatialNode: '[' HEX ',' spatialKind ',' HEX ',' optStr ',' transform ']';
spatialKind: 'S' | 'B' | 'T' | 'P';

elementsLine: 'elements' '=' '[' elementList? ']';
elementList: element (',' element)*;
element: '[' HEX ',' elementClass ',' transform ',' geometryRef ',' optStr ',' '[' psetList? ']' ']';
elementClass: 'WA' | 'SL' | 'CO' | 'BE' | 'DO' | 'WI' | 'RO' | 'ST' | 'FU' | 'OT' '[' HEX ']';
geometryRef: 'N' | 'B' '[' HEX ']' | 'M' '[' HEX ']';
psetList: propertySet (',' propertySet)*;
propertySet: '[' HEX ',' '[' propertyList? ']' ']';
propertyList: property (',' property)*;
property: '[' HEX ',' psetValue ']';
psetValue: 'T' '[' HEX ']' | 'N' '[' number ']' | 'B' '[' bit ']';
bit: '0' | '1';

relationsLine: 'relations' '=' '[' relationList? ']';
relationList: relation (',' relation)*;
relation: '[' HEX ',' relationKind ',' HEX ',' HEX ']';
relationKind: 'AG' | 'CI' | 'CN' | 'FV' | 'VE' | 'OT' '[' HEX ']';

optStr: '[' '0' ']' | '[' '1' ',' HEX ']';
transform: '[' point3 ',' quat ',' point3 ']';
point3: '[' number ',' number ',' number ']';
quat: '[' number ',' number ',' number ',' number ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
