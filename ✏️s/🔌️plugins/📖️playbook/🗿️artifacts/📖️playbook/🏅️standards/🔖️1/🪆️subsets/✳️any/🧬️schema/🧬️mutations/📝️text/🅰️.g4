// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Playbook_playbook_mutations;

DOCUMENT: 'schema' [ ]+ 'playbook.playbook.mutations' ;

line: addStep | removeStep | moveStep | addBlock | removeBlock | moveBlock | replaceBlock | updateStep | changeTitle ;
addStep: 'add-step' SP stepBlock (SP 'index' '=' number)? ;
removeStep: 'remove-step' SP stepId ;
moveStep: 'move-step' SP stepId SP 'index' '=' number ;
addBlock: 'add-block' SP stepId SP blockBlock (SP 'index' '=' number)? ;
removeBlock: 'remove-block' SP stepId SP blockId ;
moveBlock: 'move-block' SP blockId SP fromStepId SP toStepId SP 'index' '=' number ;
replaceBlock: 'replace-block' SP stepId SP blockBlock ;
updateStep: 'update-step' SP stepId SP 'title' '=' text (SP 'description' '=' text)? ;
changeTitle: 'change-title' (SP 'new-title' '=' text)? ;
stepBlock: '{' NL stepFields '}' ;
blockBlock: '{' NL blockFields '}' ;
stepFields: OCTET+ ;
blockFields: OCTET+ ;
stepId: IDENT ;
fromStepId: IDENT ;
toStepId: IDENT ;
blockId: IDENT ;
number: OCTET+ ;
text: OCTET+ ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
IDENT: [A-Za-z_] [A-Za-z0-9_]* ;
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
