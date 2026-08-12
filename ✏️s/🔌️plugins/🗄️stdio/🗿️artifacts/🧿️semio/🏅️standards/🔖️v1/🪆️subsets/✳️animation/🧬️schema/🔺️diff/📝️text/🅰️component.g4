// ANTLR4 grammar for `stdio.semio.animation`'s real `DiffCodec::print_diff`/`parse_diff` text shape
// — descriptive mirror of the authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_animation_diff;

document: timelinesClause? EOF;

timelinesClause: 'timelines' '{' '[' (index (',' index)*)? ']' ';' '[' (timelineModified (',' timelineModified)*)? ']' ';' '[' (timelineAdded (',' timelineAdded)*)? ']' '}';
timelineModified: index ':' timelineDiff;
timelineAdded: index ':' timeline;

timelineDiff: '[' (timelineDiffField (',' timelineDiffField)*)? ']';
timelineDiffField: 'N' ':' optionName | 'C' ':' '[' channelsTripleBody ']';
channelsTripleBody: '[' (index (',' index)*)? ']' ';' '[' (channelModified (',' channelModified)*)? ']' ';' '[' (channelAdded (',' channelAdded)*)? ']';
channelModified: index ':' channelDiff;
channelAdded: index ':' channel;

channelDiff: '[' (channelDiffField (',' channelDiffField)*)? ']';
channelDiffField: 'G' ':' target | 'I' ':' interpolation | 'K' ':' '[' keyframesTripleBody ']';
keyframesTripleBody: '[' (index (',' index)*)? ']' ';' '[' (keyframeModified (',' keyframeModified)*)? ']' ';' '[' (keyframeAdded (',' keyframeAdded)*)? ']';
keyframeModified: index ':' keyframeDiff;
keyframeAdded: index ':' keyframe;

keyframeDiff: '[' (keyframeDiffField (',' keyframeDiffField)*)? ']';
keyframeDiffField: 'T' ':' number | 'Y' ':' value;

optionName: '[' '0' ']' | '[' '1' ',' HEX ']';

timeline: '[' optionName ',' channelList ']';
channelList: '[' (channel (',' channel)*)? ']';
channel: '[' target ',' interpolation ',' keyframeList ']';
keyframeList: '[' (keyframe (',' keyframe)*)? ']';
keyframe: '[' number ',' value ']';
target: '[' HEX ',' property ']';
property: 't' | 'r' | 's' | 'w' | 'c' ':' HEX;
interpolation: 'l' | 's' | 'c';
value: 'S' ':' number | 'V' ':' point3 | 'Q' ':' quat | 'W' ':' numberList;

point3: '[' number ',' number ',' number ']';
quat: '[' number ',' number ',' number ',' number ']';
numberList: '[' (number (',' number)*)? ']';

index: INT;
number: INT | FLOAT;

HEX: [0-9a-f]*;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
