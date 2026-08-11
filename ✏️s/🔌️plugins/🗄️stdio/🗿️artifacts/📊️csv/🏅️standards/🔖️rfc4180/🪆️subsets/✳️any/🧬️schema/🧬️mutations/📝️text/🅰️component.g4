// 🅰️ `CsvMutation`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, tagged on the "mutation" field). Names the real variant tags
// rather than a placeholder.
grammar Stdio_csv_mutations;

mutation  : '{' '"mutation"' ':' tag (',' member)* '}' ;
tag       : '"noMutation"' | '"setSnapshot"' | '"setHasHeader"'
          | '"insertRecord"' | '"removeRecord"' | '"setField"' ;
member    : STRING ':' value ;
STRING    : '"' .*? '"' ;
