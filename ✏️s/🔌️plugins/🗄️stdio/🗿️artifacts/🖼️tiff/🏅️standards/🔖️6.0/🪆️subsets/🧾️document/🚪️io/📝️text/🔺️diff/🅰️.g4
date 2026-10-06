// 🅰️ `TiffDiff`'s wire text IS its JSON serialization (serde `camelCase`, sparse — see
// 🦀️.rs). This grammar names the real top-level fields rather than a placeholder;
// it does not restate RFC 8259's own JSON grammar in full.
grammar Stdio_tiff_diff;

diff       : '{' member (',' member)* '}' | '{' '}' ;
member     : BYTE_ORDER ':' STRING | IFDS ':' ifdsTriple | PIXELS ':' '[' INT* ']' ;
ifdsTriple : '{' '}' | '{' 'removed' ':' '[' INT* ']' (',' 'modified' ':' '[' ifdModified* ']')? (',' 'added' ':' '[' value* ']')? '}' ;
ifdModified: '{' 'index' ':' INT ',' 'diff' ':' tagsTriple '}' ;
tagsTriple : '{' '}' | '{' 'removed' ':' '[' INT* ']' (',' 'modified' ':' '[' value* ']')? (',' 'added' ':' '[' value* ']')? '}' ;
value      : '{' member2 (',' member2)* '}' | STRING | INT | BOOL | 'null' ;
member2    : STRING ':' value ;

BYTE_ORDER: '"byteOrder"'; IFDS: '"ifds"'; PIXELS: '"pixels"';
BOOL: 'true' | 'false';
INT: [0-9]+;
STRING: '"' (~["\\] | '\\' .)* '"';
