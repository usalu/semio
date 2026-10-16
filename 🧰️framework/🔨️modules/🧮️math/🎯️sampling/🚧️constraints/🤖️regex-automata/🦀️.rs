//! 🤖️ Byte-level regex parser, NFA builder and lazily memoised DFA.

use semio_framework_math_sampling_vocabulary::{SamplingError, SamplingLimits, TokenBitset, TokenId, TokenTextAdapters};

// #region 🔖️Automata
/// 🤖️ Parsed regex AST. Byte-level throughout (classes/literals match raw bytes, so multi-byte
/// UTF-8 sequences work as literal byte runs but aren't given first-class Unicode class support).
#[derive(Clone, PartialEq, Debug)]
pub(super) enum RegexNode {
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

pub(super) struct NfaNode {
    eps: Vec<usize>,
    byte_ranges: Vec<((u8, u8), usize)>,
    pub(super) accept: bool,
}

pub(super) struct Frag {
    pub(super) start: usize,
    pub(super) accept: usize,
}

pub(super) struct NfaBuilder {
    pub(super) nodes: Vec<NfaNode>,
}

impl NfaBuilder {
    async fn new_state(&mut self) -> usize {
        self.nodes.push(NfaNode { eps: Vec::new(), byte_ranges: Vec::new(), accept: false });
        self.nodes.len() - 1
    }

    pub(super) async fn build(&mut self, node: &RegexNode, limits: &SamplingLimits) -> Result<Frag, SamplingError> {
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
pub(super) async fn subset_construct(nodes: &[NfaNode], start_nfa_state: usize, limits: &SamplingLimits) -> Result<Dfa, SamplingError> {
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
    pub(super) entries: std::collections::HashMap<u32, (TokenBitset, Vec<u32>)>,
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
