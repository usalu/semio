grammar Semio_wav_mutations;
// The one-line text of a mutation op is JSON; the `mutation` member names setFmt, setData, setOtherChunks, patchData or setPadBytes.
op : object EOF ;
object : '{' (member (',' member)*)? '}' ;
member : STRING ':' value ;
value : object | array | STRING | NUMBER | 'true' | 'false' | 'null' ;
array : '[' (value (',' value)*)? ']' ;
STRING : '"' ( ~["\\] | '\\' . )* '"' ;
NUMBER : '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)? ;
WS : [ \t\r\n]+ -> skip ;
