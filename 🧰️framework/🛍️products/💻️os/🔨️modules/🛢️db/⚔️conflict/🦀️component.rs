//! 🗄️🤝️ `db_conflict` — conflict detection for concurrent commands against the same document
//! frontier: touched-region intersection (built on `db_state::TouchedSet`), a bloom-filter
//! pre-filter so a large batch doesn't pay the full intersection cost pairwise, a declarative
//! command-kind matrix for structural (non-region-derived) commutativity knowledge, constraint
//! conflicts (unique name, single parent, non-overlapping intervals — see `Constraint`), and
//! deterministic resolution planning derived from `protocol::ConflictRule`. Frozen contract:
//! `.🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_conflict` row).
//!
//! 🎯️ Design choice: per the contract's hard rule ("command payloads are opaque
//! `protocol::MutationEnvelope`/binary bytes below `db_artifact` — no db crate below it
//! interprets operation semantics"), this crate never sees a concrete `Mutation<P>`/`MutationDiff<P>`
//! type and therefore never calls `protocol_crdt::merge_concurrent_diffs` itself (that function is
//! generic over the concrete diff type, which only `db_artifact` knows). What this crate DOES own
//! is the *decision*: given two commands' declared `protocol::ConflictRule`s, it produces a
//! `ResolutionPlan` describing which class of reconciliation `db_artifact` must perform (commute
//! as-is / operational-transform / merge with a named strategy / CRDT-merge with a named strategy)
//! — `db_artifact` is the extension seam that actually executes that plan against the concrete
//! diff. This is a deliberate, documented boundary, not a hollow stub: everything on this crate's
//! own side of that boundary (detection, prioritization, the matrix, the bloom filter, constraint
//! conflicts) is real and tested.

use db_state::{TouchKind, TouchedRegion, TouchedSet};

//#region 🔖️CommandTouch
/// @emoji 🏷️ A command's declared kind — the tag `CommandKindMatrix` keys structural
/// commutativity knowledge by. Deliberately a bare string newtype (not tied to
/// `protocol_core::SchemaId`) so this crate stays usable for kinds that aren't yet registered
/// `MutationDescriptor`s.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct CommandKind(pub String);

impl From<&str> for CommandKind {
    fn from(value: &str) -> Self {
        CommandKind(value.to_string())
    }
}

/// @emoji 👣️ One command's accumulated read/write footprint against a document's overlay, plus
/// enough declared metadata (`kind`, `conflict_rule`, `timestamp`) to classify and prioritize any
/// conflict it's found to have with a concurrent sibling. `db_artifact` builds one of these per
/// admitted command (from the `TouchedSet` its `OverlayRoot` mutations accumulated) before handing
/// a batch to `ConflictDetector::detect`.
#[derive(Clone, Debug)]
pub struct CommandTouch {
    pub command_id: protocol::MutationId,
    pub actor: protocol::ActorId,
    pub kind: CommandKind,
    pub conflict_rule: protocol::ConflictRule,
    pub timestamp: protocol::HybridLogicalTimestamp,
    pub touched: TouchedSet,
    /// @emoji 🔐️ Structural `Constraint`s this command claims (unique name / single parent /
    /// non-overlapping interval) — orthogonal to `touched`, since two commands can violate a
    /// constraint while writing to entirely different overlay paths (the whole reason
    /// `db_conflict` tracks constraint conflicts as their own `ConflictKind` rather than folding
    /// them into touched-region paths).
    pub claims: Vec<Constraint>,
}

impl CommandTouch {
    pub fn new(command_id: protocol::MutationId, actor: protocol::ActorId, kind: CommandKind, conflict_rule: protocol::ConflictRule, timestamp: protocol::HybridLogicalTimestamp) -> Self {
        CommandTouch { command_id, actor, kind, conflict_rule, timestamp, touched: TouchedSet::new(), claims: Vec::new() }
    }

    /// @emoji ✏️ Builder-style: records one touched region (builds `touched` up incrementally as
    /// `db_artifact` replays the command's overlay mutations).
    pub fn touch(mut self, region: TouchedRegion) -> Self {
        self.touched.record(region);
        self
    }

    /// @emoji 🔐️ Builder-style: claims a unique-name key (`Constraint::Unique`), e.g.
    /// `"unique/email/alice@example.com"`.
    pub fn claim(mut self, key: impl Into<String>) -> Self {
        self.claims.push(Constraint::Unique(key.into()));
        self
    }

