// 🅰️ Real wire grammar for `SemioImageMutation`'s hand-rolled `OpText::print_op`/`parse_op` — see
// the `🦀️component.rs` sibling's `🔖️OpCodecs` region. `NoMutation` prints as the bare literal
// `no`; every other variant prints as `tag:` followed by comma-separated positional fields
// (bracket-depth-aware, reusing `engine::triples`' split/strip helpers so `setSnapshot`'s nested
// `[...]` snapshot payload never confuses the top-level split).
grammar Semio_image_mutation_text;

op   : 'no'
     | 'setSnapshot:' snapshot
     | 'setDimensions:' DIGIT+ ',' DIGIT+
     | 'setColorspace:' ('r'|'a'|'g'|'y'|'i')
     | 'setBitDepth:' DIGIT+
     | 'setIcc:' option
     | 'insertFrame:' DIGIT+ ',' frame
     | 'removeFrame:' DIGIT+
     | 'moveFrame:' DIGIT+ ',' DIGIT+
     | 'setFrameDelay:' DIGIT+ ',' DIGIT+
     | 'setFramePixels:' DIGIT+ ',' HEXDIGIT*
     | 'setMetadataEntry:' HEXDIGIT* ',' HEXDIGIT*
     | 'removeMetadataEntry:' HEXDIGIT*
     ;
snapshot : '[' DIGIT+ ',' DIGIT+ ',' ('r'|'a'|'g'|'y'|'i') ',' DIGIT+ ',' option ',' '[' frameList ']' ',' '[' entryList ']' ']' ;
frame    : '[' DIGIT+ ',' HEXDIGIT* ']' ;
frameList: (frame (',' frame)*)? ;
entryList: (entry (',' entry)*)? ;
entry    : HEXDIGIT* ',' HEXDIGIT* ;
option   : '[0]' | '[1,' HEXDIGIT* ']' ;

HEXDIGIT : [0-9a-f] ;
DIGIT    : [0-9] ;
