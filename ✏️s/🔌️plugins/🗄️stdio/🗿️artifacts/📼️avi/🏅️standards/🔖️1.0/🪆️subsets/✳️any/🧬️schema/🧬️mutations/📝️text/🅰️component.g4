// 🅰️ ANTLR grammar for stdio.avi's op text form (protocol::OpText in ../🦀️component.rs):
// one compact single-line JSON object per op, tagged by "mutation".
grammar Stdio_avi_mutations;
document : jsonLine EOF ;
jsonLine : ~[\r\n]+ ;