    /// @emoji 🌳️ Builder-style: claims `child`'s parent is `parent` (`Constraint::SingleParent`).
    pub fn claim_parent(mut self, child: impl Into<String>, parent: impl Into<String>) -> Self {
        self.claims.push(Constraint::SingleParent { child: child.into(), parent: parent.into() });
        self
    }

    /// @emoji ⏱️ Builder-style: claims `[start, end)` on `track` (`Constraint::NonOverlappingInterval`).
    pub fn claim_interval(mut self, track: impl Into<String>, start: u64, end: u64) -> Self {
        self.claims.push(Constraint::NonOverlappingInterval { track: track.into(), start, end });
        self
    }

    /// @emoji 🔢️ A total order key for deterministic prioritization: `(timestamp, command_id)`.
    /// `timestamp` alone (`protocol::HybridLogicalTimestamp::Ord`) is already actor-tiebroken, but
    /// two distinct commands from the SAME actor at the same tick (a batch submitted together) need
    /// a further tiebreak, hence the trailing `command_id` comparison.
    fn order_key(&self) -> (protocol::HybridLogicalTimestamp, &str) {
        (self.timestamp, self.command_id.0.as_str())
    }
}
//#endregion 🔖️CommandTouch

//#region 🔖️Bloom
/// @emoji 🌸️ A fixed-size bitset bloom filter over touched-region paths: a cheap, allocation-bounded
/// pre-filter so `ConflictDetector::detect` doesn't have to run the full O(regions²)
/// `TouchedSet::conflicts_with` scan for every pair in a large batch — most pairs in a real batch
/// touch disjoint paths, and `might_intersect` rejects those in O(bits) with zero false negatives
/// (it may say "might intersect" when the true answer is no — a false positive just falls through
/// to the real check — but it never says "no" when the true answer is yes).
#[derive(Clone, Debug)]
pub struct PathBloom {
    bits: Vec<u64>,
    num_bits: usize,
    num_hashes: usize,
}

/// @emoji 🎛️ The crate's own choice of default sizing (the contract fixes "bloom filters" as a
/// mechanism, not exact parameters): 2048 bits / 4 hash functions keeps the false-positive rate low
/// for the tens-to-low-hundreds of touched paths one command batch realistically accumulates, while
/// staying a fixed 256-byte allocation regardless of batch size.
const DEFAULT_BLOOM_BITS: usize = 2048;
const DEFAULT_BLOOM_HASHES: usize = 4;

impl PathBloom {
    /// @emoji 🏗️ Validates `num_bits`/`num_hashes` before allocating the backing `Vec` (mirrors
    /// `pack_core`'s "validate before allocating" invariant, applied to this crate's own inputs).
    pub fn new(num_bits: usize, num_hashes: usize) -> Result<PathBloom, DbError> {
        if num_bits == 0 || num_hashes == 0 {
            return Err(DbError::InvalidArgument("PathBloom requires num_bits > 0 and num_hashes > 0".to_string()));
        }
        let words = num_bits.div_ceil(64);
        Ok(PathBloom { bits: vec![0u64; words], num_bits, num_hashes })
    }

    pub fn default_sized() -> PathBloom {
        PathBloom::new(DEFAULT_BLOOM_BITS, DEFAULT_BLOOM_HASHES).expect("default bloom parameters are always valid")
    }

    /// @emoji 🏗️ Builds a default-sized bloom filter seeded with every path in `touched`.
    pub fn from_touched(touched: &TouchedSet) -> PathBloom {
        let mut bloom = PathBloom::default_sized();
        for region in &touched.regions {
            bloom.insert(&region.path);
        }
        bloom
    }

    pub fn insert(&mut self, path: &str) {
        let slots: Vec<usize> = self.bit_positions(path).collect();
        for slot in slots {
            self.bits[slot / 64] |= 1u64 << (slot % 64);
        }
    }

    pub fn might_contain(&self, path: &str) -> bool {
        self.bit_positions(path).all(|slot| self.bits[slot / 64] & (1u64 << (slot % 64)) != 0)
    }

    /// @emoji 🔀️ True iff `self` and `other` MIGHT share at least one inserted path — a cheap,
    /// conservative (never-false-negative) prefilter for `TouchedSet::conflicts_with`. Bloom
    /// filters of mismatched size can't be bitwise-compared meaningfully, so a mismatch
    /// conservatively answers `true` (falls through to the real check) rather than risking a false
    /// "no intersection".
    pub fn might_intersect(&self, other: &PathBloom) -> bool {
        if self.num_bits != other.num_bits || self.num_hashes != other.num_hashes {
            return true;
        }
        self.bits.iter().zip(other.bits.iter()).any(|(a, b)| a & b != 0)
    }

