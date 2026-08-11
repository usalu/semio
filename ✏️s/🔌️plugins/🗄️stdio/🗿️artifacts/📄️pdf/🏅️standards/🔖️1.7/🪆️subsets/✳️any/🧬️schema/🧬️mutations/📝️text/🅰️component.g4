// ANTLR4 grammar for the real wire shape of PdfMutation (1.7): `OpText::print_op` is literally
// `serde_json::to_string(self)` -- one JSON object tagged by "mutation". Matches
// ../🔣️component.json / ../🟦️component.ts.
grammar Stdio_pdf_1_7_Mutations;

pdfMutation: '{' '"mutation"' ':' MUTATION_TAG (',' field)* '}';
MUTATION_TAG: '"noMutation"' | '"setSnapshot"' | '"insertPage"' | '"removePage"'
            | '"setPageMediaBox"' | '"setPageCropBox"' | '"appendPageContent"' | '"setInfo"'
            | '"insertObject"' | '"removeObject"' | '"setObjectValue"'
            | '"setDictEntry"' | '"removeDictEntry"' | '"setTrailerEntry"' | '"removeTrailerEntry"';
field: '"snapshot"' ':' jsonObject
     | '"index"' ':' NUMBER
     | '"page"' ':' jsonObject
     | '"mediaBox"' ':' jsonArray
     | '"cropBox"' ':' (jsonArray | 'null')
     | '"text"' ':' STRING
     | '"info"' ':' jsonObject
     | '"id"' ':' jsonObject
     | '"value"' ':' jsonValue
     | '"path"' ':' jsonArray
     | '"key"' ':' STRING
     ;
jsonValue: jsonObject | jsonArray | STRING | NUMBER | 'true' | 'false' | 'null';
jsonObject: '{' (STRING ':' jsonValue (',' STRING ':' jsonValue)*)? '}';
jsonArray: '[' (jsonValue (',' jsonValue)*)? ']';
STRING: '"' (~["\\] | '\\' .)* '"';
NUMBER: '-'? [0-9]+ ('.' [0-9]+)?;
