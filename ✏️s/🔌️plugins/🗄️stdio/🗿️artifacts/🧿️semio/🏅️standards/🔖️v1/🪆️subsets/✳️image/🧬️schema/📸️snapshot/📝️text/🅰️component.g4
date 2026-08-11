// 🅰️ Real wire grammar for `s.stdio.semio.image`'s text (`ArtifactDsl`) form: a mandatory one-line
// preamble (`store::semio_format::wrap_text`/`split_text_preamble`) followed by the snapshot's own
// serde_json bytes, lowercase-hex encoded. Honest boundary: this subset is a NEUTRAL semio type
// (not an on-disk file format), so its JSON shape IS the real payload — see the sibling JSON Schema
// leaf (`🔣️component.json`, one level up) for that shape.
grammar Semio_image_snapshot_text;

document : preamble NEWLINE hexBody NEWLINE? EOF ;
preamble : 'semio' ' ' 's.stdio.semio.image.dsl' ' ' 'v' DIGIT+ ;
hexBody  : HEXDIGIT* ;

HEXDIGIT : [0-9a-f] ;
DIGIT    : [0-9] ;
NEWLINE  : '\r'? '\n' ;
