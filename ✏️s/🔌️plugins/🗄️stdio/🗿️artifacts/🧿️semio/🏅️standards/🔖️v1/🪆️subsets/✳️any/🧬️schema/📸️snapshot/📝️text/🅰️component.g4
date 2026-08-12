// ANTLR4 grammar for `stdio.semio`'s real text DSL body (the `SemioSnapshot` hand-rolled
// `print_dsl`/`parse_dsl` wire shape — see the sibling `📖️component.grammar.semio` for the
// authoritative, conformance-tested version; this is a descriptive mirror, same production names).
// Real header only (`artifactMark`/`subsetLine`) — the wrapped subset's own body is opaque REST,
// validated by THAT subset's own grammar, not re-described here (see the sibling file's comment).
grammar Semio_any_snapshot;

document: artifactMark subsetLine REST EOF;
artifactMark: 'stdio.semio';

subsetLine: 'subset' '=' subsetTag;
subsetTag: 'brep' | 'mesh' | 'model' | 'object' | 'document' | 'cad' | 'drawing' | 'image' | 'video' | 'audio' | 'animation' | 'presentation' | 'workflow';

REST: .*? ;
WS: [ \t\r\n]+ -> skip;
