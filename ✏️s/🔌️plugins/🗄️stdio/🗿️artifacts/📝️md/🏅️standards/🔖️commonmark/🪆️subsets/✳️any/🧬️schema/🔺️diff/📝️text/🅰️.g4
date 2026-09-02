grammar Stdio_md_diff;
// 🔺️ ANTLR4 mirror of ../📖️.grammar.semio -- wire-JSON shape of `MdDiff`.

mdDiff: '{' ('"blocks"' ':' blocksDiff)? '}';
blocksDiff: '{' '"removed"' ':' indexArray ',' '"modified"' ':' '[' blockModified* ']' ','
                 '"added"' ':' '[' blockAdded* ']' '}';
blockModified: '{' '"index"' ':' INDEX ',' '"diff"' ':' blockDiff '}';
blockAdded: '{' '"index"' ':' INDEX ',' '"item"' ':' MD_BLOCK '}';

blockDiff
    : headingDiff | paragraphDiff | listDiff | codeBlockDiff
    | blockQuoteDiff | thematicBreakDiff | htmlBlockDiff | replaceDiff
    ;
headingDiff: '{' KIND '"heading"' ('"level"' ':' INDEX)? ('"inlines"' ':' MD_INLINE_ARRAY)? '}';
paragraphDiff: '{' KIND '"paragraph"' ('"inlines"' ':' MD_INLINE_ARRAY)? '}';
listDiff: '{' KIND '"list"' ('"ordered"' ':' BOOL)? ('"start"' ':' (NULL | INDEX))?
              ('"tight"' ':' BOOL)? ('"items"' ':' listItemsDiff)? '}';
codeBlockDiff: '{' KIND '"codeBlock"' ('"info"' ':' (NULL | STRING))? ('"literal"' ':' STRING)? '}';
blockQuoteDiff: '{' KIND '"blockQuote"' ('"blocks"' ':' blocksDiff)? '}';
thematicBreakDiff: '{' KIND '"thematicBreak"' '}';
htmlBlockDiff: '{' KIND '"htmlBlock"' ('"raw"' ':' STRING)? '}';
replaceDiff: '{' KIND '"replace"' ',' '"block"' ':' MD_BLOCK '}';

listItemsDiff: '{' '"removed"' ':' indexArray ',' '"modified"' ':' '[' listItemModified* ']' ','
                    '"added"' ':' '[' listItemAdded* ']' '}';
listItemModified: '{' '"index"' ':' INDEX ',' '"diff"' ':' blocksDiff '}';
listItemAdded: '{' '"index"' ':' INDEX ',' '"item"' ':' '[' MD_BLOCK* ']' '}';

indexArray: '[' (INDEX ',')* ']';
KIND: '"kind"' ':';
BOOL: 'true' | 'false';
NULL: 'null';
INDEX: [0-9]+;
STRING: '"' .*? '"';
MD_BLOCK: STRING; // see ../../📸️snapshot/📝️text/🅰️.g4 for the real block grammar
MD_INLINE_ARRAY: STRING;
