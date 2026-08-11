// 🅰️ ANTLR grammar for stdio.mp4's op text form (protocol::OpText::print_op/parse_op in
// ../🦀️component.rs): one compact, single-line JSON object per op, tagged by "mutation".
grammar Stdio_mp4_mutations;
document : jsonLine EOF ;
jsonLine : ~[\r\n]+ ;
