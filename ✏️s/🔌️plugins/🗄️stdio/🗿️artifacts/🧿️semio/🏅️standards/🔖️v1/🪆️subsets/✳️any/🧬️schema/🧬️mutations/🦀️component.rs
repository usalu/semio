//! 🧬️ SemioMutation — the envelope union's own mutation vocabulary, per the master plan:
//! "NoMutation, SetSnapshot, + 13 wrapper variants embedding each subset's own mutation enum".
//! W2b closer real implementation, replacing the W1b `SetSnapshot`-only scaffold: the 13 wrapper
//! variants each carry that subset's OWN, already-real, already-hand-written `SemioXMutation`
//! enum unchanged (`SemioBrepMutation`, `SemioAudioMutation`, …) — every `diff()`/`inverse()` for
//! a wrapped variant delegates straight through to that subset's own `Mutation` impl, so this
//! module never re-derives any of the 13 subsets' own per-field mutation logic; its OWN job is
//! purely the envelope-level routing (does the wrapped mutation's kind match the base snapshot's
//! current kind, and if so thread it through).

use crate::artifacts::semio::standards::v1::subsets::animation::schema::{mutations::SemioAnimationMutation, snapshot::SemioAnimationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::{mutations::SemioAudioMutation, snapshot::SemioAudioSnapshot};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::{mutations::SemioBrepMutation, snapshot::SemioBrepSnapshot};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::{mutations::SemioCadMutation, snapshot::SemioCadSnapshot};
use crate::artifacts::semio::standards::v1::subsets::document::schema::{mutations::SemioDocumentMutation, snapshot::SemioDocumentSnapshot};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::{mutations::SemioDrawingMutation, snapshot::SemioDrawingSnapshot};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::{mutations::SemioFlowMutation, snapshot::SemioFlowSnapshot};
use crate::artifacts::semio::standards::v1::subsets::graph::schema::{mutations::SemioGraphMutation, snapshot::SemioGraphSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::{mutations::SemioImageMutation, snapshot::SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::{mutations::SemioKitMutation, snapshot::SemioKitSnapshot};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::{mutations::SemioMeshMutation, snapshot::SemioMeshSnapshot};
use crate::artifacts::semio::standards::v1::subsets::model::schema::{mutations::SemioModelMutation, snapshot::SemioModelSnapshot};
use crate::artifacts::semio::standards::v1::subsets::object::schema::{mutations::SemioObjectMutation, snapshot::SemioObjectSnapshot};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::{mutations::SemioPresentationMutation, snapshot::SemioPresentationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::table::schema::{mutations::SemioTableMutation, snapshot::SemioTableSnapshot};
use crate::artifacts::semio::standards::v1::subsets::text::schema::{mutations::SemioTextMutation, snapshot::SemioTextSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::{mutations::SemioValueMutation, snapshot::SemioValueSnapshot};
use crate::artifacts::semio::standards::v1::subsets::video::schema::{mutations::SemioVideoMutation, snapshot::SemioVideoSnapshot};
use protocol::Mutation;
use protocol::OpBinary;
use protocol::OpText;
use serde::{Deserialize, Serialize};

//#region 🔖️Mutation
/// 🔧️ Adjacently tagged (`tag = "mutation"`, `content = "payload"`), NOT internally tagged like
/// every one of the 13 wrapped subset enums' own `#[serde(tag = "mutation", ...)]` — an
/// internally-tagged wrapper here would collide key-for-key with a wrapped variant's OWN
/// `"mutation"` discriminator field when serde flattens a newtype variant's fields into the
/// outer value (real bug caught by this file's own `op_text_binary_roundtrip_law` test: printed
/// JSON came out `{"mutation":"audio","mutation":"setSampleRate",...}`, two keys with the same
/// name, which `serde_json` then refuses to parse back). `content = "payload"` nests the wrapped
/// value under its own key instead of flattening it, sidestepping the collision entirely.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", content = "payload", rename_all = "camelCase")]
pub enum SemioMutation {
    #[default]
    NoMutation,
    /// 🧨 Full-snapshot replace — the only way to change SUBSET KIND (there is no sparse
    /// representation for "this artifact used to be a video, now it's a flow").
    SetSnapshot {
        snapshot: SemioSnapshot,
    },
    Brep(SemioBrepMutation),
    Mesh(SemioMeshMutation),
    Model(SemioModelMutation),
    Value(SemioValueMutation),
    Document(SemioDocumentMutation),
    Cad(SemioCadMutation),
    Drawing(SemioDrawingMutation),
    Image(SemioImageMutation),
    Video(SemioVideoMutation),
    Audio(SemioAudioMutation),
    Animation(SemioAnimationMutation),
    Presentation(SemioPresentationMutation),
    Flow(SemioFlowMutation),
    Text(SemioTextMutation),
    Table(SemioTableMutation),
    Graph(SemioGraphMutation),
    Object(SemioObjectMutation),
    Kit(SemioKitMutation),
}

/// 🏷️ Kebab-case spelling of every `SemioMutation` variant, in declaration order — the vocabulary
/// the `semio-v1-any` mutation catalog (`../../🧪️oracle/🔣️component.json`) declares and
/// `mutate-semio-any`'s exhaustive test case measures itself against. The eighteen wrapper variants
/// are spelled by their SUBSET name (`brep`, `mesh`, …), which is what the envelope actually routes
/// on; `kinds_match_the_enum_and_the_catalog` below pins the list with a WILDCARD-FREE match, so a
/// nineteenth subset cannot be added without extending both it and `KINDS`.
pub const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "brep",
    "mesh",
    "model",
    "value",
    "document",
    "cad",
    "drawing",
    "image",
    "video",
    "audio",
    "animation",
    "presentation",
    "flow",
    "text",
    "table",
    "graph",
    "object",
    "kit",
];

impl Mutation<SemioSnapshot> for SemioMutation {
    type Diff = SemioDiff;

    fn diff(&self, base: &SemioSnapshot) -> protocol::MutationOutcome<Self::Diff> {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioMutation::NoMutation, _) => protocol::MutationOutcome::new(SemioDiff::NoChange),
            (SemioMutation::SetSnapshot { snapshot }, _) => protocol::MutationOutcome::new(SemioDiff::Replace(Box::new(snapshot.clone()))),
            (SemioMutation::Brep(m), S::Brep(b)) => <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::diff(m, b).map(SemioDiff::Brep),
            (SemioMutation::Mesh(m), S::Mesh(b)) => <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::diff(m, b).map(SemioDiff::Mesh),
            (SemioMutation::Model(m), S::Model(b)) => <SemioModelMutation as Mutation<SemioModelSnapshot>>::diff(m, b).map(SemioDiff::Model),
            (SemioMutation::Value(m), S::Value(b)) => <SemioValueMutation as Mutation<SemioValueSnapshot>>::diff(m, b).map(SemioDiff::Value),
            (SemioMutation::Document(m), S::Document(b)) => <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::diff(m, b).map(SemioDiff::Document),
            (SemioMutation::Cad(m), S::Cad(b)) => <SemioCadMutation as Mutation<SemioCadSnapshot>>::diff(m, b).map(SemioDiff::Cad),
            (SemioMutation::Drawing(m), S::Drawing(b)) => <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::diff(m, b).map(SemioDiff::Drawing),
            (SemioMutation::Image(m), S::Image(b)) => <SemioImageMutation as Mutation<SemioImageSnapshot>>::diff(m, b).map(SemioDiff::Image),
            (SemioMutation::Video(m), S::Video(b)) => <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::diff(m, b).map(SemioDiff::Video),
            (SemioMutation::Audio(m), S::Audio(b)) => <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::diff(m, b).map(SemioDiff::Audio),
            (SemioMutation::Animation(m), S::Animation(b)) => <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::diff(m, b).map(SemioDiff::Animation),
            (SemioMutation::Presentation(m), S::Presentation(b)) => <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::diff(m, b).map(SemioDiff::Presentation),
            (SemioMutation::Flow(m), S::Flow(b)) => <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::diff(m, b).map(SemioDiff::Flow),
            (SemioMutation::Text(m), S::Text(b)) => <SemioTextMutation as Mutation<SemioTextSnapshot>>::diff(m, b).map(SemioDiff::Text),
            (SemioMutation::Table(m), S::Table(b)) => <SemioTableMutation as Mutation<SemioTableSnapshot>>::diff(m, b).map(SemioDiff::Table),
            (SemioMutation::Graph(m), S::Graph(b)) => <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::diff(m, b).map(SemioDiff::Graph),
            (SemioMutation::Object(m), S::Object(b)) => <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::diff(m, b).map(SemioDiff::Object),
            (SemioMutation::Kit(m), S::Kit(b)) => <SemioKitMutation as Mutation<SemioKitSnapshot>>::diff(m, b).map(SemioDiff::Kit),
            _ => protocol::MutationOutcome::error("mutation.target-missing", "Mutation subset does not match the snapshot subset.", ["subset"]),
        }
    }

    fn inverse(&self, base: &SemioSnapshot) -> Vec<Self> {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioMutation::NoMutation, _) => vec![SemioMutation::NoMutation],
            (SemioMutation::SetSnapshot { .. }, _) => vec![SemioMutation::SetSnapshot { snapshot: base.clone() }],
            (SemioMutation::Brep(m), S::Brep(b)) => <SemioBrepMutation as Mutation<SemioBrepSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Brep).collect(),
            (SemioMutation::Mesh(m), S::Mesh(b)) => <SemioMeshMutation as Mutation<SemioMeshSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Mesh).collect(),
            (SemioMutation::Model(m), S::Model(b)) => <SemioModelMutation as Mutation<SemioModelSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Model).collect(),
            (SemioMutation::Value(m), S::Value(b)) => <SemioValueMutation as Mutation<SemioValueSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Value).collect(),
            (SemioMutation::Document(m), S::Document(b)) => <SemioDocumentMutation as Mutation<SemioDocumentSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Document).collect(),
            (SemioMutation::Cad(m), S::Cad(b)) => <SemioCadMutation as Mutation<SemioCadSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Cad).collect(),
            (SemioMutation::Drawing(m), S::Drawing(b)) => <SemioDrawingMutation as Mutation<SemioDrawingSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Drawing).collect(),
            (SemioMutation::Image(m), S::Image(b)) => <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Image).collect(),
            (SemioMutation::Video(m), S::Video(b)) => <SemioVideoMutation as Mutation<SemioVideoSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Video).collect(),
            (SemioMutation::Audio(m), S::Audio(b)) => <SemioAudioMutation as Mutation<SemioAudioSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Audio).collect(),
            (SemioMutation::Animation(m), S::Animation(b)) => <SemioAnimationMutation as Mutation<SemioAnimationSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Animation).collect(),
            (SemioMutation::Presentation(m), S::Presentation(b)) => <SemioPresentationMutation as Mutation<SemioPresentationSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Presentation).collect(),
            (SemioMutation::Flow(m), S::Flow(b)) => <SemioFlowMutation as Mutation<SemioFlowSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Flow).collect(),
            (SemioMutation::Text(m), S::Text(b)) => <SemioTextMutation as Mutation<SemioTextSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Text).collect(),
            (SemioMutation::Table(m), S::Table(b)) => <SemioTableMutation as Mutation<SemioTableSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Table).collect(),
            (SemioMutation::Graph(m), S::Graph(b)) => <SemioGraphMutation as Mutation<SemioGraphSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Graph).collect(),
            (SemioMutation::Object(m), S::Object(b)) => <SemioObjectMutation as Mutation<SemioObjectSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Object).collect(),
            (SemioMutation::Kit(m), S::Kit(b)) => <SemioKitMutation as Mutation<SemioKitSnapshot>>::inverse(m, b).into_iter().map(SemioMutation::Kit).collect(),
            // 🛡️ Same kind-mismatch fallback as `diff()` above.
            _ => vec![SemioMutation::NoMutation],
        }
    }
}