    fn bit_positions<'a>(&'a self, path: &'a str) -> impl Iterator<Item = usize> + 'a {
        (0..self.num_hashes).map(move |seed| (hash_with_seed(seed as u64, path) as usize) % self.num_bits)
    }
}

/// @emoji 🔀️ Deterministic per-hash-function seeding, matching `db_state`'s stated design choice
/// (`std::collections::hash_map::DefaultHasher` is a router, not a security primitive — a bloom
/// filter's false-positive tolerance needs exactly that, not cryptographic strength).
fn hash_with_seed(seed: u64, s: &str) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    seed.hash(&mut hasher);
    s.hash(&mut hasher);
    hasher.finish()
}
//#endregion 🔖️Bloom

//#region 🔖️KindMatrix
/// @emoji 🧮️ Declarative structural knowledge that two command KINDS never conflict, independent of
/// whatever paths their instances happen to touch — e.g. a read-only "Query" kind never conflicts
/// with anything, or a domain-specific pair of write kinds that are known commutative by
/// construction even though they both touch a shared bookkeeping path (a counter's increment vs. a
/// separate audit-log append at the same parent path, say). This is strictly an optimization/
/// override LAYERED ON TOP of touched-region detection — declaring a pair here means
/// `ConflictDetector::detect` skips the region/bloom work for that pair entirely; it must only ever
/// be used for pairs that are truly always safe to apply in either order, since nothing downstream
/// re-checks them.
#[derive(Clone, Default, Debug)]
pub struct CommandKindMatrix {
    commuting_pairs: std::collections::HashSet<(String, String)>,
    /// @emoji 📖️ Kinds declared unconditionally read-only — never conflict with any other kind
    /// (including another read-only kind), covering the "Query never conflicts" case without an
    /// O(kinds²) pairwise declaration.
    read_only_kinds: std::collections::HashSet<String>,
}

impl CommandKindMatrix {
    pub fn new() -> Self {
        Self::default()
    }

    /// @emoji ➕️ Declares `a`/`b` as always-commuting, symmetrically (`declare_commuting(a, b)` and
    /// `declare_commuting(b, a)` are equivalent, including `a == b`, the common case of "every pair
    /// of instances of this one kind commutes with each other").
    pub fn declare_commuting(&mut self, a: &CommandKind, b: &CommandKind) {
        self.commuting_pairs.insert(Self::pair_key(a, b));
    }

    /// @emoji 📖️ Declares `kind` as read-only: it never conflicts with anything.
    pub fn declare_read_only(&mut self, kind: &CommandKind) {
        self.read_only_kinds.insert(kind.0.clone());
    }

    /// @emoji 🔎️ True iff `a`/`b` are declared structurally non-conflicting (either via
    /// `declare_commuting` or because one side is `declare_read_only`).
    pub fn commutes(&self, a: &CommandKind, b: &CommandKind) -> bool {
        self.read_only_kinds.contains(&a.0) || self.read_only_kinds.contains(&b.0) || self.commuting_pairs.contains(&Self::pair_key(a, b))
    }

    fn pair_key(a: &CommandKind, b: &CommandKind) -> (String, String) {
        if a.0 <= b.0 {
            (a.0.clone(), b.0.clone())
        } else {
            (b.0.clone(), a.0.clone())
        }
    }
}
//#endregion 🔖️KindMatrix

//#region 🔖️Constraint
/// @emoji 🧱️ A structural database invariant a command declares it upholds — independent of
/// `protocol::ConflictRule` (which governs CRDT-style concurrent-diff resolution over touched
/// regions). Two commands violating the same constraint can never be reconciled by any merge
/// strategy, so `detect_constraint_conflicts` always resolves a violation to `ResolutionPlan::Reject`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Constraint {
    /// @emoji 🏷️ Exclusive ownership of a name/key within some caller-chosen scope, encoded
    /// directly into `key` (e.g. `"unique/email/alice@example.com"`) — two commands claiming the
    /// identical key conflict.
    Unique(String),
    /// @emoji 🌳️ `child` may have at most one `parent` at a time — two commands claiming the same
    /// `child` with a DIFFERENT `parent` conflict; the same `(child, parent)` claimed twice does not
    /// (both commands agree, nothing to reconcile).
    SingleParent { child: String, parent: String },
    /// @emoji ⏱️ `[start, end)` must not overlap any other interval claimed on the same `track`
    /// (e.g. a timeline track) — two commands claiming overlapping ranges on the same track
    /// conflict; touching (non-overlapping) ranges do not.
    NonOverlappingInterval { track: String, start: u64, end: u64 },
}

