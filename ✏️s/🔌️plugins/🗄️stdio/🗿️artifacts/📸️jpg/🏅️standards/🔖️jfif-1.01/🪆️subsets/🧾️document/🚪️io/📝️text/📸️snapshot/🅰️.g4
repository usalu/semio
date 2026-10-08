grammar Stdio_jpg_snapshot;
document : 'semio' 'stdio.jpg.dsl' 'v1' snapshot EOF;
snapshot : 'schema' '=' TEXT 'width' '=' UINT 'height' '=' UINT 'pixels' '=' TEXT
 'jfifVersion' '=' '[' UINT UINT ']' 'jfifDensityUnits' '=' TEXT
 'jfifXDensity' '=' UINT 'jfifYDensity' '=' UINT
 'jfifThumbnail' '=' thumbnail 'otherSegments' '=' '[' segment* ']';
octets : 'bytes64' '(' TEXT ')';
thumbnail : 'null' | '{' 'height' '=' UINT 'rgbData' '=' octets 'width' '=' UINT '}';
segment : '{' 'data' '=' octets 'marker' '=' UINT '}';
UINT : [0-9]+;
TEXT : '"' ('\\' . | ~["\\])* '"';
WS : [ \t\r\n]+ -> skip;
COMMENT : '#' ~[\r\n]* -> skip;