/// ▶️ Applies a mutation to `snapshot` in place, returning the diff (mirrors gif's
/// `apply_gif_mutation` convention — used by the builder's `mutate()` and the set-snapshot leaf).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply_semio_mutation(snapshot: &mut SemioSnapshot, mutation: &SemioMutation) -> protocol::MutationOutcome<SemioDiff> {
    let outcome = <SemioMutation as Mutation<SemioSnapshot>>::diff(mutation, snapshot);
    outcome.apply_to(snapshot)
}

/// ↩️ Free-function face of [`Mutation::inverse`], named only in this subset's own reachable types.
/// `protocol` is a private `extern crate semio_framework_os_kernel as protocol;` alias that nothing
/// re-exports, so an owner-root test adapter compiled as an external crate cannot bring the
/// `Mutation` trait into scope to call the method form — the structural gap wave 7 recorded for
/// `kit`/`object`/`text`/`table`, and the same thin-wrapper remedy `kit` adopted. Used by
/// `mutate-semio-any`'s `inverse-*` scenarios.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse_semio_mutation(mutation: &SemioMutation, base: &SemioSnapshot) -> Vec<SemioMutation> {
    <SemioMutation as Mutation<SemioSnapshot>>::inverse(mutation, base)
}

/// 🚦️ The messages in `outcome` that genuinely REFUSE the mutation — `Error` and `Fatal` only,
/// rendered for a failure report. The frozen mutation-outcome contract
/// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md` §C2)
/// makes `Info`/`Warning` ADVISORY: they ride along with a diff that WAS applied in full, and the
/// contract's own worked example is `.info("mutation.cascade", …)` — which `✳️brep`'s
/// `delete-vertex` and `✳️graph`'s `delete-node` both raise on every well-formed body, naming the
/// edges the deletion also had to remove. A caller that reads "any message" as "rejected" therefore
/// reports a refusal that never happened and fails a scenario the codec answered correctly.
///
/// Generic over the diff type and declared ONCE here rather than eighteen times, because the
/// question is the envelope's, not any one arm's: every `mutate-semio-*` adapter needs it, and none
/// of them can name this crate's private `protocol` extern-crate alias to ask it directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusals<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome
        .messages()
        .iter()
        .filter(|message| message.level >= protocol::Severity::Error)
        .map(|message| format!("{:?} {:?}: {}", message.level, message.code, message.message))
        .collect()
}

