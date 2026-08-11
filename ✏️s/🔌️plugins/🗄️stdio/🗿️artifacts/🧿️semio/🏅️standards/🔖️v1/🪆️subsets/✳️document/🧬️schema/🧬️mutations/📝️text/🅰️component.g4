grammar SemioDocumentMutation;
// `keyword arg=value ...` (space-separated), same shape docx/svg/gif's hand-rolled ops use, or
// the literal `no-mutation` token (see `print_document_mutation`/`parse_document_mutation` in
// the sibling `🦀️component.rs` for the real hand-rolled grammar and full keyword list).
document : NO_MUTATION EOF | keyword (' ' arg)* EOF ;
NO_MUTATION : 'no-mutation' ;
keyword  : 'set-snapshot' | 'insert-block' | 'remove-block' | 'set-block-content'
         | 'set-paragraph-style' | 'set-heading-level' | 'set-list-ordered' | 'set-run-text'
         | 'set-run-style' | 'set-image-block' | 'insert-style' | 'remove-style'
         | 'set-style-name' | 'set-style-based-on' | 'insert-image' | 'remove-image'
         | 'set-image-bytes' ;
arg      : IDENT '=' VALUE ;
IDENT    : [a-z-]+ ;
VALUE    : ~[ ]* ;
