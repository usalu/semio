// ANTLR4 mirror (descriptive, not test-parsed) for s.stdio.semio.kit's DSL text representation.
grammar StdioSemioKitSnapshot;
document : artifactMark schemaLine typesLine designsLine objectsLine modelsLine propertiesLine representationsLine ;
artifactMark : 'stdio.semio.kit' ;
schemaLine : 'schema' '=' HEX ;
typesLine : 'types' '=' '[' (kitType (',' kitType)*)? ']' ;
kitType : '[' HEX ',' HEX ',' HEX ']' ;
designsLine : 'designs' '=' '[' (design (',' design)*)? ']' ;
design : '[' HEX ',' HEX ',' pieceList ',' connectionList ']' ;
pieceList : '[' (piece (',' piece)*)? ']' ;
piece : '[' HEX ',' HEX ',' transform ']' ;
connectionList : '[' (connection (',' connection)*)? ']' ;
connection : '[' HEX ',' HEX ',' HEX ',' HEX ',' HEX ']' ;
transform : '[' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ']' ;
objectsLine : 'objects' '=' childList ;
modelsLine : 'models' '=' childList ;
childList : '[' (child (',' child)*)? ']' ;
child : '[' HEX ',' reference ']' ;
propertiesLine : 'properties' '=' ( '[' ']' | child ) ;
representationsLine : 'representations' '=' '[' (link (',' link)*)? ']' ;
link : '[' reference ',' pin ',' HEX ']' ;
pin : '[' 'h' ']' | '[' 'c' ',' HEX ']' | '[' 's' ',' HEX ',' UINT ',' HEX ']' ;
UINT : [0-9]+ ;
NUMBER : '-'? [0-9]+ ('.' [0-9]+)? | '-'? 'inf' | 'nan64_' HEX_WORD HEX_WORD HEX_WORD HEX_WORD ;
fragment HEX_WORD : HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT ;
fragment HEX_DIGIT : [0-9a-fA-F] ;
HEX : [0-9a-f]* ;

reference : '[' HEX ',' HEX ',' HEX ',' HEX ']' ;
