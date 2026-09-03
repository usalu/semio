// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Puzzle_puzzle2d_mutations;

DOCUMENT: 'schema' [ ]+ 'puzzle.puzzle2d.mutations' ;

line: createNode | deleteNode | moveNode | replaceNodeGeometry | changeNodeKind | editNodeText | changeNodeIcon | scaleNode | changeNodeVisible | changeNodeLocked | changeNodeRoot | changeNodeAnchor | addNodeHandle | removeNodeHandle | replaceNodeHandle | connectHandles | disconnectHandles | replaceEdgeGeometry | changeEdgeKind | changeEdgeTips | changeEdgeVisible | changeEdgeLocked | changeManifestId | connectKindCompatibility | disconnectKindCompatibility | replaceKindCatalogs ;
createNode: 'create-node' SP nodeBlock SP indexOpt ;
deleteNode: 'delete-node' SP id ;
moveNode: 'move-node' SP id SP number SP number ;
replaceNodeGeometry: 'replace-node-geometry' SP id SP textOpt SP numberOpt SP numberOpt SP numberOpt ;
changeNodeKind: 'change-node-kind' SP id SP textOpt ;
editNodeText: 'edit-node-text' SP id SP textOpt ;
changeNodeIcon: 'change-node-icon' SP id SP textOpt ;
scaleNode: 'scale-node' SP id SP numberOpt ;
changeNodeVisible: 'change-node-visible' SP id SP booleanOpt ;
changeNodeLocked: 'change-node-locked' SP id SP booleanOpt ;
changeNodeRoot: 'change-node-root' SP id SP booleanOpt ;
changeNodeAnchor: 'change-node-anchor' SP id SP anchor ;
addNodeHandle: 'add-node-handle' SP id SP handleBlock SP indexOpt ;
removeNodeHandle: 'remove-node-handle' SP id SP id ;
replaceNodeHandle: 'replace-node-handle' SP id SP id SP handleBlock ;
connectHandles: 'connect-handles' SP id SP id SP id SP edgeFields ;
disconnectHandles: 'disconnect-handles' SP id ;
replaceEdgeGeometry: 'replace-edge-geometry' SP id SP number SP number SP number SP number SP number SP number SP number SP number ;
changeEdgeKind: 'change-edge-kind' SP id SP textOpt ;
changeEdgeTips: 'change-edge-tips' SP id SP textOpt SP textOpt ;
changeEdgeVisible: 'change-edge-visible' SP id SP booleanOpt ;
changeEdgeLocked: 'change-edge-locked' SP id SP booleanOpt ;
changeManifestId: 'change-manifest-id' SP textOpt ;
connectKindCompatibility: 'connect-kind-compatibility' SP id SP id SP boolean SP boolean SP specificity ;
disconnectKindCompatibility: 'disconnect-kind-compatibility' SP id SP id ;
replaceKindCatalogs: 'replace-kind-catalogs' SP catalogsBlockOpt ;
nodeBlock: '{' NL OCTET+ '}' ;
handleBlock: '{' NL OCTET+ '}' ;
catalogsBlockOpt: ('{' NL OCTET+ '}') | 'none' ;
edgeFields: textOpt SP number SP number SP number SP number SP number SP number SP number SP number SP textOpt SP textOpt ;
anchor: 'fixed' | 'derived' ;
specificity: 'general' | 'node' | 'edge' | 'handle' | 'wire' | 'vortex' ;
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
