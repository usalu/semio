grammar Stdio_txt_mutation;
// TxtMutation delegates its payload codec to the opcode-selected direct leaf.
document      : object EOF ;
object        : '{' '"mutation"' ':' kind (',' field)* '}' ;
kind          : '"setTrailingNewline"' | '"setLineEnding"'
              | '"insertLine"' | '"removeLine"' | '"setLine"' | '"spliceText"' ;
field         : snapshotF | valueF | indexF | textF | splicesF ;
snapshotF     : '"snapshot"' ':' snapshotObj ;
valueF        : '"value"' ':' (BOOL | '"lf"' | '"crLf"') ;
indexF        : '"index"' ':' INT ;
textF         : '"text"' ':' STRING ;
splicesF      : '"splices"' ':' '[' (spliceObj (',' spliceObj)*)? ']' ;
spliceObj     : '{' '"offset"' ':' INT ',' '"delete"' ':' INT ',' '"insert"' ':' STRING '}' ;
snapshotObj   : '{' .*? '}' ;   // see snapshot/text grammar for the real shape
BOOL          : 'true' | 'false' ;
INT           : [0-9]+ ;
STRING        : '"' (~["\\] | '\\' .)* '"' ;
