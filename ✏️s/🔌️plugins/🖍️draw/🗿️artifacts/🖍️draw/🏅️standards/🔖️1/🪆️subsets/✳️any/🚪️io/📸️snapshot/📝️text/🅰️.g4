// 🅰️ ANTLR4 mirror of the normative 📖️.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start document`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Draw_draw_snapshot;

DOCUMENT: 'schema' [ ]+ 'draw.draw.snapshot' ;

document: 'schema=' text SP 'id=' text optTitle NL layersBlock optAssets optArtboard ;
optTitle: (SP 'title=' text)? ;
optAssets: ('assets' '=' '{' NL assetEntry* '}' NL)? ;
optArtboard: ('artboard' '{' NL 'width=' number SP 'height=' number NL '}' NL)? ;
assetEntry: IDENT '=' 'mime=' text SP 'data=' text optAssetWidth optAssetHeight ;
optAssetWidth: (SP 'width=' number)? ;
optAssetHeight: (SP 'height=' number)? ;
layersBlock: 'layers' '{' layer* '}' ;
layer: shapeLayer | pathLayer | textLayer | imageLayer | groupLayer | booleanLayer | traceLayer ;
shapeLayer: 'shape' layerBase 'shape-kind=' text optRect optEllipse optCircle optLine optPolygon ;
pathLayer: 'path' layerBase 'segments' '{' pathSegment* '}' ;
textLayer: 'text' layerBase 'x=' number SP 'y=' number SP 'content=' text SP 'size=' number ;
imageLayer: 'image' layerBase 'image-key=' text SP 'width=' number SP 'height=' number ;
groupLayer: 'group' layerBase 'children' '{' layer* '}' ;
booleanLayer: 'boolean' layerBase 'operation=' text SP 'children=' stringList ;
traceLayer: 'trace' layerBase 'source-key=' text traceParamsBlock ;
traceParamsBlock: 'params' '{' 'threshold=' number SP 'simplify-epsilon=' number '}' ;
layerBase: 'base' '{' 'id=' text SP 'name=' text SP 'visible=' bool SP 'locked=' bool SP 'opacity=' number SP 'blend-mode=' text transformBlock optAttributes '}' ;
transformBlock: 'transform' '{' 'x=' number SP 'y=' number SP 'scale-x=' number SP 'scale-y=' number SP 'rotation=' number 'rad' '}' ;
optAttributes: ('attributes' '{' fillBlock optStroke '}')? ;
fillBlock: 'fill' '{' fill? '}' ;
optStroke: ('stroke' '{' 'color=' number4 SP 'width=' number SP 'cap=' text SP 'join=' text optDash '}')? ;
optDash: (SP 'dash=' numberList)? ;
fill: solidFill | linearGradientFill | radialGradientFill ;
solidFill: 'solid' '{' 'color=' number4 '}' ;
linearGradientFill: 'linearGradient' '{' 'x1=' number SP 'y1=' number SP 'x2=' number SP 'y2=' number SP 'stops' stopsTable '}' ;
radialGradientFill: 'radialGradient' '{' 'cx=' number SP 'cy=' number SP 'r=' number SP 'stops' stopsTable '}' ;
stopsTable: '[' 'offset:NUM' SP 'color:NUM4' ']' '{' stopRow* '}' ;
stopRow: number SP number4 ;
optRect: ('rect' '{' 'x=' number SP 'y=' number SP 'width=' number SP 'height=' number '}')? ;
optEllipse: ('ellipse' '{' 'cx=' number SP 'cy=' number SP 'rx=' number SP 'ry=' number '}')? ;
optCircle: ('circle' '{' 'cx=' number SP 'cy=' number SP 'r=' number '}')? ;
optLine: ('line' '{' 'x1=' number SP 'y1=' number SP 'x2=' number SP 'y2=' number '}')? ;
optPolygon: ('polygon' '{' 'points=' pointList '}')? ;
pathSegment: moveSegment | lineSegment | quadSegment | cubicSegment | arcSegment | closeSegment ;
moveSegment: 'M' SP point ;
lineSegment: 'L' SP point ;
quadSegment: 'Q' SP point SP point ;
cubicSegment: 'C' SP point SP point SP point ;
arcSegment: 'A' SP number SP number SP number SP bool SP bool SP point ;
closeSegment: 'Z' ;
point: number ',' number ;
number4: number ',' number ',' number ',' number ;
pointList: '[' point* ']' ;
numberList: '[' number* ']' ;
stringList: '[' text* ']' ;
text: bareText | quotedText ;
bareText: OCTET+ ;
quotedText: DQUOTE OCTET* DQUOTE ;
number: INT | FLOAT ;
bool: 'true' | 'false' ;

// 📐 Framework dialect-primitive terminals (not defined in the .semio itself — see
// the ticket report for this deviation, same treatment as the repo's cad-mutations pair).
IDENT: [A-Za-z_] [A-Za-z0-9_]* ;
INT: '-'? [0-9]+ ;
FLOAT: '-'? [0-9]+ '.' [0-9]+ ;
SP: ' ' ;
NL: '\r'? '\n' ;
OCTET: . ;
DQUOTE: '"' ;
