// ANTLR4 grammar for the hand-rolled `stdio.semio.model.diff` text line (space-separated
// `field=value` tokens over the shared 🧰️triples bracket-triple wire shape).
grammar Stdio_semio_model_diff;

line: token (WS token)* EOF | EOF;
token: ('spatial=' | 'elements=' | 'relations=') namedTriple;
namedTriple: '[' removedCsv '];[' modifiedCsv '];[' addedCsv ']';
removedCsv: (HEX (',' HEX)*)?;
modifiedCsv: (modifiedEntry (',' modifiedEntry)*)?;
modifiedEntry: HEX ':' bracketPayload;
addedCsv: (bracketPayload (',' bracketPayload)*)?;
bracketPayload: '[' .*? ']';

HEX: [0-9a-f]+;
WS: [ ]+;
