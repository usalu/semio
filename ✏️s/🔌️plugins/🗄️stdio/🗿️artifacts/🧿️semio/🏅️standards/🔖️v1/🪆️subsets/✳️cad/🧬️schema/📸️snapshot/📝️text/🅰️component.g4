// ANTLR4 grammar for `stdio.semio.cad`'s real text DSL body (the `SemioCadSnapshot` hand-rolled
// `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️component.grammar.semio` for the
// authoritative, conformance-tested version; this is a descriptive mirror, same production names).
grammar Semio_cad_snapshot;

document: artifactMark schemaLine layersLine blocksLine entitiesLine EOF;
artifactMark: 'stdio.semio.cad';

schemaLine: 'schema' '=' HEX;

layersLine: 'layers' '=' '[' (layer (',' layer)*)? ']';
blocksLine: 'blocks' '=' '[' (block (',' block)*)? ']';
entitiesLine: 'entities' '=' '[' (entityRecord (',' entityRecord)*)? ']';

layer: '[' HEX ',' I32 ',' HEX ',' bool ']';
entityRecord: '[' HEX ',' HEX ',' entity ']';
entityRecordList: '[' (entityRecord (',' entityRecord)*)? ']';
block: '[' HEX ',' point2 ',' entityRecordList ']';

entity: 'L' '[' point2 ',' point2 ']'                                          # line
      | 'A' '[' point2 ',' number ',' number ',' number ']'                    # arc
      | 'C' '[' point2 ',' number ']'                                          # circle
      | 'E' '[' point2 ',' point2 ',' number ',' number ',' number ']'         # ellipse
      | 'P' '[' point2List ',' bool ']'                                        # polyline
      | 'T' '[' point2 ',' number ',' number ',' HEX ']'                       # text
      | 'I' '[' HEX ',' point2 ',' point2 ',' number ']'                       # insert
      | 'S' '[' point2 ',' point2 ',' point2 ',' point2 ']'                    # solid
      | 'D' '[' point2 ',' point2 ',' number ',' HEX ']'                       # dimension
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
