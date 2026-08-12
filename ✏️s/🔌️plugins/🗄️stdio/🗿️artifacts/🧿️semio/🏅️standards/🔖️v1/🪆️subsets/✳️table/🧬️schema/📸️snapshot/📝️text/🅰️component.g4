// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️component.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.table's DSL text representation.
grammar StdioSemioTableSnapshot;

document        : artifactMark schemaLine columnsLine rowsLine ;
artifactMark    : 's.stdio.semio.table' ;
schemaLine      : 'schema' '=' HEX ;
columnsLine     : 'columns' '=' '[' columnList? ']' ;
columnList      : column (',' column)* ;
column          : '[' HEX ',' cellKind ']' ;
cellKind        : 'n' | 'b' | 'i' | 'f' | 's' | 'y' ;
rowsLine        : 'rows' '=' '[' rowList? ']' ;
rowList         : row (',' row)* ;
row             : '[' listItem* ']' ;

value           : 'Z' | 'B' '[' BIT ']' | 'I' '[' HEX ']' | 'F' '[' HEX ']' | 'S' '[' HEX ']' | 'Y' '[' HEX ']' | 'L' '[' listItem* ']' | 'M' '[' mapItem* ']' | 'R' '[' HEX ']' ;
listItem        : value ','? ;
mapItem         : HEX ':' value ','? ;

HEX             : [0-9a-f]* ;
BIT             : [01] ;
