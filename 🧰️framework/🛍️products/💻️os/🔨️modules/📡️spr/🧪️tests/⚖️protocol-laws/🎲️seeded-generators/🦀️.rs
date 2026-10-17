//! 🎲️ Seeded generators of the protocol laws: the splitmix64 state, the word, text, identifier and timestamp draws and the history log and operation DAG generators. See [`super`].


//#region 🔖️Gen
// Inline splitmix64 PRNG (NOT arbitrary/quickcheck/proptest — repo precedent, `crate::os_pack::record_codec_laws::
// RecordValueGen`'s convention). Every generated string is built from an ASCII word alphabet plus,
// in `adversarial` mode, a small curated pool of multi-byte unicode characters spliced onto word
// boundaries — never at the very start/end and never containing `'\n'` or `'#'`, so a generated
// value is always safe to embed as an opaque single-line `.ops` op line (see `next_text`'s doc)
// while still exercising `dsl_schema`'s quoted-text escaping for every other field.

/// 🎲️ Deterministic seeded splitmix64 state, advanced on every draw — see
/// <https://prng.di.unimi.it/splitmix64.c>. Reused by every generator in this crate.
pub(super) struct SplitMix64(pub(super) u64);

impl SplitMix64 {
    pub(super) async fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    pub(super) async fn next_bool(&mut self) -> bool {
        self.next_u64().await & 1 == 1
    }

    pub(super) async fn next_range(&mut self, bound: u64) -> u64 {
        if bound == 0 {
            0
        } else {
            self.next_u64().await % bound
        }
    }
}

pub(super) const WORD_ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

pub(super) const ADVERSARIAL_UNICODE: &[char] = &['é', 'ø', 'ñ', 'ß', '文', '档', '漢', '中', '🎉', '🚀'];

pub(super) async fn next_word(rng: &mut SplitMix64, max_len: usize) -> String {
    let len = 1 + rng.next_range(max_len as u64).await as usize;
    // 🚫️async: R10 shape 1 — `next_range` is async but `Iterator::map`'s closure is sync; hoisted
    // into a plain loop so each character draw can be awaited.
    let mut word = String::with_capacity(len);
    for _ in 0..len {
        word.push(WORD_ALPHABET[rng.next_range(WORD_ALPHABET.len() as u64).await as usize] as char);
    }
    word
}

/// ✏️ Free-form text safe as either a quoted `dsl_schema` field OR a raw opaque `.ops` op line: at
/// least one word, no leading/trailing whitespace, no `'\n'`, never trims to something starting
/// with `'#'` (which `crate::os_spr::history::parse_ops_text` would otherwise swallow as a comment).
/// `adversarial` widens the word count (exercising long descriptions) and splices a unicode
/// character onto the first word (never at a boundary, so trimming never changes the value).
pub(super) async fn next_text(rng: &mut SplitMix64, adversarial: bool) -> String {
    let word_count = if adversarial && rng.next_bool().await { 20 + rng.next_range(60).await as usize } else { 1 + rng.next_range(5).await as usize };
    // 🚫️async: R10 shape 1 — `next_word` is async but `Iterator::map`'s closure is sync; hoisted
    // into a plain loop so each element can be awaited.
    let mut words: Vec<String> = Vec::with_capacity(word_count);
    for _ in 0..word_count {
        words.push(next_word(rng, 8).await);
    }
    if adversarial && rng.next_bool().await {
        if let Some(first) = words.first_mut() {
            first.push(ADVERSARIAL_UNICODE[rng.next_range(ADVERSARIAL_UNICODE.len() as u64).await as usize]);
        }
    }
    words.join(" ")
}

/// 🪪️ A guaranteed-unique identifier (`"{prefix}-{index}"`, optionally adversarial-unicode-suffixed)
/// — uniqueness matters here since generated ids double as dictionary/edit-ordinal references.
pub(super) async fn next_ident(rng: &mut SplitMix64, prefix: &str, index: usize, adversarial: bool) -> String {
    let mut id = format!("{prefix}-{index}");
    if adversarial && rng.next_bool().await {
        id.push(ADVERSARIAL_UNICODE[rng.next_range(ADVERSARIAL_UNICODE.len() as u64).await as usize]);
    }
    id
}

