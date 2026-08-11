// 🅰️ `TiffMutation`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, `#[serde(tag = "mutation")]`, no bespoke textual syntax). This
// grammar names the real 8 variant tags rather than a placeholder.
grammar Stdio_tiff_mutation;

mutation : '{' TAG ':' variant (',' member)* '}' ;
variant  : NO_MUTATION | SET_SNAPSHOT | SET_BYTE_ORDER | INSERT_IFD
         | REMOVE_IFD | SET_TAG | REMOVE_TAG | SET_PIXELS ;
member   : STRING ':' value ;
value    : '{' member (',' member)* '}' | '[' value* ']' | STRING | INT | BOOL | 'null' ;

TAG: '"mutation"';
NO_MUTATION: '"noMutation"'; SET_SNAPSHOT: '"setSnapshot"'; SET_BYTE_ORDER: '"setByteOrder"';
INSERT_IFD: '"insertIfd"'; REMOVE_IFD: '"removeIfd"'; SET_TAG: '"setTag"';
REMOVE_TAG: '"removeTag"'; SET_PIXELS: '"setPixels"';
BOOL: 'true' | 'false';
INT: [0-9]+;
STRING: '"' (~["\\] | '\\' .)* '"';
