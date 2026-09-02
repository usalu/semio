// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start document`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Playbook_playbook_snapshot;

DOCUMENT: 'schema' [ ]+ 'playbook.playbook.snapshot' ;

artifactMark: 'semio playbook.playbook.dsl v1' ;
document: artifactMark schemaLine idLine versionLine titleLine documentLine flowLine ;
schemaLine: 'schema' '=' HEX ;
idLine: 'id' '=' HEX ;
versionLine: 'version' '=' HEX ;
titleLine: 'title' '=' optHex ;
documentLine: 'document' '=' childHandle ;
flowLine: 'flow' '=' childHandle ;
optHex: '[0]' | '[1,' HEX ']' ;
childHandle: '[' HEX ',' HEX ']' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
HEX: [0-9a-f]* ;
