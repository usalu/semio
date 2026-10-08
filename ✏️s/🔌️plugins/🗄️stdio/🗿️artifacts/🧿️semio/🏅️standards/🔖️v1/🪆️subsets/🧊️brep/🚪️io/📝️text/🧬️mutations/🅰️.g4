// ANTLR4 grammar for `stdio.semio.brep`'s hand-rolled `SemioBrepMutation` op text form
// (protocol::OpText::print_op/parse_op) -- descriptive mirror of the authoritative
// `📖️.grammar.semio` (same production names). "keyword arg=value ..." shape, one line.
grammar Semio_brep_mutations;

op: createVertex | deleteVertex | createEdge | deleteEdge | createFace | deleteFace
  | createShell | deleteShell | createSolid | deleteSolid | replaceCurve | replaceSurface
  | moveVertex
  ;

createVertex: 'create-vertex' 'id' '=' HEX 'point' '=' point3 'tol' '=' number ('at' '=' INT)?;
deleteVertex: 'delete-vertex' 'id' '=' HEX;
createEdge: 'create-edge' 'id' '=' HEX 'start' '=' HEX 'end' '=' HEX 'curve' '=' curve 'tol' '=' number ('at' '=' INT)?;
deleteEdge: 'delete-edge' 'id' '=' HEX;
createFace: 'create-face' 'id' '=' HEX 'outer' '=' HEX 'inner' '=' hexList 'surface' '=' surface 'orientation' '=' bool 'tol' '=' number ('at' '=' INT)?;
deleteFace: 'delete-face' 'id' '=' HEX;
createShell: 'create-shell' 'id' '=' HEX 'faces' '=' shellFaceList ('at' '=' INT)?;
deleteShell: 'delete-shell' 'id' '=' HEX;
createSolid: 'create-solid' 'id' '=' HEX 'shells' '=' solidShellList ('at' '=' INT)?;
deleteSolid: 'delete-solid' 'id' '=' HEX;
replaceCurve: 'replace-curve' 'edge' '=' HEX 'curve' '=' curve;
replaceSurface: 'replace-surface' 'face' '=' HEX 'surface' '=' surface;
moveVertex: 'move-vertex' 'id' '=' HEX 'point' '=' point3;

shellFace: '[' HEX ',' bool ']';
shellFaceList: '[' (shellFace (',' shellFace)*)? ']';
solidShell: '[' HEX ',' bool ']';
solidShellList: '[' (solidShell (',' solidShell)*)? ']';
hexList: '[' (loopId (',' loopId)*)? ']';
loopId: '[' HEX ']';

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
number: INT | FLOAT | IEEE;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
SP: ' ';

IEEE: '-'? 'inf' | 'nan64_' WORD WORD WORD WORD;
fragment WORD: [0-9a-f] [0-9a-f] [0-9a-f] [0-9a-f];
