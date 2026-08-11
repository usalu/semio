grammar Stdio_gltf_diff;
// Wire form of GltfDiff: RFC8259 JSON, a sparse object (every member optional -- present iff that
// field changed). Collection fields share the generic {removed,modified,added} triple shape.
document: diffObject EOF;
diffObject: '{' (member (',' member)*)? '}';
member: STRING ':' jsonValue; // key is one of GltfDiff's field names; value shape depends on key
collectionDiff: '{' ( 'removed' ':' indexArray )? (',' 'modified' ':' modifiedArray)? (',' 'added' ':' addedArray)? '}';
indexArray: '[' (NUMBER (',' NUMBER)*)? ']';
modifiedArray: '[' (modifiedEntry (',' modifiedEntry)*)? ']';
modifiedEntry: '{' 'index' ':' NUMBER ',' 'diff' ':' jsonValue '}';
addedArray: '[' (addedEntry (',' addedEntry)*)? ']';
addedEntry: '{' 'index' ':' NUMBER ',' 'item' ':' jsonValue '}';
jsonValue: jsonObject | jsonArray | STRING | NUMBER | 'true' | 'false' | 'null';
jsonObject: '{' (member (',' member)*)? '}';
jsonArray: '[' (jsonValue (',' jsonValue)*)? ']';
STRING: '"' (ESC | ~["\\])* '"';
fragment ESC: '\\' (["\\/bfnrt] | 'u' HEX HEX HEX HEX);
fragment HEX: [0-9a-fA-F];
NUMBER: '-'? ('0' | [1-9][0-9]*) ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
WS: [ \t\n\r]+ -> skip;
