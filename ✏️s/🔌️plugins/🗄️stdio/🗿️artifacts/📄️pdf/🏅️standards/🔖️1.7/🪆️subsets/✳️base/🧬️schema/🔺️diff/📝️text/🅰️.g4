// ANTLR4 grammar for PdfDiff's sparse structural logical text protocol. Recursive payloads carry
// typed COS values, decoded stream bytes, and typed filter pipelines without JSON or native PDF.
grammar Stdio_pdf_1_7_Diff;

pdfDiff: field* EOF;
field
    : 'declared-version=' ATOM
    | 'info=' payload
    | 'pages=' triple
    | 'objects=' triple
    | 'trailer=' triple
    ;
triple: payload ';' payload ';' payload;
payload: '[' payloadItem* ']';
payloadItem: payload | ATOM | ',' | ':' | ';';

ATOM: [A-Za-z0-9.+-]+;
WS: ' '+ -> skip;