fn intervals_overlap(start_a: u64, end_a: u64, start_b: u64, end_b: u64) -> bool {
    start_a < end_b && start_b < end_a
}
//#endregion 🔖️Constraint

//#region 🔖️Resolution
/// @emoji ⚖️ What `db_artifact` must do to reconcile two concurrent commands this crate found to
/// conflict — the executable-shaped twin of `protocol::ConflictRule` (see module doc for why this
/// crate stops at the decision and never executes it). `Reject` is this crate's own addition beyond
/// `ConflictRule`'s four variants: a `ConflictKind::Constraint` violation has no CRDT-style merge
/// (two commands cannot both hold the same uniqueness key), so it always resolves to `Reject` of
/// the lower-priority side.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ResolutionPlan {
    Commutes,
    Transform,
    Merge(protocol::MergeStrategyKind),
    Crdt(protocol::MergeStrategyKind),
    Reject,
}

impl From<protocol::ConflictRule> for ResolutionPlan {
    fn from(rule: protocol::ConflictRule) -> Self {
        match rule {
            protocol::ConflictRule::Commutes => ResolutionPlan::Commutes,
            protocol::ConflictRule::Transform => ResolutionPlan::Transform,
            protocol::ConflictRule::Merge(strategy) => ResolutionPlan::Merge(strategy),
            protocol::ConflictRule::Crdt(strategy) => ResolutionPlan::Crdt(strategy),
        }
    }
}

/// @emoji 🤝️ Combines two (possibly differing) declared `ConflictRule`s from the two sides of a
/// touched-region conflict into one `ResolutionPlan`.
///
/// 🎯️ Design choice: the contract ties `ConflictRule` to an operation KIND, not to a conflicting
/// PAIR, so two commands of different kinds may legitimately declare different rules; there is no
/// principled "combine" defined by the contract. When both sides agree, use that rule outright. When
/// they disagree, fall back to `Transform` — the one mechanism in this family (`protocol_causal
/// ::MutationTransform`) that is defined generically over any `Mutation<P>` regardless of its
/// declared merge strategy, making it the safest default for a genuinely mixed-kind conflict.
pub fn combine_conflict_rules(a: protocol::ConflictRule, b: protocol::ConflictRule) -> ResolutionPlan {
    if a == b {
        ResolutionPlan::from(a)
    } else {
        ResolutionPlan::Transform
    }
}
//#endregion 🔖️Resolution

//#region 🔖️ConflictRecord
/// @emoji 🗺️ What was found to conflict: either an intersecting touched-region set, or a shared
/// uniqueness-constraint claim.
#[derive(Clone, Debug, PartialEq)]
pub enum ConflictKind {
    TouchedRegion(Vec<TouchedRegion>),
    Constraint(String),
}

/// @emoji 🧾️ One detected conflict between two commands in the same batch — the unit
/// `CommandReceipt.conflicts` (the `db` facade's frozen `Vec<ConflictRecord>` field) is built from.
/// `command_id` is always the lower-priority side (per `CommandTouch::order_key`) so a reader can
/// treat "the command this record is attached to" as the one that needed reconciling against
/// `conflicting_with`, which already held (or would otherwise hold) priority.
#[derive(Clone, Debug, PartialEq)]
pub struct ConflictRecord {
    pub command_id: protocol::MutationId,
    pub conflicting_with: protocol::MutationId,
    pub kind: ConflictKind,
    pub resolution: ResolutionPlan,
}
//#endregion 🔖️ConflictRecord

//#region 🔖️Detector
/// @emoji 🕵️ Detects every conflict within one batch of concurrent `CommandTouch`es (all assumed
/// to share the same base frontier — `db_artifact` is responsible for only ever passing commands
/// admitted against the same base, per the contract's conflict-detection placement in its command
/// pipeline).
#[derive(Clone, Default, Debug)]
pub struct ConflictDetector {
    pub kind_matrix: CommandKindMatrix,
}

impl ConflictDetector {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_matrix(kind_matrix: CommandKindMatrix) -> Self {
        ConflictDetector { kind_matrix }
    }

