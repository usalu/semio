// ANTLR4 grammar for `stdio.semio.brep`'s hand-rolled `SemioBrepDiff` text form
// (protocol::DiffCodec::print_diff/parse_diff) -- descriptive mirror of the authoritative
// `📖️component.grammar.semio` (same production names). See the sibling `../💾️binary/🔠️component.abnf`
// for the full per-collection value grammar restated as ABNF.
grammar Semio_brep_diff;

document: (verticesLine)? (edgesLine)? (loopsLine)? (facesLine)? (shellsLine)? (solidsLine)? EOF;

verticesLine: 'vertices' '=' triple;
edgesLine: 'edges' '=' triple;
loopsLine: 'loops' '=' triple;
facesLine: 'faces' '=' triple;
shellsLine: 'shells' '=' triple;
solidsLine: 'solids' '=' triple;

triple: '[' (HEX (',' HEX)*)? ']' ';' '[' (modified (',' modified)*)? ']' ';' '[' (item (',' item)*)? ']';
modified: HEX ':' itemDiff;
item: vertex | edge | brepLoop | face | shell | solid;
itemDiff: vertexDiff | edgeDiff | loopDiff | faceDiff | shellDiff | solidDiff;

vertexDiff: '[' optionPoint3 ']';
edgeDiff: '[' optionHex ',' optionHex ',' optionCurve ']';
loopDiff: '[' optionLoopEdgeList ']';
faceDiff: '[' optionHex ',' optionHexList ',' optionSurface ',' optionBool ']';
shellDiff: '[' optionShellFaceList ']';
solidDiff: '[' optionSolidShellList ']';

optionPoint3: '[0]' | '[1,' point3 ']';
optionHex: '[0]' | '[1,' HEX ']';
optionCurve: '[0]' | '[1,' curve ']';
optionSurface: '[0]' | '[1,' surface ']';
optionBool: '[0]' | '[1,' bool ']';
optionHexList: '[0]' | '[1,' hexList ']';
optionLoopEdgeList: '[0]' | '[1,' loopEdgeList ']';
optionShellFaceList: '[0]' | '[1,' shellFaceList ']';
optionSolidShellList: '[0]' | '[1,' solidShellList ']';

vertex: '[' HEX ',' point3 ']';
edge: '[' HEX ',' HEX ',' HEX ',' curve ']';
brepLoop: '[' HEX ',' loopEdgeList ']';
face: '[' HEX ',' HEX ',' hexList ',' surface ',' bool ']';
shell: '[' HEX ',' shellFaceList ']';
solid: '[' HEX ',' solidShellList ']';

loopEdge: '[' HEX ',' bool ']';
loopEdgeList: '[' (loopEdge (',' loopEdge)*)? ']';
shellFace: '[' HEX ',' bool ']';
shellFaceList: '[' (shellFace (',' shellFace)*)? ']';
solidShell: '[' HEX ',' bool ']';
solidShellList: '[' (solidShell (',' solidShell)*)? ']';
hexList: '[' (HEX (',' HEX)*)? ']';

curve: 'L' '[' point3 ',' point3 ']'
     | 'C' '[' point3 ',' point3 ',' number ']'
     | 'E' '[' point3 ',' point3 ',' number ',' number ']'
     | 'N' '[' point3List ',' numberList ',' number ',' numberList ']'
     ;
surface: 'P' '[' point3 ',' point3 ']'
        | 'C' '[' point3 ',' point3 ',' number ']'
        | 'O' '[' point3 ',' point3 ',' number ',' number ']'
        | 'S' '[' point3 ',' number ']'
        | 'T' '[' point3 ',' point3 ',' number ',' number ']'
        | 'N' '[' point3List ',' numberList ',' number ',' number ',' number ',' number ',' numberList ',' numberList ']'
        ;

point3: '[' number ',' number ',' number ']';
point3List: '[' (point3 (',' point3)*)? ']';
numberList: '[' (number (',' number)*)? ']';
bool: '0' | '1';
number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
SP: ' ';
