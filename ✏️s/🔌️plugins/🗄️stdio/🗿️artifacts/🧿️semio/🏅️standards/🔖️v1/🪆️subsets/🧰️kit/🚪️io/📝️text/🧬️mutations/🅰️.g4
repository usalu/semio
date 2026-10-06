grammar StdioSemioKitMutations;
op : createObject | deleteObject | createModel | deleteModel | createProperties | deleteProperties
   | bindRepresentation | unbindRepresentation | changeRepresentationPin
   | addType | removeType | renameType | addDesign | removeDesign | editDesign ;
createObject : 'createObject' ':' HEX ',' reference ;
deleteObject : 'deleteObject' ':' HEX ;
createModel : 'createModel' ':' HEX ',' reference ;
deleteModel : 'deleteModel' ':' HEX ;
createProperties : 'createProperties' ':' HEX ',' reference ;
deleteProperties : 'deleteProperties' ;
bindRepresentation : 'bindRepresentation' ':' reference ',' pin ',' HEX ;
unbindRepresentation : 'unbindRepresentation' ':' INT ;
changeRepresentationPin : 'changeRepresentationPin' ':' INT ',' pin ;
addType : 'addType' ':' HEX ',' HEX ',' HEX ;
removeType : 'removeType' ':' HEX ;
renameType : 'renameType' ':' HEX ',' HEX ;
addDesign : 'addDesign' ':' HEX ',' HEX ;
removeDesign : 'removeDesign' ':' HEX ;
editDesign : 'editDesign' ':' HEX ',' pieceList ',' connectionList ;
pieceList : '[' (piece (',' piece)*)? ']' ;
piece : '[' HEX ',' HEX ',' transform ']' ;
connectionList : '[' (connection (',' connection)*)? ']' ;
connection : '[' HEX ',' HEX ',' HEX ',' HEX ',' HEX ']' ;
transform : '[' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ']' ;
pin : '[' 'h' ']' | '[' 'c' ',' HEX ']' | '[' 's' ',' HEX ',' INT ',' HEX ']' ;
NUMBER : '-'? [0-9]+ ('.' [0-9]+)? | '-'? 'inf' | 'nan64_' HEX_WORD HEX_WORD HEX_WORD HEX_WORD ;
fragment HEX_WORD : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT ;
fragment HEX_DIGIT : [0-9a-fA-F] ;
HEX : [0-9a-f]* ;
INT : '-'? [0-9]+ ;

reference : '[' HEX ',' HEX ',' HEX ',' HEX ']' ;
