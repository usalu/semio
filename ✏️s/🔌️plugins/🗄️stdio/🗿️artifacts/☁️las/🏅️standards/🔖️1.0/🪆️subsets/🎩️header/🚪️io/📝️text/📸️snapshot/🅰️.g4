grammar Stdio_las_snapshot;
// stdio.las DSL text form: a `semio stdio.las vN` preamble line, then the body as lowercase hex
// pairs of the real LAS binary buffer (whitespace-insignificant -- ArtifactDsl::parse_dsl strips
// all whitespace before decoding). See the binary facet's component.ksy/component.protocol.semio
// for the real header+VLR+point-record byte layout this hex payload decodes to.
document   : preamble NEWLINE hexBody EOF ;
preamble   : 'semio' WS 'stdio.las' WS 'v' INT ;
hexBody    : (HEXPAIR | WS | NEWLINE)* ;
HEXPAIR    : [0-9a-f] [0-9a-f] ;
INT        : [0-9]+ ;
WS         : ' '+ ;
NEWLINE    : '\r\n' | '\n' ;
