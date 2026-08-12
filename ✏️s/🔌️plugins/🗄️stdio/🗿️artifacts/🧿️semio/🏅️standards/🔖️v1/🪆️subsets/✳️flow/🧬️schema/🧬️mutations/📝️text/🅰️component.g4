// ANTLR4 grammar for `stdio.semio.flow`'s real op text line — descriptive mirror of the
// authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_flow_mutations;

op: (noMutation | setSnapshot | insertNode | removeNode | setNodeKind | setNodeLabel
    | setNodePosition | setNodeParam | removeNodeParam | insertEdge | removeEdge
    | setEdgeEndpoints | setEdgeKind) EOF;

noMutation: 'no-mutation';
setSnapshot: 'set-snapshot' 'snapshot' '=' snapshotLit;
insertNode: 'insert-node' 'node' '=' node;
removeNode: 'remove-node' 'id' '=' HEX;
setNodeKind: 'set-node-kind' 'id' '=' HEX 'kind' '=' HEX;
setNodeLabel: 'set-node-label' 'id' '=' HEX 'label' '=' HEX;
setNodePosition: 'set-node-position' 'id' '=' HEX 'position' '=' point2;
setNodeParam: 'set-node-param' 'id' '=' HEX 'key' '=' HEX 'value' '=' HEX;
removeNodeParam: 'remove-node-param' 'id' '=' HEX 'key' '=' HEX;
insertEdge: 'insert-edge' 'edge' '=' edge;
removeEdge: 'remove-edge' 'id' '=' HEX;
setEdgeEndpoints: 'set-edge-endpoints' 'id' '=' HEX 'from' '=' port 'to' '=' port;
setEdgeKind: 'set-edge-kind' 'id' '=' HEX 'kind' '=' HEX;

snapshotLit: '[' HEX ',' '[' nodeList? ']' ',' '[' edgeList? ']' ']';
nodeList: node (',' node)*;
edgeList: edge (',' edge)*;

node: '[' HEX ',' HEX ',' HEX ',' '[' paramList? ']' ',' point2 ']';
paramList: param (',' param)*;
param: '[' HEX ',' HEX ']';
point2: '[' number ',' number ']';
edge: '[' HEX ',' port ',' port ',' HEX ']';
port: '[' HEX ',' HEX ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
