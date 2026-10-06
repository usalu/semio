// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// ../📖️.grammar.semio, walked by dsl::Recognizer) for `SemioImageDiff::print_diff`/
// `parse_diff` (../../🦀️.rs's `print_image_diff`/`parse_image_diff`).
grammar Stdio_semio_image_diff;

document       : widthLine? heightLine? colorspaceLine? bitDepthLine? iccLine? framesLine? metadataLine? ;
widthLine      : 'width' '=' INT ;
heightLine     : 'height' '=' INT ;
colorspaceLine : 'colorspace' '=' colorspace ;
bitDepthLine   : 'bitDepth' '=' INT ;
iccLine        : 'icc' '=' optionHex ;

framesLine        : 'frames' '{' framesTriple '}' ;
framesTriple      : '[' removedIndexList? ']' ';' '[' frameModifiedList? ']' ';' '[' frameAddedList? ']' ;
removedIndexList  : INT (',' INT)* ;
frameModifiedList : frameModified (',' frameModified)* ;
frameModified     : INT ':' frameDiff ;
frameDiff         : '[' frameDiffEntryList? ']' ;
frameDiffEntryList: frameDiffEntry (',' frameDiffEntry)* ;
frameDiffEntry    : 'D' ':' INT | 'X' ':' HEX ;
frameAddedList    : frameAdded (',' frameAdded)* ;
frameAdded        : INT ':' frame ;
frame             : '[' INT ',' HEX ']' ;

metadataLine        : 'metadata' '{' metadataTriple '}' ;
metadataTriple       : '[' removedKeyList? ']' ';' '[' metadataModifiedList? ']' ';' '[' metadataAddedList? ']' ;
removedKeyList       : HEX (',' HEX)* ;
metadataModifiedList : metadataModified (',' metadataModified)* ;
metadataModified     : HEX ':' HEX ;
metadataAddedList    : entry (',' entry)* ;
entry                : '[' HEX ',' HEX ']' ;

colorspace : 'r' | 'a' | 'g' | 'y' | 'i' ;
optionHex  : '[' '0' ']' | '[' '1' ',' HEX ']' ;

HEX : [0-9a-f]* ;
INT : '-'? [0-9]+ ;
WS  : [ \t\r\n]+ -> skip ;
