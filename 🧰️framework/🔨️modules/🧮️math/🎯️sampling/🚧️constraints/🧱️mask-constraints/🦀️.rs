//! 🧱️ Token-mask constraints: regex, trie, must-include, JSON mode, EBNF and JSON schema.

use semio_framework_math_sampling_vocabulary::{JsonValue, SamplingError, SamplingLimits, TokenBitset, TokenId, parse_json};
use semio_framework_math_sampling_workspace::{ConstraintMark, StepView};
use super::{Constraint, Dfa, DfaTokenMemo, NfaBuilder, RegexNode, subset_construct};

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
pub(super) async fn validates_json_schema(value: &JsonValue, schema: &JsonValue) -> bool {
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
    pub(super) mode: JsonModeConstraint,
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
// #endregion 🔖️Constraints
