grammar Stdio_deflate_diff;
// DeflateDiff wire form: sparse camelCase JSON object, only changed fields present.
document              : '{' (member (',' member)*)? '}' ;
member                : compressionMethod | windowBits | compressionLevelHint | dictId | payload ;
compressionMethod     : '"compressionMethod":' DIGIT+ ;
windowBits            : '"windowBits":' DIGIT+ ;
compressionLevelHint  : '"compressionLevelHint":' LEVEL_HINT ;
dictId                : '"dictId":' ( 'null' | DIGIT+ ) ;   // tri-state
payload               : '"payload":' '[' (DIGIT+ (',' DIGIT+)*)? ']' ;
LEVEL_HINT            : '"fastest"' | '"fast"' | '"default"' | '"maximum"' ;
DIGIT                 : [0-9] ;
