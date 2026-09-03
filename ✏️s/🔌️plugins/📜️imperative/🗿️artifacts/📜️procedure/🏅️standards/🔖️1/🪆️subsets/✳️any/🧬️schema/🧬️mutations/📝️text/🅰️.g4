// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Imperative_procedure_mutations;

DOCUMENT: 'schema' [ ]+ 'imperative.procedure.mutations' ;

line: createStep | deleteStep | reorderSteps | editStepParams ;
createStep: 'create-step' SP owner SP slot SP stepBlock ;
deleteStep: 'delete-step' SP owner SP slot SP id ;
reorderSteps: 'reorder-steps' SP owner SP slot SP id SP to ;
editStepParams: 'edit-step-params' SP owner SP slot SP id SP params ;
owner: 'owner=' (id | '-') ;
slot: 'slot=' (id | '-') ;
to: 'to=' number ;
params: 'params=' text ;
stepBlock: 'item=' '{' NL stepFields '}' ;
stepFields: OCTET+ ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
