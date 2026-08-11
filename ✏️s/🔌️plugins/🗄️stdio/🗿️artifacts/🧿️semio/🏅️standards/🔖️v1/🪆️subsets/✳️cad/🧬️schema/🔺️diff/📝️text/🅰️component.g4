// 🅰️ ANTLR grammar for `s.stdio.semio.cad.diff`'s `protocol::DiffCodec::print_diff`/`parse_diff`
// wire grammar -- NO semio envelope (unlike the snapshot facet): `encode_diff` is simply
// `print_diff().into_bytes()`, so this grammar IS the binary grammar too (see
// ../💾️binary/🌶️component.spicy, byte-identical structure).
grammar Stdio_semio_cad_diff;

document : (token (' ' token)*)? EOF ;
token : 'layers=' layersTriple | 'blocks=' blocksTriple | 'entities=' entitiesTriple ;

layersTriple : '[' removedKeys ']' ';' '[' layersModified ']' ';' '[' layersAdded ']' ;
layersModified : (layerMod (',' layerMod)*)? ;
layerMod : hexstr ':' layerDiff ;
layersAdded : (layer (',' layer)*)? ;

blocksTriple : '[' removedKeys ']' ';' '[' blocksModified ']' ';' '[' blocksAdded ']' ;
blocksModified : (blockMod (',' blockMod)*)? ;
blockMod : hexstr ':' blockDiff ;
blocksAdded : (block (',' block)*)? ;

entitiesTriple : '[' removedKeys ']' ';' '[' entitiesModified ']' ';' '[' entitiesAdded ']' ;
entitiesModified : (entityRecordMod (',' entityRecordMod)*)? ;
entityRecordMod : hexstr ':' entityRecordDiff ;
entitiesAdded : (entityRecord (',' entityRecord)*)? ;

removedKeys : (hexstr (',' hexstr)*)? ;

optionI32 : '[0]' | '[1,' i32 ']' ;
optionStr : '[0]' | '[1,' hexstr ']' ;
optionBool : '[0]' | '[1,' bool01 ']' ;
optionPoint2 : '[0]' | '[1,' point2 ']' ;
optionEntity : '[0]' | '[1,' entity ']' ;
optionEntitiesTriple : '[0]' | '[1,' entitiesTriple ']' ;

layerDiff : '[' optionI32 ',' optionStr ',' optionBool ']' ;
entityRecordDiff : '[' optionStr ',' optionEntity ']' ;
blockDiff : '[' optionPoint2 ',' optionEntitiesTriple ']' ;

layer : '[' hexstr ',' i32 ',' hexstr ',' bool01 ']' ;
entityRecord : '[' hexstr ',' hexstr ',' entity ']' ;
block : '[' hexstr ',' point2 ',' '[' (entityRecord (',' entityRecord)*)? ']' ']' ;

entity : line | arc | circle | ellipse | polyline | textEntity | insert | solid | dimension ;
line : 'L[' point2 ',' point2 ']' ;
arc : 'A[' point2 ',' f64 ',' f64 ',' f64 ']' ;
circle : 'C[' point2 ',' f64 ']' ;
ellipse : 'E[' point2 ',' point2 ',' f64 ',' f64 ',' f64 ']' ;
polyline : 'P[' '[' (point2 (',' point2)*)? ']' ',' bool01 ']' ;
textEntity : 'T[' point2 ',' f64 ',' f64 ',' hexstr ']' ;
insert : 'I[' hexstr ',' point2 ',' point2 ',' f64 ']' ;
solid : 'S[' point2 ',' point2 ',' point2 ',' point2 ']' ;
dimension : 'D[' point2 ',' point2 ',' f64 ',' hexstr ']' ;

point2 : '[' f64 ',' f64 ']' ;
hexstr : HEXDIGIT* ;
i32 : '-'? DIGIT+ ;
f64 : '-'? DIGIT+ ('.' DIGIT+)? (('e'|'E') ('+'|'-')? DIGIT+)? ;
bool01 : '0' | '1' ;

HEXDIGIT : [0-9a-fA-F] ;
DIGIT : [0-9] ;
