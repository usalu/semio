grammar Stdio_gltf_mutations;
// Wire form of GltfMutation: RFC8259 JSON, internally tagged on "mutation".
document: mutationObject EOF;
mutationObject: '{' '"mutation"' ':' variantTag (',' member)* '}';
variantTag: STRING; // one of the 24 GltfMutation variant names, camelCase
member: key=STRING ':' jsonValue;
jsonValue: jsonObject | jsonArray | STRING | NUMBER | 'true' | 'false' | 'null';
jsonObject: '{' (member (',' member)*)? '}';
jsonArray: '[' (jsonValue (',' jsonValue)*)? ']';
STRING: '"' (ESC | ~["\\])* '"';
fragment ESC: '\\' (["\\/bfnrt] | 'u' HEX HEX HEX HEX);
fragment HEX: [0-9a-fA-F];
NUMBER: '-'? ('0' | [1-9][0-9]*) ('.' [0-9]+)? ([eE] [+-]? [0-9]+)?;
WS: [ \t\n\r]+ -> skip;
