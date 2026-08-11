grammar Stdio_md_mutations;
// 🧬️ ANTLR4 mirror of ../📖️component.grammar.semio -- wire-JSON shape of `MdMutation`.

mdMutation
    : noMutation | setSnapshot | insertBlock | removeBlock | replaceBlock | setInlines
    ;
noMutation: '{' MUTATION '"noMutation"' '}';
setSnapshot: '{' MUTATION '"setSnapshot"' ',' '"snapshot"' ':' MD_SNAPSHOT '}';
insertBlock: '{' MUTATION '"insertBlock"' ',' '"path"' ':' pathArray ','
                 '"index"' ':' INDEX ',' '"block"' ':' MD_BLOCK '}';
removeBlock: '{' MUTATION '"removeBlock"' ',' '"path"' ':' pathArray ',' '"index"' ':' INDEX '}';
replaceBlock: '{' MUTATION '"replaceBlock"' ',' '"path"' ':' pathArray ','
                  '"index"' ':' INDEX ',' '"block"' ':' MD_BLOCK '}';
setInlines: '{' MUTATION '"setInlines"' ',' '"path"' ':' pathArray ','
                '"index"' ':' INDEX ',' '"inlines"' ':' MD_INLINE_ARRAY '}';

pathArray: '[' (pathStep ',')* ']';
pathStep: blockQuoteStep | listItemStep;
blockQuoteStep: '{' '"step"' ':' '"blockQuote"' ',' '"index"' ':' INDEX '}';
listItemStep: '{' '"step"' ':' '"listItem"' ',' '"index"' ':' INDEX ',' '"item"' ':' INDEX '}';

MUTATION: '"mutation"' ':';
INDEX: [0-9]+;
STRING: '"' .*? '"';
MD_SNAPSHOT: STRING; // see ../../📸️snapshot/📝️text/🅰️component.g4
MD_BLOCK: STRING;
MD_INLINE_ARRAY: STRING;
