// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start mutation`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Draw_draw_mutations;

DOCUMENT: 'schema' [ ]+ 'draw.draw.mutations' ;

mutation: setLayerVisible | setLayerLocked | setLayerOpacity | setLayerBlendMode | renameLayer | updateLayerTransform | replaceLayerFill | replaceLayerStroke | setLayerBooleanOperation | updateLayerTraceParams | createLayer | duplicateLayer | deleteLayer | reorderLayer ;
setLayerVisible: 'set-layer-visible' SP id SP bool ;
setLayerLocked: 'set-layer-locked' SP id SP bool ;
setLayerOpacity: 'set-layer-opacity' SP id SP number ;
setLayerBlendMode: 'set-layer-blend-mode' SP id SP text ;
renameLayer: 'rename-layer' SP id SP text ;
updateLayerTransform: 'update-layer-transform' SP id SP transformBlock ;
replaceLayerFill: 'replace-layer-fill' SP id SP fillBlock ;
replaceLayerStroke: 'replace-layer-stroke' SP id SP strokeBlock? ;
setLayerBooleanOperation: 'set-layer-boolean-operation' SP id SP text ;
updateLayerTraceParams: 'update-layer-trace-params' SP id SP traceParamsBlock ;
createLayer: 'create-layer' SP optId SP optNumber SP layer ;
duplicateLayer: 'duplicate-layer' SP id ;
deleteLayer: 'delete-layer' SP id ;
reorderLayer: 'reorder-layer' SP id SP optId SP number ;
transformBlock: 'transform' '{' 'x=' number SP 'y=' number SP 'scale-x=' number SP 'scale-y=' number SP 'rotation=' number 'rad' '}' ;
traceParamsBlock: 'params' '{' 'threshold=' number SP 'simplify-epsilon=' number '}' ;
strokeBlock: 'stroke' '{' 'color=' number4 SP 'width=' number SP 'cap=' text SP 'join=' text optDash '}' ;
optDash: (SP 'dash=' numberList)? ;
fillBlock: 'fill' '{' fill? '}' ;
fill: solidFill | linearGradientFill | radialGradientFill ;
solidFill: 'solid' '{' 'color=' number4 '}' ;
linearGradientFill: 'linearGradient' '{' 'x1=' number SP 'y1=' number SP 'x2=' number SP 'y2=' number SP 'stops' stopsTable '}' ;
radialGradientFill: 'radialGradient' '{' 'cx=' number SP 'cy=' number SP 'r=' number SP 'stops' stopsTable '}' ;
stopsTable: '[' 'offset:NUM' SP 'color:NUM4' ']' '{' stopRow* '}' ;
stopRow: number SP number4 ;
layer: shapeLayer | pathLayer | textLayer | imageLayer | groupLayer | booleanLayer | traceLayer ;
shapeLayer: 'shape' layerBase 'shape-kind=' text ;
pathLayer: 'path' layerBase 'segments' '{' pathSegment* '}' ;
textLayer: 'text' layerBase 'x=' number SP 'y=' number SP 'content=' text SP 'size=' number ;
imageLayer: 'image' layerBase 'image-key=' text SP 'width=' number SP 'height=' number ;
groupLayer: 'group' layerBase 'children' '{' layer* '}' ;
booleanLayer: 'boolean' layerBase 'operation=' text SP 'children=' stringList ;
traceLayer: 'trace' layerBase 'source-key=' text traceParamsBlock ;
layerBase: 'base' '{' 'id=' text SP 'name=' text SP 'visible=' bool SP 'locked=' bool SP 'opacity=' number SP 'blend-mode=' text transformBlock '}' ;
pathSegment: moveSegment | lineSegment | quadSegment | cubicSegment | arcSegment | closeSegment ;
moveSegment: 'M' SP point ;
lineSegment: 'L' SP point ;
quadSegment: 'Q' SP point SP point ;
cubicSegment: 'C' SP point SP point SP point ;
arcSegment: 'A' SP number SP number SP number SP bool SP bool SP point ;
closeSegment: 'Z' ;
point: number ',' number ;
number4: number ',' number ',' number ',' number ;
numberList: '[' number* ']' ;
stringList: '[' text* ']' ;
id: text ;
optId: '_' | id ;
optNumber: '_' | number ;
text: bareText | quotedText ;
bareText: OCTET+ ;
quotedText: DQUOTE OCTET* DQUOTE ;
number: INT | FLOAT ;
bool: 'true' | 'false' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
INT: '-'? [0-9]+ ;
FLOAT: '-'? [0-9]+ '.' [0-9]+ ;
SP: ' ' ;
OCTET: . ;
DQUOTE: '"' ;
