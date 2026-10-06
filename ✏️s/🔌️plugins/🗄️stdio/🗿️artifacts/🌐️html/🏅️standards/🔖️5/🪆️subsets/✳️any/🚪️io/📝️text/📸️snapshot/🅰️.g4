// ANTLR4 grammar for the WHATWG HTML5 well-formed subset this artifact accepts (the `stdio.html`
// snapshot's on-disk body). Honest boundary: no tag-soup error recovery, no foster parenting, no
// implied end tags -- see the component.rs module doc comment for the full rationale.
grammar Stdio_html_snapshot;

document: doctype? element EOF;
doctype: '<!' DOCTYPE_KEYWORD ~'>'* '>';
DOCTYPE_KEYWORD: [Dd][Oo][Cc][Tt][Yy][Pp][Ee];

element
    : voidStartTag
    | rawTextElement
    | startTag content* endTag
    ;

voidStartTag: '<' VOID_NAME attribute* '/'? '>';
VOID_NAME: 'area' | 'base' | 'br' | 'col' | 'embed' | 'hr' | 'img' | 'input' | 'link' | 'meta' | 'param' | 'source' | 'track' | 'wbr';

rawTextElement: rawStartTag RAWTEXT rawEndTag;
rawStartTag: '<' RAWTEXT_NAME attribute* '>';
rawEndTag: '</' RAWTEXT_NAME '>';
RAWTEXT_NAME: 'script' | 'style';
RAWTEXT: .*?; // verbatim up to the matching close tag -- no nested-markup parsing

startTag: '<' NAME attribute* '>';
endTag: '</' NAME '>';

content: element | comment | TEXT;
comment: '<!--' .*? '-->';

attribute: NAME ('=' attributeValue)?;
attributeValue: '"' ~'"'* '"' | '\'' ~'\''* '\'' | UNQUOTED_VALUE;

NAME: [a-zA-Z] [a-zA-Z0-9_.:-]*;
UNQUOTED_VALUE: ~[ \t\r\n>]+;
TEXT: ~'<'+;
