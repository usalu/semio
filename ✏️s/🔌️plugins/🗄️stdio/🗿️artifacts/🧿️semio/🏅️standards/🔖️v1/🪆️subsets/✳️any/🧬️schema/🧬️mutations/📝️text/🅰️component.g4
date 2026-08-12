// ANTLR4 grammar for `stdio.semio`'s real `SemioMutation::print_op` one-line wire shape —
// descriptive mirror of the authoritative `📖️component.grammar.semio` (same production names).
grammar Semio_any_mutations;

op: 'noMutation' | taggedOp;
taggedOp: tag ':' REST;
tag: 'setSnapshot' | 'brep' | 'mesh' | 'model' | 'value' | 'document' | 'cad' | 'drawing' | 'image' | 'video' | 'audio' | 'animation' | 'presentation' | 'flow';

REST: .*? ;
