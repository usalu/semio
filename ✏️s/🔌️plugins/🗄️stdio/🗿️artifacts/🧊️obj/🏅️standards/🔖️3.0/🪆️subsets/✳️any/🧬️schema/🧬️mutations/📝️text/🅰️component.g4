// 🅰️ `ObjMutation`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, tagged on the "mutation" field). Names the real variant tags
// rather than a placeholder.
grammar Stdio_obj_mutations;

mutation  : '{' '"mutation"' ':' tag (',' member)* '}' ;
tag       : '"noMutation"' | '"setSnapshot"'
          | '"insertVertex"' | '"removeVertex"' | '"setVertex"'
          | '"insertTexCoord"' | '"removeTexCoord"' | '"setTexCoord"'
          | '"insertNormal"' | '"removeNormal"' | '"setNormal"'
          | '"insertFace"' | '"removeFace"' | '"setFace"'
          | '"setGroup"' | '"removeGroup"' | '"setObject"' | '"removeObject"'
          | '"setMtllib"' | '"setUsemtl"' | '"setSmoothingGroups"' | '"setUnknownStatements"' ;
member    : STRING ':' value ;
STRING    : '"' .*? '"' ;
