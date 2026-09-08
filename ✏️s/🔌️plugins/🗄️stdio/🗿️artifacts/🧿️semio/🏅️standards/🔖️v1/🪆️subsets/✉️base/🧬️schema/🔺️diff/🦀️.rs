//! 🔺️ SemioDiff — the envelope union's own diff, per the master plan: "same-kind nested diff |
//! Replace{snapshot}". W2b closer real implementation, replacing the W1b always-full-replace
//! scaffold: when `base`/`other` carry the SAME subset kind, the diff nests that subset's own
//! REAL `DiffAlgebra`-driven diff (`SemioBrepDiff`, `SemioAudioDiff`, …) unchanged — zero
//! reinvention of any of the 13 subsets' own sparse-diff algebra. Only a genuine cross-kind
//! change (or an explicit `SetSnapshot` mutation) ever produces `Replace` — the same
//! "same-kind-nested | Replace" split gif/svg's own recursive-node diffs use for their own
//! heterogeneous variant trees, applied here one level up at the artifact-subset boundary.

use crate::standards::v1::subsets::animation::schema::{diff::SemioAnimationDiff, snapshot::SemioAnimationSnapshot};
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::standards::v1::subsets::audio::schema::{diff::SemioAudioDiff, snapshot::SemioAudioSnapshot};
use crate::standards::v1::subsets::brep::schema::{diff::SemioBrepDiff, snapshot::SemioBrepSnapshot};
use crate::standards::v1::subsets::cad::schema::{diff::SemioCadDiff, snapshot::SemioCadSnapshot};
use crate::standards::v1::subsets::document::schema::{diff::SemioDocumentDiff, snapshot::SemioDocumentSnapshot};
use crate::standards::v1::subsets::drawing::schema::{diff::SemioDrawingDiff, snapshot::SemioDrawingSnapshot};
use crate::standards::v1::subsets::flow::schema::{diff::SemioFlowDiff, snapshot::SemioFlowSnapshot};
use crate::standards::v1::subsets::graph::schema::{diff::SemioGraphDiff, snapshot::SemioGraphSnapshot};
use crate::standards::v1::subsets::image::schema::{diff::SemioImageDiff, snapshot::SemioImageSnapshot};
use crate::standards::v1::subsets::kit::schema::{diff::SemioKitDiff, snapshot::SemioKitSnapshot};
use crate::standards::v1::subsets::mesh::schema::{diff::SemioMeshDiff, snapshot::SemioMeshSnapshot};
use crate::standards::v1::subsets::model::schema::{diff::SemioModelDiff, snapshot::SemioModelSnapshot};
use crate::standards::v1::subsets::object::schema::{diff::SemioObjectDiff, snapshot::SemioObjectSnapshot};
use crate::standards::v1::subsets::presentation::schema::{diff::SemioPresentationDiff, snapshot::SemioPresentationSnapshot};
use crate::standards::v1::subsets::table::schema::{diff::SemioTableDiff, snapshot::SemioTableSnapshot};
use crate::standards::v1::subsets::text::schema::{diff::SemioTextDiff, snapshot::SemioTextSnapshot};
use crate::standards::v1::subsets::value::schema::{diff::SemioValueTreeDiff, snapshot::SemioValueSnapshot};
use crate::standards::v1::subsets::video::schema::{diff::SemioVideoDiff, snapshot::SemioVideoSnapshot};
use protocol::command::DiffAlgebra;
use protocol::DiffCodec;
use protocol::MutationApplyError;
use protocol::MutationDiff;

