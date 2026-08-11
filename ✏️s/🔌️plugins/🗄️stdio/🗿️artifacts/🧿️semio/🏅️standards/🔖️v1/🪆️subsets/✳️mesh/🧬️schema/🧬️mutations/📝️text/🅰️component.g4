// 🅰️ ANTLR grammar for `s.stdio.semio.mesh`'s hand-rolled `OpText` representation
// (protocol::OpText::print_op/parse_op, see ../🦀️component.rs `OpCodecs` region). REAL grammar:
// either the literal `no-mutation`, or `keyword arg=value ...` (space-separated), one keyword
// per `SemioMeshMutation` variant (kebab-case of the variant name).
grammar Stdio_semio_mesh_mutation;

document : 'no-mutation' EOF
         | KEYWORD (SP arg)* EOF ;
arg      : NAME '=' VALUE ;

KEYWORD : [a-z] [a-z\-]* ;
NAME    : [a-z] [a-z\-]* ;
VALUE   : ~[ ]* ;
SP      : ' ' ;
