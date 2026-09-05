// 🅰️ `ObjDiff`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, no bespoke textual syntax). This grammar names the real fields
// rather than a placeholder — it does not restate RFC 8259's own JSON grammar in full.
grammar Stdio_obj_diff;

diff        : '{' member (',' member)* '}' | '{' '}' ;
member      : VERTICES ':' collectionDiff
            | TEXCOORDS ':' collectionDiff
            | NORMALS ':' collectionDiff
            | FACES ':' collectionDiff
            | GROUPS ':' namedDiff
            | OBJECTS ':' namedDiff
            | MTLLIB ':' (STRING | 'null')
            | USEMTL ':' array
            | SMOOTHING ':' array
            | UNKNOWN ':' array ;
collectionDiff : '{' member (',' member)* '}' | '{' '}' ;
namedDiff   : '{' member (',' member)* '}' | '{' '}' ;
array       : '[' .*? ']' ;

VERTICES    : '"vertices"' ;
TEXCOORDS   : '"texcoords"' ;
NORMALS     : '"normals"' ;
FACES       : '"faces"' ;
GROUPS      : '"groups"' ;
OBJECTS     : '"objects"' ;
MTLLIB      : '"mtllib"' ;
USEMTL      : '"usemtl"' ;
SMOOTHING   : '"smoothingGroups"' ;
UNKNOWN     : '"unknownStatements"' ;
STRING      : '"' .*? '"' ;
