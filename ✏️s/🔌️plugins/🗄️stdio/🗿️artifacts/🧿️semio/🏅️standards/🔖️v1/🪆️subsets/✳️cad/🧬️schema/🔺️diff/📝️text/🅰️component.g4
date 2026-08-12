// ANTLR4 grammar for `stdio.semio.cad`'s real `DiffCodec::print_diff`/`parse_diff` wire shape —
// see the sibling `📖️component.grammar.semio` for the authoritative, conformance-tested version;
// this is a descriptive mirror, same production names.
grammar Semio_cad_diff;

document: layersLine? blocksLine? entitiesLine? EOF;

layersLine: 'layers' '=' layersTriple;
layersTriple: '[' (HEX (',' HEX)*)? ']' ';' '[' (layerModified (',' layerModified)*)? ']' ';' '[' (layer (',' layer)*)? ']';
layerModified: HEX ':' layerDiff;

blocksLine: 'blocks' '=' blocksTriple;
blocksTriple: '[' (HEX (',' HEX)*)? ']' ';' '[' (blockModified (',' blockModified)*)? ']' ';' '[' (block (',' block)*)? ']';
blockModified: HEX ':' blockDiff;

entitiesLine: 'entities' '=' entitiesTriple;
entitiesTriple: '[' (HEX (',' HEX)*)? ']' ';' '[' (entityRecordModified (',' entityRecordModified)*)? ']' ';' '[' (entityRecord (',' entityRecord)*)? ']';
entityRecordModified: HEX ':' entityRecordDiff;

layerDiff: '[' optionI32 ',' optionHex ',' optionBool ']';
entityRecordDiff: '[' optionHex ',' optionEntity ']';
blockDiff: '[' optionPoint2 ',' optionEntitiesTriple ']';

optionI32: '[0]' | '[1,' I32 ']';
optionHex: '[0]' | '[1,' HEX ']';
optionBool: '[0]' | '[1,' bool ']';
optionPoint2: '[0]' | '[1,' point2 ']';
optionEntity: '[0]' | '[1,' entity ']';
optionEntitiesTriple: '[0]' | '[1,' entitiesTriple ']';

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
