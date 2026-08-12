// ANTLR4 grammar for `stdio.semio.object`'s hand-rolled `OpText` wire text form --
// `keyword arg=value ...` (space-separated), tag-prefixed hex encoding, NOT JSON. See the
// sibling 📖️component.grammar.semio for the authoritative production set.
grammar Semio_object_mutations;

document
    : 'no-mutation'
    | 'set-snapshot' WS 'snapshot' '=' snapshot
    | 'set-value' WS 'path' '=' path WS 'value' '=' value
    | 'set-map-entry' WS 'path' '=' path WS 'key' '=' hex WS 'value' '=' value
    | 'remove-map-entry' WS 'path' '=' path WS 'key' '=' hex
    | 'insert-list-item' WS 'path' '=' path WS 'index' '=' INT WS 'value' '=' value
    | 'remove-list-item' WS 'path' '=' path WS 'index' '=' INT
    | 'set-object' WS 'id' '=' hex WS 'value' '=' value
    | 'remove-object' WS 'id' '=' hex
    ;

path: '[' (pathSegment (',' pathSegment)*)? ']';
pathSegment: 'K' '[' hex ']' | 'I' '[' INT ']';

snapshot: '[' hex ',' value ',' '[' (objectNode (',' objectNode)*)? ']' ']';
objectNode: hex ':' value;

value
    : 'Z'
    | 'B' '[' bit ']'
    | 'I' '[' hex ']'
    | 'F' '[' hex ']'
    | 'S' '[' hex ']'
    | 'Y' '[' hex ']'
    | 'L' '[' (value (',' value)*)? ']'
    | 'M' '[' (entry (',' entry)*)? ']'
    | 'R' '[' hex ']'
    ;
entry: hex ':' value;
bit: '0' | '1';

hex: HEXDIG*;
fragment HEXDIG: [0-9a-fA-F];
INT: [0-9]+;
WS: ' ';
