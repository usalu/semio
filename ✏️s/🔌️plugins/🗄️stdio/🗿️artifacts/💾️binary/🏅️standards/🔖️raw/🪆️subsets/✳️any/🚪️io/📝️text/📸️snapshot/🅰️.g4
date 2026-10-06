grammar Stdio_binary_snapshot;
// stdio.binary DSL text form: a `semio` preamble line, then the body as lowercase hex pairs
// (whitespace-insignificant -- ArtifactDsl::parse_dsl strips all whitespace before decoding).
document   : preamble NEWLINE hexBody EOF ;
preamble   : 'semio' WS envelopeId WS 'v' INT ;
envelopeId : IDENT ('.' IDENT)+ ;
hexBody    : (HEXPAIR | WS | NEWLINE)* ;
HEXPAIR    : [0-9a-f] [0-9a-f] ;
IDENT      : [a-zA-Z_][a-zA-Z0-9_-]* ;
INT        : [0-9]+ ;
WS         : ' '+ ;
NEWLINE    : '\r\n' | '\n' ;
