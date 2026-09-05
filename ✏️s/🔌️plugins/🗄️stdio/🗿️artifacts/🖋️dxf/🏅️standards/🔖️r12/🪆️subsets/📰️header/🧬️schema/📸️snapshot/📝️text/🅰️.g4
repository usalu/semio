// 🅰️ ANTLR4 grammar for the DXF R12 ASCII group-code/value tag stream (📸️snapshot facet).
// Every DXF token is a two-line pair: an integer group code, then its value on the next line.
// The document is a sequence of SECTION blocks (HEADER/TABLES/BLOCKS/ENTITIES) terminated by EOF.
grammar Stdio_dxf_snapshot;

document   : section* eofTag ;
section    : tag0 sectionKw tag2 sectionName body* endsecTag ;
body       : tag ;                          // any (code, value) pair belonging to this section
tag        : CODE NEWLINE VALUE NEWLINE ;
tag0       : '0' NEWLINE ;                   // the group-0 "kind" tag preceding an entity/entry
sectionKw  : 'SECTION' NEWLINE ;
sectionName: ('HEADER' | 'TABLES' | 'BLOCKS' | 'ENTITIES') NEWLINE ;
endsecTag  : '0' NEWLINE 'ENDSEC' NEWLINE ;
eofTag     : '0' NEWLINE 'EOF' NEWLINE ;

CODE   : '-'? [0-9]+ ;
VALUE  : ~[\r\n]* ;
NEWLINE: '\r'? '\n' ;
