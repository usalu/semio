grammar Stdio_ply_snapshot;

// 🧬️ Real PLY 1.0 header grammar (see the sibling `📖️.grammar.semio` for the fully
// normative ABNF-flavored version this mirrors; kept here as a compact ANTLR sketch of the same
// structure — magic + format + comments/element-blocks + end_header, then a generic ascii body).

document: magicLine formatLine headerLine* END_HEADER NL body EOF;
magicLine: PLY NL;
formatLine: FORMAT SP formatKind SP VERSION NL;
formatKind: ASCII | BINARY_LE | BINARY_BE;

headerLine: commentLine | objInfoLine | elementBlock;
commentLine: COMMENT (SP TEXT)? NL;
objInfoLine: OBJ_INFO (SP TEXT)? NL;

elementBlock: elementLine propertyLine*;
elementLine: ELEMENT SP IDENT SP DIGITS NL;
propertyLine: scalarPropertyLine | listPropertyLine;
scalarPropertyLine: PROPERTY SP scalarType SP IDENT NL;
listPropertyLine: PROPERTY SP LIST SP scalarType SP scalarType SP IDENT NL;
scalarType: CHAR | UCHAR | SHORT | USHORT | INT | UINT | FLOAT | DOUBLE;

body: row*;
row: cell* NL;
cell: NUMBER (SP NUMBER)*; // scalar: one NUMBER; list: count NUMBER then that many value NUMBERs

PLY: 'ply';
FORMAT: 'format';
ASCII: 'ascii';
BINARY_LE: 'binary_little_endian';
BINARY_BE: 'binary_big_endian';
VERSION: '1.0';
COMMENT: 'comment';
OBJ_INFO: 'obj_info';
ELEMENT: 'element';
PROPERTY: 'property';
LIST: 'list';
END_HEADER: 'end_header';
CHAR: 'char' | 'int8';
UCHAR: 'uchar' | 'uint8';
SHORT: 'short' | 'int16';
USHORT: 'ushort' | 'uint16';
INT: 'int' | 'int32';
UINT: 'uint' | 'uint32';
FLOAT: 'float' | 'float32';
DOUBLE: 'double' | 'float64';
IDENT: [a-zA-Z_][a-zA-Z0-9_]*;
DIGITS: [0-9]+;
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
TEXT: ~[\r\n]+;
SP: ' ';
NL: '\r'? '\n';
