// 🅰️ ANTLR mirror (descriptive, not test-parsed — the real recognizer is
// `../📖️component.grammar.semio`, walked by `dsl::Recognizer`) for `s.stdio.semio.mesh`'s DSL
// text representation (store::ArtifactDsl::parse_dsl/print_dsl,
// `../../🦀️component.rs`'s `print_mesh_snapshot_body`/`parse_mesh_snapshot_body`). The `semio
// stdio.semio.mesh.dsl v1` preamble line is stripped by store::semio_format::split_text_preamble
// before this grammar's `document` production runs — `document` below matches the RECONSTRUCTED
// body (bare `artifactMark` token standing in for the stripped preamble), same convention every
// other real pilot's own `.g4` mirror uses.
grammar Stdio_semio_mesh_snapshot;

document      : artifactMark schemaLine meshesLine materialsLine texturesLine ;
artifactMark  : 'stdio.semio.mesh' ;
schemaLine    : 'schema' '=' HEX ;

meshesLine    : 'meshes' '=' '[' meshList? ']' ;
meshList      : mesh (',' mesh)* ;
mesh          : '[' HEX ',' '[' primitiveList? ']' ']' ;
primitiveList : primitive (',' primitive)* ;
primitive     : '[' HEX ',' topology ',' '[' point3List? ']' ',' '[' point3List? ']' ',' '[' uvList? ']' ',' '[' rgbaList? ']' ',' '[' indexList? ']' ',' optionHex ']' ;

materialsLine : 'materials' '=' '[' materialList? ']' ;
materialList  : material (',' material)* ;
material      : '[' HEX ',' rgba ',' NUMBER ',' NUMBER ']' ;

texturesLine  : 'textures' '=' '[' textureList? ']' ;
textureList   : texture (',' texture)* ;
texture       : '[' HEX ',' HEX ',' HEX ']' ;

point3List    : point3 (',' point3)* ;
point3        : '[' NUMBER ',' NUMBER ',' NUMBER ']' ;
uvList        : uv (',' uv)* ;
uv            : '[' NUMBER ',' NUMBER ']' ;
rgbaList      : rgba (',' rgba)* ;
rgba          : '[' NUMBER ',' NUMBER ',' NUMBER ',' NUMBER ']' ;
indexList     : INT (',' INT)* ;
topology      : 'P' | 'L' | 'S' | 'T' | 'X' | 'F' ;
optionHex     : '[' '0' ']' | '[' '1' ',' HEX ']' ;
NUMBER        : INT | FLOAT ;

HEX   : [0-9a-f]* ;
INT   : '-'? [0-9]+ ;
FLOAT : '-'? [0-9]+ '.' [0-9]* ;
WS    : [ \t\r\n]+ -> skip ;
