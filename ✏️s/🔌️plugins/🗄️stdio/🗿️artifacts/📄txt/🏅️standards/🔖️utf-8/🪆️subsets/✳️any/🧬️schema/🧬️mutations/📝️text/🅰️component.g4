grammar Stdio_txt_mutation;
// TxtMutation delegates its payload codec to the opcode-selected direct leaf.
document      : object EOF ;
object        : '{' '"mutation"' ':' kind (',' field)* '}' ;
kind          : '"noMutation"' | '"setSnapshot"' | '"setTrailingNewline"' | '"setLineEnding"'
              | '"insertLine"' | '"removeLine"' | '"setLine"' ;
field         : snapshotF | valueF | indexF | textF ;
snapshotF     : '"snapshot"' ':' snapshotObj ;
valueF        : '"value"' ':' (BOOL | '"lf"' | '"crLf"') ;
indexF        : '"index"' ':' INT ;
textF         : '"text"' ':' STRING ;
snapshotObj   : '{' .*? '}' ;   // see snapshot/text grammar for the real shape
BOOL          : 'true' | 'false' ;
INT           : [0-9]+ ;
STRING        : '"' (~["\\] | '\\' .)* '"' ;
