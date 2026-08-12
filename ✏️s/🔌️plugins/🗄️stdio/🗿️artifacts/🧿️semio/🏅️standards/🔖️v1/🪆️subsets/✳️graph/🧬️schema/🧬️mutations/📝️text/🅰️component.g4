// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️component.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.graph's mutation op text representation.
grammar StdioSemioGraphMutations;
op : createNode | deleteNode | changeNodeKind | changeNodeLabel | moveNode
   | addNodePort | removeNodePort | addNodeProperty | removeNodeProperty
   | createEdge | deleteEdge ;
createNode : 'createNode' ':' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' '[' portList? ']' ',' '[' propertyList? ']' ;
deleteNode : 'deleteNode' ':' HEX ;
changeNodeKind : 'changeNodeKind' ':' HEX ',' HEX ;
changeNodeLabel : 'changeNodeLabel' ':' HEX ',' HEX ;
moveNode : 'moveNode' ':' HEX ',' HEX ',' HEX ;
addNodePort : 'addNodePort' ':' HEX ',' INT ',' port ;
removeNodePort : 'removeNodePort' ':' HEX ',' INT ;
addNodeProperty : 'addNodeProperty' ':' HEX ',' INT ',' property ;
removeNodeProperty : 'removeNodeProperty' ':' HEX ',' INT ;
createEdge : 'createEdge' ':' HEX ',' HEX ',' HEX ',' HEX ',' HEX ;
deleteEdge : 'deleteEdge' ':' HEX ;
portList : port (',' port)* ;
port : '[' HEX ',' portKind ']' ;
portKind : 'i' | 'o' | 'x' ;
propertyList : property (',' property)* ;
property : HEX ':' VALUE ;
HEX : [0-9a-f]* ;
VALUE : . ;
