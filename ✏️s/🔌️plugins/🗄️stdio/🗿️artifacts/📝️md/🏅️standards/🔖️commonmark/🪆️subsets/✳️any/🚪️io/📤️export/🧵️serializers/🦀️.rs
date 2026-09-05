//! 🧵️ CommonMark (md) export — a real, from-scratch block/inline renderer covering
//! https://spec.commonmark.org/'s common real-world subset. `MdSnapshot.blocks` is the complete
//! typed persisted source of truth (see the snapshot module's own doc comment for the exact
//! honest-subset scope and the deviations it lists); `render_markdown_blocks` is the codec's
//! write half, independently testable.

use crate::artifacts::md::schema::snapshot::{MdBlock, MdInline};

//#region 🔖️BlockRenderer
/// 📤 Top-level entry: renders the complete block sequence back to CommonMark text. Documented
/// normal form (see the snapshot module's doc comment): indented code re-emits fenced, list
/// tightness/marker style is normalized, and no attempt is made at byte-identical round-trip --
/// `decode(encode(x)) == x` at the SNAPSHOT level (semantic fixed point) is the contract, not
/// byte preservation of arbitrary source text (`codec_retention_law`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn render_markdown_blocks(blocks: &[MdBlock]) -> String {
    let mut out = String::new();
    render_blocks(blocks, &mut out);
    while out.ends_with('\n') {
        out.pop();
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_blocks(blocks: &[MdBlock], out: &mut String) {
    for block in blocks {
        render_block(block, out);
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_block(block: &MdBlock, out: &mut String) {
    match block {
        MdBlock::Heading { level, inlines } => {
            out.push_str(&"#".repeat((*level).clamp(1, 6) as usize));
            out.push(' ');
            out.push_str(&render_inlines(inlines));
            out.push_str("\n\n");
        }
        MdBlock::Paragraph { inlines } => {
            out.push_str(&render_inlines(inlines));
            out.push_str("\n\n");
        }
        MdBlock::CodeBlock { info, literal } => {
            out.push_str("```");
            if let Some(i) = info {
                out.push_str(i);
            }
            out.push('\n');
            out.push_str(literal);
            if !literal.is_empty() && !literal.ends_with('\n') {
                out.push('\n');
            }
            out.push_str("```\n\n");
        }
        MdBlock::BlockQuote { blocks } => {
            let mut inner = String::new();
            render_blocks(blocks, &mut inner);
            while inner.ends_with('\n') {
                inner.pop();
            }
            for line in inner.lines() {
                out.push_str("> ");
                out.push_str(line);
                out.push('\n');
            }
            out.push('\n');
        }
        MdBlock::ThematicBreak => out.push_str("---\n\n"),
        MdBlock::HtmlBlock { raw } => {
            out.push_str(raw);
            out.push_str("\n\n");
        }
        // 📃 A list must CLOSE, exactly like every other block above: a tight list's items each end
        // with a single `\n`, so without the blank line added here the next block's first line is a
        // lazy continuation of the last item's paragraph, and a following list merges into this one
        // — which CommonMark then reads back as ONE LOOSE list. That is how the real README fixture's
        // tight `**Title symbols:**` list came back `"tight": false` through `comrak` (ticket
        // 26/08/23/END-TO-END-TESTING-REFACTOR, `📝️mutate-md-commonmark`'s first subject run: all six
        // `inverse-<kind>` rows and the round trip failed on this one block). A loose list already
        // ends with the blank line `render_list_item` writes after every item, so the guard below
        // adds nothing there.
        MdBlock::List { ordered, start, tight, items } => {
            let mut n = start.unwrap_or(1);
            for item in items {
                let marker = if *ordered { format!("{n}. ") } else { "- ".to_string() };
                n += 1;
                render_list_item(&marker, item, *tight, out);
            }
            if !items.is_empty() && !out.ends_with("\n\n") {
                out.push('\n');
            }
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_list_item(marker: &str, blocks: &[MdBlock], tight: bool, out: &mut String) {
    let indent = " ".repeat(marker.chars().count());
    let mut first_line = true;
    for block in blocks {
        let mut buf = String::new();
        render_block(block, &mut buf);
        while buf.ends_with('\n') {
            buf.pop();
        }
        for line in buf.lines() {
            if first_line {
                out.push_str(marker);
            } else {
                out.push_str(&indent);
            }
            out.push_str(line);
            out.push('\n');
            first_line = false;
        }
    }
    if !tight {
        out.push('\n');
    }
}
//#endregion 🔖️BlockRenderer

//#region 🔖️InlineRenderer
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_inlines(inlines: &[MdInline]) -> String {
    let mut out = String::new();
    for inline in inlines {
        render_inline(inline, &mut out);
    }
    out
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn render_inline(inline: &MdInline, out: &mut String) {
    match inline {
        MdInline::Text { text } => out.push_str(text),
        MdInline::Emphasis { inlines } => {
            out.push('*');
            out.push_str(&render_inlines(inlines));
            out.push('*');
        }
        MdInline::Strong { inlines } => {
            out.push_str("**");
            out.push_str(&render_inlines(inlines));
            out.push_str("**");
        }
        MdInline::Code { literal } => {
            out.push('`');
            out.push_str(literal);
            out.push('`');
        }
        MdInline::Link { text, url, title } => {
            out.push('[');
            out.push_str(&render_inlines(text));
            out.push_str("](");
            out.push_str(url);
            if let Some(t) = title {
                out.push_str(" \"");
                out.push_str(t);
                out.push('"');
            }
            out.push(')');
        }
        MdInline::Image { alt, url, title } => {
            out.push_str("![");
            out.push_str(alt);
            out.push_str("](");
            out.push_str(url);
            if let Some(t) = title {
                out.push_str(" \"");
                out.push_str(t);
                out.push('"');
            }
            out.push(')');
        }
        MdInline::SoftBreak => out.push('\n'),
        MdInline::HardBreak => out.push_str("  \n"),
        MdInline::HtmlInline { raw } => out.push_str(raw),
    }
}
//#endregion 🔖️InlineRenderer
