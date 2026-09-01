// 🅰️ `DxfMutation`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, tagged on "mutation" per 🦀️component.rs's enum).
grammar Stdio_dxf_mutations;

mutation : '{' TAGKEY ':' tagValue (',' member)* '}' ;
tagValue : SETSNAPSHOT
         | SETHEADERVAR | REMOVEHEADERVAR
         | INSERTLAYER | REMOVELAYER | SETLAYER
         | INSERTSTYLE | REMOVESTYLE | SETSTYLE
         | INSERTLINETYPE | REMOVELINETYPE | SETLINETYPE
         | INSERTENTITY | REMOVEENTITY | SETENTITY
         | INSERTBLOCK | REMOVEBLOCK | SETBLOCK ;
member   : STRING ':' jsonValue ;

TAGKEY         : '"mutation"' ;
SETSNAPSHOT    : '"setSnapshot"' ;
SETHEADERVAR   : '"setHeaderVar"' ;
REMOVEHEADERVAR: '"removeHeaderVar"' ;
INSERTLAYER    : '"insertLayer"' ;
REMOVELAYER    : '"removeLayer"' ;
SETLAYER       : '"setLayer"' ;
INSERTSTYLE    : '"insertStyle"' ;
REMOVESTYLE    : '"removeStyle"' ;
SETSTYLE       : '"setStyle"' ;
INSERTLINETYPE : '"insertLinetype"' ;
REMOVELINETYPE : '"removeLinetype"' ;
SETLINETYPE    : '"setLinetype"' ;
INSERTENTITY   : '"insertEntity"' ;
REMOVEENTITY   : '"removeEntity"' ;
SETENTITY      : '"setEntity"' ;
INSERTBLOCK    : '"insertBlock"' ;
REMOVEBLOCK    : '"removeBlock"' ;
SETBLOCK       : '"setBlock"' ;
STRING         : '"' .*? '"' ;
