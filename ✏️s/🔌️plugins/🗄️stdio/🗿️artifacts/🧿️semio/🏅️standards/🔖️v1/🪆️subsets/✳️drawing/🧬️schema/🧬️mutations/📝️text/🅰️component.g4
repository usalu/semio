grammar Stdio_semio_drawing_mutation_text;
// 📖️ Real `protocol::OpText` grammar (🦀️component.rs `print_op`/`parse_op`): one line of
// compact, `mutation`-tagged JSON -- one of 18 named variants (see 🔣️component.json for the
// full per-variant field union).
document: jsonObject EOF;
jsonObject: '{' '"mutation"' ':' mutationTag (',' member)* '}';
mutationTag: 'noMutation' | 'setSnapshot' | 'setCanvasSize' | 'setCanvasBackground' | 'setStyle'
           | 'removeStyle' | 'insertLayer' | 'removeLayer' | 'setLayerMeta' | 'moveLayer'
           | 'setGroupTransform' | 'setPathSegments' | 'setNodeStyle' | 'setText' | 'setImage'
           | 'insertNode' | 'removeNode' | 'replaceNode';
member: STRING ':' jsonValue;
jsonValue: STRING | NUMBER | jsonObject | '[' (jsonValue (',' jsonValue)*)? ']' | 'true' | 'false' | 'null';
STRING: '"' .*? '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
