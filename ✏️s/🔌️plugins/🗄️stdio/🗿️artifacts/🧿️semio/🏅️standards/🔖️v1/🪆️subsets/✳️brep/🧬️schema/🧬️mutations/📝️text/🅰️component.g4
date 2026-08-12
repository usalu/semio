// ANTLR4 grammar for `stdio.semio.brep`'s hand-rolled `SemioBrepMutation` op text form
// (protocol::OpText::print_op/parse_op) -- descriptive mirror of the authoritative
// `📖️component.grammar.semio` (same production names). "keyword arg=value ..." shape, one line.
grammar Semio_brep_mutations;

op: noMutation | setSnapshot | addVertex | removeVertex | setVertexPoint | addEdge | removeEdge
  | setEdgeEndpoints | setEdgeCurve | addLoop | removeLoop | setLoopEdges | addFace | removeFace
  | setFaceSurface | setFaceOrientation | setFaceLoops | addShell | removeShell | setShellFaces
  | addSolid | removeSolid | setSolidShells
  ;

noMutation: 'no-mutation';
setSnapshot: 'set-snapshot' 'snapshot' '=' snapshotLit;
addVertex: 'add-vertex' 'vertex' '=' vertex;
removeVertex: 'remove-vertex' 'id' '=' HEX;
setVertexPoint: 'set-vertex-point' 'id' '=' HEX 'point' '=' point3;
addEdge: 'add-edge' 'edge' '=' edge;
removeEdge: 'remove-edge' 'id' '=' HEX;
setEdgeEndpoints: 'set-edge-endpoints' 'id' '=' HEX 'start' '=' HEX 'end' '=' HEX;
setEdgeCurve: 'set-edge-curve' 'id' '=' HEX 'curve' '=' curve;
addLoop: 'add-loop' 'loop' '=' brepLoop;
removeLoop: 'remove-loop' 'id' '=' HEX;
setLoopEdges: 'set-loop-edges' 'id' '=' HEX 'edges' '=' loopEdgeList;
addFace: 'add-face' 'face' '=' face;
removeFace: 'remove-face' 'id' '=' HEX;
setFaceSurface: 'set-face-surface' 'id' '=' HEX 'surface' '=' surface;
setFaceOrientation: 'set-face-orientation' 'id' '=' HEX 'orientation' '=' bool;
setFaceLoops: 'set-face-loops' 'id' '=' HEX 'outer' '=' HEX 'inner' '=' hexList;
addShell: 'add-shell' 'shell' '=' shell;
removeShell: 'remove-shell' 'id' '=' HEX;
setShellFaces: 'set-shell-faces' 'id' '=' HEX 'faces' '=' shellFaceList;
addSolid: 'add-solid' 'solid' '=' solid;
removeSolid: 'remove-solid' 'id' '=' HEX;
setSolidShells: 'set-solid-shells' 'id' '=' HEX 'shells' '=' solidShellList;

snapshotLit: '[' HEX ',' '[' (vertex (',' vertex)*)? ']' ',' '[' (edge (',' edge)*)? ']' ',' '[' (brepLoop (',' brepLoop)*)? ']' ',' '[' (face (',' face)*)? ']' ',' '[' (shell (',' shell)*)? ']' ',' '[' (solid (',' solid)*)? ']' ']';

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
