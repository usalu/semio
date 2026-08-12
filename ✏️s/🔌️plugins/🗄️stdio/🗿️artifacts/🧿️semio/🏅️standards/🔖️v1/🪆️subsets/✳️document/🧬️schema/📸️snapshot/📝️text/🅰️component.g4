// ANTLR4 grammar for `stdio.semio.document`'s real text DSL body (the `SemioDocumentSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️component.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_document_snapshot;

document: artifactMark schemaLine stylesLine imagesLine blocksLine EOF;
artifactMark: 's.stdio.semio.document';

schemaLine: 'schema' '=' HEX;

stylesLine: 'styles' '=' '[' (style (',' style)*)? ']';
imagesLine: 'images' '=' '[' (image (',' image)*)? ']';
blocksLine: 'blocks' '=' '[' (block (',' block)*)? ']';

style: '[' HEX ',' HEX ',' optionHex ']';
image: '[' HEX ',' HEX ',' HEX ']';

block
  : 'P' '[' optionHex ',' '[' (run (',' run)*)? ']' ']'
  | 'H' '[' INT ',' optionHex ',' '[' (run (',' run)*)? ']' ']'
  | 'L' '[' bool ',' '[' (listItem (',' listItem)*)? ']' ']'
  | 'T' '[' '[' (row (',' row)*)? ']' ']'
  | 'C' '[' optionHex ',' HEX ']'
  | 'Q' '[' (block (',' block)*)? ']'
  | 'I' '[' HEX ',' HEX ',' optionF64 ',' optionF64 ']'
  | 'B' '[' ']'
  ;

run: '[' HEX ',' runStyle ']';
runStyle: '[' bool ',' bool ',' bool ',' optionF64 ',' optionHex ',' optionHex ',' optionHex ']';

listItem: '[' '[' (block (',' block)*)? ']' ']';
row: '[' '[' (cell (',' cell)*)? ']' ']';
cell: '[' '[' (block (',' block)*)? ']' ']';

optionHex: '[' '0' ']' | '[' '1' ',' HEX ']';
optionF64: '[' '0' ']' | '[' '1' ',' INT ']';
bool: '0' | '1';

HEX: [0-9a-f]*;
INT: [0-9]+;
WS: [ \t\r\n]+ -> skip;
