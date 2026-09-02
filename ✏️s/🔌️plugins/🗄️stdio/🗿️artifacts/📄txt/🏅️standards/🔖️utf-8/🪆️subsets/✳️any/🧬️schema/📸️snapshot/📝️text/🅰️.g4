grammar Stdio_txt_snapshot;
// A stdio.txt DSL document: a `semio` preamble line, then the line-sequence body verbatim.
document : preamble NEWLINE body EOF ;
preamble : 'semio' WS envelopeId WS 'v' INT ;
envelopeId : IDENT ('.' IDENT)+ ;
body : (lineContent (lineBreak lineContent)*)? lineBreak? ;
lineContent : ~[\r\n]* ;
lineBreak : '\r\n' | '\n' ;
IDENT : [a-zA-Z_][a-zA-Z0-9_-]* ;
INT : [0-9]+ ;
WS : ' '+ ;
NEWLINE : '\r\n' | '\n' ;
