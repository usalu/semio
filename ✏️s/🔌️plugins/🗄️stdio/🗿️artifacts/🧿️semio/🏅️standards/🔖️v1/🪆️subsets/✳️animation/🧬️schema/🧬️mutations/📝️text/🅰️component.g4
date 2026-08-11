grammar Stdio_semio_animation_mutations;
// Real production names (not the scaffold DOCUMENT: 'schema' [ ]+ placeholder) —
// see 📖️component.grammar.semio in this same directory for the normative EBNF-ish form.
schemaHeader: 'schema' WS 'stdio.semio.animation.mutations' NEWLINE ;
body: bodyEntry* ;
