// ANTLR4 grammar for `stdio.semio.drawing`'s real one-line `OpText::print_op` shape — see the
// sibling `📖️component.grammar.semio` for the authoritative, conformance-tested version; this is a
// descriptive mirror, same production names.
grammar Semio_drawing_mutations;

op: noMutation | setSnapshot | setCanvasSize | setCanvasBackground | setStyle | removeStyle | insertLayer | removeLayer | setLayerMeta | moveLayer | setGroupTransform | setPathSegments | setNodeStyle | setText | setImage | insertNode | removeNode | replaceNode ;

noMutation: 'no-mutation';
setSnapshot: 'set-snapshot' 'snapshot' '=' snapshotLit;
setCanvasSize: 'set-canvas-size' 'width' '=' number 'height' '=' number;
setCanvasBackground: 'set-canvas-background' 'background' '=' optionRgba;
setStyle: 'set-style' 'name' '=' HEX 'fill' '=' optionRgba 'stroke' '=' optionRgba 'stroke-width' '=' optionNumber 'opacity' '=' optionNumber;
removeStyle: 'remove-style' 'name' '=' HEX;
insertLayer: 'insert-layer' 'index' '=' number 'layer' '=' layer;
removeLayer: 'remove-layer' 'index' '=' number;
setLayerMeta: 'set-layer-meta' 'index' '=' number 'id' '=' HEX 'name' '=' HEX 'visible' '=' bool;
moveLayer: 'move-layer' 'from' '=' number 'to' '=' number;
setGroupTransform: 'set-group-transform' 'path' '=' nodePath 'transform' '=' transform;
setPathSegments: 'set-path-segments' 'path' '=' nodePath 'segments' '=' segmentList;
setNodeStyle: 'set-node-style' 'path' '=' nodePath 'style' '=' optionHex;
setText: 'set-text' 'path' '=' nodePath 'value' '=' HEX 'at' '=' point2;
setImage: 'set-image' 'path' '=' nodePath 'at' '=' point2 'width' '=' number 'height' '=' number 'mime' '=' HEX 'bytes' '=' HEX;
insertNode: 'insert-node' 'path' '=' nodePath 'index' '=' number 'node' '=' node;
removeNode: 'remove-node' 'path' '=' nodePath 'index' '=' number;
replaceNode: 'replace-node' 'path' '=' nodePath 'node' '=' node;

nodePath: '[' number ',' '[' (number (',' number)*)? ']' ']';

snapshotLit: '[' HEX ',' canvas ',' '[' (style (',' style)*)? ']' ',' '[' (layer (',' layer)*)? ']' ']';

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
