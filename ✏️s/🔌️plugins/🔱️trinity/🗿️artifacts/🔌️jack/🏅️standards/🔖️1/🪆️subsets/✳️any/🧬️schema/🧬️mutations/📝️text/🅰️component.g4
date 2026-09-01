// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Trinity_jack_mutations;

DOCUMENT: 'schema' [ ]+ 'trinity.jack.mutations' ;

line: createNode | deleteNode | createEdge | deleteEdge | renameNode | moveNode | changeDataProperty | removeDataProperty ;
createNode: 'create-node' SP id SP text SP text SP number SP number SP number SP number SP portTable ;
deleteNode: 'delete-node' SP id ;
createEdge: 'create-edge' SP id SP text SP text SP text SP propertyBag ;
deleteEdge: 'delete-edge' SP id ;
renameNode: 'rename-node' SP id SP text ;
moveNode: 'move-node' SP id SP number SP number ;
changeDataProperty: 'change-data-property' SP entity SP text SP value ;
removeDataProperty: 'remove-data-property' SP entity SP text ;
entity: 'node' ':' id | 'edge' ':' id ;
portTable: '{' NL portRow* '}' ;
portRow: OCTET+ ;
propertyBag: '{' NL propertyRow* '}' ;
propertyRow: OCTET+ ;
value: OCTET+ ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
