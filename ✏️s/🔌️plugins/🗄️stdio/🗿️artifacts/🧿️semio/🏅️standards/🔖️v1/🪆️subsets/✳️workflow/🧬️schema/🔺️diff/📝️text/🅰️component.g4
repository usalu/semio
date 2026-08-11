// ANTLR4 grammar for the `s.stdio.semio.workflow` diff text wire format (`SemioWorkflowDiff`'s
// hand-rolled `protocol::DiffCodec::print_diff`/`parse_diff` — see 🔺️diff/🦀️component.rs).
grammar Stdio_semio_workflow_diff;

diffLine: (token (WS token)*)? EOF;
token: 'nodes=' namedTriple | 'edges=' namedTriple;

namedTriple: '[' keyList ']' ';' '[' modifiedList ']' ';' '[' addedList ']';
keyList: (hexStr (',' hexStr)*)?;
modifiedList: (modifiedEntry (',' modifiedEntry)*)?;
modifiedEntry: hexStr ':' bracketed;
addedList: (bracketed (',' bracketed)*)?;

bracketed: '[' .*? ']';   // node/edge/nodeDiff/edgeDiff payload — see the Rust value codecs
hexStr: HEXBYTE*;

HEXBYTE: [0-9a-f] [0-9a-f];
WS: [ ]+;
