// 🅰️ ANTLR grammar for the hand-rolled `stdio.epw.mutations` wire text (see 🦀️component.rs
// `print_epw_mutation`/`parse_epw_mutation` — the real, authoritative implementation).
grammar Stdio_epw_mutations;

mutation : 'set-snapshot' SP 'snapshot=' hex
         | 'set-location' SP 'location=' hex
         | 'set-design-conditions' SP 'value=' hex
         | 'set-typical-extreme-periods' SP 'value=' hex
         | 'set-ground-temperatures' SP 'value=' hex
         | 'set-holidays-dst' SP 'value=' hex
         | 'set-comments-1' SP 'value=' hex
         | 'set-comments-2' SP 'value=' hex
         | 'set-data-periods' SP 'data-periods=' hex
         | 'insert-record' SP 'index=' INT SP 'record=' hex
         | 'remove-record' SP 'index=' INT
         | 'set-record-field' SP 'record-index=' INT SP 'field-index=' INT SP 'value=' hex
         ;

hex : HEXPAIR* | '[' .*? ']' ;
HEXPAIR : [0-9a-f] [0-9a-f] ;
INT : [0-9]+ ;
SP  : ' ' ;