    /// @emoji 🔎️ Runs touched-region AND constraint-claim conflict detection over `commands`,
    /// returning every `ConflictRecord` found, in a canonical (input-order-independent) sort by
    /// `(command_id, conflicting_with)` — LAW: the returned set (as a set, ignoring order) depends
    /// only on the CONTENTS of `commands`, never on the order the caller happened to hand them in
    /// (the "deterministic resolution" property the contract asks for), since detection internally
    /// sorts by `CommandTouch::order_key` before ever comparing a pair.
    pub fn detect(&self, commands: &[CommandTouch]) -> Vec<ConflictRecord> {
        let mut ordered: Vec<&CommandTouch> = commands.iter().collect();
        ordered.sort_by(|a, b| a.order_key().cmp(&b.order_key()));

        let blooms: Vec<PathBloom> = ordered.iter().map(|command| PathBloom::from_touched(&command.touched)).collect();

        let mut records = Vec::new();
        for i in 0..ordered.len() {
            for j in (i + 1)..ordered.len() {
                let (earlier, later) = (ordered[i], ordered[j]);
                if self.kind_matrix.commutes(&earlier.kind, &later.kind) {
                    continue;
                }
                if !blooms[i].might_intersect(&blooms[j]) {
                    continue;
                }
                if let Some(overlap) = touched_overlap(&earlier.touched, &later.touched) {
                    let resolution = combine_conflict_rules(earlier.conflict_rule, later.conflict_rule);
                    records.push(ConflictRecord { command_id: later.command_id.clone(), conflicting_with: earlier.command_id.clone(), kind: ConflictKind::TouchedRegion(overlap), resolution });
                }
            }
        }

        records.extend(detect_constraint_conflicts(&ordered));
        records.sort_by(|a, b| (&a.command_id.0, &a.conflicting_with.0).cmp(&(&b.command_id.0, &b.conflicting_with.0)));
        records
    }
}

/// @emoji ⚔️ Every pair of regions (one from each side, at least one a `Write`) that intersect —
/// `None` if the two touched sets don't conflict at all. Built directly on
/// `db_state::TouchedSet::conflicts_with`'s law (read/read never conflicts) rather than
/// reimplementing it, but additionally collects WHICH regions overlapped (the boolean-only
/// `conflicts_with` doesn't report that), since a `ConflictRecord` needs it for diagnostics.
fn touched_overlap(a: &TouchedSet, b: &TouchedSet) -> Option<Vec<TouchedRegion>> {
    let mut overlap = Vec::new();
    for region_a in &a.regions {
        for region_b in &b.regions {
            if (region_a.kind == TouchKind::Write || region_b.kind == TouchKind::Write) && region_a.path_intersects(region_b) {
                overlap.push(region_a.clone());
            }
        }
    }
    if overlap.is_empty() {
        None
    } else {
        Some(overlap)
    }
}

/// @emoji 🔐️ Detects `Constraint` violations across `ordered` (already sorted by priority), one
/// independent holder table per `Constraint` variant (`Unique`/`SingleParent`/
/// `NonOverlappingInterval` claims never conflict across variants — different namespaces). Each
/// table follows the same "first holder wins, every later violator conflicts against it" shape: a
/// later command that violates an earlier command's claim gets a `ConflictRecord` against that
/// earlier (higher-priority) holder, resolving to `Reject` unconditionally (see
/// `ResolutionPlan::Reject`'s doc) — a claim that doesn't violate the current holder (same unique
/// key re-claimed by the same command's own priority slot, same `(child, parent)`, or a
/// non-overlapping interval) is folded in without producing a record.
fn detect_constraint_conflicts(ordered: &[&CommandTouch]) -> Vec<ConflictRecord> {
    let mut unique_holders: std::collections::HashMap<&str, &CommandTouch> = std::collections::HashMap::new();
    let mut parent_holders: std::collections::HashMap<&str, (&str, &CommandTouch)> = std::collections::HashMap::new();
    let mut interval_claims: std::collections::HashMap<&str, Vec<(u64, u64, &CommandTouch)>> = std::collections::HashMap::new();
    let mut records = Vec::new();

    for command in ordered {
        for claim in &command.claims {
            match claim {
                Constraint::Unique(key) => match unique_holders.get(key.as_str()) {
                    Some(holder) if holder.command_id != command.command_id => {
                        records.push(constraint_record(command, holder, format!("unique:{key}")));
                    }
                    _ => {
                        unique_holders.insert(key.as_str(), command);
                    }
                },
                Constraint::SingleParent { child, parent } => match parent_holders.get(child.as_str()) {
                    Some((held_parent, holder)) if *held_parent != parent.as_str() && holder.command_id != command.command_id => {
                        records.push(constraint_record(command, holder, format!("single-parent:{child}")));
                    }
                    Some(_) => {}
                    None => {
                        parent_holders.insert(child.as_str(), (parent.as_str(), command));
                    }
                },
                Constraint::NonOverlappingInterval { track, start, end } => {
                    let claimed = interval_claims.entry(track.as_str()).or_default();
                    for (held_start, held_end, holder) in claimed.iter() {
                        if holder.command_id != command.command_id && intervals_overlap(*start, *end, *held_start, *held_end) {
                            records.push(constraint_record(command, holder, format!("interval:{track}")));
                        }
                    }
                    claimed.push((*start, *end, command));
                }
            }
        }
    }
    records
}

