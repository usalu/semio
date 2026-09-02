grammar Stdio_binary_diff;
// BinaryDiff is transported as JSON (serde_json): a splice list, mirroring the Rust struct.
document    : object EOF ;
object      : '{' ('"splices"' ':' spliceArray)? '}' ;
spliceArray : '[' (splice (',' splice)*)? ']' ;
splice      : '{' '"offset"' ':' INT ',' '"removeLen"' ':' INT ',' '"insert"' ':' byteArray '}' ;
byteArray   : '[' (INT (',' INT)*)? ']' ;
INT         : [0-9]+ ;
