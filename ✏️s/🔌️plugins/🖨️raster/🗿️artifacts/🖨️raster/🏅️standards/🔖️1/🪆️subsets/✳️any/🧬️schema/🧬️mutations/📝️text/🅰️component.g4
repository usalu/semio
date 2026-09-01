// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start mutation`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Raster_raster_mutations;

DOCUMENT: 'schema' [ ]+ 'raster.raster.mutations' ;

mutation: createLayer | deleteLayer | reorderLayers | renameLayer | changeLayerVisible | changeLayerOpacity | changeLayerBlendMode | moveLayer | resizeLayer | changeLayerAdjustmentKind | addLayerAsset | removeLayerAsset ;
createLayer: 'create-layer' SP index SP layer SP ('parent=' id)? ;
deleteLayer: 'delete-layer' SP 'id=' id ;
reorderLayers: 'reorder-layers' SP 'id=' id SP index SP ('parent=' id)? ;
renameLayer: 'rename-layer' SP 'id=' id SP text ;
changeLayerVisible: 'change-layer-visible' SP 'id=' id SP bool ;
changeLayerOpacity: 'change-layer-opacity' SP 'id=' id SP number ;
changeLayerBlendMode: 'change-layer-blend-mode' SP 'id=' id SP text ;
moveLayer: 'move-layer' SP 'id=' id SP number SP number ;
resizeLayer: 'resize-layer' SP 'id=' id SP number SP number ;
changeLayerAdjustmentKind: 'change-layer-adjustment-kind' SP 'id=' id SP text ;
addLayerAsset: 'add-layer-asset' SP 'id=' id SP asset ;
removeLayerAsset: 'remove-layer-asset' SP 'id=' id ;
layer: pixelLayer | groupLayer | adjustmentLayer ;
pixelLayer: 'pixel' SP id SP text SP bool SP number SP transformBlock SP maskBlock? SP number? SP number? SP ('blend=' text)? SP ('image=' id)? ;
groupLayer: 'group' SP id SP text SP bool SP number SP transformBlock SP maskBlock? SP layer* SP ('blend=' text)? ;
adjustmentLayer: 'adjustment' SP id SP text SP bool SP number SP transformBlock SP ('blend=' text)? SP ('kind=' text)? SP param* ;
transformBlock: 'transform' '{' 'x=' number SP 'y=' number SP 'scale-x=' number SP 'scale-y=' number SP 'rotation=' number 'deg' '}' ;
maskBlock: 'mask' '{' 'enabled=' bool SP 'linked=' bool SP 'invert=' bool SP 'width=' number? SP 'height=' number? '}' ;
param: IDENT '=' TEXT ;
asset: '{' 'mime=' text 'data=' TEXT '}' ;
id: TEXT ;
text: TEXT ;
number: INT | FLOAT ;
bool: 'true' | 'false' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
IDENT: [A-Za-z_] [A-Za-z0-9_]* ;
TEXT: [^ \t\r\n]+ ;
INT: '-'? [0-9]+ ;
FLOAT: '-'? [0-9]+ '.' [0-9]+ ;
SP: ' ' ;