/// 🚦️ The FAULT CODES of the refusing messages in `outcome`, in order — the same `Error`/`Fatal`
/// filter [`semio_mutation_refusals`] applies, reduced to the frozen `mutation.*` code alone. The
/// envelope's own routing law is stated in terms of codes (a mismatched arm must raise exactly
/// `mutation.target-missing`), and `mutate-semio-any` cannot read `MutationMessage::code` itself
/// because `protocol` is a private extern-crate alias of this crate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusal_codes<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome.messages().iter().filter(|message| message.level >= protocol::Severity::Error).map(|message| message.code.0.clone()).collect()
}

/// 🏷️ Free-function face of the envelope's own subset discriminator — the tag `KINDS` spells for
/// each wrapper variant and the only observable an owner-root test can read back out of a routed
/// envelope without naming any of the eighteen arms' snapshot types.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_subset_tag(snapshot: &SemioSnapshot) -> &'static str {
    crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::subset_tag(&snapshot.subset)
}
//#endregion 🔖️Mutation

//#region OpCodecs
/// 🎙️ Real delegating text/binary op codec — replaces the old whole-enum `serde_json` passthrough.
/// Text is one `tag:payload` line: `payload` for the 13 wrapped variants is exactly that subset's
/// OWN already-real `OpText::print_op()`/`parse_op()` output (genuine reuse, never re-derived
/// here); `setSnapshot`'s payload is hex(`SemioSnapshot::print_dsl`) — real delegation to this
/// envelope's own now-real `ArtifactDsl` (📸️snapshot/🦀️component.rs), hex-flattened to keep
/// `print_op`'s one-physical-line contract; `noMutation` carries no payload.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn subset_mutation_tag(m: &SemioMutation) -> &'static str {
    match m {
        SemioMutation::NoMutation => "noMutation",
        SemioMutation::SetSnapshot { .. } => "setSnapshot",
        SemioMutation::Brep(_) => "brep",
        SemioMutation::Mesh(_) => "mesh",
        SemioMutation::Model(_) => "model",
        SemioMutation::Value(_) => "value",
        SemioMutation::Document(_) => "document",
        SemioMutation::Cad(_) => "cad",
        SemioMutation::Drawing(_) => "drawing",
        SemioMutation::Image(_) => "image",
        SemioMutation::Video(_) => "video",
        SemioMutation::Audio(_) => "audio",
        SemioMutation::Animation(_) => "animation",
        SemioMutation::Presentation(_) => "presentation",
        SemioMutation::Flow(_) => "flow",
        SemioMutation::Text(_) => "text",
        SemioMutation::Table(_) => "table",
        SemioMutation::Graph(_) => "graph",
        SemioMutation::Object(_) => "object",
        SemioMutation::Kit(_) => "kit",
    }
}

