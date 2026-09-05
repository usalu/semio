// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.text's diff text representation.
grammar StdioSemioTextDiff;
document : runsLine? ;
runsLine : 'runs' '=' '[' runList? ']' ;
runList  : run (',' run)* ;
run      : '[' HEX ',' HEX ',' '[' markList? ']' ']' ;
markList : mark (',' mark)* ;
mark     : '[' markKind ',' HEX ']' ;
markKind : 'b' | 'i' | 'c' | 'l' ;
HEX      : [0-9a-f]* ;
