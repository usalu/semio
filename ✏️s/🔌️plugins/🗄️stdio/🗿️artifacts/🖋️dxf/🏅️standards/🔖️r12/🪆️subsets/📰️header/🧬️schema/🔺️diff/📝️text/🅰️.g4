// 🅰️ `DxfDiff`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, no bespoke textual syntax). This grammar names the real fields
// rather than a placeholder — it does not restate RFC 8259's own JSON grammar in full.
grammar Stdio_dxf_diff;

diff        : '{' member (',' member)* '}' | '{' '}' ;
member      : HEADERVARS ':' namedDiff
            | TABLES ':' tablesDiff
            | BLOCKS ':' indexedDiff
            | ENTITIES ':' indexedDiff ;
tablesDiff  : '{' tmember (',' tmember)* '}' | '{' '}' ;
tmember     : LAYERS ':' namedDiff | STYLES ':' namedDiff | LINETYPES ':' namedDiff ;
namedDiff   : '{' .*? '}' ;   // removed: string[], modified: {name,diff}[], added: {index,<item>}[]
indexedDiff : '{' .*? '}' ;   // removed: number[], modified: {index,diff}[], added: {index,<item>}[]

HEADERVARS  : '"headerVars"' ;
TABLES      : '"tables"' ;
BLOCKS      : '"blocks"' ;
ENTITIES    : '"entities"' ;
LAYERS      : '"layers"' ;
STYLES      : '"styles"' ;
LINETYPES   : '"linetypes"' ;
