// ANTLR4 grammar for `stdio.semio.flow`'s real text DSL body (the `SemioFlowSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_flow_snapshot;

document: artifactMark schemaLine nodesLine edgesLine EOF;
artifactMark: 'stdio.semio.flow';

schemaLine: 'schema' '=' HEX;

nodesLine: 'nodes' '=' '[' nodeList? ']';
nodeList: node (',' node)*;
node: '[' HEX ',' HEX ',' HEX ',' '[' paramList? ']' ',' '[' number ',' number ']' ']';
paramList: param (',' param)*;
param: '[' HEX ',' HEX ']';

edgesLine: 'edges' '=' '[' edgeList? ']';
edgeList: edge (',' edge)*;
edge: '[' HEX ',' port ',' port ',' HEX ']';
port: '[' HEX ',' HEX ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
