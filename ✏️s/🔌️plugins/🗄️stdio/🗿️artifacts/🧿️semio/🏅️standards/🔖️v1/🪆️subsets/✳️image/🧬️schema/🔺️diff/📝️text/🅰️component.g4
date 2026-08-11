// 🅰️ Real wire grammar for `SemioImageDiff`'s hand-rolled `DiffCodec::print_diff`/`parse_diff` —
// see the `🦀️component.rs` sibling's `🔖️HandcraftedDiffCodec` region. One space-separated
// `name=value` token per changed top-level field; `frames`/`metadata` print as
// `name{[removed];[modified];[added]}` via the shared `engine::triples`
// `enc_indexed_triple`/`enc_named_triple` codec.
grammar Semio_image_diff_text;

line       : token (' ' token)* EOF | EOF ;
token      : scalarToken | collectionToken ;
scalarToken: ('width' | 'height' | 'bitDepth') '=' DIGIT+
           | 'colorspace' '=' ('r'|'a'|'g'|'y'|'i')
           | 'icc' '=' option ;
option     : '[0]' | '[1,' HEXDIGIT* ']' ;
collectionToken : ('frames' | 'metadata') '{' triple '}' ;
triple     : '[' items ']' ';' '[' items ']' ';' '[' items ']' ;
items      : (item (',' item)*)? ;
item       : DIGIT+ ':' HEXDIGIT* ;

HEXDIGIT : [0-9a-f] ;
DIGIT    : [0-9] ;
