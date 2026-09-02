// 🅰️ ANTLR grammar for the hand-rolled `stdio.tsv.mutations` wire text (see 🦀️.rs
// `print_tsv_mutation`/`parse_tsv_mutation` — the real, authoritative implementation).
grammar Stdio_tsv_mutations;

mutation : 'set-snapshot' SP 'snapshot=' hex
         | 'set-trailing-newline' SP 'trailing-newline=' BIT
         | 'set-line-ending' SP 'line-ending=' ('lf' | 'crlf')
         | 'insert-row' SP 'index=' INT SP 'row=' hex
         | 'remove-row' SP 'index=' INT
         | 'set-cell' SP 'row-index=' INT SP 'field-index=' INT SP 'value=' hex
         ;

hex : HEXPAIR* | '[' .*? ']' ;
HEXPAIR : [0-9a-f] [0-9a-f] ;
INT : [0-9]+ ;
BIT : [01] ;
SP  : ' ' ;
