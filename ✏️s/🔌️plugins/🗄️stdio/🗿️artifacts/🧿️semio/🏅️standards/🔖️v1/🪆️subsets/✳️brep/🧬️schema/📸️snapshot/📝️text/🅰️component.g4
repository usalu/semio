// ANTLR4 grammar for `stdio.semio.brep`'s real text DSL body (the `SemioBrepSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️component.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_brep_snapshot;

document: artifactMark schemaLine verticesLine edgesLine loopsLine facesLine shellsLine solidsLine EOF;
artifactMark: 'stdio.semio.brep';

schemaLine: 'schema' '=' HEX;

verticesLine: 'vertices' '=' '[' (vertex (',' vertex)*)? ']';
edgesLine: 'edges' '=' '[' (edge (',' edge)*)? ']';
loopsLine: 'loops' '=' '[' (brepLoop (',' brepLoop)*)? ']';
facesLine: 'faces' '=' '[' (face (',' face)*)? ']';
shellsLine: 'shells' '=' '[' (shell (',' shell)*)? ']';
solidsLine: 'solids' '=' '[' (solid (',' solid)*)? ']';

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
WS: [ \t\r\n]+ -> skip;
