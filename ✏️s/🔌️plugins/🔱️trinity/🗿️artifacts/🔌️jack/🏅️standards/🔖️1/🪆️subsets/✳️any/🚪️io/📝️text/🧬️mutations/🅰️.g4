// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names, kebab-case -> camelCase).
grammar Trinity_jack_mutations;

line: setQuery | setRoot ;
setQuery: 'set-query' SP text ;
setRoot: 'set-root' (SP text)? ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself).
OCTET: . ;
SP: ' ' ;
