// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️component.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.text's DSL text representation.
grammar StdioSemioTextSnapshot;

document        : artifactMark schemaLine runsLine ;
artifactMark    : 'stdios.stdio.semio.text' ;
schemaLine      : 'schema' '=' HEX ;
runsLine        : 'runs' '=' '[' runList? ']' ;
runList         : run (',' run)* ;
run             : '[' HEX ',' HEX ',' '[' markList? ']' ']' ;
markList        : mark (',' mark)* ;
mark            : '[' markKind ',' HEX ']' ;
markKind        : 'b' | 'i' | 'c' | 'l' ;

HEX             : [0-9a-f]* ;
