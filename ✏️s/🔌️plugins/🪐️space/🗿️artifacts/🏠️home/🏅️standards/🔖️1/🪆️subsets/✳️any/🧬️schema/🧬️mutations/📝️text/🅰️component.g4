// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Space_home_mutations;

DOCUMENT: 'schema' [ ]+ 'space.home.mutations' ;

line: changeCatalogGeneration ;
changeCatalogGeneration: 'change-catalog-generation' SP number ;
number: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
OCTET: . ;
SP: ' ' ;
