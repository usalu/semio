//! 🚦️ Stop conditions: Aho-Corasick byte matcher and the token and text stop conditions.

use semio_framework_math_sampling_vocabulary::{FinishReason, TokenId, TokenTextAdapter};
use semio_framework_math_sampling_config::StopTextMode;
use semio_framework_math_sampling_workspace::{StepView, StopMark};
use super::{StopCondition, StopConditions, StopPoll};

// #region 🔖️Stops
#[derive(Clone, Default)]
struct AcNode {
    children: std::collections::HashMap<u8, u32>,
    fail: u32,
    depth: usize,
    pattern_end: Option<usize>,
    longest_suffix_pattern: Option<usize>,
}

/// 🛑️ Flat Aho-Corasick automaton over byte patterns, built once per [`StopSpec`] and shared
/// (read-only) across a sequence's forks via `Rc`.
pub struct AhoCorasick {
    nodes: Vec<AcNode>,
    pattern_lens: Vec<usize>,
}

impl AhoCorasick {
    pub async fn build(patterns: &[Vec<u8>]) -> Self {
        let mut nodes = vec![AcNode::default()];
        for (pi, pattern) in patterns.iter().enumerate() {
            let mut state = 0u32;
            for &byte in pattern {
                state = match nodes[state as usize].children.get(&byte) {
                    Some(&next) => next,
                    None => {
                        let parent_depth = nodes[state as usize].depth;
                        nodes.push(AcNode { depth: parent_depth + 1, ..AcNode::default() });
                        let next = (nodes.len() - 1) as u32;
                        nodes[state as usize].children.insert(byte, next);
                        next
                    }
                };
            }
            nodes[state as usize].pattern_end = Some(pi);
        }
        let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();
        for &child in nodes[0].children.values() {
            queue.push_back(child);
        }
        while let Some(u) = queue.pop_front() {
            let children: Vec<(u8, u32)> = nodes[u as usize].children.iter().map(|(&b, &c)| (b, c)).collect();
            for (byte, v) in children {
                let mut f = nodes[u as usize].fail;
                while f != 0 && !nodes[f as usize].children.contains_key(&byte) {
                    f = nodes[f as usize].fail;
                }
                let candidate = nodes[f as usize].children.get(&byte).copied().unwrap_or(0);
                let final_fail = if candidate == v { 0 } else { candidate };
                nodes[v as usize].fail = final_fail;
                nodes[v as usize].longest_suffix_pattern = nodes[final_fail as usize].pattern_end.or(nodes[final_fail as usize].longest_suffix_pattern);
                queue.push_back(v);
            }
        }
        let pattern_lens = patterns.iter().map(Vec::len).collect();
        Self { nodes, pattern_lens }
    }

    /// 🛑️ Advances by one byte via the automaton's goto function (following fail links on miss).
    pub(super) async fn step(&self, state: u32, byte: u8) -> u32 {
        let mut s = state;
        loop {
            if let Some(&next) = self.nodes[s as usize].children.get(&byte) {
                return next;
            }
            if s == 0 {
                return 0;
            }
            s = self.nodes[s as usize].fail;
        }
    }

    /// 🛑️ A pattern index and byte length if `state` completes (directly or via a fail-link
    /// suffix) a pattern.
    pub(super) async fn matched_at(&self, state: u32) -> Option<(usize, usize)> {
        let node = &self.nodes[state as usize];
        node.pattern_end.or(node.longest_suffix_pattern).map(|pi| (pi, self.pattern_lens[pi]))
    }

    /// 🛑️ Bytes of unresolved trailing context at `state` — the "hold back" count.
    pub(super) async fn depth(&self, state: u32) -> usize {
        self.nodes[state as usize].depth
    }
}

/// 🛑️ Token-level stop: fires on any token in a fixed set (e.g. explicit stop tokens beyond EOS).
pub struct TokenStopCondition {
    pub tokens: Vec<TokenId>,
}

impl StopCondition for TokenStopCondition {
    async fn name(&self) -> &'static str {
        "token_stop"
    }
    async fn on_token(&mut self, _view: &StepView<'_>, token: TokenId) -> StopPoll {
        if self.tokens.contains(&token) {
            StopPoll::Finished { reason: FinishReason::StopToken, matched_bytes: 0 }
        } else {
            StopPoll::Continue
        }
    }
    async fn save(&mut self) -> StopMark {
        StopMark(0)
    }
    async fn rollback_to(&mut self, _mark: StopMark) {}
    async fn reset(&mut self) {}
    async fn fork(&self) -> StopConditions {
        (Self { tokens: self.tokens.clone() }).into()
    }
}

/// 🛑️ Text-sequence stop: feeds each token's byte representation (via the [`TokenTextAdapter`] in
/// [`StepView::adapter`]) through an [`AhoCorasick`] automaton; a no-operation (never matches) when no
/// adapter is supplied, since stop text can't be evaluated without token→byte mapping.
pub struct TextStopCondition {
    ac: std::rc::Rc<AhoCorasick>,
    #[allow(dead_code)]
    mode: StopTextMode,
    state: u32,
    snapshots: Vec<u32>,
}

impl TextStopCondition {
    pub async fn new(sequences: &[Vec<u8>], mode: StopTextMode) -> Self {
        Self { ac: std::rc::Rc::new(AhoCorasick::build(sequences).await), mode, state: 0, snapshots: Vec::new() }
    }
}

impl StopCondition for TextStopCondition {
    async fn name(&self) -> &'static str {
        "text_stop"
    }
    async fn on_token(&mut self, view: &StepView<'_>, token: TokenId) -> StopPoll {
        let Some(adapter) = view.adapter else { return StopPoll::Continue };
        let Some(bytes) = adapter.token_bytes(token).await else { return StopPoll::Continue };
        for &byte in bytes {
            self.state = self.ac.step(self.state, byte).await;
            if let Some((pattern_index, pattern_len)) = self.ac.matched_at(self.state).await {
                return StopPoll::Finished { reason: FinishReason::StopSequence { index: pattern_index }, matched_bytes: pattern_len };
            }
        }
        let hold = self.ac.depth(self.state).await;
        if hold > 0 {
            StopPoll::Hold { ambiguous_bytes: hold }
        } else {
            StopPoll::Continue
        }
    }
    async fn save(&mut self) -> StopMark {
        self.snapshots.push(self.state);
        StopMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: StopMark) {
        if let Some(&state) = self.snapshots.get(mark.0 as usize) {
            self.state = state;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.state = 0;
        self.snapshots.clear();
    }
    async fn fork(&self) -> StopConditions {
        (Self { ac: self.ac.clone(), mode: self.mode, state: self.state, snapshots: self.snapshots.clone() }).into()
    }
}

// #endregion 🔖️Stops
