// ANTLR4 mirror (descriptive, not test-parsed) for s.stdio.semio.object.mutations.
grammar StdioSemioObjectMutations;
op : moveObject | rotateObject | scaleObject | createBrep | deleteBrep | createMesh | deleteMesh | createProperties | deleteProperties ;
moveObject : 'moveObject' ':' NUMBER ',' NUMBER ',' NUMBER ;
rotateObject : 'rotateObject' ':' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ;
scaleObject : 'scaleObject' ':' NUMBER ',' NUMBER ',' NUMBER ;
createBrep : 'createBrep' ':' HEX ',' HEX ;
deleteBrep : 'deleteBrep' ;
createMesh : 'createMesh' ':' HEX ',' HEX ;
deleteMesh : 'deleteMesh' ;
createProperties : 'createProperties' ':' HEX ',' HEX ;
deleteProperties : 'deleteProperties' ;
HEX : [0-9a-f]* ;
NUMBER : '-'? [0-9]+ ('.' [0-9]+)? ;
