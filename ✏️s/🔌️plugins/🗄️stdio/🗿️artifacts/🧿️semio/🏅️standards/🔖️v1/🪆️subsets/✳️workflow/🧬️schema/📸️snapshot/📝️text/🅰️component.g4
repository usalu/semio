// ANTLR4 grammar for `stdio.semio.workflow`'s real text DSL body (the `SemioWorkflowSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️component.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_workflow_snapshot;

document: artifactMark schemaLine nodesLine edgesLine EOF;
artifactMark: 'stdio.semio.workflow';

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
