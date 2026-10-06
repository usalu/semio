// ANTLR4 mirror (descriptive, not test-parsed) for s.stdio.semio.object.mutations.
grammar StdioSemioObjectMutations;
op : moveObject | rotateObject | scaleObject | createBrep | deleteBrep | createMesh | deleteMesh | createProperties | deleteProperties ;
moveObject : 'moveObject' ':' NUMBER ',' NUMBER ',' NUMBER ;
rotateObject : 'rotateObject' ':' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ;
scaleObject : 'scaleObject' ':' NUMBER ',' NUMBER ',' NUMBER ;
createBrep : 'createBrep' ':' HEX ',' reference ;
deleteBrep : 'deleteBrep' ;
createMesh : 'createMesh' ':' HEX ',' reference ;
deleteMesh : 'deleteMesh' ;
createProperties : 'createProperties' ':' HEX ',' reference ;
deleteProperties : 'deleteProperties' ;
HEX : [0-9a-f]* ;
NUMBER : '-'? [0-9]+ ('.' [0-9]+)? | '-'? 'inf' | 'nan64_' HEX_WORD HEX_WORD HEX_WORD HEX_WORD ;
fragment HEX_WORD : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT ;
fragment HEX_DIGIT : [0-9a-fA-F] ;

reference : '[' HEX ',' HEX ',' HEX ',' HEX ']' ;
