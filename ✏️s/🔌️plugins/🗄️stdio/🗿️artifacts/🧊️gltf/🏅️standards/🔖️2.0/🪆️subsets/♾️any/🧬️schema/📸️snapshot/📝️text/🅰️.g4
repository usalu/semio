grammar Stdio_gltf_snapshot;
// A `.gltf` document: RFC8259 JSON text whose root object carries the glTF 2.0 (§5) top-level
// members. `jsonValue`/`jsonObject`/`jsonArray`/`jsonString`/`jsonNumber` are the standard JSON
// productions; `gltfDocument` narrows the root object to the real glTF shape.
document: gltfDocument EOF;
gltfDocument: '{' 'asset' ':' assetObject (',' member)* '}';
assetObject: '{' 'version' ':' STRING (',' ('generator'|'copyright'|'minVersion') ':' STRING)* '}';
member: key=STRING ':' jsonValue;
jsonValue: jsonObject | jsonArray | STRING | NUMBER | 'true' | 'false' | 'null';
jsonObject: '{' (member (',' member)*)? '}';
jsonArray: '[' (jsonValue (',' jsonValue)*)? ']';
STRING: '"' (ESC | ~["\\])* '"';
fragment ESC: '\\' (["\\/bfnrt] | UNICODE);
fragment UNICODE: 'u' HEX HEX HEX HEX;
fragment HEX: [0-9a-fA-F];
NUMBER: '-'? INT ('.' [0-9]+)? EXP?;
fragment INT: '0' | [1-9] [0-9]*;
fragment EXP: [eE] [+-]? [0-9]+;
WS: [ \t\n\r]+ -> skip;
