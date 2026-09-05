grammar StdioSemioKitDiff;
diff : fieldList? ;
fieldList : field (';' field)* ;
field : 't' '=' .*? | 'd' '=' .*? | 'o' '=' .*? | 'm' '=' .*? | 'p' '=' .*? | 'r' '=' .*? ;
