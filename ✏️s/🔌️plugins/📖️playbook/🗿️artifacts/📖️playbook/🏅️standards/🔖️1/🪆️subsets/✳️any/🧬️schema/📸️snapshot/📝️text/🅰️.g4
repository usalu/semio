// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Playbook_playbook_snapshot;

artifactMark: 'semio playbook.playbook.dsl v1' ;
document: artifactMark field* ;
field: 'schema' '=' TEXT
     | 'id' '=' TEXT
     | 'version' '=' TEXT
     | 'title' '=' TEXT
     | 'document' '=' childHandle
     | 'flow' '=' childHandle
     | 'steps' '=' TEXT ;
childHandle: 'child_id' '=' TEXT 'target' '=' TEXT ;

// 📐 Framework dialect-primitive terminal: a bare word or a double-quoted, backslash-escaped string.
TEXT: BARE | QUOTED ;
fragment BARE: ~[ \t\r\n="{}[\]]+ ;
fragment QUOTED: '"' ( '\\' . | ~["\\] )* '"' ;
WS: [ \t\r\n]+ -> skip ;
