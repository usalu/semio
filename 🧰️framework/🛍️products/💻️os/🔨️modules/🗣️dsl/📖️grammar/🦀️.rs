//! 🧩️ Explicit OS grammar families, notation matchers and envelope classification.
use semio_framework_diagnostic::TextError;
use semio_framework_dsl::LexOptions;
use semio_framework_dsl::grammar::*;

pub fn family_fragments() -> Result<FragmentRegistry, TextError> {
    let mut reg = FragmentRegistry::new();
    reg.insert("family-geo", parse_grammar(include_str!("../👪️family/🌍️geo/📖️.grammar.semio"))?);
    reg.insert("family-embed", parse_grammar(include_str!("../👪️family/📎️embed/📖️.grammar.semio"))?);
    reg.insert("family-graph", parse_grammar(include_str!("../👪️family/🕸️graph/📖️.grammar.semio"))?);
    reg.insert("family-sheet", parse_grammar(include_str!("../👪️family/📊️sheet/📖️.grammar.semio"))?);
    reg.insert("family-catalog", parse_grammar(include_str!("../👪️family/🗂️catalog/📖️.grammar.semio"))?);
    reg.insert("family-scene", parse_grammar(include_str!("../👪️family/🎬️scene/📖️.grammar.semio"))?);
    reg.insert("family-recipe", parse_grammar(include_str!("../👪️family/🧑‍🍳recipe/📖️.grammar.semio"))?);
    Ok(reg)
}


// 🚫️async: E4 fn-pointer slot — `MacroMatcher.try_match` is a bare `fn(&str) -> bool` — see R2 E4.
fn macro_table_ok(text: &str) -> bool {
    let t = text.trim();
    t.contains('|') || t.starts_with("table")
}

// 🚫️async: E4 fn-pointer slot — see `macro_table_ok` above.
fn macro_quantity_ok(text: &str) -> bool {
    let parts: Vec<_> = text.split_whitespace().collect();
    !parts.is_empty() && parts[0].chars().next().is_some_and(|c| c.is_ascii_digit() || c == '-' || c == '.')
}

// 🚫️async: E4 fn-pointer slot — see `macro_table_ok` above.
fn macro_props_ok(text: &str) -> bool {
    text.contains('=')
}

/// 🔠️ P2-P1: a run of lowercase hex digits (`enc_str`/`hex_encode`'s own alphabet), incl. the
/// empty string (an empty hex-encoded value is valid — see `match_macro_span`'s zero-width floor).
/// Exists as a MACRO, not a `hex = {INT | IDENT | FLOAT}*` production, because a Star is a single
/// greedy pass with no backtracking (`Symbol::Star`'s doc comment): a *production*-modeled `hex`
/// immediately followed by an unrelated bareword literal it happens to share a token KIND with
/// (`set-member`'s real wire shape is `key=<hex> value=<value>` — `value` tokenizes as an ordinary
/// `IDENT`, exactly like a stray hex letter run would) gets silently swallowed INTO the greedy `hex`
/// Star, desyncing everything after it — a real, silent (no parse error, just `recognize() == false`
/// or worse, a wrong match) trap for every future FG-wave author who reaches for `{INT|IDENT}*` to
/// model a generic opaque/hex content field placed before another keyword. `match_macro_span`
/// already tries the largest token span first and shrinks until `try_match` accepts, so `hex` here
/// naturally backtracks off `value` (spelled with `v`/`l`/`u`, none of them valid hex digits) without
/// any grammar-file change beyond referencing the bare `hex` ident (no production named `hex` is
/// defined — `Symbol::Ref`'s existing production-then-macro fallback routes it here automatically).
// 🚫️async: E4 fn-pointer slot — see `macro_table_ok` above.
fn macro_hex_ok(text: &str) -> bool {
    // `slice_source_text` joins multi-token spans with a synthetic `" "` (no such space exists in
    // the real source — whitespace is trivia, stripped before tokens ever reach the recognizer, so
    // e.g. hex `"6b"` lexing as two adjacent tokens `Int("6")`/`Ident("b")` joins to `"6 b"` here);
    // filter it back out before validating, or every multi-token hex value would spuriously fail.
    text.bytes().filter(|b| !b.is_ascii_whitespace()).all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

pub fn product_macros() -> Vec<MacroMatcher> {
    vec![
        MacroMatcher {
            name: "edge",
            // 🚫️async: E4 fn-pointer slot — `MacroMatcher.try_match` is a bare `fn`; `parse_edge_text`
            // `semio_framework_dsl_record::notation::parse_edge_text` was reverted to sync per R9 (whole-module,
            // 0 await/0 io measured), so the `resolve_ready` bridge this thunk used is gone — a
            // direct sync call now.
            try_match: |text| semio_framework_dsl_record::notation::parse_edge_text(text).is_ok(),
        },
        MacroMatcher { name: "table", try_match: macro_table_ok },
        MacroMatcher { name: "quantity", try_match: macro_quantity_ok },
        MacroMatcher { name: "props", try_match: macro_props_ok },
        MacroMatcher { name: "hex", try_match: macro_hex_ok },
    ]
}

/// 📡️ Shallow product envelope check: pack requires leading `0x89` magic
/// (any family) and ≥32 bytes; spr requires non-empty bytes. Deep walks use [`verify_protocol_source`].
pub fn verify_protocol_bytes(spec: &GrammarFile, bytes: &[u8]) -> Result<(), String> {
    let id = spec.id.to_ascii_lowercase();
    let start = spec.start.to_ascii_lowercase();
    let is_spr = start == "record" || id.contains("spr");
    let is_pack = start == "frame" || id.contains("pack") || (matches!(spec.dialect, SemioDialect::Protocol) && !is_spr);
    if is_spr {
        if bytes.is_empty() {
            return Err("spr envelope rejects empty bytes".into());
        }
        return Ok(());
    }
    if is_pack || bytes.first() == Some(&0x89) {
        if bytes.len() < 32 {
            return Err(format!("pack envelope requires ≥32 bytes, got {}", bytes.len()));
        }
        if bytes[0] != 0x89 {
            return Err("pack magic must start with 0x89".into());
        }
        return Ok(());
    }
    Err(format!("verify_protocol_bytes: cannot classify protocol id='{}' start='{}'", spec.id, spec.start))
}


#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "🧪️tests/🧬️originals/🦀️.rs"]
mod original_shipped_grammar_tests;
