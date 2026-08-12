// ANTLR4 mirror (descriptive, not test-parsed) for s.stdio.semio.kit's DSL text representation.
grammar StdioSemioKitSnapshot;
document : artifactMark schemaLine typesLine designsLine objectsLine modelsLine propertiesLine representationsLine ;
artifactMark : 'stdio.semio.kit' ;
schemaLine : 'schema' '=' HEX ;
typesLine : 'types' '=' '[' (kitType (',' kitType)*)? ']' ;
kitType : '[' HEX ',' HEX ',' HEX ']' ;
designsLine : 'designs' '=' '[' (design (',' design)*)? ']' ;
design : '[' HEX ',' HEX ',' '[' .*? ']' ',' '[' .*? ']' ']' ;
objectsLine : 'objects' '=' childList ;
modelsLine : 'models' '=' childList ;
childList : '[' (child (',' child)*)? ']' ;
child : '[' HEX ',' HEX ']' ;
propertiesLine : 'properties' '=' ( '[' ']' | child ) ;
representationsLine : 'representations' '=' '[' .*? ']' ;
HEX : [0-9a-f]* ;
