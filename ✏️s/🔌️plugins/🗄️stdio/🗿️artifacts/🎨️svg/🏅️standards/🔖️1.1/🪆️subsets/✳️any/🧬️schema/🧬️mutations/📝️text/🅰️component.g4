grammar Stdio_svg_mutations;
// 🧬️ Structured tagged/hex mutation text; component.grammar.semio is canonical.
document: MUTATION EOF;
MUTATION: ('no-mutation' | 'set-snapshot' | 'set-declaration' | 'set-doctype' | 'insert-element' | 'remove-element' | 'set-element-name' | 'set-attribute' | 'set-text' | 'set-view-box' | 'set-transform') .* ;
