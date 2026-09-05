// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// ../📖️.grammar.semio, walked by dsl::Recognizer) for stdio.semio.audio.diff's
// `DiffCodec::print_diff`/`parse_diff` one-line text shape (../../🦀️.rs's
// `print_audio_diff`/`parse_audio_diff`).
grammar Stdio_semio_audio_diff;

document        : rateLine? formatLine? channelsLine? tagsLine? ;
rateLine        : 'rate' '=' INT ;
formatLine      : 'format' '=' format ;

channelsLine    : 'channels' '{' channelsTriple '}' ;
channelsTriple  : '[' removedIndexList? ']' ';' '[' channelModifiedList? ']' ';' '[' channelAddedList? ']' ;
removedIndexList: INT (',' INT)* ;
channelModifiedList : channelModified (',' channelModified)* ;
channelModified : INT ':' channelDiff ;
channelDiff     : '[' '0' ']' | '[' '1' ',' channel ']' ;
channelAddedList: channelAdded (',' channelAdded)* ;
channelAdded    : INT ':' channel ;
channel         : '[' sampleList? ']' ;
sampleList      : HEX (',' HEX)* ;

tagsLine        : 'tags' '{' tagsTriple '}' ;
tagsTriple      : '[' removedIndexList? ']' ';' '[' tagModifiedList? ']' ';' '[' tagAddedList? ']' ;
tagModifiedList : tagModified (',' tagModified)* ;
tagModified     : INT ':' tag ;
tagAddedList    : tagAdded (',' tagAdded)* ;
tagAdded        : INT ':' tag ;
tag             : '[' HEX ',' HEX ']' ;

format          : 'pcm8' | 'pcm16' | 'pcm24' | 'pcm32' | 'f32' | 'f64' ;

HEX   : [0-9a-f]* ;
INT   : '-'? [0-9]+ ;
WS    : [ \t\r\n]+ -> skip ;
