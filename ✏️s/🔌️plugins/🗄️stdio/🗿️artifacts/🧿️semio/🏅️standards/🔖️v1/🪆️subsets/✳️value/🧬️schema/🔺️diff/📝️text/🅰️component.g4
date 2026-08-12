// ANTLR4 grammar for `stdio.semio.value`'s hand-rolled `DiffCodec` wire text form -- space-
// separated `key=value` tokens (either/both absent = unchanged), tag-prefixed hex encoding, NOT
// JSON. See the sibling 📖️component.grammar.semio for the authoritative production set.
grammar Semio_value_diff;

document: (rootToken (WS nodesToken)? | nodesToken)?;
rootToken: 'root' '=' valueDiff;
nodesToken: 'nodes' '=' namedTriple;

valueDiff
    : 'P' '[' value ']'
    | 'B' '[' ('0' | '1') ']'
    | 'I' '[' hex ']'
    | 'F' '[' hex ']'
    | 'S' '[' hex ']'
    | 'Y' '[' hex ']'
    | 'L' '[' indexedTriple ']'
    | 'M' '[' namedTriple ']'
    | 'R' '[' hex ']'
    ;

indexedTriple: '[' (INT (',' INT)*)? ']' ';' '[' (indexModified (',' indexModified)*)? ']' ';' '[' (indexAdded (',' indexAdded)*)? ']';
indexModified: INT ':' valueDiff;
indexAdded: INT ':' value;

namedTriple: '[' (hex (',' hex)*)? ']' ';' '[' (namedModified (',' namedModified)*)? ']' ';' '[' (namedItem (',' namedItem)*)? ']';
namedModified: hex ':' valueDiff;
namedItem: INT ':' hex ':' value;

value
    : 'Z'
    | 'B' '[' ('0' | '1') ']'
    | 'I' '[' hex ']'
    | 'F' '[' hex ']'
    | 'S' '[' hex ']'
    | 'Y' '[' hex ']'
    | 'L' '[' (value (',' value)*)? ']'
    | 'M' '[' (entry (',' entry)*)? ']'
    | 'R' '[' hex ']'
    ;
entry: hex ':' value;

hex: HEXDIG*;
fragment HEXDIG: [0-9a-fA-F];
INT: [0-9]+;
WS: ' ';
