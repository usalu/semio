//! 🚧️ Token-sampling constraints and stop conditions: stop matching, regex automata, trie, JSON mode, EBNF and JSON-schema constraints.

#![allow(async_fn_in_trait)]

use semio_framework_dispatch_macros::{dyn_enum, dyn_enum_close};
use semio_framework_math_sampling_vocabulary::{FinishReason, JsonValue, SamplingError, SamplingLimits, TokenBitset, TokenId, TokenTextAdapter, TokenTextAdapters, parse_json};
use semio_framework_math_sampling_config::{ConstraintSpec, StopTextMode};
use semio_framework_math_sampling_workspace::{ConstraintMark, StepView, StopMark};

/// 🔌️ A structural/lexical constraint on the next token (regex, grammar, JSON mode, ...).
#[dyn_enum]
pub trait Constraint {
    async fn name(&self) -> &'static str;
    /// 🧱️ ANDs the set of currently-valid tokens into `mask` (starts all-ones for the first
    /// constraint in a composition).
    async fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError>;
    async fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError>;
    async fn is_satisfied(&self) -> bool;
    async fn is_finished(&self) -> bool;
    async fn is_dead(&self) -> bool;
    async fn save(&mut self) -> ConstraintMark;
    async fn rollback_to(&mut self, mark: ConstraintMark);
    async fn reset(&mut self);
    async fn fork(&self) -> Constraints;
}

/// 🛑️ Result of feeding one token's bytes to a [`StopCondition`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StopPoll {
    Continue,
    Hold { ambiguous_bytes: usize },
    Finished { reason: FinishReason, matched_bytes: usize },
}