//#region 🔖️Diff
/// 🔺️ `NoChange` and the 13 same-kind wrappers are the common case (a mutation stayed within its
/// subset); `Replace` is the escape hatch for the two cases that genuinely have no sparse
/// representation: an explicit whole-snapshot `SetSnapshot` mutation, and `between(a, b)` where
/// `a`/`b` carry DIFFERENT subset kinds (there is no such thing as a "sparse diff" between, say,
/// a brep and a video — the kind itself changed). `Box` keeps this enum's own stack size small
/// despite embedding 13 heterogeneous, Vec-heavy nested diff types.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum SemioDiff {
    #[default]
    NoChange,
    Rejected(MutationApplyError),
    Brep(SemioBrepDiff),
    Mesh(SemioMeshDiff),
    Model(SemioModelDiff),
    Value(SemioValueTreeDiff),
    Document(SemioDocumentDiff),
    Cad(SemioCadDiff),
    Drawing(SemioDrawingDiff),
    Image(SemioImageDiff),
    Video(SemioVideoDiff),
    Audio(SemioAudioDiff),
    Animation(SemioAnimationDiff),
    Presentation(SemioPresentationDiff),
    Flow(SemioFlowDiff),
    Text(SemioTextDiff),
    Table(SemioTableDiff),
    Graph(SemioGraphDiff),
    Object(SemioObjectDiff),
    Kit(SemioKitDiff),
    Replace(Box<SemioSnapshot>),
}

impl MutationDiff<SemioSnapshot> for SemioDiff {
    fn apply(&self, base: &SemioSnapshot) -> protocol::MutationApplyResult<SemioSnapshot> {
        use SemioSubsetSnapshot as S;
        let subset = match (self, &base.subset) {
            (SemioDiff::NoChange, s) => s.clone(),
            (SemioDiff::Rejected(error), _) => return Err(error.clone()),
            (SemioDiff::Replace(snapshot), _) => return Ok((**snapshot).clone()),
            (SemioDiff::Brep(d), S::Brep(b)) => S::Brep(d.apply(b).map_err(|error| error.under(["subset", "brep"]))?),
            (SemioDiff::Mesh(d), S::Mesh(b)) => S::Mesh(d.apply(b).map_err(|error| error.under(["subset", "mesh"]))?),
            (SemioDiff::Model(d), S::Model(b)) => S::Model(d.apply(b).map_err(|error| error.under(["subset", "model"]))?),
            (SemioDiff::Value(d), S::Value(b)) => S::Value(d.apply(b).map_err(|error| error.under(["subset", "value"]))?),
            (SemioDiff::Document(d), S::Document(b)) => S::Document(d.apply(b).map_err(|error| error.under(["subset", "document"]))?),
            (SemioDiff::Cad(d), S::Cad(b)) => S::Cad(d.apply(b).map_err(|error| error.under(["subset", "cad"]))?),
            (SemioDiff::Drawing(d), S::Drawing(b)) => S::Drawing(d.apply(b).map_err(|error| error.under(["subset", "drawing"]))?),
            (SemioDiff::Image(d), S::Image(b)) => S::Image(d.apply(b).map_err(|error| error.under(["subset", "image"]))?),
            (SemioDiff::Video(d), S::Video(b)) => S::Video(d.apply(b).map_err(|error| error.under(["subset", "video"]))?),
            (SemioDiff::Audio(d), S::Audio(b)) => S::Audio(d.apply(b).map_err(|error| error.under(["subset", "audio"]))?),
            (SemioDiff::Animation(d), S::Animation(b)) => S::Animation(d.apply(b).map_err(|error| error.under(["subset", "animation"]))?),
            (SemioDiff::Presentation(d), S::Presentation(b)) => S::Presentation(d.apply(b).map_err(|error| error.under(["subset", "presentation"]))?),
            (SemioDiff::Flow(d), S::Flow(b)) => S::Flow(d.apply(b).map_err(|error| error.under(["subset", "flow"]))?),
            (SemioDiff::Text(d), S::Text(b)) => S::Text(d.apply(b).map_err(|error| error.under(["subset", "text"]))?),
            (SemioDiff::Table(d), S::Table(b)) => S::Table(d.apply(b).map_err(|error| error.under(["subset", "table"]))?),
            (SemioDiff::Graph(d), S::Graph(b)) => S::Graph(d.apply(b).map_err(|error| error.under(["subset", "graph"]))?),
            (SemioDiff::Object(d), S::Object(b)) => S::Object(d.apply(b).map_err(|error| error.under(["subset", "object"]))?),
            (SemioDiff::Kit(d), S::Kit(b)) => S::Kit(d.apply(b).map_err(|error| error.under(["subset", "kit"]))?),
            _ => {
                return Err(MutationApplyError::new("mutation.apply.kind-mismatch", "Semio subset diff kind does not match the base snapshot kind").at(["subset"]));
            }
        };
        Ok(SemioSnapshot { schema: base.schema.clone(), subset })
    }

