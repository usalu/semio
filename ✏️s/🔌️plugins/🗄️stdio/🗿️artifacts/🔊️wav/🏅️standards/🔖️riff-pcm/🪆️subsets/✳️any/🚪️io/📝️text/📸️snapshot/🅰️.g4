grammar Semio_wav_snapshot;
// The DSL text of a WAV snapshot (after the `semio stdio.wav.dsl v1` preamble): `key = value` records in a fixed order, as in 📖️.grammar.semio.
document : 'schema' '=' text 'fmt-pad-byte' '=' INT 'data-pad-byte' '=' INT fmt data otherChunks chunkOrder EOF ;
fmt : 'fmt' '=' 'audio-format' '=' INT 'channels' '=' INT 'sample-rate' '=' INT 'byte-rate' '=' INT 'block-align' '=' INT 'bits-per-sample' '=' INT ('ext' '=' '[' INT* ']')? ;
data : 'data' '=' 'kind' '=' ('pcm16' | 'pcm8' | 'float32' | 'raw') ('pcm16' | 'pcm8' | 'float32' | 'raw') '=' '[' number* ']' ;
otherChunks : 'other-chunks' '=' '[' ('{' 'fourcc' '=' text 'data' '=' STRING 'pad-byte' '=' INT '}')* ']' ;
chunkOrder : 'chunk-order' '=' '[' ('{' 'kind' '=' ('format' | 'samples' | 'other' 'index' '=' INT) '}')* ']' ;
text : STRING | IDENT | INT ;
number : INT | FLOAT ;
INT : '-'? [0-9]+ ;
FLOAT : '-'? [0-9]+ '.' [0-9]+ ([eE] [+-]? [0-9]+)? ;
STRING : '"' ( ~["\\] | '\\' . )* '"' ;
IDENT : [A-Za-z_] [A-Za-z0-9_-]* ;
WS : [ \t\r\n]+ -> skip ;
