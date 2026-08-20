//! @emoji 🖋️ `dsl_notation` — the shared notation toolkit handcrafted per-app grammars build on,
//! so "every grammar is handcrafted" doesn't mean "every grammar reinvents its own sub-parsers".
//! First resident: the edge/arrow literal, extended with an optional `[id:kind]` label so
//! graph-like documents can read as arrows (`a -> b`, `a -[e1:Connection]-> b`) instead of
//! flat `edges [id kind source target] {...}` tables. See `//#region 🔖️Edge` below.
//!
//! This crate deliberately depends on `dsl_core` only (its lexer/token alphabet), not
//! `dsl_schema`. It is additive and standalone: nothing in the repo calls it yet. The existing
//! `Shape::Wire`/`WireValue` in `dsl_schema` (the only current graph-arrow grammar, consumed by
//! `flow`, `procedural2d/3d`, `sequence`, `mathematical`, `dag`, `pack_value`, and
//! `math::graph::dsl`) is intentionally left untouched here: several of those consumer
//! trees have live uncommitted work in progress as of this writing, so migrating `Shape::Wire`
//! onto this toolkit is deferred to when that work has landed (see the ticket's collision-map.txt
//! and the `handcrafted-grammar-for-every-artifact` plan, wave W4e). This crate proves the
//! labeled-edge notation in isolation, ready to be adopted then.

use crate::os_dsl::{lex, Limits, SpannedToken, TextError, TextSpan, TokenKind};

//#region 🔖️Edge

/// @emoji 🕸️ One endpoint of an edge statement: `id[:kind][@port]`.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EdgeNode {
    pub id: String,
    pub kind: Option<String>,
    pub port: Option<String>,
}

/// @emoji 🔗️ The optional id/kind label carried by a bracketed edge arrow — `[e1:Connection]`,
/// `[e1]`, `[:Connection]`. Reuses Jack's existing `-[r:Kind]->` relationship-pattern bracket
/// notation (`🧮️math/🕸️graph/🗣️dsl`) rather than inventing a second bracket-free spelling, so the
/// two labeled-arrow grammars already in the repo agree with each other.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct EdgeLabel {
    pub id: Option<String>,
    pub kind: Option<String>,
}

impl EdgeLabel {
    pub async fn is_empty(&self) -> bool {
        self.id.is_none() && self.kind.is_none()
    }
}

/// @emoji 🔀️ The arrow between `from` and a linked node: direction plus an optional label.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeLink {
    pub directed: bool,
    pub label: EdgeLabel,
    pub to: EdgeNode,
}

/// @emoji 🕸️ One edge statement (or a bare node declaration if `link` is `None`).
///
/// Surface forms — `<-` and its labeled/reversed counterpart are sugar only, normalized by
/// endpoint-swap so the stored value (and everything reprinted from it) only ever holds a
/// forward-directed or undirected link, exactly like today's `crate::os_dsl::schema::WireValue`:
///
/// | written                     | normalized to (from, link)                              |
/// |------------------------------|----------------------------------------------------------|
/// | `a -> b`                     | `(a, directed=true,  label=∅,        to=b)`               |
/// | `a -- b`                     | `(a, directed=false, label=∅,        to=b)`               |
/// | `a <- b`                     | `(b, directed=true,  label=∅,        to=a)` (swapped)     |
/// | `a -[e1:Connection]-> b`     | `(a, directed=true,  label=e1:Connection, to=b)`          |
/// | `a -[e1]- b`                 | `(a, directed=false, label=e1,       to=b)`               |
/// | `a -[:Connection]-> b`       | `(a, directed=true,  label=:Connection, to=b)`            |
/// | `a <-[e1:Connection]- b`     | `(b, directed=true,  label=e1:Connection, to=a)` (swapped) |
///
/// Endpoints are `id[:kind][@port]` (unchanged from `WireValue::WireNode`).
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeValue {
    pub from: EdgeNode,
    pub link: Option<EdgeLink>,
}

