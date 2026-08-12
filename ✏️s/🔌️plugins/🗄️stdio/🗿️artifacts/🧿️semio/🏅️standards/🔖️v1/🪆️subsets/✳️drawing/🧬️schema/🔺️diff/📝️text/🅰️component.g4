// ANTLR4 grammar for `stdio.semio.drawing`'s real one-line `DiffCodec::print_diff` shape — see the
// sibling `📖️component.grammar.semio` for the authoritative, conformance-tested version; this is a
// descriptive mirror, same production names.
grammar Semio_drawing_diff;

document: canvasLine? stylesLine? layersLine? EOF;

canvasLine: 'canvas' '=' canvasDiff;

stylesLine: 'styles' '=' stylesTriple;
stylesTriple: '[' (HEX (',' HEX)*)? ']' ';' '[' (styleModified (',' styleModified)*)? ']' ';' '[' (style (',' style)*)? ']';
styleModified: HEX ':' styleDiff;

layersLine: 'layers' '=' layersTriple;
layersTriple: '[' (number (',' number)*)? ']' ';' '[' (layerModified (',' layerModified)*)? ']' ';' '[' (layerAdded (',' layerAdded)*)? ']';
layerModified: number ':' layerDiff;
layerAdded: number ':' layer;

canvasDiff: '[' optionNumber ',' optionNumber ',' optionOptionRgba ']';
styleDiff: '[' optionOptionRgba ',' optionOptionRgba ',' optionOptionNumber ',' optionOptionNumber ']';
layerDiff: '[' optionHex ',' optionHex ',' optionBool ',' optionNodeDiff ']';

nodeDiff: 'P' '[' optionSegmentList ',' optionOptionHex ']'
        | 'T' '[' optionHex ',' optionPoint2 ',' optionOptionHex ']'
        | 'G' '[' optionTransform ',' optionChildren ']'
        | 'I' '[' optionPoint2 ',' optionNumber ',' optionNumber ',' optionHex ',' optionHex ']'
        | 'R' '[' node ']'
        ;

optionChildren: '[' '0' ']' | '[' '1' ',' childrenTriple ']';
childrenTriple: '[' (number (',' number)*)? ']' ';' '[' (childModified (',' childModified)*)? ']' ';' '[' (childAdded (',' childAdded)*)? ']';
childModified: number ':' nodeDiff;
childAdded: number ':' node;

optionPoint2: '[' '0' ']' | '[' '1' ',' point2 ']';
optionTransform: '[' '0' ']' | '[' '1' ',' transform ']';
optionSegmentList: '[' '0' ']' | '[' '1' ',' segmentList ']';
optionNodeDiff: '[' '0' ']' | '[' '1' ',' nodeDiff ']';
optionRgba: '[' '0' ']' | '[' '1' ',' rgba ']';
optionNumber: '[' '0' ']' | '[' '1' ',' number ']';
optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionBool: '[' '0' ']' | '[' '1' ',' bool ']';
optionOptionRgba: '[' '0' ']' | '[' '1' ',' optionRgba ']';
optionOptionNumber: '[' '0' ']' | '[' '1' ',' optionNumber ']';
optionOptionHex: '[' '0' ']' | '[' '1' ',' optionHex ']';

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

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
