// ANTLR4 grammar for `stdio.semio.presentation`'s real text DSL body (the
// `SemioPresentationSnapshot` hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling
// `📖️component.grammar.semio` for the authoritative, conformance-tested version; this is a
// descriptive mirror, same production names). `block`/`run`/`runStyle`/`listItem`/`docRow`/
// `docCell` are copied from `document`'s own real grammar mirror verbatim (this facet's
// `enc_block`/`dec_block` is a direct re-export of document's own real codec).
grammar Semio_semio_presentation_snapshot;

document: artifactMark schemaLine mastersLine layoutsLine slidesLine EOF;
artifactMark: 's.stdio.semio.presentation';

schemaLine: 'schema' '=' HEX;
mastersLine: 'masters' '=' '[' (master (',' master)*)? ']';
layoutsLine: 'layouts' '=' '[' (layout (',' layout)*)? ']';
slidesLine: 'slides' '=' '[' (slide (',' slide)*)? ']';

master: '[' HEX ',' '[' (shape (',' shape)*)? ']' ']';
layout: '[' HEX ',' HEX ',' '[' (shape (',' shape)*)? ']' ']';
slide: '[' HEX ',' optionHex ',' '[' (shape (',' shape)*)? ']' ',' '[' (block (',' block)*)? ']' ']';

shape
  : 'X' '[' frame ',' '[' (block (',' block)*)? ']' ']'
  | 'P' '[' frame ',' image ']'
  | 'T' '[' frame ',' '[' (row (',' row)*)? ']' ']'
  | 'H' '[' frame ',' placeholderKind ']'
  ;

frame: '[' point2 ',' INT ',' INT ']';
point2: '[' INT ',' INT ']';
image: '[' HEX ',' HEX ',' HEX ']';
placeholderKind: 'T' | 'S' | 'B' | 'F' | 'N' | 'D' | 'O' '[' HEX ']';

row: '[' (cell (',' cell)*)? ']';
cell: '[' (block (',' block)*)? ']';

block
  : 'P' '[' optionHex ',' '[' (run (',' run)*)? ']' ']'
  | 'H' '[' INT ',' optionHex ',' '[' (run (',' run)*)? ']' ']'
  | 'L' '[' bool ',' '[' (listItem (',' listItem)*)? ']' ']'
  | 'T' '[' '[' (docRow (',' docRow)*)? ']' ']'
  | 'C' '[' optionHex ',' HEX ']'
  | 'Q' '[' (block (',' block)*)? ']'
  | 'I' '[' HEX ',' HEX ',' optionF64 ',' optionF64 ']'
  | 'B' '[' ']'
  ;

run: '[' HEX ',' runStyle ']';
runStyle: '[' bool ',' bool ',' bool ',' optionF64 ',' optionHex ',' optionHex ',' optionHex ']';
listItem: '[' '[' (block (',' block)*)? ']' ']';
docRow: '[' '[' (docCell (',' docCell)*)? ']' ']';
docCell: '[' '[' (block (',' block)*)? ']' ']';

optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionF64: '[' '0' ']' | '[' '1' ',' INT ']';
bool: '0' | '1';

HEX: [0-9a-f]*;
INT: [0-9]+;
WS: [ \t\r\n]+ -> skip;
