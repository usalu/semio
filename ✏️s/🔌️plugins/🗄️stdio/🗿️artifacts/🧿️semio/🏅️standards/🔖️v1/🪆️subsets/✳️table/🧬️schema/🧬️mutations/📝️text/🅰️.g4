// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.table's mutation text op.
grammar StdioSemioTableMutations;

op              : createColumn | deleteColumn | renameColumn | reorderColumns | insertRow | removeRow | reorderRows | editCell ;
createColumn    : 'createColumn' ':' HEX ',' cellKind ',' optIndex ;
deleteColumn    : 'deleteColumn' ':' HEX ;
renameColumn    : 'renameColumn' ':' HEX ',' HEX ;
reorderColumns  : 'reorderColumns' ':' HEX ',' INT ;
insertRow       : 'insertRow' ':' INT ',' row ;
removeRow       : 'removeRow' ':' INT ;
reorderRows     : 'reorderRows' ':' INT ',' INT ;
editCell        : 'editCell' ':' INT ',' HEX ',' value ;

optIndex        : INT? ;
cellKind        : 'n' | 'b' | 'i' | 'f' | 's' | 'y' ;
row             : '[' listItem* ']' ;

value           : 'Z' | 'B' '[' BIT ']' | 'I' '[' HEX ']' | 'F' '[' HEX ']' | 'S' '[' HEX ']' | 'Y' '[' HEX ']' | 'L' '[' listItem* ']' | 'M' '[' mapItem* ']' | 'R' '[' HEX ']' ;
listItem        : value ','? ;
mapItem         : HEX ':' value ','? ;

INT             : [0-9]+ ;
HEX             : [0-9a-f]* ;
BIT             : [01] ;
