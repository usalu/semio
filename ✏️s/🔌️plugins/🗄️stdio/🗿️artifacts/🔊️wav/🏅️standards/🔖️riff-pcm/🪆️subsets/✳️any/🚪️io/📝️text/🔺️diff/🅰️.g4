grammar Semio_wav_diff;
// One-line WavDiff text: space-separated clauses; parse_diff accepts any order, print_diff emits fmt, data, data-splices, fmt-pad-byte, data-pad-byte, other-chunks, chunk-order.
diff : (clause (' ' clause)*)? EOF ;
clause : 'fmt=' fmt | 'data=' data | 'data-splices=' splice (';' splice)* | 'fmt-pad-byte=' NUM | 'data-pad-byte=' NUM | 'other-chunks=' chunks | 'chunk-order=' order ;
fmt : '[' NUM ',' NUM ',' NUM ',' NUM ',' NUM ',' NUM ',' ext ']' ;
ext : '[0]' | '[1,' HEX ']' ;
data : TAG ':' HEX ;
splice : NUM ',' NUM ',' data ;
chunks : '[' (chunk (';' chunk)*)? ']' ;
chunk : '[' FOURCC ',' HEX ',' NUM ']' ;
order : '[' (REF (';' REF)*)? ']' ;
TAG : 'p16' | 'p8' | 'f32' | 'raw' ;
REF : 'f' | 'd' | 'o' [0-9]+ ;
FOURCC : [ -~] [ -~] [ -~] [ -~] ;
NUM : [0-9]+ ;
HEX : ([0-9a-f][0-9a-f])* ;
