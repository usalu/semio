// ANTLR4 grammar for `stdio.semio.cad`'s real `OpText::print_op`/`parse_op` wire shape — see the
// sibling `📖️.grammar.semio` for the authoritative, conformance-tested version; this is
// a descriptive mirror, same production names.
grammar Semio_cad_mutations;

op: setSnapshot
  | addLayer
  | removeLayer
  | setLayer
  | addBlock
  | removeBlock
  | setBlockBasePoint
  | addEntity
  | removeEntity
  | setEntityLayer
  | setEntityGeometry
  | addBlockEntity
  | removeBlockEntity
  | setBlockEntityLayer
  | setBlockEntityGeometry
  ;

setSnapshot: 'set-snapshot' 'snapshot' '=' snapshotLit;
addLayer: 'add-layer' 'layer' '=' layer;
removeLayer: 'remove-layer' 'name' '=' HEX;
setLayer: 'set-layer' 'name' '=' HEX 'color-index' '=' optionI32 'line-type' '=' optionHex 'visible' '=' optionBool;
addBlock: 'add-block' 'block' '=' block;
removeBlock: 'remove-block' 'name' '=' HEX;
setBlockBasePoint: 'set-block-base-point' 'name' '=' HEX 'base-point' '=' point2;
addEntity: 'add-entity' 'entity' '=' entityRecord;
removeEntity: 'remove-entity' 'handle' '=' HEX;
setEntityLayer: 'set-entity-layer' 'handle' '=' HEX 'layer' '=' HEX;
setEntityGeometry: 'set-entity-geometry' 'handle' '=' HEX 'entity' '=' entity;
addBlockEntity: 'add-block-entity' 'block-name' '=' HEX 'entity' '=' entityRecord;
removeBlockEntity: 'remove-block-entity' 'block-name' '=' HEX 'handle' '=' HEX;
setBlockEntityLayer: 'set-block-entity-layer' 'block-name' '=' HEX 'handle' '=' HEX 'layer' '=' HEX;
setBlockEntityGeometry: 'set-block-entity-geometry' 'block-name' '=' HEX 'handle' '=' HEX 'entity' '=' entity;

optionI32: '[0]' | '[1,' I32 ']';
optionHex: '[0]' | '[1,' HEX ']';
optionBool: '[0]' | '[1,' bool ']';

snapshotLit: '[' HEX ',' '[' (layer (',' layer)*)? ']' ',' '[' (block (',' block)*)? ']' ',' '[' (entityRecord (',' entityRecord)*)? ']' ']';

layer: '[' HEX ',' I32 ',' HEX ',' bool ']';
entityRecord: '[' HEX ',' HEX ',' entity ']';
entityRecordList: '[' (entityRecord (',' entityRecord)*)? ']';
block: '[' HEX ',' point2 ',' entityRecordList ']';

entity: 'L' '[' point2 ',' point2 ']'
      | 'A' '[' point2 ',' number ',' number ',' number ']'
      | 'C' '[' point2 ',' number ']'
      | 'E' '[' point2 ',' point2 ',' number ',' number ',' number ']'
      | 'P' '[' point2List ',' bool ']'
      | 'T' '[' point2 ',' number ',' number ',' HEX ']'
      | 'I' '[' HEX ',' point2 ',' point2 ',' number ']'
      | 'S' '[' point2 ',' point2 ',' point2 ',' point2 ']'
      | 'D' '[' point2 ',' point2 ',' number ',' HEX ']'
      ;

point2: '[' number ',' number ']';
point2List: '[' (point2 (',' point2)*)? ']';
bool: '0' | '1';
number: INT | FLOAT;

HEX: [0-9a-f]*;
I32: '-'? [0-9]+;
INT: '-'? [0-9]+;
FLOAT: '-'? [0-9]+ '.' [0-9]+;
WS: [ \t\r\n]+ -> skip;
