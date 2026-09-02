grammar Stdio_html_mutations;
// HtmlMutation's real text form is the hand-rolled `keyword arg=value ...` grammar `print_op`/
// `parse_op` implement in component.rs -- see the sibling `📖️.grammar.semio`.
document: KEYWORD (WS ARG '=' VALUE)* EOF;
KEYWORD: [a-z-]+;
ARG: [a-z]+;
VALUE: ~[ ]+;
WS: ' ';
