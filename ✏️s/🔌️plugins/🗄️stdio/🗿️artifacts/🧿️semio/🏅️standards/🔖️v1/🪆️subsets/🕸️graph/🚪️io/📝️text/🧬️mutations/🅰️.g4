// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.graph's mutation op text representation.
grammar StdioSemioGraphMutations;
op : createNode | deleteNode | changeNodeKind | changeNodeLabel | moveNode
   | addNodePort | removeNodePort | addNodeProperty | removeNodeProperty
   | createEdge | deleteEdge | dragNodes | setNodeProperty | resizeNode | renameNode | setEdgeProperty | addEdgeProperty | removeEdgeProperty ;
createNode : 'createNode' ':' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' '[' portList? ']' ',' '[' propertyList? ']' ',' at ;
deleteNode : 'deleteNode' ':' HEX ;
changeNodeKind : 'changeNodeKind' ':' HEX ',' HEX ;
changeNodeLabel : 'changeNodeLabel' ':' HEX ',' HEX ;
moveNode : 'moveNode' ':' HEX ',' HEX ',' HEX ;
addNodePort : 'addNodePort' ':' HEX ',' INT ',' port ;
removeNodePort : 'removeNodePort' ':' HEX ',' INT ;
addNodeProperty : 'addNodeProperty' ':' HEX ',' INT ',' property ;
removeNodeProperty : 'removeNodeProperty' ':' HEX ',' HEX ;
createEdge : 'createEdge' ':' HEX ',' HEX ',' HEX ',' HEX ',' HEX ',' optionalText ',' optionalText ',' '[' propertyList? ']' ',' at ;
optionalText : '-' | '[' HEX ']' ;
at : '-' | INT ;
deleteEdge : 'deleteEdge' ':' HEX ;
dragNodes : 'dragNodes' ':' '[' HEX (',' HEX)* ']' ',' HEX ',' HEX ;
setNodeProperty : 'setNodeProperty' ':' HEX ',' property ;
resizeNode : 'resizeNode' ':' HEX ',' HEX ',' HEX ;
renameNode : 'renameNode' ':' HEX ',' HEX ;
setEdgeProperty : 'setEdgeProperty' ':' HEX ',' property ;
addEdgeProperty : 'addEdgeProperty' ':' HEX ',' INT ',' property ;
removeEdgeProperty : 'removeEdgeProperty' ':' HEX ',' HEX ;
portList : port (',' port)* ;
port : '[' HEX ',' portKind ']' ;
portKind : 'i' | 'o' | 'x' ;
propertyList : property (',' property)* ;
property : HEX ':' VALUE ;
HEX : [0-9a-f]* ;
VALUE : . ;
