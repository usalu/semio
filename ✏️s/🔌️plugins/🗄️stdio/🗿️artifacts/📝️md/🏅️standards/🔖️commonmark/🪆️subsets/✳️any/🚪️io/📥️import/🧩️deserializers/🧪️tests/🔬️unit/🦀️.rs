
use super::*;

/// 🔬 CommonMark §4.6 start condition 2 (`<!--`): an HTML COMMENT standing alone between two
/// blocks is a block-level HTML node, not paragraph text and not part of the list above it.
///
/// 🎯 This test exists to pin the exact bytes wave 13's differential run turned up. `comrak`'s
/// WRITER injects a literal `<!-- end list -->` separator between a list and a following code
/// block (a conservative guard against an indented code block being absorbed — unnecessary for
/// the fenced block it itself always writes), and its own reader then reports that separator as
/// a sixth document block, so `mutate-md-commonmark :: mutate-set-snapshot` compares six oracle
/// blocks against this repository's five. `📓️w13-final-audit.md` §2.2(11) attributed that to
/// THIS parser dropping the node. It does not: fed the oracle's own output, this parser reports
/// the `htmlBlock` in position 3, exactly as the oracle does. The divergence is the reference
/// writer's injected content, which is why `mutate-md-commonmark`'s feature leaves the row red
/// rather than teaching this renderer to emit a separator no specification asks for.
#[semio_framework_async_macros::async_test]
async fn html_comment_between_a_list_and_a_code_block_is_an_html_block() {
    let source = "- First replacement item\n- Second replacement item\n\n<!-- end list -->\n\n```bash\necho hi\n```\n";
    let blocks = parse_markdown_blocks(source);
    assert_eq!(blocks.len(), 3, "list, html block, code block — got {blocks:?}");
    assert!(matches!(blocks[0], MdBlock::List { .. }), "block 0 must be the list, got {:?}", blocks[0]);
    match &blocks[1] {
        MdBlock::HtmlBlock { raw } => assert_eq!(raw, "<!-- end list -->"),
        other => panic!("block 1 must be the HTML comment, got {other:?}"),
    }
    assert!(matches!(blocks[2], MdBlock::CodeBlock { .. }), "block 2 must be the fenced code block, got {:?}", blocks[2]);
}
