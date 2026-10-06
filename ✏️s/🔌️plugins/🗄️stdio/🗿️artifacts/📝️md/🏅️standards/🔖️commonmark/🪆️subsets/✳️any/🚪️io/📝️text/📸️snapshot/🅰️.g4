grammar Stdio_md_snapshot;
// 🧩 ANTLR4 mirror of 📖️.grammar.semio (the live-wired grammar) -- same honest
// commonmark subset, not a placeholder.

document: block* EOF;

block
    : thematicBreak
    | fencedCodeBlock
    | indentedCodeBlock
    | blockQuote
    | atxHeading
    | htmlBlock
    | list
    | paragraph
    ;

thematicBreak: THEMATIC_LINE NL;
fencedCodeBlock: FENCE infoString? NL codeLine* FENCE NL;
indentedCodeBlock: (INDENT codeLine NL)+;
blockQuote: (QUOTE_MARKER line NL)+;
atxHeading: HASHES inlineRun? NL;
htmlBlock: HTML_START (line NL)*;
list: listItem+;
listItem: (BULLET | ORDINAL) block+;
paragraph: (line NL)+ NL;

inlineRun: inline*;
inline
    : image
    | link
    | codeSpan
    | strong
    | emphasis
    | htmlInline
    | hardBreak
    | softBreak
    | text
    ;

image: '!' '[' inlineRun ']' '(' URL title? ')';
link: '[' inlineRun ']' '(' URL title? ')';
codeSpan: BACKTICKS CHAR* BACKTICKS;
strong: STRONG_DELIM inlineRun STRONG_DELIM;
emphasis: EM_DELIM inlineRun EM_DELIM;
htmlInline: '<' '/'? TAG_NAME attr* '>' | HTML_COMMENT;
hardBreak: TRAILING_SPACES NL | '\\' NL;
softBreak: NL;
text: CHAR+;
title: '"' CHAR* '"';
attr: TAG_NAME '=' '"' CHAR* '"';
infoString: CHAR*;
codeLine: CHAR*;
line: CHAR*;

FENCE: '```' | '~~~';
HASHES: '#' '#'? '#'? '#'? '#'? '#'?;
BULLET: '-' | '*' | '+';
ORDINAL: [0-9]+ ('.' | ')');
QUOTE_MARKER: '>';
HTML_START: '<' [a-zA-Z] | '</' [a-zA-Z] | '<!--';
THEMATIC_LINE: ('-' | '_' | '*') ('-' | '_' | '*' | ' ')*;
STRONG_DELIM: '**' | '__';
EM_DELIM: '*' | '_';
BACKTICKS: '`'+;
TRAILING_SPACES: ' ' ' '+;
HTML_COMMENT: '<!--' .*? '-->';
URL: CHAR+;
TAG_NAME: [a-zA-Z] [a-zA-Z0-9\-]*;
INDENT: '    ';
CHAR: . ;
NL: '\n';
