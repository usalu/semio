//! 🎞️ Protocol laws: seeded generators for `crate::HistoryLog`/causal-DAG fixtures, the
//! cross-crate law assertions the `protocol_*` family's round-trip/determinism/tamper-detection
//! invariants boil down to, a re-export of the pack corruption sweeps, and the
//! golden-hash helper. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md` (`## protocol_testkit` +
//! `## Amendment`'s testing notes).
//!
//! This crate deliberately depends on the `protocol` facade rather than the individual
//! `protocol_*` sub-crates (matching every downstream consumer's own dependency rule) — with two
//! narrow exceptions (`protocol_core`, `protocol_format`) documented in `Cargo.toml`, needed only
//! because the facade re-exports frame-level *types* but not the raw-record-stream free functions
//! this crate's frame-level laws walk directly.

#[path = "🎲️seeded-generators/🦀️.rs"]
mod seeded_generators;
pub use seeded_generators::*;
use crate::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
//#region 🔖️Gen
// Inline splitmix64 PRNG (NOT arbitrary/quickcheck/proptest — repo precedent, `crate::os_pack::record_codec_laws::
// RecordValueGen`'s convention). Every generated string is built from an ASCII word alphabet plus,
// in `adversarial` mode, a small curated pool of multi-byte unicode characters spliced onto word
// boundaries — never at the very start/end and never containing `'\n'` or `'#'`, so a generated
// value is always safe to embed as an opaque single-line `.ops` op line (see `next_text`'s doc)
// while still exercising `dsl_schema`'s quoted-text escaping for every other field.

#[path = "📼️history-log-laws/🦀️.rs"]
mod history_log_laws;
pub use history_log_laws::*;

#[path = "🧮️operation-and-mutation-laws/🦀️.rs"]
mod operation_and_mutation_laws;
pub use operation_and_mutation_laws::*;

#[path = "🔁️dag-convergence-and-frame-laws/🦀️.rs"]
mod dag_convergence_and_frame_laws;
pub use dag_convergence_and_frame_laws::*;

#[path = "🏛️policy-merge-and-corpus-laws/🦀️.rs"]
mod policy_merge_and_corpus_laws;
pub use policy_merge_and_corpus_laws::*;

//#region 🔖️Laws

// 26/08/16 MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS §C2/C3/C5 law family — every
// helper below is generic over the `Mutation`/`MutationKind`/`MutationDiff` traits (or takes the
// store/history operation it needs as an injected closure) so any artifact/mutation pair a W3 facet
// owns can plug in without this crate depending on `os_store`/`os_spr::history`'s own concrete
// surfaces, several of which are still landing concurrently in lanes 1-A/1-B/1-C
// (`📓️w1-d-report.md` records exactly which closures stand in for a not-yet-landed method today).

//#endregion 🔖️Laws

//#region 🔖️Corrupt
/// 🛡️ Reused verbatim from the pack corruption tests — closure-generic panic-safety fuzzers plus their level/
/// report types. LAW (exercised in this crate's own tests against `crate::HistoryReader::open`
/// and `crate::format::recover`): `CorruptionReport::cases_panicked` must always be empty.
pub use semio_framework_os_pack::record_codec_laws::{fuzz_bit_flips, fuzz_truncation, CorruptionLevel, CorruptionReport};
//#endregion 🔖️Corrupt

//#region 🔖️Golden
pub use semio_framework_os_pack::record_codec_laws::golden_hash_hex;
//#endregion 🔖️Golden
