// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names, kebab-case -> camelCase).
grammar Trinity_jack_mutations;

line: setQuery ;
setQuery: 'set-query' SP text ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself).
OCTET: . ;
SP: ' ' ;