/// ⏱️ Either a canonical `YYYY-MM-DDTHH:MM:SS[.fff]Z` string (exercising `crate::os_spr::wire::scalar`'s
/// compact tag-1/2 timestamp encoding) or, in adversarial mode, a deliberately non-canonical raw
/// string (a non-UTC offset, free text, or a Z-suffixed string with an out-of-grammar shape) that
/// forces the tag-0 raw-text fallback — see `crate::os_spr::wire::scalar`'s module note: correctness
/// never depends on which tag gets chosen, only on the round-trip equality check, so both branches
/// are safe regardless of exact calendar validity.
pub(super) async fn next_timestamp(rng: &mut SplitMix64, adversarial: bool) -> String {
    if adversarial && rng.next_bool().await {
        match rng.next_range(3).await {
            0 => "not-a-timestamp".to_string(),
            1 => format!("2024-01-15T10:30:00+0{}:00", 1 + rng.next_range(8).await),
            _ => format!("2024-13-{:02}T99:99:99Z", 1 + rng.next_range(28).await),
        }
    } else {
        let year = 2020 + rng.next_range(10).await;
        let month = 1 + rng.next_range(12).await;
        let day = 1 + rng.next_range(28).await;
        let hour = rng.next_range(24).await;
        let minute = rng.next_range(60).await;
        let second = rng.next_range(60).await;
        let ms = rng.next_range(1000).await;
        if ms == 0 {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
        } else {
            format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}.{ms:03}Z")
        }
    }
}

/// 🎞️ Shape knobs for `HistoryLogGen::generate`. `adversarial: true` widens every generated field
/// (unicode ids, long descriptions, non-canonical timestamps forcing the raw-text fallback) and
/// makes a zero-op edit likely — the contract's "tiny/typical/adversarial" profile density is a
/// caller-side convention (pick small/medium/large `edit_count`/`max_ops_per_edit` plus this flag),
/// not three hardcoded presets.
#[derive(Clone, Debug)]
pub struct GenProfile {
    pub edit_count: usize,
    pub max_ops_per_edit: usize,
    pub checkpoint_every: usize,
    pub adversarial: bool,
}

/// 🔀️ Appends one generated [`crate::os_spr::HistoryTransitionRecord`] carrying `transition`, causally
/// chained to the previous one, at a strictly increasing physical clock.
pub(super) async fn push_transition(rng: &mut SplitMix64, clock: &mut u64, adversarial: bool, index: usize, transition: crate::HistoryTransition, transitions: &mut Vec<crate::HistoryTransitionRecord>) {
    *clock += 1;
    let actor = next_ident(rng, "actor", index, adversarial).await;
    let hlt = (rng.next_range(4).await, *clock, rng.next_range(2).await);
    let id = next_ident(rng, "transition", transitions.len(), adversarial).await;
    let dependencies = transitions.last().map(|prior| vec![prior.id.clone()]).unwrap_or_default();
    transitions.push(crate::HistoryTransitionRecord { id, actor:actor.into(), hlt, dependencies, observed: None, payload: crate::encode_history_transition(&transition) });
}

/// 🎞️ Deterministic seeded `crate::os_spr::HistoryLog` fabricator.
pub struct HistoryLogGen {
    pub(super) state: u64,
}

