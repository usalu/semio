// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Trinity_rewriting_mutations;

DOCUMENT: 'schema' [ ]+ 'trinity.rewriting.mutations' ;

line: editBeforeFixture | editLhs | editRhs | changeParameterBinding | removeParameterBinding | changeRuleLayoutPoint | removeRuleLayoutPoint | dragWorkingNodes | patchWorkingNodes | dragRuleNodes | setRuleLayoutPoints | deleteWorkingNodes | connectWorkingPorts | disconnectWorkingEdges ;
editBeforeFixture: 'edit-before-fixture' SP text ;
editLhs: 'edit-lhs' SP text ;
editRhs: 'edit-rhs' SP text ;
changeParameterBinding: 'change-parameter-binding' SP key SP value ;
removeParameterBinding: 'remove-parameter-binding' SP key ;
changeRuleLayoutPoint: 'change-rule-layout-point' SP key SP pointBlock ;
removeRuleLayoutPoint: 'remove-rule-layout-point' SP key ;
dragWorkingNodes: 'drag-working-nodes' SP targets SP number SP number ;
patchWorkingNodes: 'patch-working-nodes' SP targets SP key SP value ;
dragRuleNodes: 'drag-rule-nodes' SP targets SP number SP number ;
setRuleLayoutPoints: 'set-rule-layout-points' SP placementTable SP targets ;
deleteWorkingNodes: 'delete-working-nodes' SP targets ;
connectWorkingPorts: 'connect-working-ports' SP key SP key SP key ;
disconnectWorkingEdges: 'disconnect-working-edges' SP targets ;
pointBlock: '{' NL number SP number '}' ;
placementTable: '{' ( NL key SP number SP number )* '}' ;
targets: '[' ( key SP? )* ']' ;
key: OCTET+ ;
value: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
