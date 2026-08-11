// ANTLR4 grammar for the `s.stdio.semio.workflow` snapshot text wire format — a
// `store::semio_format` envelope wrapping a hex-encoded JSON blob (this subset's snapshot is a
// NEUTRAL semio type, not an on-disk file format with its own byte grammar — see
// `SemioWorkflowSnapshot::print_dsl`/`parse_dsl`).
grammar Stdio_semio_workflow_snapshot;

document: envelopeLine NL hexBody EOF;
envelopeLine: 'schema' WS 'stdio.semio.workflow' ;
hexBody: HEXBYTE* ;

HEXBYTE: [0-9a-f] [0-9a-f];
WS: [ \t]+ -> skip;
NL: '\r'? '\n';
