// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️component.grammar.semio)
// for s.stdio.semio.object's DSL text representation.
grammar StdioSemioObjectSnapshot;

document        : artifactMark schemaLine transformLine brepLine meshLine propertiesLine ;
artifactMark    : 's.stdio.semio.object' ;
schemaLine      : 'schema' '=' HEX ;
transformLine   : 'transform' '=' '[' NUMBER (',' NUMBER)* ']' ;
brepLine        : 'brep' '=' child ;
meshLine        : 'mesh' '=' child ;
propertiesLine  : 'properties' '=' child ;
child           : '[' ']' | '[' HEX ',' HEX ']' ;

HEX             : [0-9a-f]* ;
NUMBER          : '-'? [0-9]+ ('.' [0-9]+)? ;