#[dyn_enum]
pub trait StopCondition {
    async fn name(&self) -> &'static str;
    async fn on_token(&mut self, view: &StepView<'_>, token: TokenId) -> StopPoll;
    async fn save(&mut self) -> StopMark;
    async fn rollback_to(&mut self, mark: StopMark);
    async fn reset(&mut self);
    async fn fork(&self) -> StopConditions;
}

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
    async fn step(&self, state: u32, byte: u8) -> u32 {
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
    async fn matched_at(&self, state: u32) -> Option<(usize, usize)> {
        let node = &self.nodes[state as usize];
        node.pattern_end.or(node.longest_suffix_pattern).map(|pi| (pi, self.pattern_lens[pi]))
    }

    /// 🛑️ Bytes of unresolved trailing context at `state` — the "hold back" count.
    async fn depth(&self, state: u32) -> usize {
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

dyn_enum_close! {
    pub enum StopConditions: StopCondition {
        Token(TokenStopCondition),
        Text(TextStopCondition),
    }
}
// #endregion 🔖️Stops

// #region 🔖️Automata
/// 🤖️ Parsed regex AST. Byte-level throughout (classes/literals match raw bytes, so multi-byte
/// UTF-8 sequences work as literal byte runs but aren't given first-class Unicode class support).
#[derive(Clone, PartialEq, Debug)]
enum RegexNode {
    Literal(u8),
    AnyByte,
    Class { ranges: Vec<(u8, u8)>, negate: bool },
    Concat(Vec<RegexNode>),
    Alt(Vec<RegexNode>),
    Star(Box<RegexNode>),
    Plus(Box<RegexNode>),
    Opt(Box<RegexNode>),
    Repeat(Box<RegexNode>, usize, Option<usize>),
}

async fn unescape_byte(b: u8) -> u8 {
    match b {
        b'n' => b'\n',
        b't' => b'\t',
        b'r' => b'\r',
        other => other,
    }
}

/// 🤖️ Recursive-descent regex parser: alternation (lowest) → concatenation → postfix quantifiers
/// (`* + ? {m,n}`) → atoms (literals, ``, `[...]` classes, `(...)` groups, `\`-escapes). No
/// backtracking at parse OR match time — this only builds the AST; matching happens via the NFA
/// → DFA pipeline below.
async fn parse_regex(pattern: &str) -> Result<RegexNode, SamplingError> {
    let bytes = pattern.as_bytes();
    let mut pos = 0usize;
    let node = parse_alt(bytes, &mut pos).await?;
    if pos != bytes.len() {
        return Err(SamplingError::RegexParse { offset: pos, reason: "unexpected trailing characters" });
    }
    Ok(node)
}

async fn parse_alt(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    let mut branches = vec![parse_concat(bytes, pos).await?];
    while bytes.get(*pos) == Some(&b'|') {
        *pos += 1;
        branches.push(parse_concat(bytes, pos).await?);
    }
    Ok(if branches.len() == 1 { branches.pop().expect("non-empty branches") } else { RegexNode::Alt(branches) })
}

async fn parse_concat(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    let mut parts = Vec::new();
    while let Some(&b) = bytes.get(*pos) {
        if b == b'|' || b == b')' {
            break;
        }
        parts.push(parse_quantified(bytes, pos).await?);
    }
    Ok(RegexNode::Concat(parts))
}

async fn parse_quantified(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    let atom = parse_atom(bytes, pos).await?;
    match bytes.get(*pos) {
        Some(b'*') => {
            *pos += 1;
            Ok(RegexNode::Star(Box::new(atom)))
        }
        Some(b'+') => {
            *pos += 1;
            Ok(RegexNode::Plus(Box::new(atom)))
        }
        Some(b'?') => {
            *pos += 1;
            Ok(RegexNode::Opt(Box::new(atom)))
        }
        Some(b'{') => {
            *pos += 1;
            let (min, max) = parse_repeat_bounds(bytes, pos).await?;
            Ok(RegexNode::Repeat(Box::new(atom), min, max))
        }
        _ => Ok(atom),
    }
}

async fn parse_repeat_bounds(bytes: &[u8], pos: &mut usize) -> Result<(usize, Option<usize>), SamplingError> {
    let parse_uint = |bytes: &[u8], pos: &mut usize| -> Result<usize, SamplingError> {
        let start = *pos;
        while bytes.get(*pos).is_some_and(u8::is_ascii_digit) {
            *pos += 1;
        }
        core::str::from_utf8(&bytes[start..*pos]).ok().and_then(|s| s.parse().ok()).ok_or(SamplingError::RegexParse { offset: start, reason: "invalid repeat bound" })
    };
    let min = parse_uint(bytes, pos)?;
    let max = if bytes.get(*pos) == Some(&b',') {
        *pos += 1;
        if bytes.get(*pos) == Some(&b'}') {
            None
        } else {
            Some(parse_uint(bytes, pos)?)
        }
    } else {
        Some(min)
    };
    if bytes.get(*pos) != Some(&b'}') {
        return Err(SamplingError::RegexParse { offset: *pos, reason: "expected '}'" });
    }
    *pos += 1;
    Ok((min, max))
}

async fn parse_atom(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    match bytes.get(*pos) {
        Some(b'(') => {
            *pos += 1;
            // 🔀️ `Box::pin(..)` breaks the `parse_atom` → `parse_alt` recursion cycle (E0733) —
            // see the matching note in `parse_json_array` (O1 de-dyn, math-dedyn).
            let inner = Box::pin(parse_alt(bytes, pos)).await?;
            if bytes.get(*pos) != Some(&b')') {
                return Err(SamplingError::RegexParse { offset: *pos, reason: "expected ')'" });
            }
            *pos += 1;
            Ok(inner)
        }
        Some(b'.') => {
            *pos += 1;
            Ok(RegexNode::AnyByte)
        }
        Some(b'[') => parse_class(bytes, pos).await,
        Some(b'\\') => {
            *pos += 1;
            let &b = bytes.get(*pos).ok_or(SamplingError::RegexParse { offset: *pos, reason: "dangling escape" })?;
            *pos += 1;
            Ok(RegexNode::Literal(unescape_byte(b).await))
        }
        Some(&b) => {
            *pos += 1;
            Ok(RegexNode::Literal(b))
        }
        None => Err(SamplingError::RegexParse { offset: *pos, reason: "unexpected end of pattern" }),
    }
}

async fn parse_class(bytes: &[u8], pos: &mut usize) -> Result<RegexNode, SamplingError> {
    debug_assert_eq!(bytes[*pos], b'[');
    *pos += 1;
    let negate = bytes.get(*pos) == Some(&b'^');
    if negate {
        *pos += 1;
    }
    let mut ranges = Vec::new();
    while let Some(&b) = bytes.get(*pos) {
        if b == b']' {
            break;
        }
        let lo = if b == b'\\' {
            *pos += 1;
            let e = *bytes.get(*pos).ok_or(SamplingError::RegexParse { offset: *pos, reason: "dangling escape in class" })?;
            unescape_byte(e).await
        } else {
            b
        };
        *pos += 1;
        if bytes.get(*pos) == Some(&b'-') && bytes.get(*pos + 1) != Some(&b']') {
            *pos += 1;
            let &hi = bytes.get(*pos).ok_or(SamplingError::RegexParse { offset: *pos, reason: "dangling range in class" })?;
            *pos += 1;
            ranges.push((lo, hi));
        } else {
            ranges.push((lo, lo));
        }
    }
    if bytes.get(*pos) != Some(&b']') {
        return Err(SamplingError::RegexParse { offset: *pos, reason: "expected ']'" });
    }
    *pos += 1;
    Ok(RegexNode::Class { ranges, negate })
}

struct NfaNode {
    eps: Vec<usize>,
    byte_ranges: Vec<((u8, u8), usize)>,
    accept: bool,
}

struct Frag {
    start: usize,
    accept: usize,
}

struct NfaBuilder {
    nodes: Vec<NfaNode>,
}

impl NfaBuilder {
    async fn new_state(&mut self) -> usize {
        self.nodes.push(NfaNode { eps: Vec::new(), byte_ranges: Vec::new(), accept: false });
        self.nodes.len() - 1
    }

    async fn build(&mut self, node: &RegexNode, limits: &SamplingLimits) -> Result<Frag, SamplingError> {
        if self.nodes.len() > limits.max_automaton_states {
            return Err(SamplingError::AutomatonBudget { budget: "max_automaton_states" });
        }
        match node {
            RegexNode::Literal(b) => {
                let s = self.new_state().await;
                let a = self.new_state().await;
                self.nodes[s].byte_ranges.push(((*b, *b), a));
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::AnyByte => {
                let s = self.new_state().await;
                let a = self.new_state().await;
                self.nodes[s].byte_ranges.push(((0, 255), a));
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Class { ranges, negate } => {
                let s = self.new_state().await;
                let a = self.new_state().await;
                if *negate {
                    let mut covered = [false; 256];
                    for &(lo, hi) in ranges {
                        for b in lo..=hi {
                            covered[b as usize] = true;
                        }
                    }
                    let mut b = 0u16;
                    while b <= 255 {
                        if !covered[b as usize] {
                            let start = b;
                            while b <= 255 && !covered[b as usize] {
                                b += 1;
                            }
                            self.nodes[s].byte_ranges.push(((start as u8, (b - 1) as u8), a));
                        } else {
                            b += 1;
                        }
                    }
                } else {
                    for &(lo, hi) in ranges {
                        self.nodes[s].byte_ranges.push(((lo, hi), a));
                    }
                }
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Concat(parts) => {
                if parts.is_empty() {
                    let s = self.new_state().await;
                    return Ok(Frag { start: s, accept: s });
                }
                let mut frags = Vec::with_capacity(parts.len());
                for p in parts {
                    frags.push(Box::pin(self.build(p, limits)).await?);
                }
                for w in frags.windows(2) {
                    self.nodes[w[0].accept].eps.push(w[1].start);
                }
                Ok(Frag { start: frags[0].start, accept: frags[frags.len() - 1].accept })
            }
            RegexNode::Alt(branches) => {
                let s = self.new_state().await;
                let a = self.new_state().await;
                for b in branches {
                    let f = Box::pin(self.build(b, limits)).await?;
                    self.nodes[s].eps.push(f.start);
                    self.nodes[f.accept].eps.push(a);
                }
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Star(inner) => {
                let s = self.new_state().await;
                let a = self.new_state().await;
                let f = Box::pin(self.build(inner, limits)).await?;
                self.nodes[s].eps.push(f.start);
                self.nodes[s].eps.push(a);
                self.nodes[f.accept].eps.push(f.start);
                self.nodes[f.accept].eps.push(a);
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Plus(inner) => {
                let first = Box::pin(self.build(inner, limits)).await?;
                let star = Box::pin(self.build(&RegexNode::Star(inner.clone()), limits)).await?;
                self.nodes[first.accept].eps.push(star.start);
                Ok(Frag { start: first.start, accept: star.accept })
            }
            RegexNode::Opt(inner) => {
                let s = self.new_state().await;
                let a = self.new_state().await;
                let f = Box::pin(self.build(inner, limits)).await?;
                self.nodes[s].eps.push(f.start);
                self.nodes[s].eps.push(a);
                self.nodes[f.accept].eps.push(a);
                Ok(Frag { start: s, accept: a })
            }
            RegexNode::Repeat(inner, min, max) => {
                let mut parts = Vec::new();
                for _ in 0..*min {
                    parts.push((**inner).clone());
                }
                match max {
                    Some(max) if *max > *min => {
                        for _ in 0..(*max - *min) {
                            parts.push(RegexNode::Opt(inner.clone()));
                        }
                    }
                    None => parts.push(RegexNode::Star(inner.clone())),
                    _ => {}
                }
                Box::pin(self.build(&RegexNode::Concat(parts), limits)).await
            }
        }
    }
}

async fn eps_closure(nodes: &[NfaNode], start: &[usize]) -> Vec<usize> {
    let mut stack: Vec<usize> = start.to_vec();
    let mut seen: std::collections::BTreeSet<usize> = start.iter().copied().collect();
    while let Some(s) = stack.pop() {
        for &e in &nodes[s].eps {
            if seen.insert(e) {
                stack.push(e);
            }
        }
    }
    seen.into_iter().collect()
}

async fn nfa_set_has_accept(nodes: &[NfaNode], states: &[usize]) -> bool {
    states.iter().any(|&s| nodes[s].accept)
}

/// 🤖️ A byte-level DFA over 256-wide dense transition rows. State `0` is a permanent, unreachable,
/// non-accepting dead state (the "no transition" sentinel); the real start state is always `>= 1`
/// once the pattern accepts at least one string. `alive[s]` is precomputed once via reverse
/// reachability from every accept state, so `Constraint::is_dead` is an O(1) lookup rather than a
/// live search.
pub struct Dfa {
    transitions: Vec<u32>,
    accept: Vec<bool>,
    alive: Vec<bool>,
    start: u32,
    num_states: usize,
}

const DFA_DEAD: u32 = 0;

/// 🤖️ Subset construction: an NFA (with accept flags already set) plus a designated start state
/// becomes a byte-level DFA, budget-checked against `limits.max_automaton_states`. Shared by
/// [`Dfa::from_pattern`] and [`EbnfConstraint::new`] (which compiles a grammar into an NFA via a
/// different front end but needs the exact same back end).
async fn subset_construct(nodes: &[NfaNode], start_nfa_state: usize, limits: &SamplingLimits) -> Result<Dfa, SamplingError> {
    let mut dfa_states: Vec<Vec<usize>> = vec![Vec::new()];
    let mut state_index: std::collections::HashMap<Vec<usize>, u32> = std::collections::HashMap::new();
    state_index.insert(Vec::new(), DFA_DEAD);
    let mut accept = vec![false];
    let mut transitions: Vec<u32> = vec![DFA_DEAD; 256];

    let start_set = eps_closure(nodes, &[start_nfa_state]).await;
    let start = if let Some(&id) = state_index.get(&start_set) {
        id
    } else {
        let id = dfa_states.len() as u32;
        dfa_states.push(start_set.clone());
        state_index.insert(start_set.clone(), id);
        accept.push(nfa_set_has_accept(nodes, &start_set).await);
        transitions.resize(dfa_states.len() * 256, DFA_DEAD);
        id
    };

    let mut queue: std::collections::VecDeque<u32> = std::collections::VecDeque::new();
    queue.push_back(start);
    let mut queued: std::collections::HashSet<u32> = std::collections::HashSet::new();
    queued.insert(start);
    while let Some(state_id) = queue.pop_front() {
        if dfa_states.len() > limits.max_automaton_states {
            return Err(SamplingError::AutomatonBudget { budget: "max_automaton_states" });
        }
        let nfa_set = dfa_states[state_id as usize].clone();
        for byte in 0..=255u16 {
            let byte = byte as u8;
            let mut targets: Vec<usize> = Vec::new();
            for &s in &nfa_set {
                for &((lo, hi), to) in &nodes[s].byte_ranges {
                    if byte >= lo && byte <= hi {
                        targets.push(to);
                    }
                }
            }
            if targets.is_empty() {
                continue;
            }
            let closure = eps_closure(nodes, &targets).await;
            if closure.is_empty() {
                continue;
            }
            let next_id = if let Some(&id) = state_index.get(&closure) {
                id
            } else {
                let id = dfa_states.len() as u32;
                dfa_states.push(closure.clone());
                state_index.insert(closure.clone(), id);
                accept.push(nfa_set_has_accept(nodes, &closure).await);
                transitions.resize(dfa_states.len() * 256, DFA_DEAD);
                id
            };
            transitions[state_id as usize * 256 + byte as usize] = next_id;
            if queued.insert(next_id) {
                queue.push_back(next_id);
            }
        }
    }

    let num_states = dfa_states.len();
    let mut incoming: Vec<Vec<u32>> = vec![Vec::new(); num_states];
    for s in 0..num_states {
        for b in 0..256usize {
            let t = transitions[s * 256 + b];
            if t != DFA_DEAD {
                incoming[t as usize].push(s as u32);
            }
        }
    }
    let mut alive = vec![false; num_states];
    let mut stack: Vec<u32> = (0..num_states as u32).filter(|&s| accept[s as usize]).collect();
    for &s in &stack {
        alive[s as usize] = true;
    }
    while let Some(s) = stack.pop() {
        for &p in &incoming[s as usize] {
            if !alive[p as usize] {
                alive[p as usize] = true;
                stack.push(p);
            }
        }
    }

    Ok(Dfa { transitions, accept, alive, start, num_states })
}

impl Dfa {
    /// 🤖️ Parses `pattern`, builds its Thompson NFA, then performs subset construction into a DFA,
    /// erroring if either the NFA or the DFA would exceed `limits.max_automaton_states`.
    pub async fn from_pattern(pattern: &str, limits: &SamplingLimits) -> Result<Self, SamplingError> {
        let ast = parse_regex(pattern).await?;
        let mut builder = NfaBuilder { nodes: Vec::new() };
        let frag = builder.build(&ast, limits).await?;
        builder.nodes[frag.accept].accept = true;
        subset_construct(&builder.nodes, frag.start, limits).await
    }

    pub async fn start(&self) -> u32 {
        self.start
    }

    #[inline]
    pub async fn step(&self, state: u32, byte: u8) -> u32 {
        self.transitions[state as usize * 256 + byte as usize]
    }

    pub async fn is_accept(&self, state: u32) -> bool {
        self.accept.get(state as usize).copied().unwrap_or(false)
    }

    pub async fn is_alive(&self, state: u32) -> bool {
        self.alive.get(state as usize).copied().unwrap_or(false)
    }

    pub async fn is_dead(&self, state: u32) -> bool {
        state == DFA_DEAD
    }

    pub async fn num_states(&self) -> usize {
        self.num_states
    }
}

/// 🤖️ Per-DFA-state lazily computed `(allowed token mask, next state per token)`, so a constraint
/// doesn't re-walk every token's bytes through the DFA on every step at the same automaton state.
/// Bounded by `max_entries`; a full-clear eviction policy (not LRU) once the bound is hit — simple
/// and correct, if not optimal; a proper clock/LRU is a hardening-wave follow-up.
pub struct DfaTokenMemo {
    entries: std::collections::HashMap<u32, (TokenBitset, Vec<u32>)>,
    max_entries: usize,
}

impl DfaTokenMemo {
    pub async fn new(max_entries: usize) -> Self {
        Self { entries: std::collections::HashMap::new(), max_entries }
    }

    pub async fn get_or_compute(&mut self, dfa: &Dfa, state: u32, adapter: &TokenTextAdapters) -> &(TokenBitset, Vec<u32>) {
        if !self.entries.contains_key(&state) {
            if self.entries.len() >= self.max_entries {
                self.entries.clear();
            }
            let vocab_size = adapter.vocab_size().await;
            let mut allowed = TokenBitset::new_empty(vocab_size).await;
            let mut next_state = vec![DFA_DEAD; vocab_size];
            for (i, next_state_i) in next_state.iter_mut().enumerate() {
                let token = TokenId::new(i as u32);
                if let Some(bytes) = adapter.token_bytes(token).await {
                    let mut s = state;
                    let mut ok = true;
                    for &b in bytes {
                        s = dfa.step(s, b).await;
                        if dfa.is_dead(s).await {
                            ok = false;
                            break;
                        }
                    }
                    if ok && dfa.is_alive(s).await {
                        allowed.set(token, true).await;
                        *next_state_i = s;
                    }
                }
            }
            self.entries.insert(state, (allowed, next_state));
        }
        self.entries.get(&state).expect("just inserted or already present")
    }
}
// #endregion 🔖️Automata

// #region 🔖️Constraints
/// 🧱️ Constrains generation to text matching a regex (interpreted as a whole-string match: the
/// final generated text must end in an accept state).
pub struct RegexConstraint {
    dfa: std::rc::Rc<Dfa>,
    cache: DfaTokenMemo,
    max_cache_entries: usize,
    state: u32,
    snapshots: Vec<u32>,
}

impl RegexConstraint {
    pub async fn new(pattern: &str, limits: &SamplingLimits) -> Result<Self, SamplingError> {
        let dfa = Dfa::from_pattern(pattern, limits).await?;
        let start = dfa.start().await;
        Ok(Self { dfa: std::rc::Rc::new(dfa), cache: DfaTokenMemo::new(limits.max_dfa_cache_entries).await, max_cache_entries: limits.max_dfa_cache_entries, state: start, snapshots: Vec::new() })
    }
}

impl Constraint for RegexConstraint {
    async fn name(&self) -> &'static str {
        "regex"
    }
    async fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        let Some(adapter) = view.adapter else { return Ok(()) };
        let (allowed, _) = self.cache.get_or_compute(&self.dfa, self.state, adapter).await;
        mask.and_with(allowed).await;
        Ok(())
    }
    async fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        let Some(adapter) = view.adapter else { return Ok(()) };
        let (_, next_state) = self.cache.get_or_compute(&self.dfa, self.state, adapter).await;
        if let Some(&next) = next_state.get(token.get() as usize) {
            self.state = next;
        }
        Ok(())
    }
    async fn is_satisfied(&self) -> bool {
        self.dfa.is_accept(self.state).await
    }
    async fn is_finished(&self) -> bool {
        self.is_satisfied().await
    }
    async fn is_dead(&self) -> bool {
        self.dfa.is_dead(self.state).await || !self.dfa.is_alive(self.state).await
    }
    async fn save(&mut self) -> ConstraintMark {
        self.snapshots.push(self.state);
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some(&s) = self.snapshots.get(mark.0 as usize) {
            self.state = s;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.state = self.dfa.start().await;
        self.snapshots.clear();
    }
    async fn fork(&self) -> Constraints {
        (Self { dfa: self.dfa.clone(), cache: DfaTokenMemo::new(self.max_cache_entries).await, max_cache_entries: self.max_cache_entries, state: self.state, snapshots: self.snapshots.clone() }).into()
    }
}

/// 🧱️ Constrains generation to one of a fixed set of allowed token-id phrases (a trie over
/// `TokenId` sequences rather than bytes).
pub struct TrieConstraint {
    nodes: Vec<std::collections::HashMap<TokenId, usize>>,
    accept: Vec<bool>,
    state: usize,
    snapshots: Vec<usize>,
}

impl TrieConstraint {
    pub async fn new(phrases: &[Vec<TokenId>]) -> Self {
        let mut nodes = vec![std::collections::HashMap::new()];
        let mut accept = vec![false];
        for phrase in phrases {
            let mut state = 0usize;
            for &token in phrase {
                if !nodes[state].contains_key(&token) {
                    nodes.push(std::collections::HashMap::new());
                    accept.push(false);
                    let new_id = nodes.len() - 1;
                    nodes[state].insert(token, new_id);
                }
                state = nodes[state][&token];
            }
            accept[state] = true;
        }
        Self { nodes, accept, state: 0, snapshots: Vec::new() }
    }
}

impl Constraint for TrieConstraint {
    async fn name(&self) -> &'static str {
        "trie"
    }
    async fn fill_mask(&mut self, _view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        let mut allow = TokenBitset::new_empty(mask.len().await).await;
        for &token in self.nodes[self.state].keys() {
            allow.set(token, true).await;
        }
        mask.and_with(&allow).await;
        Ok(())
    }
    async fn accept(&mut self, _view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        if let Some(&next) = self.nodes[self.state].get(&token) {
            self.state = next;
        }
        Ok(())
    }
    async fn is_satisfied(&self) -> bool {
        self.accept[self.state]
    }
    async fn is_finished(&self) -> bool {
        self.accept[self.state] && self.nodes[self.state].is_empty()
    }
    async fn is_dead(&self) -> bool {
        self.nodes[self.state].is_empty() && !self.accept[self.state]
    }
    async fn save(&mut self) -> ConstraintMark {
        self.snapshots.push(self.state);
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some(&s) = self.snapshots.get(mark.0 as usize) {
            self.state = s;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.state = 0;
        self.snapshots.clear();
    }
    async fn fork(&self) -> Constraints {
        (Self { nodes: self.nodes.clone(), accept: self.accept.clone(), state: self.state, snapshots: self.snapshots.clone() }).into()
    }
}

/// 🧱️ Requires at least one of several token-sequence alternatives to appear somewhere in the
/// generated continuation; does not restrict candidates, only tracks completion (EOS/finish are
/// gated on [`Constraint::is_satisfied`] by the engine's constraint composition).
pub struct MustIncludeConstraint {
    alternatives: Vec<Vec<TokenId>>,
    satisfied: bool,
    history: Vec<TokenId>,
    snapshots: Vec<(bool, usize)>,
}

impl MustIncludeConstraint {
    pub async fn new(alternatives: Vec<Vec<TokenId>>) -> Self {
        Self { alternatives, satisfied: false, history: Vec::new(), snapshots: Vec::new() }
    }
}

impl Constraint for MustIncludeConstraint {
    async fn name(&self) -> &'static str {
        "must_include"
    }
    async fn fill_mask(&mut self, _view: &StepView<'_>, _mask: &mut TokenBitset) -> Result<(), SamplingError> {
        Ok(())
    }
    async fn accept(&mut self, _view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        self.history.push(token);
        if !self.satisfied {
            for phrase in &self.alternatives {
                if self.history.len() >= phrase.len() && self.history[self.history.len() - phrase.len()..] == phrase[..] {
                    self.satisfied = true;
                    break;
                }
            }
        }
        Ok(())
    }
    async fn is_satisfied(&self) -> bool {
        self.satisfied || self.alternatives.is_empty()
    }
    async fn is_finished(&self) -> bool {
        self.satisfied
    }
    async fn is_dead(&self) -> bool {
        false
    }
    async fn save(&mut self) -> ConstraintMark {
        self.snapshots.push((self.satisfied, self.history.len()));
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some(&(satisfied, len)) = self.snapshots.get(mark.0 as usize) {
            self.satisfied = satisfied;
            self.history.truncate(len);
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.satisfied = false;
        self.history.clear();
        self.snapshots.clear();
    }
    async fn fork(&self) -> Constraints {
        (Self { alternatives: self.alternatives.clone(), satisfied: self.satisfied, history: self.history.clone(), snapshots: self.snapshots.clone() }).into()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum JsonFrame {
    Object,
    Array,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum JsonExpect {
    Value,
    ObjectKeyOrClose,
    ObjectKey,
    ObjectColon,
    ObjectCommaOrClose,
    ArrayValueOrClose,
    ArrayCommaOrClose,
    Done,
}

/// 🧱️ Byte-level incremental JSON structural constraint: a small explicit push-down automaton
/// (object/array frame stack + an "expected next token class" state) that validates JSON syntax
/// reactively as token bytes are accepted. Numbers, booleans, and `null` are treated as atomic
/// (their internal digits/characters aren't byte-validated) — a deliberate simplification, not a
/// full JSON-number grammar. Does not proactively mask (`fill_mask` is a no-operation): building a
/// per-state token-feasibility cache for this hand-written automaton (mirroring [`DfaTokenMemo`])
/// is a hardening-wave follow-up.
pub struct JsonModeConstraint {
    stack: Vec<JsonFrame>,
    expect: JsonExpect,
    in_string: bool,
    string_escaped: bool,
    string_is_key: bool,
    dead: bool,
    snapshots: Vec<(Vec<JsonFrame>, JsonExpect, bool, bool, bool, bool)>,
}

impl JsonModeConstraint {
    pub async fn new() -> Self {
        Self { stack: Vec::new(), expect: JsonExpect::Value, in_string: false, string_escaped: false, string_is_key: false, dead: false, snapshots: Vec::new() }
    }

    async fn after_value(&mut self) {
        self.expect = match self.stack.last() {
            None => JsonExpect::Done,
            Some(JsonFrame::Object) => JsonExpect::ObjectCommaOrClose,
            Some(JsonFrame::Array) => JsonExpect::ArrayCommaOrClose,
        };
    }

    /// 🧱️ Consumes one byte, returning `false` if it is structurally invalid in the current state.
    async fn feed_byte(&mut self, b: u8) -> bool {
        if self.in_string {
            if self.string_escaped {
                self.string_escaped = false;
                return true;
            }
            match b {
                b'\\' => {
                    self.string_escaped = true;
                    true
                }
                b'"' => {
                    self.in_string = false;
                    if self.string_is_key {
                        self.expect = JsonExpect::ObjectColon;
                    } else {
                        self.after_value().await;
                    }
                    true
                }
                _ => true,
            }
        } else {
            match b {
                b' ' | b'\t' | b'\n' | b'\r' => true,
                b'"' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.in_string = true;
                    self.string_is_key = false;
                    true
                }
                b'"' if matches!(self.expect, JsonExpect::ObjectKey | JsonExpect::ObjectKeyOrClose) => {
                    self.in_string = true;
                    self.string_is_key = true;
                    true
                }
                b'{' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.stack.push(JsonFrame::Object);
                    self.expect = JsonExpect::ObjectKeyOrClose;
                    true
                }
                b'[' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.stack.push(JsonFrame::Array);
                    self.expect = JsonExpect::ArrayValueOrClose;
                    true
                }
                b'}' if matches!(self.expect, JsonExpect::ObjectKeyOrClose | JsonExpect::ObjectCommaOrClose) && self.stack.last() == Some(&JsonFrame::Object) => {
                    self.stack.pop();
                    self.after_value().await;
                    true
                }
                b']' if matches!(self.expect, JsonExpect::ArrayValueOrClose | JsonExpect::ArrayCommaOrClose) && self.stack.last() == Some(&JsonFrame::Array) => {
                    self.stack.pop();
                    self.after_value().await;
                    true
                }
                b':' if self.expect == JsonExpect::ObjectColon => {
                    self.expect = JsonExpect::Value;
                    true
                }
                b',' if self.expect == JsonExpect::ObjectCommaOrClose => {
                    self.expect = JsonExpect::ObjectKey;
                    true
                }
                b',' if self.expect == JsonExpect::ArrayCommaOrClose => {
                    self.expect = JsonExpect::ArrayValueOrClose;
                    true
                }
                b't' | b'f' | b'n' | b'-' | b'0'..=b'9' if matches!(self.expect, JsonExpect::Value | JsonExpect::ArrayValueOrClose) => {
                    self.after_value().await;
                    true
                }
                _ => false,
            }
        }
    }
}

impl Default for JsonModeConstraint {
    // 🚫️async: E1 impl of an externally-declared trait (`Default`) — inlines `new()`'s trivial
    // body directly, same as `SamplingConfigBuilder`'s (O1 de-dyn, math-dedyn).
    fn default() -> Self {
        Self { stack: Vec::new(), expect: JsonExpect::Value, in_string: false, string_escaped: false, string_is_key: false, dead: false, snapshots: Vec::new() }
    }
}

impl Constraint for JsonModeConstraint {
    async fn name(&self) -> &'static str {
        "json_mode"
    }
    async fn fill_mask(&mut self, _view: &StepView<'_>, _mask: &mut TokenBitset) -> Result<(), SamplingError> {
        Ok(())
    }
    async fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        let Some(adapter) = view.adapter else { return Ok(()) };
        let Some(bytes) = adapter.token_bytes(token).await else { return Ok(()) };
        for &b in bytes {
            if !self.feed_byte(b).await {
                self.dead = true;
                break;
            }
        }
        Ok(())
    }
    async fn is_satisfied(&self) -> bool {
        self.expect == JsonExpect::Done && !self.in_string
    }
    async fn is_finished(&self) -> bool {
        self.is_satisfied().await
    }
    async fn is_dead(&self) -> bool {
        self.dead
    }
    async fn save(&mut self) -> ConstraintMark {
        self.snapshots.push((self.stack.clone(), self.expect, self.in_string, self.string_escaped, self.string_is_key, self.dead));
        ConstraintMark((self.snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: ConstraintMark) {
        if let Some((stack, expect, in_string, string_escaped, string_is_key, dead)) = self.snapshots.get(mark.0 as usize).cloned() {
            self.stack = stack;
            self.expect = expect;
            self.in_string = in_string;
            self.string_escaped = string_escaped;
            self.string_is_key = string_is_key;
            self.dead = dead;
        }
        self.snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        *self = Self::new().await;
    }
    async fn fork(&self) -> Constraints {
        (Self { stack: self.stack.clone(), expect: self.expect, in_string: self.in_string, string_escaped: self.string_escaped, string_is_key: self.string_is_key, dead: self.dead, snapshots: self.snapshots.clone() }).into()
    }
}

/// 🧱️ EBNF grammar rules: `name ::= expr ;` where `expr` reuses [`RegexNode`]'s alternation,
/// concatenation, and quantifier operators over quoted terminals and rule references. Compiled by
/// inlining rule references into a single [`RegexNode`] (bounded by `max_expansions`), so it
/// supports non-recursive and boundedly-recursive grammars — not general (potentially
/// left-recursive or unbounded) context-free grammars, which would need a real Earley/GLR parser.
/// That is a deliberate, documented scope cut: [`ConstraintSpec::Ebnf`] compiles through this path
/// and therefore shares the same limitation.
struct EbnfGrammar {
    rules: std::collections::HashMap<String, GrammarExpr>,
    start: String,
}

#[derive(Clone)]
enum GrammarExpr {
    Terminal(String),
    Rule(String),
    Concat(Vec<GrammarExpr>),
    Alt(Vec<GrammarExpr>),
    Star(Box<GrammarExpr>),
    Plus(Box<GrammarExpr>),
    Opt(Box<GrammarExpr>),
}

async fn parse_ebnf(text: &str) -> Result<EbnfGrammar, SamplingError> {
    let mut rules = std::collections::HashMap::new();
    let mut start = None;
    for (line_no, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(sep) = line.find("::=") else {
            return Err(SamplingError::GrammarParse { offset: line_no, reason: "expected '::=' rule separator" });
        };
        let name = line[..sep].trim().to_string();
        let body = line[sep + 3..].trim().trim_end_matches(';').trim();
        let expr = parse_grammar_alt(body.as_bytes(), &mut 0).await?;
        if start.is_none() {
            start = Some(name.clone());
        }
        rules.insert(name, expr);
    }
    let start = start.ok_or(SamplingError::GrammarParse { offset: 0, reason: "grammar defines no rules" })?;
    Ok(EbnfGrammar { rules, start })
}

async fn parse_grammar_alt(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    skip_grammar_ws(bytes, pos).await;
    let mut branches = vec![parse_grammar_concat(bytes, pos).await?];
    loop {
        skip_grammar_ws(bytes, pos).await;
        if bytes.get(*pos) == Some(&b'|') {
            *pos += 1;
            branches.push(parse_grammar_concat(bytes, pos).await?);
        } else {
            break;
        }
    }
    Ok(if branches.len() == 1 { branches.pop().expect("non-empty branches") } else { GrammarExpr::Alt(branches) })
}

async fn parse_grammar_concat(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    let mut parts = Vec::new();
    loop {
        skip_grammar_ws(bytes, pos).await;
        match bytes.get(*pos) {
            Some(b'|') | Some(b')') | None => break,
            _ => parts.push(parse_grammar_quantified(bytes, pos).await?),
        }
    }
    Ok(GrammarExpr::Concat(parts))
}

async fn parse_grammar_quantified(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    let atom = parse_grammar_atom(bytes, pos).await?;
    match bytes.get(*pos) {
        Some(b'*') => {
            *pos += 1;
            Ok(GrammarExpr::Star(Box::new(atom)))
        }
        Some(b'+') => {
            *pos += 1;
            Ok(GrammarExpr::Plus(Box::new(atom)))
        }
        Some(b'?') => {
            *pos += 1;
            Ok(GrammarExpr::Opt(Box::new(atom)))
        }
        _ => Ok(atom),
    }
}

async fn skip_grammar_ws(bytes: &[u8], pos: &mut usize) {
    while bytes.get(*pos).is_some_and(u8::is_ascii_whitespace) {
        *pos += 1;
    }
}

async fn parse_grammar_atom(bytes: &[u8], pos: &mut usize) -> Result<GrammarExpr, SamplingError> {
    skip_grammar_ws(bytes, pos).await;
    match bytes.get(*pos) {
        Some(b'(') => {
            *pos += 1;
            // 🔀️ `Box::pin(..)` breaks the `parse_grammar_atom` → `parse_grammar_alt` recursion
            // cycle (E0733) — see the matching note in `parse_json_array` (O1 de-dyn, math-dedyn).
            let inner = Box::pin(parse_grammar_alt(bytes, pos)).await?;
            skip_grammar_ws(bytes, pos).await;
            if bytes.get(*pos) != Some(&b')') {
                return Err(SamplingError::GrammarParse { offset: *pos, reason: "expected ')'" });
            }
            *pos += 1;
            Ok(inner)
        }
        Some(b'"') => {
            *pos += 1;
            let start = *pos;
            while bytes.get(*pos).is_some_and(|&b| b != b'"') {
                *pos += 1;
            }
            let text = core::str::from_utf8(&bytes[start..*pos]).map_err(|_| SamplingError::GrammarParse { offset: start, reason: "invalid utf-8 in terminal" })?.to_string();
            if bytes.get(*pos) != Some(&b'"') {
                return Err(SamplingError::GrammarParse { offset: *pos, reason: "unterminated terminal" });
            }
            *pos += 1;
            Ok(GrammarExpr::Terminal(text))
        }
        Some(&b) if b.is_ascii_alphabetic() || b == b'_' => {
            let start = *pos;
            while bytes.get(*pos).is_some_and(|&b| b.is_ascii_alphanumeric() || b == b'_') {
                *pos += 1;
            }
            let name = core::str::from_utf8(&bytes[start..*pos]).expect("ASCII rule name").to_string();
            Ok(GrammarExpr::Rule(name))
        }
        _ => Err(SamplingError::GrammarParse { offset: *pos, reason: "expected '(', '\"', or a rule name" }),
    }
}

/// 🧱️ Inlines `expr` into a [`RegexNode`], expanding rule references by substitution up to
/// `budget` total expansions (shared, decremented across the whole compilation) — the bound that
/// turns unsupported unbounded recursion into a clean [`SamplingError::AutomatonBudget`] instead
/// of an infinite expansion.
async fn compile_grammar_expr(expr: &GrammarExpr, grammar: &EbnfGrammar, budget: &mut usize) -> Result<RegexNode, SamplingError> {
    if *budget == 0 {
        return Err(SamplingError::AutomatonBudget { budget: "grammar expansion (possible unbounded recursion)" });
    }
    *budget -= 1;
    // 🔀️ `Box::pin(..)` on every direct-recursive call (self-recursion needs it at each call
    // site, not just one edge of a cycle — E0733), and sequential `.await` loops in place of
    // `.iter().map(async_fn).collect()` (a sync closure cannot call an `async fn`, and cannot
    // hold `&mut budget` across iterations either) — see `parse_json_array`'s note (O1 de-dyn,
    // math-dedyn).
    match expr {
        GrammarExpr::Terminal(text) => Ok(RegexNode::Concat(text.bytes().map(RegexNode::Literal).collect())),
        GrammarExpr::Rule(name) => {
            let inner = grammar.rules.get(name).ok_or(SamplingError::GrammarParse { offset: 0, reason: "reference to undefined rule" })?;
            Box::pin(compile_grammar_expr(inner, grammar, budget)).await
        }
        GrammarExpr::Concat(parts) => {
            let mut compiled = Vec::with_capacity(parts.len());
            for p in parts {
                compiled.push(Box::pin(compile_grammar_expr(p, grammar, budget)).await?);
            }
            Ok(RegexNode::Concat(compiled))
        }
        GrammarExpr::Alt(branches) => {
            let mut compiled = Vec::with_capacity(branches.len());
            for p in branches {
                compiled.push(Box::pin(compile_grammar_expr(p, grammar, budget)).await?);
            }
            Ok(RegexNode::Alt(compiled))
        }
        GrammarExpr::Star(inner) => Ok(RegexNode::Star(Box::new(Box::pin(compile_grammar_expr(inner, grammar, budget)).await?))),
        GrammarExpr::Plus(inner) => Ok(RegexNode::Plus(Box::new(Box::pin(compile_grammar_expr(inner, grammar, budget)).await?))),
        GrammarExpr::Opt(inner) => Ok(RegexNode::Opt(Box::new(Box::pin(compile_grammar_expr(inner, grammar, budget)).await?))),
    }
}

/// 🧱️ An EBNF-grammar constraint, compiled through [`compile_grammar_expr`] into the same
/// DFA/[`DfaTokenMemo`] machinery as [`RegexConstraint`] — see [`EbnfGrammar`]'s doc for the
/// supported (non-recursive/boundedly-recursive) grammar subset.
pub struct EbnfConstraint(RegexConstraint);

impl EbnfConstraint {
    pub async fn new(grammar_text: &str, limits: &SamplingLimits) -> Result<Self, SamplingError> {
        let grammar = parse_ebnf(grammar_text).await?;
        let start_expr = grammar.rules.get(&grammar.start).ok_or(SamplingError::GrammarParse { offset: 0, reason: "start rule missing" })?.clone();
        let mut budget = limits.max_grammar_bytes;
        let ast = compile_grammar_expr(&start_expr, &grammar, &mut budget).await?;
        let mut builder = NfaBuilder { nodes: Vec::new() };
        let frag = builder.build(&ast, limits).await?;
        builder.nodes[frag.accept].accept = true;
        let dfa = subset_construct(&builder.nodes, frag.start, limits).await?;
        let start = dfa.start().await;
        Ok(Self(RegexConstraint { dfa: std::rc::Rc::new(dfa), cache: DfaTokenMemo::new(limits.max_dfa_cache_entries).await, max_cache_entries: limits.max_dfa_cache_entries, state: start, snapshots: Vec::new() }))
    }
}

impl Constraint for EbnfConstraint {
    async fn name(&self) -> &'static str {
        "ebnf"
    }
    async fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        self.0.fill_mask(view, mask).await
    }
    async fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        self.0.accept(view, token).await
    }
    async fn is_satisfied(&self) -> bool {
        self.0.is_satisfied().await
    }
    async fn is_finished(&self) -> bool {
        self.0.is_finished().await
    }
    async fn is_dead(&self) -> bool {
        self.0.is_dead().await
    }
    async fn save(&mut self) -> ConstraintMark {
        self.0.save().await
    }
    async fn rollback_to(&mut self, mark: ConstraintMark) {
        self.0.rollback_to(mark).await;
    }
    async fn reset(&mut self) {
        self.0.reset().await;
    }
    async fn fork(&self) -> Constraints {
        (Self(RegexConstraint { dfa: self.0.dfa.clone(), cache: DfaTokenMemo::new(self.0.max_cache_entries).await, max_cache_entries: self.0.max_cache_entries, state: self.0.state, snapshots: self.0.snapshots.clone() })).into()
    }
}

/// 🧱️ Recursive JSON-Schema subset validator: `type`, `enum`, numeric `minimum`/`maximum`, string
/// `minLength`/`maxLength`, array `minItems`/`maxItems`/`items`, and object `required`/`properties`.
/// Unrecognized schema keywords (`oneOf`, `pattern`, ...) are silently ignored rather than
/// rejected — a documented subset, not full JSON Schema.
async fn validates_json_schema(value: &JsonValue, schema: &JsonValue) -> bool {
    // 🔀️ `.and_then(JsonValue::as_X)`/`.iter().all(async_fn)` no longer type-check once
    // `JsonValue`'s accessors and this fn itself are `async fn` — see `processor_spec_from_json`'s
    // note (O1 de-dyn, math-dedyn). `Box::pin(validates_json_schema(..))` handles the recursive
    // call inside a loop (an `async fn` calling itself directly has an infinite-size future type).
    async fn opt_str<'a>(schema: &'a JsonValue, key: &'static str) -> Option<&'a str> {
        match schema.get(key).await {
            Some(v) => v.as_str().await,
            None => None,
        }
    }
    async fn opt_f64(schema: &JsonValue, key: &'static str) -> Option<f64> {
        match schema.get(key).await {
            Some(v) => v.as_f64().await,
            None => None,
        }
    }
    async fn opt_array<'a>(schema: &'a JsonValue, key: &'static str) -> Option<&'a [JsonValue]> {
        match schema.get(key).await {
            Some(v) => v.as_array().await,
            None => None,
        }
    }

    if let Some(type_name) = opt_str(schema, "type").await {
        let matches_type = match type_name {
            "object" => matches!(value, JsonValue::Object(_)),
            "array" => matches!(value, JsonValue::Array(_)),
            "string" => matches!(value, JsonValue::Str(_)),
            "number" => matches!(value, JsonValue::Num(_)),
            "integer" => matches!(value, JsonValue::Num(n) if n.fract() == 0.0),
            "boolean" => matches!(value, JsonValue::Bool(_)),
            "null" => matches!(value, JsonValue::Null),
            _ => true,
        };
        if !matches_type {
            return false;
        }
    }
    if let Some(enum_values) = opt_array(schema, "enum").await {
        if !enum_values.contains(value) {
            return false;
        }
    }
    match value {
        JsonValue::Num(n) => {
            if let Some(min) = opt_f64(schema, "minimum").await {
                if *n < min {
                    return false;
                }
            }
            if let Some(max) = opt_f64(schema, "maximum").await {
                if *n > max {
                    return false;
                }
            }
        }
        JsonValue::Str(s) => {
            if let Some(min_len) = opt_f64(schema, "minLength").await {
                if (s.len() as f64) < min_len {
                    return false;
                }
            }
            if let Some(max_len) = opt_f64(schema, "maxLength").await {
                if (s.len() as f64) > max_len {
                    return false;
                }
            }
        }
        JsonValue::Array(items) => {
            if let Some(min_items) = opt_f64(schema, "minItems").await {
                if (items.len() as f64) < min_items {
                    return false;
                }
            }
            if let Some(max_items) = opt_f64(schema, "maxItems").await {
                if (items.len() as f64) > max_items {
                    return false;
                }
            }
            if let Some(item_schema) = schema.get("items").await {
                for item in items {
                    if !Box::pin(validates_json_schema(item, item_schema)).await {
                        return false;
                    }
                }
            }
        }
        JsonValue::Object(entries) => {
            if let Some(required) = opt_array(schema, "required").await {
                for req in required {
                    if let Some(key) = req.as_str().await {
                        if !entries.iter().any(|(k, _)| k == key) {
                            return false;
                        }
                    }
                }
            }
            if let Some(JsonValue::Object(props)) = schema.get("properties").await {
                for (key, val) in entries {
                    if let Some((_, prop_schema)) = props.iter().find(|(k, _)| k == key) {
                        if !Box::pin(validates_json_schema(val, prop_schema)).await {
                            return false;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    true
}

/// 🧱️ JSON syntax (via [`JsonModeConstraint`]) plus a schema check run once the accumulated text
/// is syntactically complete — schema compliance is a completion gate, not a proactive per-token
/// mask (that would need compiling the schema into the DFA machinery, a hardening-wave follow-up).
pub struct JsonSchemaConstraint {
    mode: JsonModeConstraint,
    schema: JsonValue,
    text: Vec<u8>,
    schema_violated: bool,
    text_snapshots: Vec<(Vec<u8>, bool)>,
}

impl JsonSchemaConstraint {
    pub async fn new(schema: JsonValue) -> Self {
        Self { mode: JsonModeConstraint::new().await, schema, text: Vec::new(), schema_violated: false, text_snapshots: Vec::new() }
    }
}

impl Constraint for JsonSchemaConstraint {
    async fn name(&self) -> &'static str {
        "json_schema"
    }
    async fn fill_mask(&mut self, view: &StepView<'_>, mask: &mut TokenBitset) -> Result<(), SamplingError> {
        self.mode.fill_mask(view, mask).await
    }
    async fn accept(&mut self, view: &StepView<'_>, token: TokenId) -> Result<(), SamplingError> {
        if let Some(adapter) = view.adapter {
            if let Some(bytes) = adapter.token_bytes(token).await {
                self.text.extend_from_slice(bytes);
            }
        }
        self.mode.accept(view, token).await?;
        if self.mode.is_satisfied().await {
            let text = String::from_utf8_lossy(&self.text);
            match parse_json(&text, 64).await {
                Ok(value) => {
                    if !validates_json_schema(&value, &self.schema).await {
                        self.schema_violated = true;
                    }
                }
                Err(_) => self.schema_violated = true,
            }
        }
        Ok(())
    }
    async fn is_satisfied(&self) -> bool {
        self.mode.is_satisfied().await && !self.schema_violated
    }
    async fn is_finished(&self) -> bool {
        self.is_satisfied().await
    }
    async fn is_dead(&self) -> bool {
        self.mode.is_dead().await || self.schema_violated
    }
    async fn save(&mut self) -> ConstraintMark {
        let inner_mark = self.mode.save().await;
        self.text_snapshots.push((self.text.clone(), self.schema_violated));
        debug_assert_eq!(inner_mark.0, (self.text_snapshots.len() - 1) as u64, "mode and text snapshot stacks must grow in lockstep");
        ConstraintMark((self.text_snapshots.len() - 1) as u64)
    }
    async fn rollback_to(&mut self, mark: ConstraintMark) {
        self.mode.rollback_to(mark).await;
        if let Some((text, violated)) = self.text_snapshots.get(mark.0 as usize).cloned() {
            self.text = text;
            self.schema_violated = violated;
        }
        self.text_snapshots.truncate(mark.0 as usize + 1);
    }
    async fn reset(&mut self) {
        self.mode.reset().await;
        self.text.clear();
        self.schema_violated = false;
        self.text_snapshots.clear();
    }
    async fn fork(&self) -> Constraints {
        (Self {
            mode: JsonModeConstraint {
                stack: self.mode.stack.clone(),
                expect: self.mode.expect,
                in_string: self.mode.in_string,
                string_escaped: self.mode.string_escaped,
                string_is_key: self.mode.string_is_key,
                dead: self.mode.dead,
                snapshots: self.mode.snapshots.clone(),
            },
            schema: self.schema.clone(),
            text: self.text.clone(),
            schema_violated: self.schema_violated,
            text_snapshots: self.text_snapshots.clone(),
        })
        .into()
    }
}

dyn_enum_close! {
    pub enum Constraints: Constraint {
        Regex(RegexConstraint),
        Trie(TrieConstraint),
        MustInclude(MustIncludeConstraint),
        JsonMode(JsonModeConstraint),
        Ebnf(EbnfConstraint),
        JsonSchema(JsonSchemaConstraint),
    }
}

/// 🧱️ Builds a [`Constraint`] from a [`ConstraintSpec`]; exhaustive over every variant.
pub async fn build_constraint(spec: &ConstraintSpec, limits: &SamplingLimits) -> Result<Constraints, SamplingError> {
    match spec {
        ConstraintSpec::Regex(pattern) => Ok((RegexConstraint::new(pattern, limits).await?).into()),
        ConstraintSpec::Trie(phrases) => Ok((TrieConstraint::new(phrases).await).into()),
        ConstraintSpec::MustInclude(alternatives) => Ok((MustIncludeConstraint::new(alternatives.clone()).await).into()),
        ConstraintSpec::JsonMode => Ok((JsonModeConstraint::new().await).into()),
        ConstraintSpec::Ebnf(text) => Ok((EbnfConstraint::new(text, limits).await?).into()),
        ConstraintSpec::JsonSchema(schema) => Ok((JsonSchemaConstraint::new(schema.clone()).await).into()),
    }
}
// #endregion 🔖️Constraints

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
