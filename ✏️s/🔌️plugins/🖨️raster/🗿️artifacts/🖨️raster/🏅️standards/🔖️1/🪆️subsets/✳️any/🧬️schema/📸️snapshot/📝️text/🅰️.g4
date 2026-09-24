// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Raster_raster_snapshot;

artifactMark: 'semio raster.raster.dsl v1' ;
document: artifactMark field* ;
field: 'schema' '=' TEXT
     | 'id' '=' TEXT
     | 'title' '=' TEXT
     | 'layers' '=' '[' TEXT* ']'
     | 'assets' '=' '[' assetRow* ']' ;
assetRow: assetField+ ;
assetField: 'key' '=' TEXT
          | 'child' '=' childHandle
          | 'content' '=' TEXT ;
childHandle: 'child_id' '=' TEXT 'target' '=' TEXT ;

// 📐 Framework dialect-primitive terminals.
VALUE: TEXT ;
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\],]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
