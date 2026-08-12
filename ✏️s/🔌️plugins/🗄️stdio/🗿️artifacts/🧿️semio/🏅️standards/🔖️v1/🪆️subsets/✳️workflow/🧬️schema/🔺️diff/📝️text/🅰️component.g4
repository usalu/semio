// ANTLR4 grammar for `stdio.semio.workflow`'s real diff text line — descriptive mirror of the
// authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_workflow_diff;

document: nodesLine? edgesLine? EOF;

nodesLine: 'nodes' '=' nodesTriple;
nodesTriple: '[' removedList? ']' ';' '[' nodeModifiedList? ']' ';' '[' nodeAddedList? ']';
nodeModifiedList: nodeModified (',' nodeModified)*;
nodeModified: HEX ':' nodeDiff;
nodeAddedList: node (',' node)*;

edgesLine: 'edges' '=' edgesTriple;
edgesTriple: '[' removedList? ']' ';' '[' edgeModifiedList? ']' ';' '[' edgeAddedList? ']';
edgeModifiedList: edgeModified (',' edgeModified)*;
edgeModified: HEX ':' edgeDiff;
edgeAddedList: edge (',' edge)*;

removedList: HEX (',' HEX)*;

node: '[' HEX ',' HEX ',' HEX ',' '[' paramList? ']' ',' '[' number ',' number ']' ']';
paramList: param (',' param)*;
param: '[' HEX ',' HEX ']';
edge: '[' HEX ',' port ',' port ',' HEX ']';
port: '[' HEX ',' HEX ']';

nodeDiff: '[' optionHex ',' optionHex ',' optionParamsDiff ',' optionPoint2 ']';
optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionParamsDiff: '[' '0' ']' | '[' '1' ',' paramsTriple ']';
optionPoint2: '[' '0' ']' | '[' '1' ',' '[' number ',' number ']' ']';
paramsTriple: '[' removedList? ']' ';' '[' paramModifiedList? ']' ';' '[' paramAddedList? ']';
paramModifiedList: paramModified (',' paramModified)*;
paramModified: HEX ':' paramDiff;
paramAddedList: param (',' param)*;
paramDiff: '[' optionHex ']';

edgeDiff: '[' optionPort ',' optionPort ',' optionHex ']';
optionPort: '[' '0' ']' | '[' '1' ',' port ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