    fn absorb(&mut self, other: Self) {
        use SemioDiff::*;
        let combined = match (std::mem::take(self), other) {
            (Rejected(error), _) | (_, Rejected(error)) => Rejected(error),
            (NoChange, o) => o,
            (s, NoChange) => s,
            // 🧨 A later full replace always wins outright, regardless of what came before —
            // matches every other artifact's own "hard reset supersedes prior incremental diffs"
            // absorb convention.
            (_, Replace(s2)) => Replace(s2),
            // 🪢 An earlier replace absorbing a LATER same/foreign-kind diff: fold the later diff
            // into the replacement snapshot by re-using this impl's own `apply` (self-consistent,
            // no duplicated dispatch logic) and keep the result as the new replacement.
            (Replace(s1), o) => match o.apply(&s1) {
                Ok(snapshot) => Replace(Box::new(snapshot)),
                Err(error) => Rejected(error),
            },
            (Brep(mut d1), Brep(d2)) => {
                d1.absorb(d2);
                Brep(d1)
            }
            (Mesh(mut d1), Mesh(d2)) => {
                d1.absorb(d2);
                Mesh(d1)
            }
            (Model(mut d1), Model(d2)) => {
                d1.absorb(d2);
                Model(d1)
            }
            (Value(mut d1), Value(d2)) => {
                d1.absorb(d2);
                Value(d1)
            }
            (Document(mut d1), Document(d2)) => {
                d1.absorb(d2);
                Document(d1)
            }
            (Cad(mut d1), Cad(d2)) => {
                d1.absorb(d2);
                Cad(d1)
            }
            (Drawing(mut d1), Drawing(d2)) => {
                d1.absorb(d2);
                Drawing(d1)
            }
            (Image(mut d1), Image(d2)) => {
                d1.absorb(d2);
                Image(d1)
            }
            (Video(mut d1), Video(d2)) => {
                d1.absorb(d2);
                Video(d1)
            }
            (Audio(mut d1), Audio(d2)) => {
                d1.absorb(d2);
                Audio(d1)
            }
            (Animation(mut d1), Animation(d2)) => {
                d1.absorb(d2);
                Animation(d1)
            }
            (Presentation(mut d1), Presentation(d2)) => {
                d1.absorb(d2);
                Presentation(d1)
            }
            (Flow(mut d1), Flow(d2)) => {
                d1.absorb(d2);
                Flow(d1)
            }
            (Text(mut d1), Text(d2)) => {
                d1.absorb(d2);
                Text(d1)
            }
            (Table(mut d1), Table(d2)) => {
                d1.absorb(d2);
                Table(d1)
            }
            (Graph(mut d1), Graph(d2)) => {
                d1.absorb(d2);
                Graph(d1)
            }
            (Object(mut d1), Object(d2)) => {
                d1.absorb(d2);
                Object(d1)
            }
            (Kit(mut d1), Kit(d2)) => {
                d1.absorb(d2);
                Kit(d1)
            }
            _ => Rejected(MutationApplyError::new("mutation.absorb.kind-mismatch", "Semio subset diffs of different kinds cannot be composed").at(["subset"])),
        };
        *self = combined;
    }
}

