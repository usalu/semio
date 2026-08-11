// 🅰️ `PngMutation`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, `#[serde(tag = "mutation")]`, no bespoke textual syntax). This
// grammar names the real 17 variant tags rather than a placeholder.
grammar Stdio_png_mutation;

mutation : '{' TAG ':' variant (',' member)* '}' ;
variant  : NO_MUTATION | SET_SNAPSHOT | SET_HEADER | SET_PALETTE | SET_TRANSPARENCY
         | SET_GAMMA | SET_CHROMATICITIES | SET_SRGB_INTENT | SET_PHYSICAL_DIMS
         | SET_TIMESTAMP | SET_BACKGROUND | INSERT_TEXT_CHUNK | REMOVE_TEXT_CHUNK
         | SET_TEXT_CHUNK | SET_PIXELS | INSERT_UNKNOWN_CHUNK | REMOVE_UNKNOWN_CHUNK ;
member   : STRING ':' value ;
value    : '{' member (',' member)* '}' | '[' value* ']' | STRING | INT | BOOL | 'null' ;

TAG: '"mutation"';
NO_MUTATION: '"noMutation"'; SET_SNAPSHOT: '"setSnapshot"'; SET_HEADER: '"setHeader"';
SET_PALETTE: '"setPalette"'; SET_TRANSPARENCY: '"setTransparency"'; SET_GAMMA: '"setGamma"';
SET_CHROMATICITIES: '"setChromaticities"'; SET_SRGB_INTENT: '"setSrgbIntent"';
SET_PHYSICAL_DIMS: '"setPhysicalDims"'; SET_TIMESTAMP: '"setTimestamp"';
SET_BACKGROUND: '"setBackground"'; INSERT_TEXT_CHUNK: '"insertTextChunk"';
REMOVE_TEXT_CHUNK: '"removeTextChunk"'; SET_TEXT_CHUNK: '"setTextChunk"';
SET_PIXELS: '"setPixels"'; INSERT_UNKNOWN_CHUNK: '"insertUnknownChunk"';
REMOVE_UNKNOWN_CHUNK: '"removeUnknownChunk"';
BOOL: 'true' | 'false';
INT: [0-9]+;
STRING: '"' (~["\\] | '\\' .)* '"';
