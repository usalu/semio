// 🅰️ `PngDiff`'s wire text IS its JSON serialization (serde `camelCase`, sparse — see
// 🦀️.rs). This grammar names the real top-level fields rather than a placeholder;
// it does not restate RFC 8259's own JSON grammar in full.
grammar Stdio_png_diff;

diff   : '{' member (',' member)* '}' | '{' '}' ;
member : WIDTH ':' INT | HEIGHT ':' INT | BIT_DEPTH ':' INT | COLOR_TYPE ':' STRING
       | INTERLACE ':' BOOL | PLTE ':' plteDiff | TRNS ':' value | GAMA ':' value
       | CHRM ':' value | SRGB ':' value | PHYS ':' value | TIME ':' value | BKGD ':' value
       | TEXT_CHUNKS ':' triple | PIXELS ':' '[' INT* ']' | CHUNK_ORDER ':' triple
       | UNKNOWN_CHUNKS ':' triple ;
plteDiff : '{' '}' | 'null' | triple ;
triple   : '{' '}' | '{' 'removed' ':' '[' INT* ']' (',' 'modified' ':' '[' value* ']')? (',' 'added' ':' '[' value* ']')? '}' ;
value    : '{' member (',' member)* '}' | STRING | INT | BOOL | 'null' ;

WIDTH: '"width"'; HEIGHT: '"height"'; BIT_DEPTH: '"bitDepth"'; COLOR_TYPE: '"colorType"';
INTERLACE: '"interlace"'; PLTE: '"plte"'; TRNS: '"trns"'; GAMA: '"gama"'; CHRM: '"chrm"';
SRGB: '"srgb"'; PHYS: '"phys"'; TIME: '"time"'; BKGD: '"bkgd"'; TEXT_CHUNKS: '"textChunks"';
PIXELS: '"pixels"'; CHUNK_ORDER: '"chunkOrder"'; UNKNOWN_CHUNKS: '"unknownChunks"';
BOOL: 'true' | 'false';
INT: [0-9]+;
STRING: '"' (~["\\] | '\\' .)* '"';