impl DiffAlgebra<SemioSnapshot> for SemioDiff {
    fn between(base: &SemioSnapshot, other: &SemioSnapshot) -> Self {
        use SemioSubsetSnapshot as S;
        match (&base.subset, &other.subset) {
            (S::Brep(b), S::Brep(o)) => SemioDiff::Brep(<SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(b, o)),
            (S::Mesh(b), S::Mesh(o)) => SemioDiff::Mesh(<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::between(b, o)),
            (S::Model(b), S::Model(o)) => SemioDiff::Model(<SemioModelDiff as DiffAlgebra<SemioModelSnapshot>>::between(b, o)),
            (S::Value(b), S::Value(o)) => SemioDiff::Value(<SemioValueTreeDiff as DiffAlgebra<SemioValueSnapshot>>::between(b, o)),
            (S::Document(b), S::Document(o)) => SemioDiff::Document(<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::between(b, o)),
            (S::Cad(b), S::Cad(o)) => SemioDiff::Cad(<SemioCadDiff as DiffAlgebra<SemioCadSnapshot>>::between(b, o)),
            (S::Drawing(b), S::Drawing(o)) => SemioDiff::Drawing(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::between(b, o)),
            (S::Image(b), S::Image(o)) => SemioDiff::Image(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::between(b, o)),
            (S::Video(b), S::Video(o)) => SemioDiff::Video(<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::between(b, o)),
            (S::Audio(b), S::Audio(o)) => SemioDiff::Audio(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::between(b, o)),
            (S::Animation(b), S::Animation(o)) => SemioDiff::Animation(<SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::between(b, o)),
            (S::Presentation(b), S::Presentation(o)) => SemioDiff::Presentation(<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::between(b, o)),
            (S::Flow(b), S::Flow(o)) => SemioDiff::Flow(<SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::between(b, o)),
            (S::Text(b), S::Text(o)) => SemioDiff::Text(<SemioTextDiff as DiffAlgebra<SemioTextSnapshot>>::between(b, o)),
            (S::Table(b), S::Table(o)) => SemioDiff::Table(<SemioTableDiff as DiffAlgebra<SemioTableSnapshot>>::between(b, o)),
            (S::Graph(b), S::Graph(o)) => SemioDiff::Graph(<SemioGraphDiff as DiffAlgebra<SemioGraphSnapshot>>::between(b, o)),
            (S::Object(b), S::Object(o)) => SemioDiff::Object(<SemioObjectDiff as DiffAlgebra<SemioObjectSnapshot>>::between(b, o)),
            (S::Kit(b), S::Kit(o)) => SemioDiff::Kit(<SemioKitDiff as DiffAlgebra<SemioKitSnapshot>>::between(b, o)),
            // 🧭 Different kinds (or, degenerately, the exact same value): a cross-kind change has
            // no sparse representation, so it's `Replace`; an identical pair collapses to `NoChange`
            // so `between(a, a).is_empty()` holds even when `a`/`b` happen to share a reference.
            _ => {
                if base == other {
                    SemioDiff::NoChange
                } else {
                    SemioDiff::Replace(Box::new(other.clone()))
                }
            }
        }
    }

