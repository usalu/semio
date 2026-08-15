// ANTLR4 grammar for the decoded PDF 1.7 (ISO 32000-1) COS object syntax the `stdio.pdf.1.7`
// snapshot's on-disk body hex-encodes (the wrap_text envelope's payload IS the hex encoding of
// these real bytes, see the sibling `📖️component.grammar.semio` for that envelope detail).
grammar Stdio_pdf_1_7_snapshot;

pdfFile: headerLine indirectObject* xrefSection trailerSection;
headerLine: '%PDF-' DIGIT '.' DIGIT;

indirectObject: INT INT 'obj' object 'endobj';
object: dict ('stream' STREAM_DATA 'endstream')? | array | string | name | reference | NUMBER | 'true' | 'false' | 'null';

dict: '<<' (name object)* '>>';
array: '[' object* ']';
name: '/' NAME_CHARS;
reference: INT INT 'R';
string: '(' STRING_CHARS ')' | '<' HEX_CHARS '>';

xrefSection: 'xref' xrefSubsection+;
xrefSubsection: INT INT xrefEntry+;
xrefEntry: DIGIT10 DIGIT5 ('n' | 'f');
trailerSection: 'trailer' dict 'startxref' INT '%%EOF';

INT: '-'? [0-9]+;
NUMBER: '-'? [0-9]* '.' [0-9]*;
DIGIT: [0-9];
DIGIT5: [0-9] [0-9] [0-9] [0-9] [0-9];
DIGIT10: DIGIT5 DIGIT5;
NAME_CHARS: (~[ \t\r\n()<>[]{}/%])*;
STRING_CHARS: (~[()\\] | '\\' .)*;
HEX_CHARS: [0-9a-fA-F]*;
STREAM_DATA: .*?; // decoded logical stream value; filter pipeline is modeled separately
