// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Puzzle_puzzle5d_mutations;

DOCUMENT: 'schema' [ ]+ 'puzzle.puzzle5d.mutations' ;

line: createPart | deletePart | movePart2d | replacePart2dGeometry | editPart2dText | changePart2dIcon | changePart2dHidden | changePart2dLocked | movePart3d | rotatePart3d | scalePart3d | changePart3dMesh | editPart3dLabel | changePartKind | changePartAnchor | addPartGrip | removePartGrip | replacePartGrip | connectGrips | disconnectGrips | replaceFastenerGeometry | changeFastenerKind | renamePuzzle5d | changeDomain | changeDescription | connectKindCompatibility | disconnectKindCompatibility | replaceKindCatalogs ;
createPart: 'create-part' SP partBlock SP indexOpt ;
deletePart: 'delete-part' SP id ;
movePart2d: 'move-part2d' SP id SP number SP number ;
replacePart2dGeometry: 'replace-part2d-geometry' SP id SP textOpt SP numberOpt SP numberOpt SP numberOpt ;
editPart2dText: 'edit-part2d-text' SP id SP textOpt ;
changePart2dIcon: 'change-part2d-icon' SP id SP textOpt ;
changePart2dHidden: 'change-part2d-hidden' SP id SP booleanOpt ;
changePart2dLocked: 'change-part2d-locked' SP id SP booleanOpt ;
movePart3d: 'move-part3d' SP id SP number SP number SP number ;
rotatePart3d: 'rotate-part3d' SP id SP quatOpt ;
scalePart3d: 'scale-part3d' SP id SP scaleOpt ;
changePart3dMesh: 'change-part3d-mesh' SP id SP textOpt ;
editPart3dLabel: 'edit-part3d-label' SP id SP textOpt ;
changePartKind: 'change-part-kind' SP id SP textOpt ;
changePartAnchor: 'change-part-anchor' SP id SP anchor ;
addPartGrip: 'add-part-grip' SP id SP gripBlock SP indexOpt ;
removePartGrip: 'remove-part-grip' SP id SP id ;
replacePartGrip: 'replace-part-grip' SP id SP id SP gripBlock ;
connectGrips: 'connect-grips' SP id SP fullGripId SP fullGripId SP fastenerFields ;
disconnectGrips: 'disconnect-grips' SP id ;
replaceFastenerGeometry: 'replace-fastener-geometry' SP id SP number SP number SP number SP number SP number SP number SP number SP number ;
changeFastenerKind: 'change-fastener-kind' SP id SP textOpt ;
renamePuzzle5d: 'rename-puzzle5d' SP textOpt ;
changeDomain: 'change-domain' SP text ;
changeDescription: 'change-description' SP text ;
connectKindCompatibility: 'connect-kind-compatibility' SP id SP id SP boolean SP boolean SP specificity ;
disconnectKindCompatibility: 'disconnect-kind-compatibility' SP id SP id ;
replaceKindCatalogs: 'replace-kind-catalogs' SP catalogsBlockOpt ;
partBlock: '{' NL OCTET+ '}' ;
gripBlock: '{' NL OCTET+ '}' ;
catalogsBlockOpt: ('{' NL OCTET+ '}') | 'none' ;
fastenerFields: textOpt SP number SP number SP number SP number SP number SP number SP number SP number ;
fullGripId: OCTET+ ;
anchor: 'fixed' | 'derived' ;
specificity: 'general' | 'part' | 'fastener' | 'grip' | 'rope' ;
quatOpt: (number SP number SP number SP number) | 'none' ;
scaleOpt: number+ | 'none' ;
indexOpt: number | 'none' ;
textOpt: text | 'none' ;
numberOpt: number | 'none' ;
booleanOpt: boolean | 'none' ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;
boolean: 'true' | 'false' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
