grammar Stdio_binary_mutation;
// BinaryMutation is transported as JSON (serde_json, `#[serde(tag = "mutation")]`).
document   : object EOF ;
object     : '{' '"mutation"' ':' kind (',' field)* '}' ;
kind       : '"setSnapshot"' | '"splice"' | '"appendBytes"' | '"truncateAt"' ;
field      : snapshotF | offsetF | removeLenF | insertF | dataF ;
snapshotF  : '"snapshot"' ':' snapshotObj ;
offsetF    : '"offset"' ':' INT ;
removeLenF : '"removeLen"' ':' INT ;
insertF    : '"insert"' ':' byteArray ;
dataF      : '"data"' ':' byteArray ;
byteArray  : '[' (INT (',' INT)*)? ']' ;
snapshotObj: '{' .*? '}' ;   // see snapshot/binary grammar for the real shape
INT        : [0-9]+ ;
