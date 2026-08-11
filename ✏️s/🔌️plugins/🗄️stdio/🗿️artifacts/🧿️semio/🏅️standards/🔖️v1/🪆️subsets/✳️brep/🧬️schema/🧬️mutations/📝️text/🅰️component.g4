// 🅰️ ANTLR grammar for `stdio.semio.brep`'s hand-rolled `SemioBrepMutation` op text form
// (protocol::OpText::print_op/parse_op). One line of compact RFC8259 JSON, tag field `mutation`
// discriminating the 23 variants (see ../🔣️component.json for the variant/field enumeration).
grammar Stdio_semio_brep_mutations;

opLine : jsonValue EOF ;
jsonValue : jsonObject ;
jsonObject : '{' (jsonMember (',' jsonMember)*)? '}' ;
jsonMember : STRING ':' jsonValue ;

STRING : '"' (~["\\] | '\\' .)* '"' ;
