// ANTLR4 grammar for `stdio.semio.drawing`'s real one-line `OpText::print_op` shape — see the
// sibling `📖️.grammar.semio` for the authoritative, conformance-tested version; this is a
// descriptive mirror, same production names. Seventeen keywords, one per SMO-approved verb this
// facet registers (📌️important.md's binding vocabulary ruling) — no `noMutation`/`setSnapshot`/
// `set*` (banned/superseded vocabulary).
grammar Semio_drawing_mutations;

op: createLayer | deleteLayer | createNode | deleteNode | moveNode | dragNodes | rotate | scale | reorderNodes | group | ungroup | flatten | unflatten | replacePath | replaceFill | changeStrokeColor | changeStrokeWidth ;

createLayer: 'createLayer' ':' INT ',' layer;
deleteLayer: 'deleteLayer' ':' HEX;
createNode: 'createNode' ':' nodePath ',' INT ',' node;
deleteNode: 'deleteNode' ':' nodePath;
moveNode: 'moveNode' ':' nodePath ',' point2;
dragNodes: 'dragNodes' ':' '[' (nodePath (',' nodePath)*)? ']' ',' point2;
rotate: 'rotate' ':' nodePath ',' quaternion;
scale: 'scale' ':' nodePath ',' point3;
reorderNodes: 'reorderNodes' ':' nodePath ',' INT ',' INT;
group: 'group' ':' nodePath ',' '[' (INT (',' INT)*)? ']' ',' transform;
ungroup: 'ungroup' ':' nodePath;
flatten: 'flatten' ':' nodePath;
unflatten: 'unflatten' ':' nodePath ',' node;
replacePath: 'replacePath' ':' nodePath ',' segmentList;
replaceFill: 'replaceFill' ':' HEX ',' optionRgba;
changeStrokeColor: 'changeStrokeColor' ':' HEX ',' optionRgba;
changeStrokeWidth: 'changeStrokeWidth' ':' HEX ',' optionNumber;

nodePath: '[' number ',' '[' (number (',' number)*)? ']' ']';

// Same full-VALUE codecs `📸️snapshot/📝️text/🅰️.g4` declares (restated here so this leaf
// grammar is self-contained, matching the repo's existing per-facet convention).
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

optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionRgba: '[' '0' ']' | '[' '1' ',' rgba ']';
optionNumber: '[' '0' ']' | '[' '1' ',' number ']';

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
