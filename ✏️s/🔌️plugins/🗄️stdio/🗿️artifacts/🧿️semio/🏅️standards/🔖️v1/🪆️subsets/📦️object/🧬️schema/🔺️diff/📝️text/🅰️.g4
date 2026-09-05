// ANTLR4 mirror (descriptive, not test-parsed) for s.stdio.semio.object.diff's DSL text form.
grammar StdioSemioObjectDiff;
diff       : fieldList? ;
fieldList  : field (';' field)* ;
field      : 't' '=' transformValue | 'b' '=' child | 'm' '=' child | 'p' '=' child ;
transformValue : '[' NUMBER (',' NUMBER)* ']' ;
child      : '[' ']' | '[' HEX ',' HEX ']' ;
HEX        : [0-9a-f]* ;
NUMBER     : '-'? [0-9]+ ('.' [0-9]+)? ;
