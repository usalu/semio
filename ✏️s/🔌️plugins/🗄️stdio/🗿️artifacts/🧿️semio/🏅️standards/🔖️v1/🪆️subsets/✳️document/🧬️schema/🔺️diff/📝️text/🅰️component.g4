grammar SemioDocumentDiff;
// Space-separated `key=value` tokens; each value is `[removed];[modified];[added]` (bracket-
// delimited, comma-separated within each section, per `engine::triples::enc_indexed_triple`/
// `enc_named_triple`'s real hex-value shape).
document : token (' ' token)* EOF | /* empty */ ;
token    : STYLES_TOKEN | IMAGES_TOKEN | BLOCKS_TOKEN ;
STYLES_TOKEN : 'styles=' TRIPLE ;
IMAGES_TOKEN : 'images=' TRIPLE ;
BLOCKS_TOKEN : 'blocks=' TRIPLE ;
TRIPLE   : '[' .*? '];[' .*? '];[' .*? ']' ;
