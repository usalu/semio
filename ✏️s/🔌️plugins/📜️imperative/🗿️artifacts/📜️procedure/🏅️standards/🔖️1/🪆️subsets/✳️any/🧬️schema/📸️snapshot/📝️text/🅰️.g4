// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase).
grammar Imperative_procedure_snapshot;

artifactMark: 'semio imperative.imperative.dsl v1' ;
document: artifactMark schemaLine flowLine textLine pathLine seedLine ;
schemaLine: 'schema' '=' HEX ;
flowLine: 'flow' '=' childHandle ;
textLine: 'text' '=' childHandle ;
pathLine: 'path' '=' HEX ;
seedLine: 'seed' '=' HEX ;
childHandle: '[' HEX ',' HEX ']' ;

// 📐 Framework dialect-primitive terminal (not defined in the .semio itself).
HEX: [0-9a-f]* ;
