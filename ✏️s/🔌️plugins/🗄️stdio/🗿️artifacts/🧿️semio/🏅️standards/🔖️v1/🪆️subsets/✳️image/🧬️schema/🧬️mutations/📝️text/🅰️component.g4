// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// ../📖️component.grammar.semio, walked by dsl::Recognizer) for `SemioImageMutation::print_op`/
// `parse_op` (../../🦀️component.rs's `print_image_mutation`/`parse_image_mutation`).
grammar Stdio_semio_image_mutations;

op : noOp | setSnapshot | setDimensions | setColorspace | setBitDepth | setIcc | insertFrame | removeFrame | moveFrame | setFrameDelay | setFramePixels | setMetadataEntry | removeMetadataEntry ;

noOp                 : 'no' ;
setSnapshot          : 'setSnapshot' ':' snapshot ;
setDimensions        : 'setDimensions' ':' INT ',' INT ;
setColorspace        : 'setColorspace' ':' colorspace ;
setBitDepth          : 'setBitDepth' ':' INT ;
setIcc                : 'setIcc' ':' optionHex ;
insertFrame           : 'insertFrame' ':' INT ',' frame ;
removeFrame            : 'removeFrame' ':' INT ;
moveFrame              : 'moveFrame' ':' INT ',' INT ;
setFrameDelay          : 'setFrameDelay' ':' INT ',' INT ;
setFramePixels         : 'setFramePixels' ':' INT ',' HEX ;
setMetadataEntry       : 'setMetadataEntry' ':' HEX ',' HEX ;
removeMetadataEntry    : 'removeMetadataEntry' ':' HEX ;

snapshot   : '[' INT ',' INT ',' colorspace ',' INT ',' optionHex ',' '[' frameList? ']' ',' '[' entryList? ']' ']' ;
frameList  : frame (',' frame)* ;
frame      : '[' INT ',' HEX ']' ;
entryList  : entry (',' entry)* ;
entry      : '[' HEX ',' HEX ']' ;
colorspace : 'r' | 'a' | 'g' | 'y' | 'i' ;
optionHex  : '[' '0' ']' | '[' '1' ',' HEX ']' ;

HEX : [0-9a-f]* ;
INT : '-'? [0-9]+ ;
WS  : [ \t\r\n]+ -> skip ;