    fn inverse(&self, base: &SemioSnapshot) -> Self {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioDiff::NoChange, _) => SemioDiff::NoChange,
            (SemioDiff::Rejected(error), _) => SemioDiff::Rejected(error.clone()),
            (SemioDiff::Replace(_), _) => SemioDiff::Replace(Box::new(base.clone())),
            (SemioDiff::Brep(d), S::Brep(b)) => SemioDiff::Brep(<SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::inverse(d, b)),
            (SemioDiff::Mesh(d), S::Mesh(b)) => SemioDiff::Mesh(<SemioMeshDiff as DiffAlgebra<SemioMeshSnapshot>>::inverse(d, b)),
            (SemioDiff::Model(d), S::Model(b)) => SemioDiff::Model(<SemioModelDiff as DiffAlgebra<SemioModelSnapshot>>::inverse(d, b)),
            (SemioDiff::Value(d), S::Value(b)) => SemioDiff::Value(<SemioValueTreeDiff as DiffAlgebra<SemioValueSnapshot>>::inverse(d, b)),
            (SemioDiff::Document(d), S::Document(b)) => SemioDiff::Document(<SemioDocumentDiff as DiffAlgebra<SemioDocumentSnapshot>>::inverse(d, b)),
            (SemioDiff::Cad(d), S::Cad(b)) => SemioDiff::Cad(<SemioCadDiff as DiffAlgebra<SemioCadSnapshot>>::inverse(d, b)),
            (SemioDiff::Drawing(d), S::Drawing(b)) => SemioDiff::Drawing(<SemioDrawingDiff as DiffAlgebra<SemioDrawingSnapshot>>::inverse(d, b)),
            (SemioDiff::Image(d), S::Image(b)) => SemioDiff::Image(<SemioImageDiff as DiffAlgebra<SemioImageSnapshot>>::inverse(d, b)),
            (SemioDiff::Video(d), S::Video(b)) => SemioDiff::Video(<SemioVideoDiff as DiffAlgebra<SemioVideoSnapshot>>::inverse(d, b)),
            (SemioDiff::Audio(d), S::Audio(b)) => SemioDiff::Audio(<SemioAudioDiff as DiffAlgebra<SemioAudioSnapshot>>::inverse(d, b)),
            (SemioDiff::Animation(d), S::Animation(b)) => SemioDiff::Animation(<SemioAnimationDiff as DiffAlgebra<SemioAnimationSnapshot>>::inverse(d, b)),
            (SemioDiff::Presentation(d), S::Presentation(b)) => SemioDiff::Presentation(<SemioPresentationDiff as DiffAlgebra<SemioPresentationSnapshot>>::inverse(d, b)),
            (SemioDiff::Flow(d), S::Flow(b)) => SemioDiff::Flow(<SemioFlowDiff as DiffAlgebra<SemioFlowSnapshot>>::inverse(d, b)),
            (SemioDiff::Text(d), S::Text(b)) => SemioDiff::Text(<SemioTextDiff as DiffAlgebra<SemioTextSnapshot>>::inverse(d, b)),
            (SemioDiff::Table(d), S::Table(b)) => SemioDiff::Table(<SemioTableDiff as DiffAlgebra<SemioTableSnapshot>>::inverse(d, b)),
            (SemioDiff::Graph(d), S::Graph(b)) => SemioDiff::Graph(<SemioGraphDiff as DiffAlgebra<SemioGraphSnapshot>>::inverse(d, b)),
            (SemioDiff::Object(d), S::Object(b)) => SemioDiff::Object(<SemioObjectDiff as DiffAlgebra<SemioObjectSnapshot>>::inverse(d, b)),
            (SemioDiff::Kit(d), S::Kit(b)) => SemioDiff::Kit(<SemioKitDiff as DiffAlgebra<SemioKitSnapshot>>::inverse(d, b)),
            _ => SemioDiff::Rejected(MutationApplyError::new("mutation.inverse.kind-mismatch", "Semio subset diff kind does not match the base snapshot kind").at(["subset"])),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            SemioDiff::NoChange => true,
            SemioDiff::Rejected(_) => false,
            SemioDiff::Replace(_) => false,
            SemioDiff::Brep(d) => d.is_empty(),
            SemioDiff::Mesh(d) => d.is_empty(),
            SemioDiff::Model(d) => d.is_empty(),
            SemioDiff::Value(d) => d.is_empty(),
            SemioDiff::Document(d) => d.is_empty(),
            SemioDiff::Cad(d) => d.is_empty(),
            SemioDiff::Drawing(d) => d.is_empty(),
            SemioDiff::Image(d) => d.is_empty(),
            SemioDiff::Video(d) => d.is_empty(),
            SemioDiff::Audio(d) => d.is_empty(),
            SemioDiff::Animation(d) => d.is_empty(),
            SemioDiff::Presentation(d) => d.is_empty(),
            SemioDiff::Flow(d) => d.is_empty(),
            SemioDiff::Text(d) => d.is_empty(),
            SemioDiff::Table(d) => d.is_empty(),
            SemioDiff::Graph(d) => d.is_empty(),
            SemioDiff::Object(d) => d.is_empty(),
            SemioDiff::Kit(d) => d.is_empty(),
        }
    }
}

/// 🧩 Set-snapshot diff helper — used by the `📸️set-snapshot/🔺️diff` leaf.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_set_snapshot(base: &SemioSnapshot, snapshot: &SemioSnapshot) -> SemioDiff {
    <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Handcrafted `protocol::DiffCodec` — one `tag:payload` line, where `payload` for the 13
