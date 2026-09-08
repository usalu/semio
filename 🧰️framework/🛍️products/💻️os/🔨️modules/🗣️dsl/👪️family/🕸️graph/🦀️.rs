//! @emoji 🕸️ `dsl_family_graph` — the graph family notation kit: shared statement piece-parsers
//! for graph-shaped app grammars (trinity, dag, flow, sequence, wires, puzzle2d/3d/5d, space,
//! architect, procedural2d/3d). Builds on `dsl_notation`'s edge/arrow literal and adds the one
//! genuinely graph-family-specific convenience that literal isn't responsible for: node **chains**
//! (`v1 -- v2 -- v3 -- v1`), sugar for a run of anonymous, unlabeled, same-directed edges sharing
//! consecutive endpoints.
//!
//! Re-exports `dsl_notation`'s edge types so an app grammar's handcrafted parser only needs to
//! depend on this one family crate, not both.

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

use crate::os_dsl::{lex, Limits, TextError, TokenKind};

//#region 🔖️Chain
/// @emoji ⛓️ A run of nodes joined by uniformly-directed, unlabeled edges: `v1 -- v2 -- v3 -- v1`
/// or `a -> b -> c`. Chains never carry per-edge ids/kinds/properties — an edge that needs any of
/// those breaks the chain and must be written as its own statement (see `expand`/`contract`).
#[derive(Clone, Debug, PartialEq)]
pub struct ChainValue {
    pub nodes: Vec<EdgeNode>,
    pub directed: bool,
}

impl ChainValue {
    /// @emoji 📤️ Lowers a chain into the individual edges it's sugar for — `n-1` edges over `n`
    /// nodes — so the semantic model only ever needs to store plain edges, never chain structure.
    pub async fn expand(&self) -> Vec<EdgeValue> {
        self.nodes.windows(2).map(|pair| EdgeValue { from: pair[0].clone(), link: Some(EdgeLink { directed: self.directed, label: EdgeLabel::default(), to: pair[1].clone() }) }).collect()
    }
}

/// @emoji 📥️ The printer-side inverse of `expand`: contracts a maximal PREFIX of `edges` that
/// shares one direction, carries no labels, and threads consecutive endpoints (each edge's `to`
/// equals the next edge's `from`) into one `ChainValue`, returning how many edges it consumed.
/// Returns `None` if `edges` doesn't even start such a run (the caller should print `edges[0]` as
/// a standalone statement instead and retry `contract` on the remainder).
pub fn contract(edges: &[EdgeValue]) -> Option<(ChainValue, usize)> {
    let first_link = edges.first()?.link.as_ref()?;
    if !first_link.label.is_empty() {
        return None;
    }
    let directed = first_link.directed;
    let mut nodes = vec![edges[0].from.clone(), first_link.to.clone()];
    let mut consumed = 1;
    for edge in &edges[1..] {
        let Some(link) = &edge.link else { break };
        if link.directed != directed || !link.label.is_empty() || edge.from != *nodes.last().unwrap() {
            break;
        }
        nodes.push(link.to.clone());
        consumed += 1;
    }
    if consumed < 2 {
        return None;
    }
    Some((ChainValue { nodes, directed }, consumed))
}

// 🚫️async: E1 pure, passed as a bare fn item into `Iterator::map` in `print_chain` below — see R9
fn node_text(node: &EdgeNode) -> String {
    let mut s = node.id.clone();
    if let Some(kind) = &node.kind {
        s.push(':');
        s.push_str(kind);
    }
    if let Some(port) = &node.port {
        s.push('@');
        s.push_str(port);
    }
    s
}

/// @emoji 🖨️ Canonical printer for one chain: `v1--v2--v3` (directed: `v1->v2->v3`) — matches the
/// existing no-space style of plain unlabeled edges, since a chain of unlabeled edges never hits
/// the label-adjacent dash-fusion issue `crate::os_dsl::notation::print_edge` documents.
pub async fn print_chain(chain: &ChainValue) -> String {
    let joiner = if chain.directed { "->" } else { "--" };
    chain.nodes.iter().map(node_text).collect::<Vec<_>>().join(joiner)
}

/// @emoji 🔌️ Parses one standalone chain literal — at least two nodes, uniformly directed,
/// unlabeled throughout. A single edge (`a->b`, no third node) still parses here and round-trips
/// fine; whether the caller treats a 2-node chain as "just an edge" is a printing-style choice
/// (`contract` naturally returns a 2-node chain for any single unlabeled edge — call sites that
/// want to keep single edges unchained should check `chain.nodes.len() > 2` before using this).
pub async fn parse_chain_text(text: &str) -> Result<ChainValue, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let mut pos = 0usize;
    let mut nodes = Vec::new();
    let mut directed: Option<bool> = None;

    loop {
        let (node, next) = parse_node(&tokens, pos).await?;
        nodes.push(node);
        pos = next;
        match tokens.get(pos).map(|t| t.kind) {
            Some(TokenKind::Arrow) => {
                if directed == Some(false) {
                    return Err(node_error("a chain cannot mix `->` and `--`", &tokens, pos));
                }
                directed = Some(true);
                pos += 1;
            }
            Some(TokenKind::DashArrow) => {
                if directed == Some(true) {
                    return Err(node_error("a chain cannot mix `->` and `--`", &tokens, pos));
                }
                directed = Some(false);
                pos += 1;
            }
            _ => break,
        }
    }
    if pos != tokens.len() {
        return Err(node_error("unexpected trailing content after chain literal", &tokens, pos));
    }
    if nodes.len() < 2 {
        return Err(node_error("a chain literal needs at least one edge (two nodes)", &tokens, pos));
    }
    Ok(ChainValue { nodes, directed: directed.unwrap_or(true) })
}

// 🚫️async: E1 pure, consumed by `Option::ok_or_else` sync closures in `parse_node` below (as well
// as directly, per `O1`, elsewhere in this file) — see R9
fn node_error(message: &str, tokens: &[crate::os_dsl::SpannedToken], pos: usize) -> TextError {
    let span = tokens.get(pos).or_else(|| tokens.last()).map_or(crate::os_dsl::TextSpan::at(1, 1), |t| t.span);
    TextError::new(message.to_string(), span)
}

async fn parse_node(tokens: &[crate::os_dsl::SpannedToken], mut pos: usize) -> Result<(EdgeNode, usize), TextError> {
    let id_token = tokens.get(pos).filter(|t| t.kind == TokenKind::Ident).ok_or_else(|| node_error("expected a node identifier", tokens, pos))?;
    let id = id_token.text.as_str().to_string();
    pos += 1;
    let kind = if tokens.get(pos).map(|t| t.kind) == Some(TokenKind::Colon) {
        pos += 1;
        let text = tokens.get(pos).filter(|t| t.kind == TokenKind::Ident).ok_or_else(|| node_error("expected a kind after `:`", tokens, pos))?.text.as_str().to_string();
        pos += 1;
        Some(text)
    } else {
        None
    };
    let port = if tokens.get(pos).map(|t| t.kind) == Some(TokenKind::At) {
        pos += 1;
        let text = tokens.get(pos).filter(|t| t.kind == TokenKind::Ident).ok_or_else(|| node_error("expected a port after `@`", tokens, pos))?.text.as_str().to_string();
        pos += 1;
        Some(text)
    } else {
        None
    };
    Ok((EdgeNode { id, kind, port }, pos))
}
//#endregion 🔖️Chain

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
