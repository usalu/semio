// ANTLR4 grammar for the `stdio.semio.value` snapshot's wire text form -- a tag-prefixed,
// hex-encoded recursive encoding of the WHOLE `SemioValueSnapshot`, genuinely walked/parsed --
// NOT JSON, NOT hex-of-JSON. See the sibling 📖️component.grammar.semio for the authoritative
// production set this mirrors.
grammar Semio_value_snapshot;

document: '[' HEX ',' value ',' '[' (valueNode (',' valueNode)*)? ']' ']';
valueNode: HEX ':' value;

value
    : 'Z'
    | 'B' '[' BIT ']'
    | 'I' '[' HEX ']'
    | 'F' '[' HEX ']'
    | 'S' '[' HEX ']'
    | 'Y' '[' HEX ']'
    | 'L' '[' (value (',' value)*)? ']'
    | 'M' '[' (HEX ':' value (',' HEX ':' value)*)? ']'
    | 'R' '[' HEX ']'
    ;

BIT: [01];
HEX: [0-9a-f]*;
