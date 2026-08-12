// ANTLR4 grammar for `stdio.semio.model`'s real diff text line — descriptive mirror of the
// authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_model_diff;

document: spatialLine? elementsLine? relationsLine? EOF;

spatialLine: 'spatial' '=' spatialTriple;
spatialTriple: '[' removedList? ']' ';' '[' spatialModifiedList? ']' ';' '[' spatialAddedList? ']';
spatialModifiedList: spatialModified (',' spatialModified)*;
spatialModified: HEX ':' spatialNodeDiff;
spatialAddedList: spatialNode (',' spatialNode)*;

elementsLine: 'elements' '=' elementsTriple;
elementsTriple: '[' removedList? ']' ';' '[' elementModifiedList? ']' ';' '[' elementAddedList? ']';
elementModifiedList: elementModified (',' elementModified)*;
elementModified: HEX ':' elementDiff;
elementAddedList: element (',' element)*;

relationsLine: 'relations' '=' relationsTriple;
relationsTriple: '[' removedList? ']' ';' '[' relationModifiedList? ']' ';' '[' relationAddedList? ']';
relationModifiedList: relationModified (',' relationModified)*;
relationModified: HEX ':' relationDiff;
relationAddedList: relation (',' relation)*;

removedList: HEX (',' HEX)*;

spatialNode: '[' HEX ',' spatialKind ',' HEX ',' optStr ',' transform ']';
spatialKind: 'S' | 'B' | 'T' | 'P';
element: '[' HEX ',' elementClass ',' transform ',' geometryRef ',' optStr ',' '[' psetList? ']' ']';
elementClass: 'WA' | 'SL' | 'CO' | 'BE' | 'DO' | 'WI' | 'RO' | 'ST' | 'FU' | 'OT' '[' HEX ']';
geometryRef: 'N' | 'B' '[' HEX ']' | 'M' '[' HEX ']';
psetList: propertySet (',' propertySet)*;
propertySet: '[' HEX ',' '[' propertyList? ']' ']';
propertyList: property (',' property)*;
property: '[' HEX ',' psetValue ']';
psetValue: 'T' '[' HEX ']' | 'N' '[' number ']' | 'B' '[' bit ']';
bit: '0' | '1';
relation: '[' HEX ',' relationKind ',' HEX ',' HEX ']';
relationKind: 'AG' | 'CI' | 'CN' | 'FV' | 'VE' | 'OT' '[' HEX ']';
optStr: '[' '0' ']' | '[' '1' ',' HEX ']';
transform: '[' point3 ',' quat ',' point3 ']';
point3: '[' number ',' number ',' number ']';
quat: '[' number ',' number ',' number ',' number ']';

spatialNodeDiff: '[' optionSpatialKind ',' optionHex ',' optionOptionHex ',' optionTransform ']';
optionSpatialKind: '[' '0' ']' | '[' '1' ',' spatialKind ']';
optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionOptionHex: '[' '0' ']' | '[' '1' ',' optionHex ']';
optionTransform: '[' '0' ']' | '[' '1' ',' transform ']';

elementDiff: '[' optionElementClass ',' optionTransform ',' optionGeometryRef ',' optionOptionHex ',' optionPsets ']';
optionElementClass: '[' '0' ']' | '[' '1' ',' elementClass ']';
optionGeometryRef: '[' '0' ']' | '[' '1' ',' geometryRef ']';
optionPsets: '[' '0' ']' | '[' '1' ',' '[' psetList? ']' ']';

relationDiff: '[' optionRelationKind ',' optionHex ',' optionHex ']';
optionRelationKind: '[' '0' ']' | '[' '1' ',' relationKind ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
