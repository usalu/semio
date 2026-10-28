grammar Stdio_ifc_mutations;
document : mutation EOF;
mutation
  : 'set-file-description' 'values' '=' values
  | 'set-file-name' 'values' '=' values
  | 'set-file-schema' 'values' '=' values
  | 'insert-entity' 'index' '=' unsigned 'entity' '=' entity
  | 'remove-entity' 'id' '=' unsigned
  | 'set-entity-name' 'id' '=' unsigned 'name' '=' hex
  | 'set-entity-arg' 'id' '=' unsigned 'index' '=' unsigned 'value' '=' value
  | 'insert-entity-arg' 'id' '=' unsigned 'index' '=' unsigned 'value' '=' value
  | 'remove-entity-arg' 'id' '=' unsigned 'index' '=' unsigned
  | 'set-entity-complex' 'id' '=' unsigned 'complex' '=' complexList
  ;
value
  : 'U' | 'D'
  | 'I' '[' integer ']'
  | 'R' '[' realNumber ']'
  | ('S' | 'E' | 'B') '[' hex ']'
  | 'F' '[' unsigned ']'
  | 'A' values
  | 'T' '[' hex ',' values ']'
  ;
values : '[' (value (',' value)*)? ']';
complexItem : '[' hex ',' values ']';
complexList : '[' (complexItem (',' complexItem)*)? ']';
entity : '[' unsigned ',' hex ',' values ',' complexList ']';
unsigned : DIGIT+;
integer : '-'? unsigned;
realNumber : integer ('.' DIGIT*)? (('e' | 'E') ('+' | '-')? unsigned)?;
hex : hexDigit*;
hexDigit : DIGIT | HEX_LETTER | 'A' | 'B' | 'D' | 'E' | 'F' | 'e';
DIGIT : [0-9];
HEX_LETTER : [a-fA-F];
WS : [ \t\r\n]+ -> skip;
