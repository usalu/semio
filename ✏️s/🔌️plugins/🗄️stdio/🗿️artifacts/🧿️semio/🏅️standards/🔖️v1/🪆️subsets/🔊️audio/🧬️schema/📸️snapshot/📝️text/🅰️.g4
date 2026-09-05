// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// ../📖️.grammar.semio, walked by dsl::Recognizer) for stdio.semio.audio's DSL text
// representation (store::ArtifactDsl::parse_dsl/print_dsl, ../../🦀️.rs's
// print_audio_snapshot_body/parse_audio_snapshot_body). The `semio stdio.semio.audio.dsl v1`
// preamble line is stripped by store::semio_format::split_text_preamble before this grammar's
// `document` production runs — `document` below matches the RECONSTRUCTED body (bare
// `artifactMark` token standing in for the stripped preamble), same convention every other real
// pilot's own `.g4` mirror uses.
grammar Stdio_semio_audio_snapshot;

document        : artifactMark schemaLine sampleRateLine formatLine channelsLine tagsLine ;
artifactMark    : 'stdio.semio.audio' ;
schemaLine      : 'schema' '=' HEX ;
sampleRateLine  : 'sampleRate' '=' INT ;
formatLine      : 'format' '=' format ;

channelsLine    : 'channels' '=' '[' channelList? ']' ;
channelList     : channel (',' channel)* ;
channel         : '[' sampleList? ']' ;
sampleList      : HEX (',' HEX)* ;

tagsLine        : 'tags' '=' '[' tagList? ']' ;
tagList         : tag (',' tag)* ;
tag             : '[' HEX ',' HEX ']' ;

format          : 'pcm8' | 'pcm16' | 'pcm24' | 'pcm32' | 'f32' | 'f64' ;

HEX   : [0-9a-f]* ;
INT   : '-'? [0-9]+ ;
WS    : [ \t\r\n]+ -> skip ;
