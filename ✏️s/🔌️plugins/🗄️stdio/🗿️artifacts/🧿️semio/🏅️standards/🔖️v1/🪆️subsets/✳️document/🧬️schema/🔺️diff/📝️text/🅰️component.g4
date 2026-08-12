// ANTLR4 grammar for `stdio.semio.document`'s real one-line `DiffCodec` text form — descriptive
// mirror of the authoritative `📖️component.grammar.semio` (same production names, abridged: full
// per-field diff detail lives there).
grammar Semio_document_diff;

document: stylesLine? imagesLine? blocksLine? EOF;

stylesLine: 'styles' '=' triple;
imagesLine: 'images' '=' triple;
blocksLine: 'blocks' '=' triple;

triple: '[' items? ']' ';' '[' modifiedItems? ']' ';' '[' addedItems? ']';
items: item (',' item)*;
item: HEX | INT;
modifiedItems: modifiedItem (',' modifiedItem)*;
modifiedItem: (HEX | INT) ':' diffBody;
addedItems: addedItem (',' addedItem)*;
addedItem: value | (INT ':' value);

diffBody: '[' .*? ']';
value: '[' .*? ']';

HEX: [0-9a-f]*;
INT: [0-9]+;
WS: [ \t\r\n]+ -> skip;
