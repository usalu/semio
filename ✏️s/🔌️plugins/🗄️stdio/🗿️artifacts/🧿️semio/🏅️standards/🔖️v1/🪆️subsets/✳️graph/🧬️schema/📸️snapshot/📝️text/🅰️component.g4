// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️component.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.graph's DSL text representation.
grammar StdioSemioGraphSnapshot;

document        : artifactMark schemaLine nodesLine edgesLine ;
artifactMark    : 'stdios.stdio.semio.graph' ;
schemaLine      : 'schema' '=' HEX ;
nodesLine       : 'nodes' '=' '[' nodeList? ']' ;
nodeList        : node (',' node)* ;
node            : '[' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' '[' portList? ']' ',' '[' propertyList? ']' ']' ;
portList        : port (',' port)* ;
port            : '[' HEX ',' portKind ']' ;
portKind        : 'i' | 'o' | 'x' ;
propertyList    : property (',' property)* ;
property        : HEX ':' VALUE ;

edgesLine       : 'edges' '=' '[' edgeList? ']' ;
edgeList        : edge (',' edge)* ;
edge            : '[' HEX ',' HEX ',' HEX ',' HEX ',' HEX ']' ;

HEX             : [0-9a-f]* ;
VALUE           : . ;
