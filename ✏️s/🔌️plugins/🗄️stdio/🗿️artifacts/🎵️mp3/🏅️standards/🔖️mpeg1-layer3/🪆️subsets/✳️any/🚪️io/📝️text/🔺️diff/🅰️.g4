grammar Semio_mp3_diff;
// One-line Mp3Diff text: space-separated clauses; parse_diff accepts any order, print_diff emits id3v2, frames, splices, id3v1.
diff : (clause (' ' clause)*)? EOF ;
clause : 'id3v2=' meta | 'frames=' frameList | 'splices=' splice (';' splice)* | 'id3v1=' meta ;
meta : '[0]' | '[1,[' HEX ']]' ;
frameList : '[' (frame (';' frame)*)? ']' ;
frame : '[' header ',' HEX ']' ;
header : '[' NUM ',' NUM ',' BIT ',' NUM ',' NUM ',' BIT ',' BIT ',' NUM ',' NUM ',' BIT ',' BIT ',' NUM ']' ;
splice : NUM ',' NUM ',' frameList ;
BIT : '0' | '1' ;
NUM : [0-9]+ ;
HEX : ([0-9a-f][0-9a-f])* ;
