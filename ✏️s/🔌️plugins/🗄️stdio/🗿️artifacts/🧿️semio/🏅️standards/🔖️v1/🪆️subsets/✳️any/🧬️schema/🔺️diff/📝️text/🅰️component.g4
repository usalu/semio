// ANTLR4 grammar for `stdio.semio`'s real `SemioDiff::print_diff` one-line wire shape —
// descriptive mirror of the authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_any_diff;

diff: 'noChange' | taggedDiff;
taggedDiff: tag ':' REST;
tag: 'replace' | 'brep' | 'mesh' | 'model' | 'value' | 'document' | 'cad' | 'drawing' | 'image' | 'video' | 'audio' | 'animation' | 'presentation' | 'flow';

REST: .*? ;
