// 🅰️ ANTLR grammar for `s.stdio.semio.cad.mutations`'s `protocol::OpText::print_op`/`parse_op`
// wire grammar -- NO semio envelope; `OpBinary` is `print_op().into_bytes()` (see
// ../💾️binary/🌶️component.spicy, byte-identical). Grammar: `keyword arg=value ...`.
grammar Stdio_semio_cad_mutations;

document : 'no-mutation' | keyword ' ' args ;
args : arg (' ' arg)* ;
arg : KEY '=' HEXOREXPR ;

keyword : 'set-snapshot' | 'add-layer' | 'remove-layer' | 'set-layer' | 'add-block' | 'remove-block'
        | 'set-block-base-point' | 'add-entity' | 'remove-entity' | 'set-entity-layer'
        | 'set-entity-geometry' | 'add-block-entity' | 'remove-block-entity'
        | 'set-block-entity-layer' | 'set-block-entity-geometry' ;

// value shapes reuse the diff facet's entity/layer/block/point2/option grammar verbatim (see
// ../../🔺️diff/📝️text/🅰️component.g4) -- each keyword's real arg list is documented in
// ../📖️component.grammar.semio (informative) and authoritatively in 🦀️component.rs's
// `print_cad_mutation`.
KEY : [a-z] [a-z-]* ;
HEXOREXPR : ~[ ]+ ;

