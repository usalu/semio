// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// ../📖️component.grammar.semio, walked by dsl::Recognizer) for s.stdio.semio.image's DSL text
// representation (store::ArtifactDsl::parse_dsl/print_dsl, ../../🦀️component.rs's
// print_image_snapshot_body/parse_image_snapshot_body). The `semio s.stdio.semio.image.dsl v1`
// preamble line is stripped by store::semio_format::split_text_preamble before this grammar's
// `document` production runs — `document` below matches the RECONSTRUCTED body (bare
// `artifactMark` token standing in for the stripped preamble), same convention every other real
// pilot's own `.g4` mirror uses.
grammar Stdio_semio_image_snapshot;

document        : artifactMark schemaLine widthLine heightLine colorspaceLine bitDepthLine iccLine framesLine metadataLine ;
artifactMark    : 's.stdio.semio.image' ;
schemaLine      : 'schema' '=' HEX ;
widthLine       : 'width' '=' INT ;
heightLine      : 'height' '=' INT ;
colorspaceLine  : 'colorspace' '=' colorspace ;
bitDepthLine    : 'bitDepth' '=' INT ;
iccLine         : 'icc' '=' optionHex ;

framesLine      : 'frames' '=' '[' frameList? ']' ;
frameList       : frame (',' frame)* ;
frame           : '[' INT ',' HEX ']' ;

metadataLine    : 'metadata' '=' '[' entryList? ']' ;
entryList       : entry (',' entry)* ;
entry           : '[' HEX ',' HEX ']' ;

colorspace      : 'r' | 'a' | 'g' | 'y' | 'i' ;
optionHex       : '[' '0' ']' | '[' '1' ',' HEX ']' ;

HEX   : [0-9a-f]* ;
INT   : '-'? [0-9]+ ;
WS    : [ \t\r\n]+ -> skip ;