impl HistoryLogGen {
    pub async fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// 🌱️ Fabricates a `HistoryLog` matching `profile`: a doc identity, `edit_count` edits (each
    /// with 0..=`max_ops_per_edit` opaque op lines), a `Commit` transition every `checkpoint_every`
    /// edits (chained via `parent_id`), then 0..=2 `Branch` transitions at a random checkpoint —
    /// every field this crate's own `assert_history_*` laws round-trip.
    pub async fn generate(&mut self, profile: &GenProfile) -> crate::HistoryLog {
        let mut rng = SplitMix64(self.state);

        let doc_id = next_ident(&mut rng, "doc", 0, profile.adversarial).await;
        let schema = next_ident(&mut rng, "schema", 0, profile.adversarial).await;

        let mut edits = crate::HistoryEdits::empty();
        for i in 0..profile.edit_count {
            let id = next_ident(&mut rng, "edit", i, profile.adversarial).await;
            let started_at = next_timestamp(&mut rng, profile.adversarial).await;
            let finished_at = if rng.next_bool().await { Some(next_timestamp(&mut rng, profile.adversarial).await) } else { None };
            let actor = if rng.next_bool().await { Some(next_ident(&mut rng, "actor", i, profile.adversarial).await) } else { None };
            let op_count = if profile.adversarial && rng.next_bool().await { 0 } else { rng.next_range(profile.max_ops_per_edit as u64 + 1).await as usize };
            let mut ops = Vec::with_capacity(op_count);
            for _ in 0..op_count {
                ops.push(crate::OpPayload { text: Some(next_text(&mut rng, profile.adversarial).await), binary: None });
            }
            edits.push(crate::HistoryEdit { line: None, id, actor:actor.map(Into::into), started_at, finished_at, verb: None, ops, inverse: Vec::new(), meta: None, lane: None });
        }

        let mut transitions: Vec<crate::HistoryTransitionRecord> = Vec::new();
        let mut checkpoint_ids: Vec<String> = Vec::new();
        let mut clock = 0u64;
        if profile.checkpoint_every > 0 {
            let mut boundary = 0usize;
            let mut index = 0usize;
            while boundary < edits.len() {
                let end = (boundary + profile.checkpoint_every).min(edits.len());
                let mutation_ids = edits.iter().skip(boundary).take(end-boundary).flat_map(|edit| (0..edit.ops.len()).map(|op| crate::MutationId(format!("{}#{op}", edit.id)))).collect();
                let author_count = rng.next_range(3).await as usize;
                let mut authors = Vec::with_capacity(author_count);
                for a in 0..author_count {
                    authors.push(crate::TransitionAuthor { id: next_ident(&mut rng, "author", a, profile.adversarial).await, name: next_text(&mut rng, false).await, avatar: None });
                }
                let checkpoint = crate::TransitionCheckpoint {
                    checkpoint_id: next_ident(&mut rng, "checkpoint", index, profile.adversarial).await,
                    parent_id: checkpoint_ids.last().cloned(),
                    change_id: next_ident(&mut rng, "change", index, profile.adversarial).await,
                    mutation_ids,
                    description: if rng.next_bool().await { Some(next_text(&mut rng, profile.adversarial).await) } else { None },
                    saved_at: next_timestamp(&mut rng, profile.adversarial).await,
                    authors,
                    message: if rng.next_bool().await { Some(next_text(&mut rng, profile.adversarial).await) } else { None },
                    timestamp: next_timestamp(&mut rng, profile.adversarial).await,
                    line_id: None,
                };
                checkpoint_ids.push(checkpoint.checkpoint_id.clone());
                push_transition(&mut rng, &mut clock, profile.adversarial, index, crate::HistoryTransition::Commit(checkpoint), &mut transitions).await;
                index += 1;
                boundary = end;
            }
        }
        if !checkpoint_ids.is_empty() {
            for i in 0..rng.next_range(3).await as usize {
                let checkpoint_id = checkpoint_ids[rng.next_range(checkpoint_ids.len() as u64).await as usize].clone();
                let branch = crate::HistoryTransition::Branch { alternative_id: next_ident(&mut rng, "alt", i, profile.adversarial).await, name: next_text(&mut rng, false).await, checkpoint_id };
                push_transition(&mut rng, &mut clock, profile.adversarial, i, branch, &mut transitions).await;
            }
        }

        self.state = rng.0;
        crate::HistoryLog { doc_id, schema, edits, transitions, composition: None, conflicts: Vec::new(), viewer_line: None, viewer_checkpoint: None }
    }
}

/// 🎞️ Deterministic seeded random causal-DAG fabricator for `assert_op_dag_convergence`'s
/// exhaustive tier — `protocol_causal`'s own inline tests already cover a handful of hand-built
/// 3-4-node diamonds in every topological order at the `quick` tier (per the amendment's testing
/// note); this generates larger closed DAGs for a random-permutation sweep.
pub struct OpDagGen {
    pub(super) state: u64,
}

impl OpDagGen {
    pub async fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// 🌱️ Generates `node_count` envelopes `"op-0".."op-{node_count-1}"`, each depending on 0..=2
    /// strictly earlier-indexed operations — dependencies always reference a smaller index, so the
    /// result is by construction a closed, acyclic dependency graph over exactly this returned set
    /// (every id a dependency names is itself present in the returned `Vec`).
    pub async fn generate(&mut self, node_count: usize) -> Vec<crate::MutationEnvelope> {
        let mut rng = SplitMix64(self.state);
        let mut envelopes = Vec::with_capacity(node_count);
        for i in 0..node_count {
            let dep_count = if i == 0 { 0 } else { rng.next_range(3.min(i as u64 + 1)).await as usize };
            let mut dependencies: Vec<crate::MutationId> = Vec::with_capacity(dep_count);
            for _ in 0..dep_count {
                let dep_id = crate::MutationId(format!("op-{}", rng.next_range(i as u64).await));
                if !dependencies.contains(&dep_id) {
                    dependencies.push(dep_id);
                }
            }
            envelopes.push(crate::MutationEnvelope {
                mutation_id: crate::MutationId(format!("op-{i}")),
                document_id: crate::ArtifactId("doc-1".to_string()),
                actor: crate::ActorId(format!("actor-{}", rng.next_range(4).await).into()),
                dependencies,
                observed: None,
                target: Vec::new(),
                diff: crate::ArtifactDiff { schema: crate::SchemaId("testkit.op".to_string()), payload: format!("index:{i}").into_bytes() },
                inverse: crate::InverseMutation { schema: crate::SchemaId("testkit.op".to_string()), payload: Vec::new() },
                timestamp: crate::HybridLogicalTimestamp::new(i as u64, i as u64 * 10),
                transaction: None, verb: None, line: None,
            });
        }
        self.state = rng.0;
        envelopes
    }
}
//#endregion 🔖️Gen
