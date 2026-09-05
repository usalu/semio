grammar StdioSemioKitMutations;
op : createObject | deleteObject | createModel | deleteModel | createProperties | deleteProperties
   | bindRepresentation | unbindRepresentation | changeRepresentationPin
   | addType | removeType | renameType | addDesign | removeDesign | editDesign ;
createObject : 'createObject' ':' HEX ',' HEX ;
deleteObject : 'deleteObject' ':' HEX ;
createModel : 'createModel' ':' HEX ',' HEX ;
deleteModel : 'deleteModel' ':' HEX ;
createProperties : 'createProperties' ':' HEX ',' HEX ;
deleteProperties : 'deleteProperties' ;
bindRepresentation : 'bindRepresentation' ':' HEX ',' .*? ',' HEX ;
unbindRepresentation : 'unbindRepresentation' ':' INT ;
changeRepresentationPin : 'changeRepresentationPin' ':' INT ',' .*? ;
addType : 'addType' ':' HEX ',' HEX ',' HEX ;
removeType : 'removeType' ':' HEX ;
renameType : 'renameType' ':' HEX ',' HEX ;
addDesign : 'addDesign' ':' HEX ',' HEX ;
removeDesign : 'removeDesign' ':' HEX ;
editDesign : 'editDesign' ':' HEX ',' .*? ;
HEX : [0-9a-f]* ;
INT : '-'? [0-9]+ ;