/// same-kind variants is exactly that subset's OWN already-real, already-hand-rolled
/// `print_diff()`/`parse_diff()` output (genuine reuse — this module never re-derives any of the
/// 13 subsets' own bracket/triple grammars). `Replace`'s payload is hex(`SemioSnapshot::print_dsl`)
/// — real delegation to THIS envelope's own now-real `ArtifactDsl` (📸️snapshot/🦀️.rs,
/// itself a real delegating codec over the same 13 subsets), hex-flattened to keep `print_diff`'s
/// mandatory one-physical-line contract despite `print_dsl`'s own embedded newlines.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_replace_snapshot(snapshot: &SemioSnapshot) -> String {
    let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(snapshot);
    text.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_replace_snapshot(hex: &str) -> Result<SemioSnapshot, String> {
    if !hex.len().is_multiple_of(2) {
        return Err("replace: odd hex length".to_string());
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    let mut i = 0usize;
    while i < hex.len() {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).map_err(|e| format!("replace: invalid hex: {e}"))?;
        bytes.push(byte);
        i += 2;
    }
    let text = String::from_utf8(bytes).map_err(|e| format!("replace: utf8 decode: {e}"))?;
    <SemioSnapshot as store::ArtifactDsl>::parse_dsl(&text).map_err(|e| format!("replace: dsl decode: {e}"))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_rejection(error: &MutationApplyError) -> String {
    std::iter::once(error.code.as_str())
        .chain(std::iter::once(error.message.as_str()))
        .chain(error.target.iter().map(String::as_str))
        .map(|value| value.as_bytes().iter().map(|byte| format!("{byte:02x}")).collect::<String>())
        .collect::<Vec<_>>()
        .join(",")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_rejection(payload: &str) -> Result<MutationApplyError, String> {
    let fields = payload
        .split(',')
        .map(|hex| {
            if hex.len() % 2 != 0 {
                return Err("rejected: odd hex length".to_string());
            }
            let bytes = (0..hex.len()).step_by(2).map(|index| u8::from_str_radix(&hex[index..index + 2], 16)).collect::<Result<Vec<_>, _>>().map_err(|error| format!("rejected: invalid hex: {error}"))?;
            String::from_utf8(bytes).map_err(|error| format!("rejected: utf8 decode: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if fields.len() < 2 {
        return Err("rejected: expected code and message".to_string());
    }
    Ok(MutationApplyError { code: fields[0].clone(), message: fields[1].clone(), target: fields[2..].to_vec() })
}

/// 🏷️ Binary tag ordinal for [`SemioDiff`] — `0` = `NoChange`, `1..=18` = the 18 wrapped subset
/// kinds (same enum declaration order as [`crate::standards::v1::subsets::base::schema::snapshot::subset_ordinal`],
/// offset by one to make room for `NoChange`), `19` = `Replace`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn diff_tag(d: &SemioDiff) -> u8 {
    match d {
        SemioDiff::NoChange => 0,
        SemioDiff::Rejected(_) => 20,
        SemioDiff::Brep(_) => 1,
        SemioDiff::Mesh(_) => 2,
        SemioDiff::Model(_) => 3,
        SemioDiff::Value(_) => 4,
        SemioDiff::Document(_) => 5,
        SemioDiff::Cad(_) => 6,
        SemioDiff::Drawing(_) => 7,
        SemioDiff::Image(_) => 8,
        SemioDiff::Video(_) => 9,
        SemioDiff::Audio(_) => 10,
        SemioDiff::Animation(_) => 11,
        SemioDiff::Presentation(_) => 12,
        SemioDiff::Flow(_) => 13,
        SemioDiff::Text(_) => 14,
        SemioDiff::Table(_) => 15,
        SemioDiff::Graph(_) => 16,
        SemioDiff::Object(_) => 17,
        SemioDiff::Kit(_) => 18,
        SemioDiff::Replace(_) => 19,
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_semio_diff(d: &SemioDiff) -> String {
    match d {
        SemioDiff::NoChange => "noChange".to_string(),
        SemioDiff::Rejected(error) => format!("rejected:{}", enc_rejection(error)),
        SemioDiff::Replace(s) => format!("replace:{}", enc_replace_snapshot(s)),
        SemioDiff::Brep(d) => format!("brep:{}", d.print_diff()),
        SemioDiff::Mesh(d) => format!("mesh:{}", d.print_diff()),
        SemioDiff::Model(d) => format!("model:{}", d.print_diff()),
        SemioDiff::Value(d) => format!("value:{}", d.print_diff()),
        SemioDiff::Document(d) => format!("document:{}", d.print_diff()),
        SemioDiff::Cad(d) => format!("cad:{}", d.print_diff()),
        SemioDiff::Drawing(d) => format!("drawing:{}", d.print_diff()),
        SemioDiff::Image(d) => format!("image:{}", d.print_diff()),
        SemioDiff::Video(d) => format!("video:{}", d.print_diff()),
        SemioDiff::Audio(d) => format!("audio:{}", d.print_diff()),
        SemioDiff::Animation(d) => format!("animation:{}", d.print_diff()),
        SemioDiff::Presentation(d) => format!("presentation:{}", d.print_diff()),
        SemioDiff::Flow(d) => format!("flow:{}", d.print_diff()),
        SemioDiff::Text(d) => format!("text:{}", d.print_diff()),
        SemioDiff::Table(d) => format!("table:{}", d.print_diff()),
        SemioDiff::Graph(d) => format!("graph:{}", d.print_diff()),
        SemioDiff::Object(d) => format!("object:{}", d.print_diff()),
        SemioDiff::Kit(d) => format!("kit:{}", d.print_diff()),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_semio_diff(line: &str) -> Result<SemioDiff, String> {
    if line == "noChange" {
        return Ok(SemioDiff::NoChange);
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("semio diff: missing ':' in {line:?}"))?;
    match tag {
        "replace" => Ok(SemioDiff::Replace(Box::new(dec_replace_snapshot(rest)?))),
        "rejected" => Ok(SemioDiff::Rejected(dec_rejection(rest)?)),
        "brep" => Ok(SemioDiff::Brep(SemioBrepDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "mesh" => Ok(SemioDiff::Mesh(SemioMeshDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "model" => Ok(SemioDiff::Model(SemioModelDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "value" => Ok(SemioDiff::Value(SemioValueTreeDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "document" => Ok(SemioDiff::Document(SemioDocumentDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "cad" => Ok(SemioDiff::Cad(SemioCadDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "drawing" => Ok(SemioDiff::Drawing(SemioDrawingDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "image" => Ok(SemioDiff::Image(SemioImageDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "video" => Ok(SemioDiff::Video(SemioVideoDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "audio" => Ok(SemioDiff::Audio(SemioAudioDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "animation" => Ok(SemioDiff::Animation(SemioAnimationDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "presentation" => Ok(SemioDiff::Presentation(SemioPresentationDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "flow" => Ok(SemioDiff::Flow(SemioFlowDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "text" => Ok(SemioDiff::Text(SemioTextDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "table" => Ok(SemioDiff::Table(SemioTableDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "graph" => Ok(SemioDiff::Graph(SemioGraphDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "object" => Ok(SemioDiff::Object(SemioObjectDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        "kit" => Ok(SemioDiff::Kit(SemioKitDiff::parse_diff(rest).map_err(|e| e.to_string())?)),
        other => Err(format!("semio diff: unknown tag {other:?}")),
    }
}

impl DiffCodec for SemioDiff {
    fn print_diff(&self) -> String {
        print_semio_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_semio_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }

    /// ⚡️ Real delegating binary: `format u8` + `tag u8` ([`diff_tag`]) as two genuine,
    /// individually protocol-walkable fixed header fields, then ONE opaque trailing payload —
    /// for the 13 same-kind variants, that payload is exactly the wrapped subset's OWN real
    /// `DiffCodec::encode_diff()` bytes (genuine reuse, never re-derived here); for `Replace`, the
    /// wrapped snapshot's own real `ArtifactPack::encode_pack()` bytes (📸️snapshot's real binary
    /// delegation, applied one level deeper); `NoChange` carries no payload at all.
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut out = vec![DIFF_BINARY_FORMAT, diff_tag(self)];
        let payload: Vec<u8> = match self {
            SemioDiff::NoChange => Vec::new(),
            SemioDiff::Rejected(error) => enc_rejection(error).into_bytes(),
            SemioDiff::Replace(s) => <SemioSnapshot as store::ArtifactPack>::encode_pack(s),
            SemioDiff::Brep(d) => d.encode_diff()?,
            SemioDiff::Mesh(d) => d.encode_diff()?,
            SemioDiff::Model(d) => d.encode_diff()?,
            SemioDiff::Value(d) => d.encode_diff()?,
            SemioDiff::Document(d) => d.encode_diff()?,
            SemioDiff::Cad(d) => d.encode_diff()?,
            SemioDiff::Drawing(d) => d.encode_diff()?,
            SemioDiff::Image(d) => d.encode_diff()?,
            SemioDiff::Video(d) => d.encode_diff()?,
            SemioDiff::Audio(d) => d.encode_diff()?,
            SemioDiff::Animation(d) => d.encode_diff()?,
            SemioDiff::Presentation(d) => d.encode_diff()?,
            SemioDiff::Flow(d) => d.encode_diff()?,
            SemioDiff::Text(d) => d.encode_diff()?,
            SemioDiff::Table(d) => d.encode_diff()?,
            SemioDiff::Graph(d) => d.encode_diff()?,
            SemioDiff::Object(d) => d.encode_diff()?,
            SemioDiff::Kit(d) => d.encode_diff()?,
        };
        out.extend_from_slice(&payload);
        Ok(out)
    }

    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated".to_string() });
        }
        let format = bytes[0];
        if format != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported format {format}") });
        }
        let tag = bytes[1];
        let payload = &bytes[2..];
        Ok(match tag {
            0 => SemioDiff::NoChange,
            1 => SemioDiff::Brep(SemioBrepDiff::decode_diff(payload)?),
            2 => SemioDiff::Mesh(SemioMeshDiff::decode_diff(payload)?),
            3 => SemioDiff::Model(SemioModelDiff::decode_diff(payload)?),
            4 => SemioDiff::Value(SemioValueTreeDiff::decode_diff(payload)?),
            5 => SemioDiff::Document(SemioDocumentDiff::decode_diff(payload)?),
            6 => SemioDiff::Cad(SemioCadDiff::decode_diff(payload)?),
            7 => SemioDiff::Drawing(SemioDrawingDiff::decode_diff(payload)?),
            8 => SemioDiff::Image(SemioImageDiff::decode_diff(payload)?),
            9 => SemioDiff::Video(SemioVideoDiff::decode_diff(payload)?),
            10 => SemioDiff::Audio(SemioAudioDiff::decode_diff(payload)?),
            11 => SemioDiff::Animation(SemioAnimationDiff::decode_diff(payload)?),
            12 => SemioDiff::Presentation(SemioPresentationDiff::decode_diff(payload)?),
            13 => SemioDiff::Flow(SemioFlowDiff::decode_diff(payload)?),
            14 => SemioDiff::Text(SemioTextDiff::decode_diff(payload)?),
            15 => SemioDiff::Table(SemioTableDiff::decode_diff(payload)?),
            16 => SemioDiff::Graph(SemioGraphDiff::decode_diff(payload)?),
            17 => SemioDiff::Object(SemioObjectDiff::decode_diff(payload)?),
            18 => SemioDiff::Kit(SemioKitDiff::decode_diff(payload)?),
            19 => SemioDiff::Replace(Box::new(<SemioSnapshot as store::ArtifactPack>::decode_pack(payload)?)),
            20 => SemioDiff::Rejected(
                dec_rejection(std::str::from_utf8(payload).map_err(|error| protocol::ProtocolError::Malformed { what: "rejected diff", offset: 2, detail: error.to_string() })?).map_err(|error| protocol::ProtocolError::Malformed {
                    what: "rejected diff",
                    offset: 2,
                    detail: error,
                })?,
            ),
            other => return Err(protocol::ProtocolError::Malformed { what: "diff tag", offset: 1, detail: format!("unknown tag {other}") }),
        })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioDiff` cases for this facet's conformance-law tests: `NoChange`, all 13
/// same-kind (empty-but-real-tagged) nested diffs, and one `Replace`. Single source of truth for
/// both this file's own round-trip test and `🎹️composer/🦀️.rs`'s `diff_grammar_
/// conformance_law`/`protocol_walk_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioDiff> {
    let subsets: Vec<SemioSubsetSnapshot> = vec![
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
    let mut cases = vec![SemioDiff::NoChange];
    for subset in subsets {
        let snap = SemioSnapshot { schema: "stdio.semio".into(), subset };
        cases.push(<SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&snap, &snap));
    }
    cases.push(SemioDiff::Replace(Box::new(SemioSnapshot { schema: "stdio.semio".into(), subset: SemioSubsetSnapshot::Flow(Default::default()) })));
    cases
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
