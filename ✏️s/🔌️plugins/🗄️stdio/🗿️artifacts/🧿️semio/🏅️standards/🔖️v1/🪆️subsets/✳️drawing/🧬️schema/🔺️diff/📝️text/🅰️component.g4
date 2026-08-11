grammar Stdio_semio_drawing_diff_text;
// 📖️ Real hand-rolled DiffCodec grammar (🦀️component.rs `print_diff`/`parse_diff`): space-
// separated `field=value` tokens, each value bracket-delimited (never a raw octet run).
document: (token (' ' token)*)? EOF;
token: CANVAS_TOK | STYLES_TOK | LAYERS_TOK;
CANVAS_TOK: 'canvas=' BRACKETED;
STYLES_TOK: 'styles=' TRIPLE;
LAYERS_TOK: 'layers=' TRIPLE;
// TRIPLE = "[removed];[modified];[added]"; BRACKETED = one bracket-delimited option/value chain.
// Leaf scalar values inside brackets are hex-encoded JSON -- HEXDIG below, see 🔣️component.json.
BRACKETED: '[' .*? ']';
TRIPLE: '[' .*? '];[' .*? '];[' .*? ']';
fragment HEXDIG: [0-9a-f];