/// 🏷️ Binary tag ordinal for [`SemioMutation`] — `0` = `NoMutation`, `1` = `SetSnapshot`,
/// `2..=19` = the 18 wrapped subset kinds (enum declaration order).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn mutation_tag(m: &SemioMutation) -> u8 {
    match m {
        SemioMutation::NoMutation => 0,
        SemioMutation::SetSnapshot { .. } => 1,
        SemioMutation::Brep(_) => 2,
        SemioMutation::Mesh(_) => 3,
        SemioMutation::Model(_) => 4,
        SemioMutation::Value(_) => 5,
        SemioMutation::Document(_) => 6,
        SemioMutation::Cad(_) => 7,
        SemioMutation::Drawing(_) => 8,
        SemioMutation::Image(_) => 9,
        SemioMutation::Video(_) => 10,
        SemioMutation::Audio(_) => 11,
        SemioMutation::Animation(_) => 12,
        SemioMutation::Presentation(_) => 13,
        SemioMutation::Flow(_) => 14,
        SemioMutation::Text(_) => 15,
        SemioMutation::Table(_) => 16,
        SemioMutation::Graph(_) => 17,
        SemioMutation::Object(_) => 18,
        SemioMutation::Kit(_) => 19,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_hex_snapshot(snapshot: &SemioSnapshot) -> String {
    let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(snapshot);
    text.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_hex_snapshot(hex: &str) -> Result<SemioSnapshot, String> {
    if hex.len() % 2 != 0 {
        return Err("setSnapshot: odd hex length".to_string());
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut i = 0usize;
    while i < hex.len() {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| format!("setSnapshot: invalid hex: {e}"))?;
        bytes.push(byte);
        i += 2;
    }
    let text = String::from_utf8(bytes).map_err(|e| format!("setSnapshot: utf8 decode: {e}"))?;
    <SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|e| format!("setSnapshot: dsl decode: {e}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_mutation(m: &SemioMutation) -> String {
    let tag = subset_mutation_tag(m);
    match m {
        SemioMutation::NoMutation => tag.to_string(),
        SemioMutation::SetSnapshot { snapshot } => format!("{tag}:{}", enc_hex_snapshot(snapshot)),
        SemioMutation::Brep(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Mesh(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Model(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Value(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Document(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Cad(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Drawing(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Image(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Video(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Audio(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Animation(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Presentation(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Flow(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Text(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Table(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Graph(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Object(m) => format!("{tag}:{}", m.print_op()),
        SemioMutation::Kit(m) => format!("{tag}:{}", m.print_op()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_mutation(line: &str) -> Result<SemioMutation, String> {
    if line == "noMutation" {
        return Ok(SemioMutation::NoMutation);
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("semio mutation: missing ':' in {line:?}"))?;
    match tag {
        "setSnapshot" => Ok(SemioMutation::SetSnapshot { snapshot: dec_hex_snapshot(rest)? }),
        "brep" => Ok(SemioMutation::Brep(SemioBrepMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "mesh" => Ok(SemioMutation::Mesh(SemioMeshMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "model" => Ok(SemioMutation::Model(SemioModelMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "value" => Ok(SemioMutation::Value(SemioValueMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "document" => Ok(SemioMutation::Document(SemioDocumentMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "cad" => Ok(SemioMutation::Cad(SemioCadMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "drawing" => Ok(SemioMutation::Drawing(SemioDrawingMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "image" => Ok(SemioMutation::Image(SemioImageMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "video" => Ok(SemioMutation::Video(SemioVideoMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "audio" => Ok(SemioMutation::Audio(SemioAudioMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "animation" => Ok(SemioMutation::Animation(SemioAnimationMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "presentation" => Ok(SemioMutation::Presentation(SemioPresentationMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "flow" => Ok(SemioMutation::Flow(SemioFlowMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "text" => Ok(SemioMutation::Text(SemioTextMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "table" => Ok(SemioMutation::Table(SemioTableMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "graph" => Ok(SemioMutation::Graph(SemioGraphMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "object" => Ok(SemioMutation::Object(SemioObjectMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        "kit" => Ok(SemioMutation::Kit(SemioKitMutation::parse_op(rest).map_err(|e| e.to_string())?)),
        other => Err(format!("semio mutation: unknown tag {other:?}")),
    }
}

impl OpText for SemioMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_semio_mutation(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    fn print_op(&self) -> String {
        print_semio_mutation(self)
    }
}

impl OpBinary for SemioMutation {
    /// ⚡️ Real delegating binary: `format u8` + `tag u8` ([`mutation_tag`]) as two genuine,
    /// individually protocol-walkable fixed header fields, then ONE opaque trailing payload — for
    /// the 13 wrapped variants, the wrapped subset's OWN real `OpBinary::encode_op()` bytes
    /// (genuine reuse); for `SetSnapshot`, the wrapped snapshot's own real
    /// `ArtifactPack::encode_pack()` bytes; `NoMutation` carries no payload.
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut out = vec![OP_BINARY_FORMAT, mutation_tag(self)];
        let payload: Vec<u8> = match self {
            SemioMutation::NoMutation => Vec::new(),
            SemioMutation::SetSnapshot { snapshot } => <SemioSnapshot as store::ArtifactPack>::encode_pack(snapshot),
            SemioMutation::Brep(m) => m.encode_op()?,
            SemioMutation::Mesh(m) => m.encode_op()?,
            SemioMutation::Model(m) => m.encode_op()?,
            SemioMutation::Value(m) => m.encode_op()?,
            SemioMutation::Document(m) => m.encode_op()?,
            SemioMutation::Cad(m) => m.encode_op()?,
            SemioMutation::Drawing(m) => m.encode_op()?,
            SemioMutation::Image(m) => m.encode_op()?,
            SemioMutation::Video(m) => m.encode_op()?,
            SemioMutation::Audio(m) => m.encode_op()?,
            SemioMutation::Animation(m) => m.encode_op()?,
            SemioMutation::Presentation(m) => m.encode_op()?,
            SemioMutation::Flow(m) => m.encode_op()?,
            SemioMutation::Text(m) => m.encode_op()?,
            SemioMutation::Table(m) => m.encode_op()?,
            SemioMutation::Graph(m) => m.encode_op()?,
            SemioMutation::Object(m) => m.encode_op()?,
            SemioMutation::Kit(m) => m.encode_op()?,
        };
        out.extend_from_slice(&payload);
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "op header", offset: 0, detail: "truncated".to_string() });
        }
        let format = bytes[0];
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported format {format}") });
        }
        let tag = bytes[1];
        let payload = &bytes[2..];
        Ok(match tag {
            0 => SemioMutation::NoMutation,
            1 => SemioMutation::SetSnapshot { snapshot: <SemioSnapshot as store::ArtifactPack>::decode_pack(payload)? },
            2 => SemioMutation::Brep(SemioBrepMutation::decode_op(payload)?),
            3 => SemioMutation::Mesh(SemioMeshMutation::decode_op(payload)?),
            4 => SemioMutation::Model(SemioModelMutation::decode_op(payload)?),
            5 => SemioMutation::Value(SemioValueMutation::decode_op(payload)?),
            6 => SemioMutation::Document(SemioDocumentMutation::decode_op(payload)?),
            7 => SemioMutation::Cad(SemioCadMutation::decode_op(payload)?),
            8 => SemioMutation::Drawing(SemioDrawingMutation::decode_op(payload)?),
            9 => SemioMutation::Image(SemioImageMutation::decode_op(payload)?),
            10 => SemioMutation::Video(SemioVideoMutation::decode_op(payload)?),
            11 => SemioMutation::Audio(SemioAudioMutation::decode_op(payload)?),
            12 => SemioMutation::Animation(SemioAnimationMutation::decode_op(payload)?),
            13 => SemioMutation::Presentation(SemioPresentationMutation::decode_op(payload)?),
            14 => SemioMutation::Flow(SemioFlowMutation::decode_op(payload)?),
            15 => SemioMutation::Text(SemioTextMutation::decode_op(payload)?),
            16 => SemioMutation::Table(SemioTableMutation::decode_op(payload)?),
            17 => SemioMutation::Graph(SemioGraphMutation::decode_op(payload)?),
            18 => SemioMutation::Object(SemioObjectMutation::decode_op(payload)?),
            19 => SemioMutation::Kit(SemioKitMutation::decode_op(payload)?),
            other => return Err(protocol::ProtocolError::Malformed { what: "op tag", offset: 1, detail: format!("unknown tag {other}") }),
        })
    }
}
//#endregion OpCodecs

//#region 🔖️Demo
/// 🌱 All 18 top-level [`SemioMutation`] tags (`NoMutation`, `SetSnapshot`, and each of the 16
/// wrapped-kind representative variants) — full dispatch-table coverage for this facet's
/// grammar/protocol conformance-law tests. Single source of truth shared with
/// `🎹️composer/🦀️component.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`. `text`/
/// `table`/`graph` (UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W2a/W2b), `brep` (DKM #2550, SMO's
/// approved 13-verb vocabulary), and `mesh` (DKM #2550, this subset's own 17-verb rewrite) have no
/// bare-no-op-variant equivalent — that vocabulary is banned for new facets (`📓️taxonomy.md`) — so
/// each's representative case is a real out-of-range/absent-target op, a genuine no-op at the
/// snapshot-apply level.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioMutation> {
    vec![
        SemioMutation::NoMutation,
        SemioMutation::SetSnapshot { snapshot: SemioSnapshot::default() },
        SemioMutation::Brep(SemioBrepMutation::DeleteVertex(crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::delete_vertex::mutation::DeleteVertex { id: "v-absent".into() })),
        SemioMutation::Mesh(SemioMeshMutation::DeleteMesh(crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::delete_mesh::mutation::DeleteMesh { id: "mesh-absent".into() })),
        SemioMutation::Model(SemioModelMutation::NoMutation),
        SemioMutation::Value(SemioValueMutation::NoMutation),
        SemioMutation::Document(SemioDocumentMutation::NoMutation),
        SemioMutation::Cad(SemioCadMutation::NoMutation),
        SemioMutation::Drawing(SemioDrawingMutation::DragNodes(crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::drag_nodes::mutation::DragNodes {
            ats: Vec::new(),
            offset: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2::default(),
        })),
        SemioMutation::Image(SemioImageMutation::NoMutation),
        SemioMutation::Video(SemioVideoMutation::NoMutation),
        SemioMutation::Audio(SemioAudioMutation::NoMutation),
        SemioMutation::Animation(SemioAnimationMutation::NoMutation),
        SemioMutation::Presentation(SemioPresentationMutation::NoMutation),
        SemioMutation::Flow(SemioFlowMutation::NoMutation),
        SemioMutation::Text(SemioTextMutation::RemoveRun(crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::remove_run::mutation::RemoveRun { index: 99 })),
        SemioMutation::Table(SemioTableMutation::RemoveRow(crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::remove_row::mutation::RemoveRow { index: 99 })),
        SemioMutation::Graph(SemioGraphMutation::DeleteNode(crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::delete_node::mutation::DeleteNode {
            id: crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::GraphNodeId::new("absent"),
        })),
        SemioMutation::Object(SemioObjectMutation::DeleteBrep(crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::delete_brep::mutation::DeleteBrep {})),
        SemioMutation::Kit(SemioKitMutation::RemoveType(crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::remove_type::mutation::RemoveType { id: "absent".into() })),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{FlowNode, SemioFlowSnapshot};
    use protocol::command::DiffAlgebra;
    use protocol::MutationDiff;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn audio_base() -> SemioSnapshot {
        SemioSnapshot { subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate: 44_100, format: SemioAudioFormat::Pcm16, ..Default::default() }), ..Default::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn flow_base() -> SemioSnapshot {
        SemioSnapshot { subset: SemioSubsetSnapshot::Flow(SemioFlowSnapshot::default()), ..Default::default() }
    }

    /// 🧪️ mutation_diff_law + inverse_law: `NoMutation`, `SetSnapshot` (cross-kind), and a real
    /// wrapped per-field mutation (`Audio(SetSampleRate)`).
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_covers_no_mutation_set_snapshot_and_a_wrapped_variant() {
        let base = audio_base();

        let no_mut = SemioMutation::NoMutation;
        let d0 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&no_mut, &base);
        assert_eq!(d0.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), base);

        let target = flow_base();
        let set_snap = SemioMutation::SetSnapshot { snapshot: target.clone() };
        let d1 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&set_snap, &base);
        assert_eq!(d1.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), target);
        let inv1 = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&set_snap, &base);
        let mut round = target.clone();
        let _ = apply_semio_mutation(&mut round, &inv1[0]);
        assert_eq!(round, base);

        let wrapped = SemioMutation::Audio(SemioAudioMutation::SetSampleRate { sample_rate: 96_000 });
        let d2 = <SemioMutation as Mutation<SemioSnapshot>>::diff(&wrapped, &base);
        assert!(matches!(d2.diff(), SemioDiff::Audio(_)));
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &wrapped);
        assert_eq!(d2.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, d2);
        let inv2 = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base);
        assert_eq!(inv2.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv2[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ mutation_diff_law, second wrapped subset (flow's id-keyed `InsertNode`) — proves
    /// the dispatch works for a collection-shaped mutation, not just a scalar one.
    #[semio_framework_async_macros::async_test]
    async fn mutation_diff_law_flow_insert_node() {
        let base = flow_base();
        let node = FlowNode { id: "n1".into(), kind: "task".into(), label: "N1".into(), params: vec![], position: SemioPoint2 { x: 1.0, y: 2.0 } };
        let wrapped = SemioMutation::Flow(SemioFlowMutation::InsertNode { node: node.clone() });
        let mut applied = base.clone();
        let diff = apply_semio_mutation(&mut applied, &wrapped);
        assert!(matches!(diff.diff(), SemioDiff::Flow(_)));
        match &applied.subset {
            SemioSubsetSnapshot::Flow(s) => assert_eq!(s.nodes, vec![node]),
            other => panic!("expected Flow, got {other:?}"),
        }
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ A wrapped mutation for the wrong kind remains unapplied and records its mismatch diagnostic.
    #[semio_framework_async_macros::async_test]
    async fn kind_mismatch_wrapped_mutation_records_an_error_outcome() {
        let base = flow_base();
        let wrapped = SemioMutation::Audio(SemioAudioMutation::SetSampleRate { sample_rate: 1 });
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&wrapped, &base);
        assert_eq!(diff.diff(), &SemioDiff::NoChange);
        assert!(diff.messages().await.iter().any(|message| message.code.0 == "mutation.target-missing"));
        assert_eq!(diff.await.diff().apply(&base).unwrap(), base);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&wrapped, &base);
        assert_eq!(inv, vec![SemioMutation::NoMutation]);
    }

    /// 🧪️ Dispatch-table coverage: every one of the (now 11) legacy wrapped-kind arms round-trips a
    /// `NoMutation`-shaped payload (proves the exhaustive `diff`/`inverse` match compiles and routes
    /// correctly for every subset still carrying that vocabulary). `text` (UNIFIED-COMPOSABLE-
    /// ARTIFACT-SYSTEM W2a), `brep` (DKM #2550, SMO's approved 13-verb vocabulary), and `mesh`
    /// (DKM #2550, this subset's own 17-verb rewrite) have no `NoMutation`-equivalent variant —
    /// that vocabulary is banned for new facets — so each is exercised separately below
    /// (`wrapped_text_kind_diff_and_inverse_route_correctly`,
    /// `wrapped_brep_kind_diff_and_inverse_route_correctly`,
    /// `wrapped_mesh_kind_diff_and_inverse_route_correctly`), NOT folded into this loop's
    /// `is_empty()`/`inv.len() == 1` assumptions (a real `NoMutation` variant's inverse is always
    /// `vec![NoMutation]` unconditionally — brep's/mesh's absent-target verbs correctly return
    /// `Vec::new()` instead, since there is nothing to undo, which is the opposite shape this loop
    /// assumes).
    #[semio_framework_async_macros::async_test]
    async fn all_eleven_wrapped_kinds_diff_and_inverse_route_correctly() {
        let bases: Vec<SemioSubsetSnapshot> = vec![
            SemioSubsetSnapshot::Model(Default::default()),
            SemioSubsetSnapshot::Value(Default::default()),
            SemioSubsetSnapshot::Document(Default::default()),
            SemioSubsetSnapshot::Cad(Default::default()),
            SemioSubsetSnapshot::Drawing(Default::default()),
            SemioSubsetSnapshot::Image(Default::default()),
            SemioSubsetSnapshot::Video(Default::default()),
            SemioSubsetSnapshot::Audio(Default::default()),
            SemioSubsetSnapshot::Animation(Default::default()),
            SemioSubsetSnapshot::Presentation(Default::default()),
            SemioSubsetSnapshot::Flow(Default::default()),
        ];
        // 🔧️ `match` stays exhaustive over all 14 `SemioSubsetSnapshot` arms (compiler-enforced);
        // the `Text` arm is never reached since `bases` above excludes it.
        let wrap_absent_mutation = |s: &SemioSubsetSnapshot| -> SemioMutation {
            match s {
                SemioSubsetSnapshot::Brep(_) => unreachable!("excluded from bases above"),
                SemioSubsetSnapshot::Mesh(_) => unreachable!("excluded from bases above"),
                SemioSubsetSnapshot::Model(_) => SemioMutation::Model(SemioModelMutation::NoMutation),
                SemioSubsetSnapshot::Value(_) => SemioMutation::Value(SemioValueMutation::NoMutation),
                SemioSubsetSnapshot::Document(_) => SemioMutation::Document(SemioDocumentMutation::NoMutation),
                SemioSubsetSnapshot::Cad(_) => SemioMutation::Cad(SemioCadMutation::NoMutation),
                SemioSubsetSnapshot::Drawing(_) => {
                    SemioMutation::Drawing(SemioDrawingMutation::DragNodes(crate::artifacts::semio::standards::v1::subsets::drawing::schema::mutations::drag_nodes::mutation::DragNodes { ats: Vec::new(), offset: SemioPoint2::default() }))
                }
                SemioSubsetSnapshot::Image(_) => SemioMutation::Image(SemioImageMutation::NoMutation),
                SemioSubsetSnapshot::Video(_) => SemioMutation::Video(SemioVideoMutation::NoMutation),
                SemioSubsetSnapshot::Audio(_) => SemioMutation::Audio(SemioAudioMutation::NoMutation),
                SemioSubsetSnapshot::Animation(_) => SemioMutation::Animation(SemioAnimationMutation::NoMutation),
                SemioSubsetSnapshot::Presentation(_) => SemioMutation::Presentation(SemioPresentationMutation::NoMutation),
                SemioSubsetSnapshot::Flow(_) => SemioMutation::Flow(SemioFlowMutation::NoMutation),
                SemioSubsetSnapshot::Text(_) => unreachable!("excluded from `bases` above"),
                SemioSubsetSnapshot::Table(_) => unreachable!("excluded from `bases` above"),
                SemioSubsetSnapshot::Graph(_) => unreachable!("excluded from `bases` above"),
                SemioSubsetSnapshot::Object(_) => unreachable!("excluded from `bases` above"),
                SemioSubsetSnapshot::Kit(_) => unreachable!("excluded from `bases` above"),
            }
        };
        for subset in bases {
            let base = SemioSnapshot { schema: "stdio.semio".into(), subset };
            let m = wrap_absent_mutation(&base.subset);
            let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
            assert!(diff.await.diff().is_empty(), "wrapped NoMutation must diff empty: {diff:?}");
            let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
            assert_eq!(inv.await.len(), 1);
        }
    }

    /// 🧪️ `text`'s own wrapped-kind coverage: a real `InsertRun` routes through the any-level
    /// dispatch, produces a nested `SemioDiff::Text`, and its inverse restores `base` exactly.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_text_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::text::schema::mutations::insert_run;
        use crate::artifacts::semio::standards::v1::subsets::text::schema::snapshot::SemioTextRun;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Text(Default::default()) };
        let m = SemioMutation::Text(SemioTextMutation::InsertRun(insert_run::mutation::InsertRun { index: 0, run: SemioTextRun { language: "en".into(), content: "hi".into(), marks: vec![] } }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Text(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ `brep`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above, same reason:
    /// DKM #2550 migrated `✳️brep` off the banned `NoMutation`/`SetSnapshot` vocabulary onto SMO's
    /// approved 13-verb table). A real `CreateVertex` routes through the any-level dispatch,
    /// produces a nested `SemioDiff::Brep`, and its inverse restores `base` exactly.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_brep_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::brep::schema::mutations::create_vertex;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Brep(Default::default()) };
        let m = SemioMutation::Brep(SemioBrepMutation::CreateVertex(create_vertex::mutation::CreateVertex { id: "v1".into(), point: crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Brep(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ `mesh`'s own wrapped-kind coverage (mirrors `wrapped_brep_kind_…` above, same reason:
    /// DKM #2550 migrated `✳️mesh` off the banned `NoMutation`/`SetSnapshot` vocabulary onto its
    /// own 17-verb table). A real `CreateMesh` routes through the any-level dispatch, produces a
    /// nested `SemioDiff::Mesh`, and its inverse restores `base` exactly.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_mesh_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::mesh::schema::mutations::create_mesh;
        use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMesh;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Mesh(Default::default()) };
        let m = SemioMutation::Mesh(SemioMeshMutation::CreateMesh(create_mesh::mutation::CreateMesh { mesh: SemioMesh { id: "m1".into(), primitives: vec![] } }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Mesh(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ `table`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
    /// `InsertRow` routes through the any-level dispatch, produces a nested `SemioDiff::Table`, and
    /// its inverse restores `base` exactly.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_table_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::table::schema::mutations::insert_row;
        use crate::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableRow;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Table(Default::default()) };
        let m = SemioMutation::Table(SemioTableMutation::InsertRow(insert_row::mutation::InsertRow { index: 0, row: SemioTableRow { cells: vec![] } }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Table(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ `graph`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
    /// `CreateNode` routes through the any-level dispatch, produces a nested `SemioDiff::Graph`,
    /// and its inverse restores `base` exactly.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_graph_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint2;
        use crate::artifacts::semio::standards::v1::subsets::graph::schema::mutations::create_node;
        use crate::artifacts::semio::standards::v1::subsets::graph::schema::snapshot::GraphNodeId;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Graph(Default::default()) };
        let m = SemioMutation::Graph(SemioGraphMutation::CreateNode(create_node::mutation::CreateNode { id: GraphNodeId::new("n1"), kind: "task".into(), label: "N1".into(), position: SemioPoint2::default(), ports: vec![], properties: vec![] }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Graph(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ `object`'s own wrapped-kind coverage (mirrors `wrapped_text_kind_…` above): a real
    /// `CreateBrep` routes through the any-level dispatch, produces a nested `SemioDiff::Object`,
    /// and its inverse restores `base` exactly. `object` is the first COMPOSITE subset wrapped
    /// here — the mutation touches a CHILD slot (`brep`), not a scalar/collection field.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_object_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::object::schema::mutations::create_brep;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Object(Default::default()) };
        let target = store::os_io::ArtifactRef { artifact_id: "brep-x".into(), dialect: store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: "brep".into() } };
        let m = SemioMutation::Object(SemioObjectMutation::CreateBrep(create_brep::mutation::CreateBrep { child_id: "b1".into(), target }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Object(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ `kit`'s own wrapped-kind coverage (mirrors `wrapped_object_kind_…` above): a real
    /// `AddType` routes through the any-level dispatch, produces a nested `SemioDiff::Kit`, and its
    /// inverse restores `base` exactly. `kit` is the SECOND composite subset and the first to carry
    /// a LINK slot, though this particular case exercises a plain value-collection mutation.
    #[semio_framework_async_macros::async_test]
    async fn wrapped_kit_kind_diff_and_inverse_route_correctly() {
        use crate::artifacts::semio::standards::v1::subsets::kit::schema::mutations::add_type;

        let base = SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Kit(Default::default()) };
        let m = SemioMutation::Kit(SemioKitMutation::AddType(add_type::mutation::AddType { id: "chair".into(), name: "Chair".into(), category: "furniture".into() }));
        let diff = <SemioMutation as Mutation<SemioSnapshot>>::diff(&m, &base);
        assert!(matches!(diff.await.diff(), SemioDiff::Kit(_)));
        assert!(!diff.await.diff().is_empty());
        let mut applied = base.clone();
        let returned_diff = apply_semio_mutation(&mut applied, &m);
        assert_eq!(diff.await.diff().apply(&base).expect("apply must succeed for a well-formed fixture"), applied);
        assert_eq!(returned_diff, diff);
        let inv = <SemioMutation as Mutation<SemioSnapshot>>::inverse(&m, &base);
        assert_eq!(inv.await.len(), 1);
        let mut restored = applied;
        let _ = apply_semio_mutation(&mut restored, &inv[0]);
        assert_eq!(restored, base);
    }

    /// 🧪️ op_text_binary_roundtrip_law across `NoMutation`, `SetSnapshot`, and a wrapped variant.
    #[semio_framework_async_macros::async_test]
    async fn op_text_binary_roundtrip_law() {
        let base = audio_base();
        let cases = [SemioMutation::NoMutation, SemioMutation::SetSnapshot { snapshot: base.clone() }, SemioMutation::Audio(SemioAudioMutation::SetSampleRate { sample_rate: 22_050 }), SemioMutation::Flow(SemioFlowMutation::NoMutation)];
        for m in cases {
            let printed = m.print_op();
            assert!(!printed.await.contains('\n'), "print_op must be one line, got {printed:?}");
            let parsed = SemioMutation::parse_op(&printed).await.unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
            assert_eq!(parsed, m, "print_op/parse_op round-trip mismatch for {m:?}");

            let encoded = m.encode_op().await.unwrap_or_else(|e| panic!("encode_op({m:?}) failed: {e}"));
            let decoded = SemioMutation::decode_op(&encoded).await.unwrap_or_else(|e| panic!("decode_op failed: {e}"));
            assert_eq!(decoded, m, "encode_op/decode_op round-trip mismatch for {m:?}");
        }
    }

    //#region 🔖️CatalogLaw
    /// 🏷️ The wildcard-free spelling map that makes [`KINDS`] compiler-checked: a nineteenth arm has
    /// no case here, so the crate stops building until both this match and `KINDS` name it.
    fn kind_of(mutation: &SemioMutation) -> &'static str {
        match mutation {
            SemioMutation::NoMutation => "no-mutation",
            SemioMutation::SetSnapshot { .. } => "set-snapshot",
            SemioMutation::Brep(_) => "brep",
            SemioMutation::Mesh(_) => "mesh",
            SemioMutation::Model(_) => "model",
            SemioMutation::Value(_) => "value",
            SemioMutation::Document(_) => "document",
            SemioMutation::Cad(_) => "cad",
            SemioMutation::Drawing(_) => "drawing",
            SemioMutation::Image(_) => "image",
            SemioMutation::Video(_) => "video",
            SemioMutation::Audio(_) => "audio",
            SemioMutation::Animation(_) => "animation",
            SemioMutation::Presentation(_) => "presentation",
            SemioMutation::Flow(_) => "flow",
            SemioMutation::Text(_) => "text",
            SemioMutation::Table(_) => "table",
            SemioMutation::Graph(_) => "graph",
            SemioMutation::Object(_) => "object",
            SemioMutation::Kit(_) => "kit",
        }
    }

    fn enveloped(subset: SemioSubsetSnapshot) -> SemioSnapshot {
        SemioSnapshot { schema: crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::STDIO_SEMIO_DOCUMENT_SCHEMA.into(), subset }
    }

    /// 🏷️ `KINDS` must name every declared variant, in declaration order and in the exact spelling
    /// the committed `semio-v1-any` catalog carries — the framework never parses Rust, so this is
    /// the only thing that keeps the catalog honest against the enum. The eighteen wrapper spellings
    /// are checked against `semio_subset_tag`, the envelope's OWN runtime discriminator, rather than
    /// against a second hand-written list: a routed mutation that reported under a name the catalog
    /// does not know would otherwise pass unnoticed. The two envelope-owned verbs are nullary and
    /// full-replace, so they are checked directly.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let arms = [
            SemioSubsetSnapshot::Brep(Default::default()),
            SemioSubsetSnapshot::Mesh(Default::default()),
            SemioSubsetSnapshot::Model(Default::default()),
            SemioSubsetSnapshot::Value(Default::default()),
            SemioSubsetSnapshot::Document(Default::default()),
            SemioSubsetSnapshot::Cad(Default::default()),
            SemioSubsetSnapshot::Drawing(Default::default()),
            SemioSubsetSnapshot::Image(Default::default()),
            SemioSubsetSnapshot::Video(Default::default()),
            SemioSubsetSnapshot::Audio(Default::default()),
            SemioSubsetSnapshot::Animation(Default::default()),
            SemioSubsetSnapshot::Presentation(Default::default()),
            SemioSubsetSnapshot::Flow(Default::default()),
            SemioSubsetSnapshot::Text(Default::default()),
            SemioSubsetSnapshot::Table(Default::default()),
            SemioSubsetSnapshot::Graph(Default::default()),
            SemioSubsetSnapshot::Object(Default::default()),
            SemioSubsetSnapshot::Kit(Default::default()),
        ];
        assert_eq!(KINDS.len(), arms.len() + 2, "KINDS must name the two envelope-owned verbs plus exactly one entry per subset arm");
        assert_eq!(KINDS[0], kind_of(&SemioMutation::NoMutation), "the nullary verb comes first");
        assert_eq!(KINDS[1], kind_of(&SemioMutation::SetSnapshot { snapshot: SemioSnapshot::default() }), "the full-replace verb comes second");
        for (kind, arm) in KINDS[2..].iter().zip(arms) {
            assert_eq!(*kind, semio_subset_tag(&enveloped(arm)), "KINDS must follow SemioSubsetSnapshot's own declaration order and the envelope's own runtime subset tag");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
    }
    //#endregion 🔖️CatalogLaw
}
//#endregion 🔖️Tests

//#region 🧪️FixtureCases
/// 🧪️ Handcrafted `📄set-snapshot` fixture cases, wired from this tree's own mutations root so
/// `📦️glue.rs` stays untouched (`#[path]` on a non-inline module resolves against this file's own
/// directory).
#[cfg(test)]
#[path = "📄set-snapshot/🧪️tests/replaces-the-envelope-wrapping-a-value-subset/🦀️component.rs"]
mod set_snapshot_replaces_the_envelope_wrapping_a_value_subset;
//#endregion 🧪️FixtureCases
