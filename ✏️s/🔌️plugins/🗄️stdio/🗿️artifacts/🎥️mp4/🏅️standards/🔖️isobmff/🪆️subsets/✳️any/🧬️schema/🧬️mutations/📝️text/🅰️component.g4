// 🅰️ ANTLR grammar for stdio.mp4's structured named-record operation form.
grammar Stdio_mp4_mutations;
document : keyword (SPACE field (SPACE field)*)? EOF ;
keyword : 'no-mutation' | 'set-snapshot' | 'set-ftyp' | 'insert-track' | 'remove-track' | 'set-track-dimensions' | 'set-track-codec' | 'insert-sample' | 'remove-sample' | 'set-sample-sync' ;
field : KEY '=' VALUE ;
KEY : [A-Za-z-]+ ;
VALUE : ~[ \r\n]+ ;
SPACE : ' ' ;
