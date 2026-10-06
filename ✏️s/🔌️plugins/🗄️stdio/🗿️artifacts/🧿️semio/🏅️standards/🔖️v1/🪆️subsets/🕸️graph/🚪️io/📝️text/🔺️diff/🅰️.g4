// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.graph's diff text representation.
grammar StdioSemioGraphDiff;
document : fields? ;
fields   : field (';' field)* ;
field    : nodesLine | edgesLine ;
nodesLine : 'nodes' '=' '[' nodeList? ']' ;
nodeList : node (',' node)* ;
node     : '[' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' '[' portList? ']' ',' '[' propertyList? ']' ']' ;
portList : port (',' port)* ;
port     : '[' HEX ',' portKind ']' ;
portKind : 'i' | 'o' | 'x' ;
propertyList : property (',' property)* ;
property : HEX ':' VALUE ;
edgesLine : 'edges' '=' '[' edgeList? ']' ;
edgeList : edge (',' edge)* ;
edge     : '[' HEX ',' HEX ',' HEX ',' HEX ',' HEX ']' ;
HEX      : [0-9a-f]* ;
VALUE    : . ;
