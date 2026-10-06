// ANTLR4 grammar for `stdio.semio.presentation`'s real one-line `DiffCodec` text form —
// descriptive mirror of the authoritative `📖️.grammar.semio` (same production names,
// abridged: full per-field diff detail lives there).
grammar Semio_semio_presentation_diff;

document: mastersLine? layoutsLine? slidesLine? EOF;

mastersLine: 'masters' '=' triple;
layoutsLine: 'layouts' '=' triple;
slidesLine: 'slides' '=' triple;

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
