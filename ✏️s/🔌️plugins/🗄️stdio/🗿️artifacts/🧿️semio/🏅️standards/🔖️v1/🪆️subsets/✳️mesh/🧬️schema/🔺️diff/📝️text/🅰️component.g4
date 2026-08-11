// 🅰️ ANTLR grammar for `s.stdio.semio.mesh`'s hand-rolled `DiffCodec` text representation
// (protocol::DiffCodec::print_diff/parse_diff, see ../🦀️component.rs). REAL grammar (not a
// hex-JSON envelope like the snapshot facet): space-separated `key=value` tokens, one per
// changed top-level field; each value is a bracket-depth-aware `[removed];[modified];[added]`
// named-triple (engine::triples::enc_named_triple/dec_named_triple).
grammar Stdio_semio_mesh_diff;

document   : (token (SP token)*)? EOF ;
token      : 'meshes=' triple | 'materials=' triple | 'textures=' triple ;
triple     : '[' list ']' ';' '[' list ']' ';' '[' list ']' ;
list       : (item (',' item)*)? ;
item       : HEXSTR (':' payload)? ;
payload    : ~[,;]* ;   // nested `[...]` diff/item encodings, opaque at this grammar level

HEXSTR : [0-9a-fA-F]* ;
SP     : ' ' ;
