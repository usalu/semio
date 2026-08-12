// ANTLR4 grammar for `stdio.semio.drawing`'s real text DSL body (the `SemioDrawingSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️component.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_drawing_snapshot;

document: artifactMark schemaLine canvasLine stylesLine layersLine EOF;
artifactMark: 'stdio.semio.drawing';

schemaLine: 'schema' '=' HEX;
canvasLine: 'canvas' '=' canvas;
stylesLine: 'styles' '=' '[' (style (',' style)*)? ']';
layersLine: 'layers' '=' '[' (layer (',' layer)*)? ']';

canvas: '[' number ',' number ',' optionRgba ']';
style: '[' HEX ',' optionRgba ',' optionRgba ',' optionNumber ',' optionNumber ']';
layer: '[' HEX ',' HEX ',' bool ',' node ']';

node: 'P' '[' segmentList ',' optionHex ']'
    | 'T' '[' HEX ',' point2 ',' optionHex ']'
    | 'G' '[' transform ',' nodeList ']'
    | 'I' '[' point2 ',' number ',' number ',' HEX ',' HEX ']'
    ;
nodeList: '[' (node (',' node)*)? ']';

segment: 'M' '[' point2 ']'
       | 'L' '[' point2 ']'
       | 'C' '[' point2 ',' point2 ',' point2 ']'
       | 'Q' '[' point2 ',' point2 ']'
       | 'A' '[' number ',' number ',' number ',' bool ',' bool ',' point2 ']'
       | 'Z'
       ;
segmentList: '[' (segment (',' segment)*)? ']';

transform: '[' point3 ',' quaternion ',' point3 ']';
point2: '[' number ',' number ']';
point3: '[' number ',' number ',' number ']';
quaternion: '[' number ',' number ',' number ',' number ']';
rgba: '[' number ',' number ',' number ',' number ']';
bool: '0' | '1';
number: INT | FLOAT;

optionRgba: '[' '0' ']' | '[' '1' ',' rgba ']';
optionNumber: '[' '0' ']' | '[' '1' ',' number ']';
optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
