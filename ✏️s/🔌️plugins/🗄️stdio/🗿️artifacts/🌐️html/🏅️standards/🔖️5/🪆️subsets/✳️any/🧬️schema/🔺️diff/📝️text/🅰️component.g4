grammar Stdio_html_diff;
// HtmlDiff's real text form is the hand-rolled bracket-token grammar `print_diff`/`parse_diff`
// implement in component.rs (space-separated key=value tokens, tag-prefixed recursive node diffs)
// -- see the sibling `📖️component.grammar.semio` for the full production rules; intentionally not
// re-derived as a second ANTLR grammar here (same "restated in ABNF/ksy/spicy elsewhere, not
// re-derived a third time" convention the sibling `../../📸️snapshot` facets use for their own
// prose-heavy leaves).
document: TOKEN* EOF;
TOKEN: .*?;