struct Cursor {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Cursor {
    async fn new(tokens: Vec<SpannedToken>) -> Self {
        let tokens = tokens.into_iter().filter(|t| !t.kind.is_trivia()).collect();
        Self { tokens, pos: 0 }
    }

    async fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    async fn peek_at(&self, offset: usize) -> Option<&SpannedToken> {
        self.tokens.get(self.pos + offset)
    }

    async fn advance(&mut self) -> SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    async fn expect(&mut self, kind: TokenKind) -> Result<SpannedToken, TextError> {
        if self.peek().await.kind == kind {
            Ok(self.advance().await)
        } else {
            Err(TextError::new(format!("expected {kind:?}, found {:?}", self.peek().await.kind), self.peek().await.span.clone()))
        }
    }

    async fn span(&self) -> TextSpan {
        self.peek().await.span.clone()
    }
}

async fn parse_edge_node(cursor: &mut Cursor) -> Result<EdgeNode, TextError> {
    let id = cursor.expect(TokenKind::Ident).await?.text.as_str().to_string();
    let kind = if cursor.peek().await.kind == TokenKind::Colon {
        cursor.advance().await;
        Some(cursor.expect(TokenKind::Ident).await?.text.as_str().to_string())
    } else {
        None
    };
    let port = if cursor.peek().await.kind == TokenKind::At {
        cursor.advance().await;
        Some(cursor.expect(TokenKind::Ident).await?.text.as_str().to_string())
    } else {
        None
    };
    Ok(EdgeNode { id, kind, port })
}

pub async fn decode_fused_edge_arrow(text: &str) -> Result<(bool, EdgeLabel), TextError> {
    let body = text.strip_prefix('-').ok_or_else(|| TextError::new("fused edge must start with `-`", TextSpan::at(1, 1)))?;
    let (core, directed) = if let Some(core) = body.strip_suffix('>') {
        (core, true)
    } else if let Some(core) = body.strip_suffix('-') {
        (core, false)
    } else {
        return Err(TextError::new("fused edge must end with `>` or `-`", TextSpan::at(1, 1)));
    };
    if core.is_empty() {
        return Err(TextError::new("fused edge label is empty", TextSpan::at(1, 1)));
    }
    let (id, kind) = if let Some(rest) = core.strip_prefix(':') {
        (None, Some(rest.to_string()))
    } else if let Some((id_part, kind_part)) = core.split_once(':') {
        (Some(id_part.to_string()), Some(kind_part.to_string()))
    } else {
        (Some(core.to_string()), None)
    };
    Ok((directed, EdgeLabel { id, kind }))
}

async fn print_fused_edge_arrow(label: &EdgeLabel, directed: bool) -> String {
    let mut out = String::from('-');
    if let Some(id) = &label.id {
        out.push_str(id);
    }
    if let Some(kind) = &label.kind {
        out.push(':');
        out.push_str(kind);
    }
    out.push(if directed { '>' } else { '-' });
    out
}

async fn parse_edge_label(cursor: &mut Cursor) -> Result<EdgeLabel, TextError> {
    cursor.expect(TokenKind::LBracket).await?;
    let id = if cursor.peek().await.kind == TokenKind::Ident {
        Some(cursor.advance().await.text.as_str().to_string())
    } else {
        None
    };
    let kind = if cursor.peek().await.kind == TokenKind::Colon {
        cursor.advance().await;
        Some(cursor.expect(TokenKind::Ident).await?.text.as_str().to_string())
    } else {
        None
    };
    let label = EdgeLabel { id, kind };
    if label.is_empty().await {
        return Err(TextError::new("edge label `[...]` must name an id and/or a `:kind`", cursor.span().await));
    }
    cursor.expect(TokenKind::RBracket).await?;
    Ok(label)
}

/// @emoji 🕸️ Parses one edge (or bare node) statement from an already-lexed cursor.
async fn parse_edge(cursor: &mut Cursor) -> Result<EdgeValue, TextError> {
    let mut from = parse_edge_node(cursor).await?;
    let link = match cursor.peek().await.kind {
        TokenKind::Arrow => {
            cursor.advance().await;
            let to = parse_edge_node(cursor).await?;
            Some(EdgeLink { directed: true, label: EdgeLabel::default(), to })
        }
        TokenKind::DashArrow => {
            cursor.advance().await;
            let to = parse_edge_node(cursor).await?;
            Some(EdgeLink { directed: false, label: EdgeLabel::default(), to })
        }
        TokenKind::BackArrow => {
            cursor.advance().await;
            let label = if cursor.peek().await.kind == TokenKind::LBracket {
                let label = parse_edge_label(cursor).await?;
                cursor.expect(TokenKind::Minus).await?;
                label
            } else {
                EdgeLabel::default()
            };
            let to = parse_edge_node(cursor).await?;
            let swapped_to = std::mem::replace(&mut from, to);
            Some(EdgeLink { directed: true, label, to: swapped_to })
        }
        TokenKind::Minus if cursor.peek_at(1).await.map(|t| t.kind) == Some(TokenKind::LBracket) => {
            cursor.advance().await;
            let label = parse_edge_label(cursor).await?;
            let directed = match cursor.peek().await.kind {
                TokenKind::Arrow => {
                    cursor.advance().await;
                    true
                }
                TokenKind::Minus => {
                    cursor.advance().await;
                    false
                }
                other => return Err(TextError::new(format!("expected `->` or `-` to close a labeled edge, found {other:?}"), cursor.span().await)),
            };
            let to = parse_edge_node(cursor).await?;
            Some(EdgeLink { directed, label, to })
        }
        TokenKind::EdgeArrow => {
            let token = cursor.advance();
            let (directed, label) = decode_fused_edge_arrow(&token.await.text.as_str()).await?;
            let to = parse_edge_node(cursor).await?;
            Some(EdgeLink { directed, label, to })
        }
        _ => None,
    };
    Ok(EdgeValue { from, link })
}

/// @emoji 🔌️ Lexes + parses one standalone edge literal — the entry point family/app grammars
/// call for the `edge` macro-production.
pub async fn parse_edge_text(text: &str) -> Result<EdgeValue, TextError> {
    let limits = Limits::default();
    let tokens = lex(text, &limits, false).await?;
    let mut cursor = Cursor::new(tokens).await;
    let edge = parse_edge(&mut cursor).await?;
    if cursor.peek().await.kind != TokenKind::Eof {
        return Err(TextError::new(format!("unexpected trailing {:?} after edge literal", cursor.peek().await.kind), cursor.span().await));
    }
    Ok(edge)
}

async fn print_edge_node(node: &EdgeNode, out: &mut String) {
    out.push_str(&node.id);
    if let Some(kind) = &node.kind {
        out.push(':');
        out.push_str(kind);
    }
    if let Some(port) = &node.port {
        out.push('@');
        out.push_str(port);
    }
}

/// @emoji 🖨️ Canonical printer for [`EdgeValue`] — `parse_edge_text(print_edge(x)) == x` is the
/// round-trip law every macro-production in this toolkit must satisfy.
///
/// Labeled arrows print with a leading space before the opening `-[`: `a` is `is_ident_continue`
/// through the whole run of `-`, and the shared `dsl_core` lexer only stops an identifier early at
/// a `-` when the *next* char is `>` or `-` (so plain `->`/`--` tokenize cleanly) — it has no
/// special case for a following `[`. Without the space, `a-[e1:Connection]->b` would lex as ident
/// `"a-"` followed by `[`, not `a` then a fresh `-`. A leading space sidesteps this by ending the
/// identifier at whitespace instead, with no change to the shared lexer needed. The unlabeled
/// forms (`->`/`--`) are unaffected by this and keep the original no-space style unchanged.
pub async fn print_edge(edge: &EdgeValue) -> String {
    let mut out = String::new();
    print_edge_node(&edge.from, &mut out).await;
    if let Some(link) = &edge.link {
        if link.label.is_empty().await {
            out.push_str(if link.directed { "->" } else { "--" });
        } else {
            out.push(' ');
            out.push_str(&print_fused_edge_arrow(&link.label, link.directed).await);
        }
        print_edge_node(&link.to, &mut out).await;
    }
    out
}
//#endregion 🔖️Edge

//#region 🔖️Quantity
/// @emoji 📐️ Parses a number glued to a unit suffix — `210GPa`, `0.8kN/m2`, `45%` — converting
/// into `native`'s scale. `dsl_core`'s number lexer doesn't glue the suffix on itself (`210GPa`
/// lexes as two adjacent tokens, `Float("210")` then `Ident("GPa")`, exactly like
/// `crate::os_dsl::schema::parse_scalar`'s existing `Quantity`/`Angle` shapes already rely on) — this checks
/// the two tokens are byte-adjacent (no whitespace) before treating the ident as a unit suffix
/// rather than the start of the next statement. A number with no suffix at all is accepted too,
/// read as already being in `native`'s unit. Rejects a suffix whose dimension doesn't match
/// `native`'s (a length unit on an angle field, say) rather than silently reinterpreting it.
pub async fn parse_quantity_text(text: &str, native: &'static crate::os_dsl::UnitSpec) -> Result<f64, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false).await?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let number = tokens.first().ok_or_else(|| TextError::new("expected a quantity", TextSpan::at(1, 1)))?;
    if !matches!(number.kind, TokenKind::Float | TokenKind::Int) {
        return Err(TextError::new(format!("expected a number, found {:?}", number.kind), number.span));
    }
    let raw: f64 = number.text.as_str().parse().map_err(|_| TextError::new(format!("not a valid number: {}", number.text.as_str()), number.span))?;

    let suffix = tokens.get(1).filter(|t| t.kind == TokenKind::Ident && t.byte_range.0 == number.byte_range.1);
    let value = match suffix {
        Some(suffix_token) => {
            let symbol = suffix_token.text.as_str();
            let unit = crate::os_dsl::unit_by_symbol(&symbol).await.ok_or_else(|| TextError::new(format!("unknown unit `{symbol}`"), suffix_token.span))?;
            crate::os_dsl::convert(raw, unit, native).await.ok_or_else(|| TextError::new(format!("unit `{symbol}` is not compatible with `{}`", native.symbol), suffix_token.span))?
        }
        None => raw,
    };

    let consumed = if suffix.is_some() { 2 } else { 1 };
    if tokens.len() > consumed {
        return Err(TextError::new("unexpected trailing content after quantity", tokens[consumed].span));
    }
    Ok(value)
}

/// @emoji 🖨️ Canonical printer — always suffixes in `native`'s own unit (never the alien unit a
/// value might have been parsed from), so re-parsing the printed form is always a same-unit,
/// lossless round trip.
pub async fn print_quantity(value: f64, native: &'static crate::os_dsl::UnitSpec) -> String {
    format!("{}{}", crate::os_dsl::format_f64(value), native.symbol)
}

/// @emoji 📐️ `parse_quantity_text` specialized to degrees: a bare number with no suffix is read
/// as degrees (unlike `parse_quantity_text`'s general "no suffix = already-native-unit" rule,
/// this pins the no-suffix case specifically, since degrees are what the architecture calls the
/// canonical angle unit); `rad`/`turn` suffixes convert in.
pub async fn parse_angle_text(text: &str) -> Result<f64, TextError> {
    let deg = crate::os_dsl::unit_by_symbol("deg").await.expect("`deg` is a built-in unit");
    parse_quantity_text(text, deg).await
}

/// @emoji 🖨️ Canonical printer for an angle in degrees — `45°` (the `°` symbol, not `deg`).
pub async fn print_angle(value_deg: f64) -> String {
    let degree_symbol = crate::os_dsl::unit_by_symbol("°").await.expect("`°` is a built-in unit");
    format!("{}{}", crate::os_dsl::format_f64(value_deg), degree_symbol.symbol)
}
//#endregion 🔖️Quantity

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    async fn node(id: &str) -> EdgeNode {
        EdgeNode { id: id.to_string(), kind: None, port: None }
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_a_bare_node_with_no_link() {
        let value = parse_edge_text("branch-root").await.expect("parse_edge_text");
        assert_eq!(value, EdgeValue { from: node("branch-root").await, link: None });
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_plain_directed_arrow() {
        let value = parse_edge_text("a->b").await.expect("parse_edge_text");
        assert_eq!(value.link.as_ref().unwrap().directed, true);
        assert!(value.link.as_ref().unwrap().label.is_empty().await);
        assert_eq!(print_edge(&value).await, "a->b");
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_plain_undirected_dash() {
        let value = parse_edge_text("a--b").await.expect("parse_edge_text");
        assert_eq!(value.link.as_ref().unwrap().directed, false);
        assert_eq!(print_edge(&value).await, "a--b");
    }

    #[semio_framework_async_macros::async_test]
    async fn back_arrow_is_sugar_normalized_by_endpoint_swap() {
        let value = parse_edge_text("b<-a").await.expect("parse_edge_text");
        assert_eq!(value.from, node("a").await);
        assert_eq!(value.link.as_ref().unwrap().to, node("b").await);
        assert_eq!(value.link.as_ref().unwrap().directed, true);
        // Canonical print never re-emits `<-`.
        assert_eq!(print_edge(&value).await, "a->b");
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_labeled_directed_edge_with_id_and_kind() {
        let value = parse_edge_text("a -e1:Connection>b").await.expect("parse_edge_text");
        let link = value.link.expect("link");
        assert_eq!(link.directed, true);
        assert_eq!(link.label, EdgeLabel { id: Some("e1".to_string()), kind: Some("Connection".to_string()) });
        assert_eq!(link.to, node("b").await);
        assert_eq!(print_edge(&EdgeValue { from: node("a").await, link: Some(link) }).await, "a -e1:Connection>b");
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_labeled_undirected_edge_id_only() {
        let value = parse_edge_text("a -e1-b").await.expect("parse_edge_text");
        let link = value.link.expect("link");
        assert_eq!(link.directed, false);
        assert_eq!(link.label, EdgeLabel { id: Some("e1".to_string()), kind: None });
        assert_eq!(print_edge(&EdgeValue { from: node("a").await, link: Some(link) }).await, "a -e1-b");
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_labeled_edge_kind_only() {
        let value = parse_edge_text("a -:Connection>b").await.expect("parse_edge_text");
        let link = value.link.expect("link");
        assert_eq!(link.label, EdgeLabel { id: None, kind: Some("Connection".to_string()) });
        assert_eq!(print_edge(&EdgeValue { from: node("a").await, link: Some(link) }).await, "a -:Connection>b");
    }

    #[semio_framework_async_macros::async_test]
    async fn bracket_labeled_edge_still_parses() {
        let value = parse_edge_text("a -[e1:Connection]->b").await.expect("parse_edge_text");
        let link = value.link.expect("link");
        assert_eq!(link.label.id.as_deref(), Some("e1"));
    }

    #[semio_framework_async_macros::async_test]
    async fn labeled_back_arrow_is_sugar_normalized_by_endpoint_swap() {
        let value = parse_edge_text("b<-[e1:Connection]-a").await.expect("parse_edge_text");
        assert_eq!(value.from, node("a").await);
        let printed = print_edge(&value).await;
        let link = value.link.expect("link");
        assert_eq!(link.to, node("b").await);
        assert_eq!(link.directed, true);
        assert_eq!(link.label, EdgeLabel { id: Some("e1".to_string()), kind: Some("Connection".to_string()) });
        assert_eq!(printed, "a -e1:Connection>b");
    }

    #[semio_framework_async_macros::async_test]
    async fn endpoints_carry_kind_and_port() {
        let value = parse_edge_text("capsule@in-a -c1:Connection>tower@out-b").await.expect("parse_edge_text");
        assert_eq!(value.from, EdgeNode { id: "capsule".to_string(), kind: None, port: Some("in-a".to_string()) });
        let link = value.link.expect("link");
        assert_eq!(link.to, EdgeNode { id: "tower".to_string(), kind: None, port: Some("out-b".to_string()) });
    }

    #[semio_framework_async_macros::async_test]
    async fn node_kind_and_port_round_trip() {
        let value = parse_edge_text("v1:Vertex@p0->v2:Vertex@p1").await.expect("parse_edge_text");
        assert_eq!(print_edge(&value).await, "v1:Vertex@p0->v2:Vertex@p1");
    }

    #[semio_framework_async_macros::async_test]
    async fn empty_label_is_rejected() {
        let err = parse_edge_text("a -[]->b").await.unwrap_err();
        assert!(err.message.contains("must name an id"), "unexpected message: {}", err.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn round_trip_matrix_over_representative_values() {
        let cases = vec![
            EdgeValue { from: node("a").await, link: None },
            EdgeValue { from: node("a").await, link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("b").await }) },
            EdgeValue { from: node("a").await, link: Some(EdgeLink { directed: false, label: EdgeLabel::default(), to: node("b").await }) },
            EdgeValue {
                from: node("a").await,
                link: Some(EdgeLink { directed: true, label: EdgeLabel { id: Some("e1".to_string()), kind: Some("Connection".to_string()) }, to: node("b").await }),
            },
        ];
        for case in cases {
            let printed = print_edge(&case).await;
            let reparsed = parse_edge_text(&printed).await.unwrap_or_else(|e| panic!("reparse of {printed:?} failed: {e:?}"));
            assert_eq!(reparsed, case, "round trip mismatch for {printed:?}");
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_quantity_in_native_unit() {
        let gpa = crate::os_dsl::unit_by_symbol("GPa").await.unwrap();
        let value = parse_quantity_text("210GPa", gpa).await.expect("parse_quantity_text");
        assert!((value - 210.0).abs() < 1e-9);
        assert_eq!(print_quantity(value, gpa).await, "210GPa");
    }

    #[semio_framework_async_macros::async_test]
    async fn converts_a_compatible_alien_unit_into_native_scale() {
        let gpa = crate::os_dsl::unit_by_symbol("GPa").await.unwrap();
        // 210000 MPa == 210 GPa
        let value = parse_quantity_text("210000MPa", gpa).await.expect("parse_quantity_text");
        assert!((value - 210.0).abs() < 1e-6, "got {value}");
    }

    #[semio_framework_async_macros::async_test]
    async fn bare_number_with_no_suffix_is_read_in_native_unit() {
        let gpa = crate::os_dsl::unit_by_symbol("GPa").await.unwrap();
        let value = parse_quantity_text("210", gpa).await.expect("parse_quantity_text");
        assert!((value - 210.0).abs() < 1e-9);
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_a_dimensionally_incompatible_unit() {
        let gpa = crate::os_dsl::unit_by_symbol("GPa").await.unwrap();
        let err = parse_quantity_text("210m", gpa).await.unwrap_err();
        assert!(err.message.contains("not compatible"), "unexpected message: {}", err.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn rejects_an_unknown_unit_symbol() {
        let gpa = crate::os_dsl::unit_by_symbol("GPa").await.unwrap();
        let err = parse_quantity_text("210Zorkels", gpa).await.unwrap_err();
        assert!(err.message.contains("unknown unit"), "unexpected message: {}", err.message);
    }

    #[semio_framework_async_macros::async_test]
    async fn parses_and_prints_angles_in_degrees() {
        let value = parse_angle_text("45").await.expect("parse_angle_text");
        assert!((value - 45.0).abs() < 1e-9);
        assert_eq!(print_angle(value).await, "45°");
    }

    #[semio_framework_async_macros::async_test]
    async fn angle_accepts_radians_and_converts_to_degrees() {
        let value = parse_angle_text("3.14159265358979rad").await.expect("parse_angle_text");
        assert!((value - 180.0).abs() < 1e-6, "got {value}");
    }
}
//#endregion 🔖️Tests
