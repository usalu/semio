// ANTLR4 grammar for `stdio.semio.animation`'s real `OpText::print_op`/`parse_op` text shape —
// descriptive mirror of the authoritative `📖️.grammar.semio` (same production names).
grammar Semio_animation_mutations;

op: (setSnapshot | insertTimeline | removeTimeline | setTimelineName | insertChannel | removeChannel | setChannelTarget | setChannelInterpolation | insertKeyframe | removeKeyframe | setKeyframeTime | setKeyframeValue) EOF;

setSnapshot: 'S' ':' snapshotLit;
insertTimeline: 'IT' ':' index ',' timeline;
removeTimeline: 'RT' ':' index;
setTimelineName: 'TN' ':' index ',' optionName;
insertChannel: 'IC' ':' index ',' index ',' channel;
removeChannel: 'RC' ':' index ',' index;
setChannelTarget: 'CT' ':' index ',' index ',' target;
setChannelInterpolation: 'CI' ':' index ',' index ',' interpolation;
insertKeyframe: 'IK' ':' index ',' index ',' index ',' keyframe;
removeKeyframe: 'RK' ':' index ',' index ',' index;
setKeyframeTime: 'KT' ':' index ',' index ',' index ',' number;
setKeyframeValue: 'KV' ':' index ',' index ',' index ',' value;

snapshotLit: '[' HEX ',' timelineList ']';
timelineList: '[' (timeline (',' timeline)*)? ']';

timeline: '[' optionName ',' channelList ']';
optionName: '[' '0' ']' | '[' '1' ',' HEX ']';
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
