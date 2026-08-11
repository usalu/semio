grammar SemioDocumentSnapshot;
// DSL text form: a one-line `store::semio_format` preamble naming the envelope id, a newline,
// then the snapshot's JSON body re-encoded as lowercase hex (2 hex digits per byte).
document : preamble NEWLINE hexBody EOF ;
preamble : 'stdio.semio.document.dsl.v1' ;
hexBody  : HEXPAIR* ;
HEXPAIR  : [0-9a-f] [0-9a-f] ;
NEWLINE  : '\n' ;
