// 🅰️ `BmpDiff`'s wire text IS its JSON serialization (serde, camelCase, no bespoke textual
// syntax). This grammar names the real sparse fields rather than a placeholder — it does
// not restate RFC 8259's own JSON grammar in full.
grammar Stdio_bmp_diff;

diff       : '{' member (',' member)* '}' | '{' '}' ;
member     : HEADER_SIZE ':' INT
           | WIDTH ':' INT
           | HEIGHT ':' INT
           | ROW_ORDER ':' rowOrder
           | PLANES ':' INT
           | BITS_PER_PIXEL ':' INT
           | COMPRESSION ':' INT
           | IMAGE_SIZE ':' INT
           | X_PPM ':' INT
           | Y_PPM ':' INT
           | COLORS_USED ':' INT
           | COLORS_IMPORTANT ':' INT
           | PALETTE ':' paletteDiff
           | PIXELS ':' '[' INT* ']' ;
rowOrder   : '"bottomUp"' | '"topDown"' ;
paletteDiff: '{' pmember (',' pmember)* '}' | '{' '}' ;
pmember    : REMOVED ':' '[' INT* ']'
           | MODIFIED ':' '[' paletteModified* ']'
           | ADDED ':' '[' paletteAdded* ']' ;
paletteModified : '{' INDEX ':' INT ',' ENTRY ':' paletteEntry '}' ;
paletteAdded    : '{' INDEX ':' INT ',' ENTRY ':' paletteEntry '}' ;
paletteEntry    : '{' '"b"' ':' INT ',' '"g"' ':' INT ',' '"r"' ':' INT ',' '"reserved"' ':' INT '}' ;

HEADER_SIZE: '"headerSize"' ;
WIDTH: '"width"' ;
HEIGHT: '"height"' ;
ROW_ORDER: '"rowOrder"' ;
PLANES: '"planes"' ;
BITS_PER_PIXEL: '"bitsPerPixel"' ;
COMPRESSION: '"compression"' ;
IMAGE_SIZE: '"imageSize"' ;
X_PPM: '"xPixelsPerMeter"' ;
Y_PPM: '"yPixelsPerMeter"' ;
COLORS_USED: '"colorsUsed"' ;
COLORS_IMPORTANT: '"colorsImportant"' ;
PALETTE: '"palette"' ;
PIXELS: '"pixels"' ;
REMOVED: '"removed"' ;
MODIFIED: '"modified"' ;
ADDED: '"added"' ;
INDEX: '"index"' ;
ENTRY: '"entry"' ;
INT: [0-9]+ ;
