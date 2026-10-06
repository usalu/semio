// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Imperative_procedure_snapshot;

artifactMark: 'semio imperative.imperative.dsl v1' ;
document: artifactMark field* ;
field: 'schema' '=' TEXT
     | 'flow' '=' childHandle
     | 'text' '=' childHandle ;
childHandle: 'child_id' '=' TEXT 'target' '=' TEXT ;

// 📐 Framework dialect-primitive terminals.
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\],]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
