// ANTLR4 mirror (descriptive, not test-parsed — the real recognizer is ../📖️.grammar.semio,
// walked by dsl::Recognizer) for s.stdio.semio.text's mutation op text representation.
grammar StdioSemioTextMutations;
op : insertRun | removeRun | editRun | changeRunLanguage | reorderRuns | addMark | removeMark ;
insertRun : 'insertRun' ':' INT ',' run ;
removeRun : 'removeRun' ':' INT ;
editRun : 'editRun' ':' INT ',' HEX ;
changeRunLanguage : 'changeRunLanguage' ':' INT ',' HEX ;
reorderRuns : 'reorderRuns' ':' INT ',' INT ;
addMark : 'addMark' ':' INT ',' INT ',' mark ;
removeMark : 'removeMark' ':' INT ',' INT ;
run : '[' HEX ',' HEX ',' '[' markList? ']' ']' ;
markList : mark (',' mark)* ;
mark : '[' markKind ',' HEX ']' ;
markKind : 'b' | 'i' | 'c' | 'l' ;
HEX : [0-9a-f]* ;
