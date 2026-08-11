// 🅰️ `CsvDiff`'s wire text IS its JSON serialization (protocol::OpText::print_op /
// parse_op — serde_json, no bespoke textual syntax). This grammar names the real fields
// rather than a placeholder — it does not restate RFC 8259's own JSON grammar in full.
grammar Stdio_csv_diff;

diff        : '{' member (',' member)* '}' | '{' '}' ;
member      : HAS_HEADER ':' BOOL
            | RECORDS ':' recordsDiff ;
recordsDiff : '{' member (',' member)* '}' ;

HAS_HEADER  : '"hasHeader"' ;
RECORDS     : '"records"' ;
BOOL        : 'true' | 'false' ;
