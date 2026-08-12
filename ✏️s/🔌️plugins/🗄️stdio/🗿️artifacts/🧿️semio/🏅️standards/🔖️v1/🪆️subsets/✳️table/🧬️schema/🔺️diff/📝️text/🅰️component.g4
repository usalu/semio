// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️component.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.table's diff text line.
grammar StdioSemioTableDiff;

document        : fieldList? ;
fieldList       : field (';' field)* ;
field           : columnsField | rowsField ;
columnsField    : 'columns' '=' '[' columnList? ']' ;
columnList      : column (',' column)* ;
column          : '[' HEX ',' cellKind ']' ;
cellKind        : 'n' | 'b' | 'i' | 'f' | 's' | 'y' ;
rowsField       : 'rows' '=' '[' rowList? ']' ;
rowList         : row (',' row)* ;
row             : '[' listItem* ']' ;

value           : 'Z' | 'B' '[' BIT ']' | 'I' '[' HEX ']' | 'F' '[' HEX ']' | 'S' '[' HEX ']' | 'Y' '[' HEX ']' | 'L' '[' listItem* ']' | 'M' '[' mapItem* ']' | 'R' '[' HEX ']' ;
listItem        : value ','? ;
mapItem         : HEX ':' value ','? ;

HEX             : [0-9a-f]* ;
BIT             : [01] ;
