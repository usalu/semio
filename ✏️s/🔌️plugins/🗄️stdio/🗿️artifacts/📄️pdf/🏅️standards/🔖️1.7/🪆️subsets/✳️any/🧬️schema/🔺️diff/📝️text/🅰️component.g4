// ANTLR4 grammar for the real serde_json-tagged wire shape of `PdfDiff` (1.7) -- matches
// ../🔣️component.json / ../🟦️component.ts field-for-field. `PdfDiff` has no dedicated OpText
// envelope yet (F6 wave); this is plain JSON.
grammar Stdio_pdf_1_7_Diff;

pdfDiff: '{' (member (',' member)*)? '}';
member: '"declaredVersion"' ':' STRING
      | '"info"' ':' pdfInfo
      | '"pages"' ':' pdfPagesDiff
      | '"objects"' ':' pdfObjectsDiff
      | '"trailer"' ':' pdfDictDiff
      ;
pdfInfo: '{' (infoMember (',' infoMember)*)? '}';
infoMember: ('"title"' | '"author"' | '"subject"' | '"keywords"' | '"creator"' | '"producer"') ':' STRING;
pdfPagesDiff: '{' (tripleMember (',' tripleMember)*)? '}';
pdfObjectsDiff: '{' (tripleMember (',' tripleMember)*)? '}';
pdfDictDiff: '{' (tripleMember (',' tripleMember)*)? '}';
tripleMember: ('"removed"' | '"modified"' | '"added"') ':' jsonArray;

jsonValue: jsonObject | jsonArray | STRING | NUMBER | 'true' | 'false' | 'null';
jsonObject: '{' (STRING ':' jsonValue (',' STRING ':' jsonValue)*)? '}';
jsonArray: '[' (jsonValue (',' jsonValue)*)? ']';
STRING: '"' (~["\\] | '\\' .)* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
