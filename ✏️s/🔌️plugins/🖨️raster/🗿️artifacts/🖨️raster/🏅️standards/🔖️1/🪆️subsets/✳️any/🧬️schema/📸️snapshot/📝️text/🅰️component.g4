// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start document`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Raster_raster_snapshot;

DOCUMENT: 'schema' [ ]+ 'raster.raster.snapshot' ;

artifactMark: 'semio raster.raster.dsl v1' ;
document: artifactMark schemaLine idLine titleLine layersLine assetsLine ;
schemaLine: 'schema' '=' HEX ;
idLine: 'id' '=' HEX ;
titleLine: 'title' '=' optHex ;
layersLine: 'layers' '=' '[' layerItems? ']' ;
layerItems: layer (',' layer)* ;
assetsLine: 'assets' '=' '[' assetItems? ']' ;
assetItems: assetEntry (',' assetEntry)* ;
optHex: '[0]' | '[1,' HEX ']' ;
optU32: '[0]' | '[1,' number ']' ;
layer: pixelLayer | groupLayer | adjustmentLayer ;
pixelLayer: 'p' '[' HEX ',' HEX ',' bool ',' number ',' HEX ',' transform ',' optMask ',' optU32 ',' optU32 ',' optHex ']' ;
groupLayer: 'g' '[' HEX ',' HEX ',' bool ',' number ',' HEX ',' transform ',' optMask ',' '[' layerItems? ']' ']' ;
adjustmentLayer: 'a' '[' HEX ',' HEX ',' bool ',' number ',' HEX ',' transform ',' HEX ',' '[' paramItems? ']' ']' ;
paramItems: paramEntry (',' paramEntry)* ;
paramEntry: '[' HEX ',' HEX ']' ;
transform: '[' number ',' number ',' number ',' number ',' number ']' ;
optMask: '[0]' | '[1,' mask ']' ;
mask: '[' bool ',' bool ',' bool ',' optU32 ',' optU32 ']' ;
assetEntry: '[' HEX ',' childHandle ']' ;
childHandle: '[' HEX ',' HEX ']' ;
bool: 'true' | 'false' ;
number: INT | FLOAT ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
HEX: [0-9a-f]* ;
INT: '-'? [0-9]+ ;
FLOAT: '-'? [0-9]+ '.' [0-9]+ ;
