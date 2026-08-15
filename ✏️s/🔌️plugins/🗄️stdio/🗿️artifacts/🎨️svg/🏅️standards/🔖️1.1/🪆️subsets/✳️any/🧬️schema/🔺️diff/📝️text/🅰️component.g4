grammar Stdio_svg_diff;
// 🧬️ Structured tagged/hex diff text; component.grammar.semio is canonical.
document: STRUCTURED_DIFF EOF;
STRUCTURED_DIFF: ('declaration=' | 'doctype=' | 'root=' | 'prolog=') .+ ;
