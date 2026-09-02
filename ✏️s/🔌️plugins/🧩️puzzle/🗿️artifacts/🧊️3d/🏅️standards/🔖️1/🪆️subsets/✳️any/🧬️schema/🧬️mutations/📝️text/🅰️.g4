// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start line`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Puzzle_puzzle3d_mutations;

DOCUMENT: 'schema' [ ]+ 'puzzle.puzzle3d.mutations' ;

line: createObject | deleteObject | moveObject | rotateObject | scaleObject | changeObjectMesh | editObjectLabel | changeObjectKind | changeObjectAnchor | changeObjectHidden | changeObjectLocked | addObjectVortex | removeObjectVortex | replaceObjectVortex | connectVortices | disconnectVortices | replaceAttractionGeometry | createTargetVolume | deleteTargetVolume | moveTargetVolume | rotateTargetVolume | scaleTargetVolume | changeTargetVolumeHidden | changeTargetVolumeLocked | createReference | deleteReference | moveReference | resizeReference | replaceReferenceSource | changeReferenceHidden | changeReferenceLocked | changeDomain | connectKindCompatibility | disconnectKindCompatibility | replaceKindCatalogs ;
createObject: 'create-object' SP objectBlock SP indexOpt ;
deleteObject: 'delete-object' SP id ;
moveObject: 'move-object' SP id SP number SP number SP number ;
rotateObject: 'rotate-object' SP id SP quatOpt ;
scaleObject: 'scale-object' SP id SP scaleOpt ;
changeObjectMesh: 'change-object-mesh' SP id SP textOpt ;
editObjectLabel: 'edit-object-label' SP id SP textOpt ;
changeObjectKind: 'change-object-kind' SP id SP textOpt ;
changeObjectAnchor: 'change-object-anchor' SP id SP anchor ;
changeObjectHidden: 'change-object-hidden' SP id SP boolean ;
changeObjectLocked: 'change-object-locked' SP id SP boolean ;
addObjectVortex: 'add-object-vortex' SP id SP vortexBlock SP indexOpt ;
removeObjectVortex: 'remove-object-vortex' SP id SP id ;
replaceObjectVortex: 'replace-object-vortex' SP id SP id SP vortexBlock ;
connectVortices: 'connect-vortices' SP id SP fullVortexId SP fullVortexId SP attractionFields ;
disconnectVortices: 'disconnect-vortices' SP id ;
replaceAttractionGeometry: 'replace-attraction-geometry' SP id SP number SP number SP number SP number SP number SP number SP number SP number ;
createTargetVolume: 'create-target-volume' SP volumeBlock SP indexOpt ;
deleteTargetVolume: 'delete-target-volume' SP id ;
moveTargetVolume: 'move-target-volume' SP id SP number SP number SP number ;
rotateTargetVolume: 'rotate-target-volume' SP id SP quatOpt ;
scaleTargetVolume: 'scale-target-volume' SP id SP scaleOpt ;
changeTargetVolumeHidden: 'change-target-volume-hidden' SP id SP boolean ;
changeTargetVolumeLocked: 'change-target-volume-locked' SP id SP boolean ;
createReference: 'create-reference' SP referenceBlock SP indexOpt ;
deleteReference: 'delete-reference' SP id ;
moveReference: 'move-reference' SP id SP number SP number SP number ;
resizeReference: 'resize-reference' SP id SP number ;
replaceReferenceSource: 'replace-reference-source' SP id SP sourceBlock ;
changeReferenceHidden: 'change-reference-hidden' SP id SP boolean ;
changeReferenceLocked: 'change-reference-locked' SP id SP boolean ;
changeDomain: 'change-domain' SP text ;
connectKindCompatibility: 'connect-kind-compatibility' SP id SP id SP boolean SP boolean SP specificity ;
disconnectKindCompatibility: 'disconnect-kind-compatibility' SP id SP id ;
replaceKindCatalogs: 'replace-kind-catalogs' SP catalogsBlockOpt ;
objectBlock: '{' NL OCTET+ '}' ;
vortexBlock: '{' NL OCTET+ '}' ;
volumeBlock: '{' NL OCTET+ '}' ;
referenceBlock: '{' NL OCTET+ '}' ;
sourceBlock: '{' NL OCTET+ '}' ;
catalogsBlockOpt: ('{' NL OCTET+ '}') | 'none' ;
attractionFields: number SP number SP number SP number SP number SP number SP number SP number ;
fullVortexId: OCTET+ ;
anchor: 'fixed' | 'derived' ;
specificity: 'general' | 'object' | 'attraction' | 'cable' | 'vortex' ;
quatOpt: (number SP number SP number SP number) | 'none' ;
scaleOpt: number+ | 'none' ;
indexOpt: number | 'none' ;
textOpt: text | 'none' ;
id: OCTET+ ;
number: OCTET+ ;
text: OCTET+ ;
boolean: 'true' | 'false' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
NL: '\r'? '\n' ;
OCTET: . ;
SP: ' ' ;
