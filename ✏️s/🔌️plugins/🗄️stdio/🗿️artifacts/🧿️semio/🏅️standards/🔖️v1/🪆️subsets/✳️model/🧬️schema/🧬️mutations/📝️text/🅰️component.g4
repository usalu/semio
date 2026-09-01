// ANTLR4 grammar for `stdio.semio.model`'s real op text line — descriptive mirror of the
// authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_model_mutations;

op: (setSnapshot | insertSpatialNode | removeSpatialNode | setSpatialNode
    | insertElement | removeElement | setElement | insertRelation | removeRelation | setRelation) EOF;

setSnapshot: 'set-snapshot' 'snapshot' '=' snapshotLit;
insertSpatialNode: 'insert-spatial-node' 'node' '=' spatialNode;
removeSpatialNode: 'remove-spatial-node' 'id' '=' HEX;
setSpatialNode: 'set-spatial-node' 'id' '=' HEX 'kind' '=' optionSpatialKind 'name' '=' optionHex 'parent_id' '=' optionOptionHex 'placement' '=' optionTransform;
insertElement: 'insert-element' 'element' '=' element;
removeElement: 'remove-element' 'id' '=' HEX;
setElement: 'set-element' 'id' '=' HEX 'class' '=' optionElementClass 'placement' '=' optionTransform 'geometry' '=' optionGeometryRef 'spatial_id' '=' optionOptionHex 'psets' '=' optionPsets;
insertRelation: 'insert-relation' 'relation' '=' relation;
removeRelation: 'remove-relation' 'id' '=' HEX;
setRelation: 'set-relation' 'id' '=' HEX 'kind' '=' optionRelationKind 'from' '=' optionHex 'to' '=' optionHex;

snapshotLit: '[' HEX ',' '[' spatialList? ']' ',' '[' elementList? ']' ',' '[' relationList? ']' ']';
spatialList: spatialNode (',' spatialNode)*;
elementList: element (',' element)*;
relationList: relation (',' relation)*;

spatialNode: '[' HEX ',' spatialKind ',' HEX ',' optionHex ',' transform ']';
spatialKind: 'S' | 'B' | 'T' | 'P';
element: '[' HEX ',' elementClass ',' transform ',' geometryRef ',' optionHex ',' '[' psetList? ']' ']';
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
transform: '[' point3 ',' quat ',' point3 ']';
point3: '[' number ',' number ',' number ']';
quat: '[' number ',' number ',' number ',' number ']';

optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionOptionHex: '[' '0' ']' | '[' '1' ',' optionHex ']';
optionSpatialKind: '[' '0' ']' | '[' '1' ',' spatialKind ']';
optionTransform: '[' '0' ']' | '[' '1' ',' transform ']';
optionElementClass: '[' '0' ']' | '[' '1' ',' elementClass ']';
optionGeometryRef: '[' '0' ']' | '[' '1' ',' geometryRef ']';
optionPsets: '[' '0' ']' | '[' '1' ',' '[' psetList? ']' ']';
optionRelationKind: '[' '0' ']' | '[' '1' ',' relationKind ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
