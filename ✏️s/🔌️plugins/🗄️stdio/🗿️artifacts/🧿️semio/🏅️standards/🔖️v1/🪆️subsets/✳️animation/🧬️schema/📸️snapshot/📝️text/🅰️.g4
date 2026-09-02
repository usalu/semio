// ANTLR4 grammar for `stdio.semio.animation`'s real text DSL body (the `SemioAnimationSnapshot`
// hand-rolled `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️.grammar.semio` for
// the authoritative, conformance-tested version; this is a descriptive mirror, same production
// names).
grammar Semio_animation_snapshot;

document: artifactMark schemaLine timelinesLine EOF;
artifactMark: 's.stdio.semio.animation';

schemaLine: 'schema' '=' HEX;
timelinesLine: 'timelines' '=' '[' (timeline (',' timeline)*)? ']';

timeline: '[' optionName ',' channelList ']';
optionName: '[' '0' ']' | '[' '1' ',' HEX ']';
channelList: '[' (channel (',' channel)*)? ']';

channel: '[' target ',' interpolation ',' keyframeList ']';
keyframeList: '[' (keyframe (',' keyframe)*)? ']';

target: '[' HEX ',' property ']';
property: 't' | 'r' | 's' | 'w' | 'c' ':' HEX;
interpolation: 'l' | 's' | 'c';

keyframe: '[' number ',' value ']';
value: 'S' ':' number | 'V' ':' point3 | 'Q' ':' quat | 'W' ':' numberList;

point3: '[' number ',' number ',' number ']';
quat: '[' number ',' number ',' number ',' number ']';
numberList: '[' (number (',' number)*)? ']';

number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