/// @emoji 🧾️ Shared `ConflictRecord` builder for every `Constraint` violation branch in
/// `detect_constraint_conflicts` — always `ResolutionPlan::Reject` (see `Constraint`'s doc).
fn constraint_record(command: &CommandTouch, holder: &CommandTouch, description: String) -> ConflictRecord {
    ConflictRecord { command_id: command.command_id.clone(), conflicting_with: holder.command_id.clone(), kind: ConflictKind::Constraint(description), resolution: ResolutionPlan::Reject }
}
//#endregion 🔖️Detector

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🧸️Fixtures
    fn hlt(physical_ms: u64, actor: u64) -> protocol::HybridLogicalTimestamp {
        protocol::HybridLogicalTimestamp { actor, physical_ms, logical: 0 }
    }

    fn command(id: &str, actor: u64, physical_ms: u64, kind: &str, rule: protocol::ConflictRule) -> CommandTouch {
        CommandTouch::new(protocol::MutationId(id.into()), protocol::ActorId(format!("actor-{actor}")), CommandKind::from(kind), rule, hlt(physical_ms, actor))
    }
    //#endregion 🧸️Fixtures

    //#region 🔖️CommandTouch
    #[test]
    fn order_key_breaks_ties_on_command_id_when_timestamps_are_equal() {
        let a = command("cmd-a", 1, 1000, "write", protocol::ConflictRule::Commutes);
        let b = command("cmd-b", 1, 1000, "write", protocol::ConflictRule::Commutes);
        assert!(a.order_key() < b.order_key(), "same timestamp -> command_id is the final tiebreak");
    }
    //#endregion 🔖️CommandTouch

    //#region 🔖️Bloom
    #[test]
    fn bloom_rejects_new_requires_positive_params() {
        assert!(PathBloom::new(0, 4).is_err());
        assert!(PathBloom::new(1024, 0).is_err());
        assert!(PathBloom::new(1024, 4).is_ok());
    }

    #[test]
    fn bloom_might_contain_has_no_false_negatives() {
        let mut bloom = PathBloom::default_sized();
        let paths = ["a/b/c", "x/y", "counter/1", "very/long/nested/path/segment"];
        for path in paths {
            bloom.insert(path);
        }
        for path in paths {
            assert!(bloom.might_contain(path), "inserted path must never be reported absent");
        }
    }

    #[test]
    fn bloom_might_intersect_has_no_false_negatives_across_thousands_of_random_disjoint_and_overlapping_sets() {
        let mut rng_state: u64 = 0x9E3779B97F4A7C15;
        let mut next = move || {
            rng_state ^= rng_state << 13;
            rng_state ^= rng_state >> 7;
            rng_state ^= rng_state << 17;
            rng_state
        };
        for trial in 0..500 {
            let mut left = PathBloom::default_sized();
            let mut right = PathBloom::default_sized();
            let mut left_paths = Vec::new();
            let mut right_paths = Vec::new();
            for i in 0..20 {
                left_paths.push(format!("trial-{trial}/left/{i}/{}", next() % 997));
            }
            for i in 0..20 {
                right_paths.push(format!("trial-{trial}/right/{i}/{}", next() % 997));
            }
            let shares = trial % 3 == 0;
            if shares {
                right_paths.push(left_paths[0].clone());
            }
            for path in &left_paths {
                left.insert(path);
            }
            for path in &right_paths {
                right.insert(path);
            }
            let truly_shared = left_paths.iter().any(|path| right_paths.contains(path));
            if truly_shared {
                assert!(left.might_intersect(&right), "trial {trial}: a genuinely shared path must never be missed");
            }
        }
    }

    #[test]
    fn bloom_mismatched_sizes_conservatively_report_might_intersect() {
        let a = PathBloom::new(1024, 4).unwrap();
        let b = PathBloom::new(2048, 4).unwrap();
        assert!(a.might_intersect(&b));
    }
    //#endregion 🔖️Bloom

    //#region 🔖️KindMatrix
    #[test]
    fn kind_matrix_declared_commuting_pair_is_symmetric() {
        let mut matrix = CommandKindMatrix::new();
        let counter = CommandKind::from("counter.increment");
        let audit = CommandKind::from("audit.append");
        matrix.declare_commuting(&counter, &audit);
        assert!(matrix.commutes(&counter, &audit));
        assert!(matrix.commutes(&audit, &counter));
        assert!(!matrix.commutes(&counter, &CommandKind::from("other")));
    }

    #[test]
    fn kind_matrix_read_only_kind_commutes_with_everything() {
        let mut matrix = CommandKindMatrix::new();
        let query = CommandKind::from("query");
        matrix.declare_read_only(&query);
        assert!(matrix.commutes(&query, &CommandKind::from("anything")));
        assert!(matrix.commutes(&CommandKind::from("anything"), &query));
    }
    //#endregion 🔖️KindMatrix

    //#region 🔖️Resolution
    #[test]
    fn combine_conflict_rules_uses_the_shared_rule_when_both_sides_agree() {
        let rule = protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister);
        assert_eq!(combine_conflict_rules(rule, rule), ResolutionPlan::Merge(protocol::MergeStrategyKind::LwwRegister));
    }

    #[test]
    fn combine_conflict_rules_falls_back_to_transform_on_disagreement() {
        let a = protocol::ConflictRule::Commutes;
        let b = protocol::ConflictRule::Crdt(protocol::MergeStrategyKind::TextSequence);
        assert_eq!(combine_conflict_rules(a, b), ResolutionPlan::Transform);
    }

    #[test]
    fn resolution_plan_from_conflict_rule_covers_every_variant() {
        assert_eq!(ResolutionPlan::from(protocol::ConflictRule::Commutes), ResolutionPlan::Commutes);
        assert_eq!(ResolutionPlan::from(protocol::ConflictRule::Transform), ResolutionPlan::Transform);
        assert_eq!(ResolutionPlan::from(protocol::ConflictRule::Merge(protocol::MergeStrategyKind::OrderedSequence)), ResolutionPlan::Merge(protocol::MergeStrategyKind::OrderedSequence));
        assert_eq!(ResolutionPlan::from(protocol::ConflictRule::Crdt(protocol::MergeStrategyKind::TombstonedGraphSet)), ResolutionPlan::Crdt(protocol::MergeStrategyKind::TombstonedGraphSet));
    }
    //#endregion 🔖️Resolution

    //#region 🔖️Detector
    #[test]
    fn detects_write_write_touched_region_conflict() {
        let a = command("cmd-a", 1, 1000, "write", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/title"));
        let b = command("cmd-b", 2, 2000, "write", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/title"));

        let detector = ConflictDetector::new();
        let records = detector.detect(&[a, b]);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].command_id, protocol::MutationId("cmd-b".into()), "the later command carries the record");
        assert_eq!(records[0].conflicting_with, protocol::MutationId("cmd-a".into()));
        assert_eq!(records[0].resolution, ResolutionPlan::Merge(protocol::MergeStrategyKind::LwwRegister));
        match &records[0].kind {
            ConflictKind::TouchedRegion(regions) => assert_eq!(regions.len(), 1),
            other => panic!("expected TouchedRegion, got {other:?}"),
        }
    }

    #[test]
    fn read_read_never_conflicts() {
        let a = command("cmd-a", 1, 1000, "read", protocol::ConflictRule::Commutes).touch(TouchedRegion::read("artifacts/doc-1/title"));
        let b = command("cmd-b", 2, 2000, "read", protocol::ConflictRule::Commutes).touch(TouchedRegion::read("artifacts/doc-1/title"));
        assert!(ConflictDetector::new().detect(&[a, b]).is_empty());
    }

    #[test]
    fn disjoint_paths_never_conflict() {
        let a = command("cmd-a", 1, 1000, "write", protocol::ConflictRule::Commutes).touch(TouchedRegion::write("artifacts/doc-1/title"));
        let b = command("cmd-b", 2, 2000, "write", protocol::ConflictRule::Commutes).touch(TouchedRegion::write("artifacts/doc-1/body"));
        assert!(ConflictDetector::new().detect(&[a, b]).is_empty());
    }

    #[test]
    fn kind_matrix_override_suppresses_an_otherwise_conflicting_pair() {
        let a = command("cmd-a", 1, 1000, "counter.increment", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/counter"));
        let b = command("cmd-b", 2, 2000, "counter.increment", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/counter"));

        let mut matrix = CommandKindMatrix::new();
        matrix.declare_commuting(&CommandKind::from("counter.increment"), &CommandKind::from("counter.increment"));
        let detector = ConflictDetector::with_matrix(matrix);
        assert!(detector.detect(&[a, b]).is_empty(), "declared-commuting kinds must skip region detection entirely");
    }

    #[test]
    fn constraint_conflict_detected_across_disjoint_touched_paths() {
        let a = command("cmd-a", 1, 1000, "create-user", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-1")).claim("unique/email/alice@example.com");
        let b = command("cmd-b", 2, 2000, "create-user", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-2")).claim("unique/email/alice@example.com");

        let records = ConflictDetector::new().detect(&[a, b]);
        assert_eq!(records.len(), 1, "disjoint overlay paths must not mask the shared constraint claim");
        assert_eq!(records[0].resolution, ResolutionPlan::Reject);
        assert_eq!(records[0].kind, ConflictKind::Constraint("unique:unique/email/alice@example.com".to_string()));
    }

    #[test]
    fn single_parent_constraint_conflicts_only_on_diverging_parent() {
        let a = command("cmd-a", 1, 1000, "reparent", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-1")).claim_parent("node-1", "parent-a");
        let b = command("cmd-b", 2, 2000, "reparent", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-2")).claim_parent("node-1", "parent-b");
        let records = ConflictDetector::new().detect(&[a, b]);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].resolution, ResolutionPlan::Reject);
        assert_eq!(records[0].kind, ConflictKind::Constraint("single-parent:node-1".to_string()));

        let c = command("cmd-c", 1, 1000, "reparent", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-3")).claim_parent("node-1", "parent-a");
        let d = command("cmd-d", 2, 2000, "reparent", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-4")).claim_parent("node-1", "parent-a");
        assert!(ConflictDetector::new().detect(&[c, d]).is_empty(), "the same (child, parent) claimed twice is not a conflict");
    }

    #[test]
    fn non_overlapping_interval_constraint_conflicts_only_when_ranges_actually_overlap() {
        let a = command("cmd-a", 1, 1000, "schedule", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-1")).claim_interval("track-1", 0, 10);
        let overlapping = command("cmd-b", 2, 2000, "schedule", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-2")).claim_interval("track-1", 5, 15);
        let records = ConflictDetector::new().detect(&[a.clone(), overlapping]);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].resolution, ResolutionPlan::Reject);
        assert_eq!(records[0].kind, ConflictKind::Constraint("interval:track-1".to_string()));

        let touching = command("cmd-c", 3, 3000, "schedule", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-3")).claim_interval("track-1", 10, 20);
        assert!(ConflictDetector::new().detect(&[a.clone(), touching]).is_empty(), "touching (non-overlapping) intervals must not conflict");

        let other_track = command("cmd-d", 4, 4000, "schedule", protocol::ConflictRule::Transform).touch(TouchedRegion::write("artifacts/doc-4")).claim_interval("track-2", 0, 10);
        assert!(ConflictDetector::new().detect(&[a, other_track]).is_empty(), "different tracks never conflict");
    }

    #[test]
    fn detection_result_is_independent_of_input_order() {
        let a = command("cmd-a", 1, 1000, "write", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/x"));
        let b = command("cmd-b", 2, 2000, "write", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/x"));
        let c = command("cmd-c", 3, 3000, "write", protocol::ConflictRule::Merge(protocol::MergeStrategyKind::LwwRegister)).touch(TouchedRegion::write("artifacts/doc-1/x"));

        let forward = ConflictDetector::new().detect(&[a.clone(), b.clone(), c.clone()]);
        let shuffled = ConflictDetector::new().detect(&[c, a, b]);
        assert_eq!(forward, shuffled, "the detected conflict set must not depend on caller-supplied batch order");
        assert_eq!(forward.len(), 3, "three mutually-conflicting commands -> 3 pairwise records");
    }

    #[test]
    fn priority_side_is_the_earlier_command_by_timestamp_regardless_of_batch_order() {
        let later = command("cmd-later", 1, 5000, "write", protocol::ConflictRule::Commutes).touch(TouchedRegion::write("p"));
        let earlier = command("cmd-earlier", 1, 1000, "write", protocol::ConflictRule::Commutes).touch(TouchedRegion::write("p"));

        let records = ConflictDetector::new().detect(&[later, earlier]);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].conflicting_with, protocol::MutationId("cmd-earlier".into()));
        assert_eq!(records[0].command_id, protocol::MutationId("cmd-later".into()));
    }
    //#endregion 🔖️Detector
}
//#endregion 🧪️Tests
