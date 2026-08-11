// 🅰️ ANTLR grammar for `stdio.semio.brep`'s hand-rolled `SemioBrepDiff` text form
// (protocol::DiffCodec::print_diff/parse_diff). See ../💾️binary/🔠️component.abnf for the full
// per-collection field grammar this restates as ANTLR productions (top-level token shape only,
// entity/value literals refer there).
grammar Stdio_semio_brep_diff;

diffLine  : (token (SP token)*)? EOF ;
token     : collection '=' namedTriple ;
collection: 'vertices' | 'edges' | 'loops' | 'faces' | 'shells' | 'solids' ;
namedTriple: '[' bracketBody? ']' ';' '[' bracketBody? ']' ';' '[' bracketBody? ']' ;
bracketBody: ~[\]]+ ;

SP : ' ' ;
