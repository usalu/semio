// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// ../📖️component.grammar.semio, walked by dsl::Recognizer) for stdio.semio.audio.mutations'
// `OpText::print_op`/`parse_op` one-line text shape (../../🦀️component.rs's
// `print_audio_mutation`/`parse_audio_mutation`).
grammar Stdio_semio_audio_mutations;

op                : noMutation | setSnapshot | setSampleRate | setFormat | insertChannel | removeChannel | setChannelSamples | insertTag | removeTag | setTagValue ;
noMutation        : 'no-mutation' ;
setSnapshot       : 'set-snapshot' snapshot ;
setSampleRate     : 'set-sample-rate' INT ;
setFormat         : 'set-format' format ;
insertChannel     : 'insert-channel' INT channel ;
removeChannel     : 'remove-channel' INT ;
setChannelSamples : 'set-channel-samples' INT channel ;
insertTag         : 'insert-tag' INT tag ;
removeTag         : 'remove-tag' INT ;
setTagValue       : 'set-tag-value' INT HEX ;

snapshot     : '[' HEX ',' INT ',' format ',' '[' channelList? ']' ',' '[' tagList? ']' ']' ;
channelList  : channel (',' channel)* ;
channel      : '[' sampleList? ']' ;
sampleList   : HEX (',' HEX)* ;
tagList      : tag (',' tag)* ;
tag          : '[' HEX ',' HEX ']' ;
format       : 'pcm8' | 'pcm16' | 'pcm24' | 'pcm32' | 'f32' | 'f64' ;

HEX   : [0-9a-f]* ;
INT   : '-'? [0-9]+ ;
WS    : [ \t\r\n]+ -> skip ;
