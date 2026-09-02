// 🅰️ ANTLR grammar for the ASCII STL wire format itself (not a serialization of the Rust
// snapshot struct — the snapshot's `solidName`/`triangles` ARE this grammar's parse).
// https://en.wikipedia.org/wiki/STL_(file_format)
grammar Stdio_stl_snapshot;

solid   : 'solid' NAME? facet* 'endsolid' NAME? EOF ;
facet   : 'facet' 'normal' vector3
          'outer' 'loop'
          vertex vertex vertex
          'endloop'
          'endfacet' ;
vertex  : 'vertex' vector3 ;
vector3 : REAL REAL REAL ;

NAME  : [A-Za-z0-9_.-]+ ;
REAL  : '-'? [0-9]+ ('.' [0-9]+)? ([eE] [-+]? [0-9]+)? ;
WS    : [ \t\r\n]+ -> skip ;
