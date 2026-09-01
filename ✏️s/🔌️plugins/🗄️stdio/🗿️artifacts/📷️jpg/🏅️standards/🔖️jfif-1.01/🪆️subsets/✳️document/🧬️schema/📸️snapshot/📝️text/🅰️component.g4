grammar Stdio_jpg_snapshot;
// 🅰️ TEXT envelope: a semio preamble line, then the real JFIF byte stream hex-encoded
// (ITU-T T.81/ISO 10918-1) -- what `store::ArtifactDsl::parse_dsl`/`print_dsl` actually
// round-trip, not a placeholder octet blob.

document   : preamble NEWLINE payload EOF ;
preamble   : 'semio' WS 'dsl' WS INT WS 'stdio.jpg' ;
payload    : HEXBYTE* ;                 // hex-decodes to a real jpeg-stream, see the
                                          // 📖 grammar.semio leaf for the marker-segment shape
HEXBYTE    : [0-9a-fA-F] [0-9a-fA-F] ;
INT        : [0-9]+ ;
WS         : [ \t]+ ;
NEWLINE    : '\r'? '\n' ;
