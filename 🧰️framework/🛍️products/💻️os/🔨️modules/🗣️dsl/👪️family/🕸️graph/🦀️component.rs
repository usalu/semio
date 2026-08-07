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
    pub fn expand(&self) -> Vec<EdgeValue> {
        self.nodes
            .windows(2)
            .map(|pair| EdgeValue { from: pair[0].clone(), link: Some(EdgeLink { directed: self.directed, label: EdgeLabel::default(), to: pair[1].clone() }) })
            .collect()
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
pub fn print_chain(chain: &ChainValue) -> String {
    let joiner = if chain.directed { "->" } else { "--" };
    chain.nodes.iter().map(node_text).collect::<Vec<_>>().join(joiner)
}

/// @emoji 🔌️ Parses one standalone chain literal — at least two nodes, uniformly directed,
/// unlabeled throughout. A single edge (`a->b`, no third node) still parses here and round-trips
/// fine; whether the caller treats a 2-node chain as "just an edge" is a printing-style choice
/// (`contract` naturally returns a 2-node chain for any single unlabeled edge — call sites that
/// want to keep single edges unchained should check `chain.nodes.len() > 2` before using this).
pub fn parse_chain_text(text: &str) -> Result<ChainValue, TextError> {
    let limits = Limits::default();
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia() && t.kind != TokenKind::Eof).collect();
    let mut pos = 0usize;
    let mut nodes = Vec::new();
    let mut directed: Option<bool> = None;

    loop {
        let (node, next) = parse_node(&tokens, pos)?;
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

fn node_error(message: &str, tokens: &[crate::os_dsl::SpannedToken], pos: usize) -> TextError {
    let span = tokens.get(pos).or_else(|| tokens.last()).map(|t| t.span).unwrap_or(crate::os_dsl::TextSpan::at(1, 1));
    TextError::new(message.to_string(), span)
}

fn parse_node(tokens: &[crate::os_dsl::SpannedToken], mut pos: usize) -> Result<(EdgeNode, usize), TextError> {
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
mod tests {
    use super::*;

    fn node(id: &str) -> EdgeNode {
        EdgeNode { id: id.to_string(), kind: None, port: None }
    }

    #[test]
    fn parses_a_directed_chain() {
        let chain = parse_chain_text("v1->v2->v3->v1").expect("parse_chain_text");
        assert_eq!(chain.nodes, vec![node("v1"), node("v2"), node("v3"), node("v1")]);
        assert!(chain.directed);
        assert_eq!(print_chain(&chain), "v1->v2->v3->v1");
    }

    #[test]
    fn parses_an_undirected_chain() {
        let chain = parse_chain_text("v1--v2--v3").expect("parse_chain_text");
        assert!(!chain.directed);
        assert_eq!(print_chain(&chain), "v1--v2--v3");
    }

    #[test]
    fn rejects_mixed_direction_chains() {
        let err = parse_chain_text("v1->v2--v3").unwrap_err();
        assert!(err.message.contains("cannot mix"), "unexpected message: {}", err.message);
    }

    #[test]
    fn expand_lowers_a_chain_into_plain_edges() {
        let chain = parse_chain_text("a->b->c").expect("parse_chain_text");
        let edges = chain.expand();
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0], EdgeValue { from: node("a"), link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("b") }) });
        assert_eq!(edges[1], EdgeValue { from: node("b"), link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("c") }) });
    }

    #[test]
    fn contract_reassembles_a_maximal_anonymous_run() {
        let chain = parse_chain_text("a->b->c->d").expect("parse_chain_text");
        let edges = chain.expand();
        let (contracted, consumed) = contract(&edges).expect("contract");
        assert_eq!(consumed, edges.len());
        assert_eq!(contracted, chain);
    }

    /// @emoji ⛓️‍💥️ A single unlabeled edge followed by a labeled one is NOT a 1-edge "chain of
    /// one" — `contract` returns `None` so the caller prints `edges[0]` as a standalone statement
    /// (never through `print_chain`) and retries `contract` from index 1. This matters for a
    /// three-edge run where only the first edge is unlabeled: the run never reaches 2 chainable
    /// edges, so it must fall back to two ordinary edge statements, not one bogus 1-edge chain.
    #[test]
    fn contract_returns_none_when_the_run_never_reaches_two_edges() {
        let mut edges = ChainValue { nodes: vec![node("a"), node("b"), node("c")], directed: true }.expand();
        edges[1].link.as_mut().unwrap().label = EdgeLabel { id: Some("e1".to_string()), kind: None };
        assert_eq!(contract(&edges), None);
    }

    #[test]
    fn contract_returns_none_when_endpoints_dont_thread() {
        let edges = vec![
            EdgeValue { from: node("a"), link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("b") }) },
            EdgeValue { from: node("x"), link: Some(EdgeLink { directed: true, label: EdgeLabel::default(), to: node("y") }) },
        ];
        assert_eq!(contract(&edges), None);
    }

    /// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser
    /// (the spec+conformance role decided for grammar files — this doesn't yet prove the
    /// recognizer accepts every fixture, only that the spec itself is well-formed).
    #[test]
    fn grammar_file_is_syntactically_valid() {
        let source = include_str!("📖️family-graph.grammar.semio");
        let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-graph.grammar must parse");
        assert_eq!(grammar.id, "family-graph");
        assert!(grammar.productions.len() > 6, "family-graph should expose edge/chain/label vocabulary");
    }

    #[test]
    fn round_trip_matrix() {
        let sources = vec!["a->b", "a--b", "a->b->c->d->e", "v1:Vertex@p0->v2:Vertex@p1"];
        for source in sources {
            let chain = parse_chain_text(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
            let printed = print_chain(&chain);
            assert_eq!(printed, source, "canonical print should match already-canonical input for {source:?}");
            let reparsed = parse_chain_text(&printed).unwrap_or_else(|e| panic!("reparse of {printed:?} failed: {e:?}"));
            assert_eq!(reparsed, chain);
        }
    }
}
//#endregion 🔖️Tests
