grammar Stdio_txt_diff;
// TxtDiff is transported as JSON (serde_json), matching the Rust `TxtDiff` struct exactly.
document       : object EOF ;
object         : '{' (member (',' member)*)? '}' ;
member         : trailingNewlineM | lineEndingM | linesM ;
trailingNewlineM : '"trailingNewline"' ':' BOOL ;
lineEndingM    : '"lineEnding"' ':' ('"lf"' | '"crLf"') ;
linesM         : '"lines"' ':' linesDiff ;
linesDiff      : '{' '"removed"' ':' intArray ',' '"modified"' ':' modArray ',' '"added"' ':' addArray '}' ;
intArray       : '[' (INT (',' INT)*)? ']' ;
modArray       : '[' (lineEntry (',' lineEntry)*)? ']' ;
addArray       : '[' (lineEntry (',' lineEntry)*)? ']' ;
lineEntry      : '{' '"index"' ':' INT ',' '"text"' ':' STRING '}' ;
BOOL           : 'true' | 'false' ;
INT            : [0-9]+ ;
STRING         : '"' (~["\\] | '\\' .)* '"' ;
