grammar Stdio_md_mutations;
// 🧬️ ANTLR4 mirror of ../📖️.grammar.semio -- wire-JSON shape of `MdMutation`.

mdMutation
    : insertBlock | removeBlock | replaceBlock | setInlines | spliceSource
    ;
insertBlock: '{' MUTATION '"insertBlock"' ',' '"path"' ':' pathArray ','
                 '"index"' ':' INDEX ',' '"block"' ':' MD_BLOCK '}';
removeBlock: '{' MUTATION '"removeBlock"' ',' '"path"' ':' pathArray ',' '"index"' ':' INDEX '}';
replaceBlock: '{' MUTATION '"replaceBlock"' ',' '"path"' ':' pathArray ','
                  '"index"' ':' INDEX ',' '"block"' ':' MD_BLOCK '}';
setInlines: '{' MUTATION '"setInlines"' ',' '"path"' ':' pathArray ','
                '"index"' ':' INDEX ',' '"inlines"' ':' MD_INLINE_ARRAY '}';

spliceSource: '{' MUTATION '"spliceSource"' ',' '"splices"' ':' '[' (spliceObj (',' spliceObj)*)? ']' '}';
spliceObj: '{' '"offset"' ':' INDEX ',' '"delete"' ':' INDEX ',' '"insert"' ':' STRING '}';

pathArray: '[' (pathStep ',')* ']';
pathStep: blockQuoteStep | listItemStep;
blockQuoteStep: '{' '"step"' ':' '"blockQuote"' ',' '"index"' ':' INDEX '}';
listItemStep: '{' '"step"' ':' '"listItem"' ',' '"index"' ':' INDEX ',' '"item"' ':' INDEX '}';

MUTATION: '"mutation"' ':';
INDEX: [0-9]+;
STRING: '"' .*? '"';
MD_SNAPSHOT: STRING; // see ../../📸️snapshot/📝️text/🅰️.g4
MD_BLOCK: STRING;
MD_INLINE_ARRAY: STRING;
