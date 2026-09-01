// 🅰️ ANTLR4 mirror of the normative 📖️component.grammar.semio (same production names,
// kebab-case -> camelCase). The grammar/slug identity below (`grammar` line + `DOCUMENT`
// lexer rule) is preserved verbatim from the prior identity fix; the .semio itself defines
// no `header`/`body`/`payload` envelope for this facet (`start diff`), so DOCUMENT is
// kept for traceability but is not referenced by the transcribed rules below.
grammar Draw_draw_diff;

DOCUMENT: 'schema' [ ]+ 'draw.draw.diff' ;

diff: unimplementedNotice ;
unimplementedNotice: 'NO-TEXT-CODEC-EXISTS-FOR-THIS-FACET' ;
