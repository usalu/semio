grammar Stdio_jpg_snapshot;
document : 'semio' 'stdio.jpg.dsl' 'v1' snapshot EOF;
snapshot : 'schema' '=' TEXT 'width' '=' UINT 'height' '=' UINT 'pixels' '=' TEXT
 'jfifXDensity' '=' UINT 'jfifYDensity' '=' UINT 'sofMarker' '=' UINT 'arithmetic' '=' boolean
 'reEncodeQuality' '=' optionalUint 'jfifVersion' '=' '[' UINT UINT ']'
 'jfifDensityUnits' '=' TEXT 'jfifThumbnail' '=' thumbnail 'frame' '=' frame
 'quantTables' '=' '[' quantization* ']' 'huffmanTables' '=' '[' huffman* ']'
 'restartInterval' '=' optionalUint 'otherSegments' '=' '[' segment* ']';
optionalUint : 'null' | UINT;
octets : 'bytes64' '(' TEXT ')';
thumbnail : 'null' | '{' 'height' '=' UINT 'rgbData' '=' octets 'width' '=' UINT '}';
frame : 'null' | '{' 'components' '=' '[' component* ']' 'height' '=' UINT 'precision' '=' UINT 'width' '=' UINT '}';
component : '{' 'hSampling' '=' UINT 'id' '=' UINT 'quantTableId' '=' UINT 'vSampling' '=' UINT '}';
quantization : '{' 'id' '=' UINT 'precision' '=' UINT 'values' '=' '[' UINT* ']' '}';
huffman : '{' 'bits' '=' '[' UINT* ']' 'class' '=' TEXT 'id' '=' UINT 'values' '=' octets '}';
segment : '{' 'data' '=' octets 'marker' '=' UINT '}';
boolean : 'true' | 'false';
UINT : [0-9]+;
TEXT : '"' ('\\' . | ~["\\])* '"';
WS : [ \t\r\n]+ -> skip;
COMMENT : '#' ~[\r\n]* -> skip;
