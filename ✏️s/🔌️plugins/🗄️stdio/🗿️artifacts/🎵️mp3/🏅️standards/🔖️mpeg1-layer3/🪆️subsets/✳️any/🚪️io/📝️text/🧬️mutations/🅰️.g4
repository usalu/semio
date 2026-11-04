grammar Semio_mp3_mutations;
// The one-line text of a mutation op is JSON; the `mutation` member names setId3v2, setFrames, setId3v1 or patchFrames.
op : object EOF ;
object : '{' (member (',' member)*)? '}' ;
member : STRING ':' value ;
value : object | array | STRING | NUMBER | 'true' | 'false' | 'null' ;
array : '[' (value (',' value)*)? ']' ;
STRING : '"' ( ~["\\] | '\\' . )* '"' ;
NUMBER : '-'? [0-9]+ ('.' [0-9]+)? ([eE] [+-]? [0-9]+)? ;
WS : [ \t\r\n]+ -> skip ;
