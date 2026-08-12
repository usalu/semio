//! 🔺️ SemioDiff — the envelope union's own diff, per the master plan: "same-kind nested diff |
//! Replace{snapshot}". W2b closer real implementation, replacing the W1b always-full-replace
//! scaffold: when `base`/`other` carry the SAME subset kind, the diff nests that subset's own
//! REAL `DiffAlgebra`-driven diff (`SemioBrepDiff`, `SemioAudioDiff`, …) unchanged — zero
//! reinvention of any of the 13 subsets' own sparse-diff algebra. Only a genuine cross-kind
//! change (or an explicit `SetSnapshot` mutation) ever produces `Replace` — the same
//! "same-kind-nested | Replace" split gif/svg's own recursive-node diffs use for their own
//! heterogeneous variant trees, applied here one level up at the artifact-subset boundary.

use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::{diff::SemioBrepDiff, snapshot::SemioBrepSnapshot};
use crate::artifacts::semio::standards::v1::subsets::mesh::schema::{diff::SemioMeshDiff, snapshot::SemioMeshSnapshot};
use crate::artifacts::semio::standards::v1::subsets::model::schema::{diff::SemioModelDiff, snapshot::SemioModelSnapshot};
use crate::artifacts::semio::standards::v1::subsets::value::schema::{diff::SemioValueTreeDiff, snapshot::SemioValueSnapshot};
use crate::artifacts::semio::standards::v1::subsets::document::schema::{diff::SemioDocumentDiff, snapshot::SemioDocumentSnapshot};
use crate::artifacts::semio::standards::v1::subsets::cad::schema::{diff::SemioCadDiff, snapshot::SemioCadSnapshot};
use crate::artifacts::semio::standards::v1::subsets::drawing::schema::{diff::SemioDrawingDiff, snapshot::SemioDrawingSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::{diff::SemioImageDiff, snapshot::SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::video::schema::{diff::SemioVideoDiff, snapshot::SemioVideoSnapshot};
use crate::artifacts::semio::standards::v1::subsets::audio::schema::{diff::SemioAudioDiff, snapshot::SemioAudioSnapshot};
use crate::artifacts::semio::standards::v1::subsets::animation::schema::{diff::SemioAnimationDiff, snapshot::SemioAnimationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::presentation::schema::{diff::SemioPresentationDiff, snapshot::SemioPresentationSnapshot};
use crate::artifacts::semio::standards::v1::subsets::flow::schema::{diff::SemioFlowDiff, snapshot::SemioFlowSnapshot};
use protocol::DiffCodec;
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ `NoChange` and the 13 same-kind wrappers are the common case (a mutation stayed within its
/// subset); `Replace` is the escape hatch for the two cases that genuinely have no sparse
/// representation: an explicit whole-snapshot `SetSnapshot` mutation, and `between(a, b)` where
/// `a`/`b` carry DIFFERENT subset kinds (there is no such thing as a "sparse diff" between, say,
/// a brep and a video — the kind itself changed). `Box` keeps this enum's own stack size small
/// despite embedding 13 heterogeneous, Vec-heavy nested diff types.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum SemioDiff {
    #[default]
    NoChange,
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
    Replace(Box<SemioSnapshot>),
}

impl MutationDiff<SemioSnapshot> for SemioDiff {
    fn apply(&self, base: &SemioSnapshot) -> SemioSnapshot {
        use SemioSubsetSnapshot as S;
        let subset = match (self, &base.subset) {
            (SemioDiff::NoChange, s) => s.clone(),
            (SemioDiff::Replace(snapshot), _) => return (**snapshot).clone(),
            (SemioDiff::Brep(d), S::Brep(b)) => S::Brep(<SemioBrepDiff as MutationDiff<SemioBrepSnapshot>>::apply(d, b)),
            (SemioDiff::Mesh(d), S::Mesh(b)) => S::Mesh(<SemioMeshDiff as MutationDiff<SemioMeshSnapshot>>::apply(d, b)),
            (SemioDiff::Model(d), S::Model(b)) => S::Model(<SemioModelDiff as MutationDiff<SemioModelSnapshot>>::apply(d, b)),
            (SemioDiff::Value(d), S::Value(b)) => S::Value(<SemioValueTreeDiff as MutationDiff<SemioValueSnapshot>>::apply(d, b)),
            (SemioDiff::Document(d), S::Document(b)) => S::Document(<SemioDocumentDiff as MutationDiff<SemioDocumentSnapshot>>::apply(d, b)),
            (SemioDiff::Cad(d), S::Cad(b)) => S::Cad(<SemioCadDiff as MutationDiff<SemioCadSnapshot>>::apply(d, b)),
            (SemioDiff::Drawing(d), S::Drawing(b)) => S::Drawing(<SemioDrawingDiff as MutationDiff<SemioDrawingSnapshot>>::apply(d, b)),
            (SemioDiff::Image(d), S::Image(b)) => S::Image(<SemioImageDiff as MutationDiff<SemioImageSnapshot>>::apply(d, b)),
            (SemioDiff::Video(d), S::Video(b)) => S::Video(<SemioVideoDiff as MutationDiff<SemioVideoSnapshot>>::apply(d, b)),
            (SemioDiff::Audio(d), S::Audio(b)) => S::Audio(<SemioAudioDiff as MutationDiff<SemioAudioSnapshot>>::apply(d, b)),
            (SemioDiff::Animation(d), S::Animation(b)) => S::Animation(<SemioAnimationDiff as MutationDiff<SemioAnimationSnapshot>>::apply(d, b)),
            (SemioDiff::Presentation(d), S::Presentation(b)) => S::Presentation(<SemioPresentationDiff as MutationDiff<SemioPresentationSnapshot>>::apply(d, b)),
            (SemioDiff::Flow(d), S::Flow(b)) => S::Flow(<SemioFlowDiff as MutationDiff<SemioFlowSnapshot>>::apply(d, b)),
            // 🛡️ Kind mismatch (a nested-kind diff paired with a base of a DIFFERENT kind): can
            // only arise from a malformed/foreign diff, never from this module's own `between`/
            // `diff` — `apply` stays TOTAL (never panics) per `MutationDiff`'s contract by
            // degrading to a safe no-op (returns `base` unchanged) rather than guessing.
            (_, s) => s.clone(),
        };
        SemioSnapshot { schema: base.schema.clone(), subset }
    }

    fn absorb(&mut self, other: Self) {
        use SemioDiff::*;
        let combined = match (std::mem::take(self), other) {
            (NoChange, o) => o,
            (s, NoChange) => s,
            // 🧨 A later full replace always wins outright, regardless of what came before —
            // matches every other artifact's own "hard reset supersedes prior incremental diffs"
            // absorb convention.
            (_, Replace(s2)) => Replace(s2),
            // 🪢 An earlier replace absorbing a LATER same/foreign-kind diff: fold the later diff
            // into the replacement snapshot by re-using this impl's own `apply` (self-consistent,
            // no duplicated dispatch logic) and keep the result as the new replacement.
            (Replace(s1), o) => Replace(Box::new(o.apply(&s1))),
            (Brep(mut d1), Brep(d2)) => { d1.absorb(d2); Brep(d1) }
            (Mesh(mut d1), Mesh(d2)) => { d1.absorb(d2); Mesh(d1) }
            (Model(mut d1), Model(d2)) => { d1.absorb(d2); Model(d1) }
            (Value(mut d1), Value(d2)) => { d1.absorb(d2); Value(d1) }
            (Document(mut d1), Document(d2)) => { d1.absorb(d2); Document(d1) }
            (Cad(mut d1), Cad(d2)) => { d1.absorb(d2); Cad(d1) }
            (Drawing(mut d1), Drawing(d2)) => { d1.absorb(d2); Drawing(d1) }
            (Image(mut d1), Image(d2)) => { d1.absorb(d2); Image(d1) }
            (Video(mut d1), Video(d2)) => { d1.absorb(d2); Video(d1) }
            (Audio(mut d1), Audio(d2)) => { d1.absorb(d2); Audio(d1) }
            (Animation(mut d1), Animation(d2)) => { d1.absorb(d2); Animation(d1) }
            (Presentation(mut d1), Presentation(d2)) => { d1.absorb(d2); Presentation(d1) }
            (Flow(mut d1), Flow(d2)) => { d1.absorb(d2); Flow(d1) }
            // 🛡️ Mismatched, non-`Replace` kind pair — structurally impossible from this module's
            // OWN sequential diffs (a kind change always goes through `Replace`), but `absorb`
            // must stay TOTAL over every pair per its trait contract; last-diff-wins is the
            // honest, never-panicking fallback for a malformed/foreign pair.
            (_, other) => other,
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
            // 🧭 Different kinds (or, degenerately, the exact same value): a cross-kind change has
            // no sparse representation, so it's `Replace`; an identical pair collapses to `NoChange`
            // so `between(a, a).is_empty()` holds even when `a`/`b` happen to share a reference.
            _ => if base == other { SemioDiff::NoChange } else { SemioDiff::Replace(Box::new(other.clone())) },
        }
    }

    fn inverse(&self, base: &SemioSnapshot) -> Self {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioDiff::NoChange, _) => SemioDiff::NoChange,
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
            // 🛡️ Kind mismatch: same total-fallback stance as `apply`/`absorb` above — the safe
            // inverse of "unknown, ill-typed change" is "restore base wholesale".
            (_, b) => SemioDiff::Replace(Box::new(SemioSnapshot { schema: base.schema.clone(), subset: b.clone() })),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            SemioDiff::NoChange => true,
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
        }
    }
}

/// 🧩 Set-snapshot diff helper — used by the `📄set-snapshot/🔺️diff` leaf.
pub fn diff_set_snapshot(base: &SemioSnapshot, snapshot: &SemioSnapshot) -> SemioDiff {
    <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(base, snapshot)
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec
/// 🎙️ Handcrafted `protocol::DiffCodec` — one `tag:payload` line, where `payload` for the 13
/// same-kind variants is exactly that subset's OWN already-real, already-hand-rolled
/// `print_diff()`/`parse_diff()` output (genuine reuse — this module never re-derives any of the
/// 13 subsets' own bracket/triple grammars). `Replace`'s payload is hex(`SemioSnapshot::print_dsl`)
/// — real delegation to THIS envelope's own now-real `ArtifactDsl` (📸️snapshot/🦀️component.rs,
/// itself a real delegating codec over the same 13 subsets), hex-flattened to keep `print_diff`'s
/// mandatory one-physical-line contract despite `print_dsl`'s own embedded newlines.
fn enc_replace_snapshot(snapshot: &SemioSnapshot) -> String {
    let text = <SemioSnapshot as store::ArtifactDsl>::print_dsl(snapshot);
    text.as_bytes().iter().map(|b| format!("{b:02x}")).collect()
}

fn dec_replace_snapshot(hex: &str) -> Result<SemioSnapshot, String> {
    if hex.len() % 2 != 0 {
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

/// 🏷️ Binary tag ordinal for [`SemioDiff`] — `0` = `NoChange`, `1..=13` = the 13 wrapped subset
/// kinds (same enum declaration order as [`crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::subset_ordinal`],
/// offset by one to make room for `NoChange`), `14` = `Replace`.
fn diff_tag(d: &SemioDiff) -> u8 {
    match d {
        SemioDiff::NoChange => 0,
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
        SemioDiff::Replace(_) => 14,
    }
}

fn print_semio_diff(d: &SemioDiff) -> String {
    match d {
        SemioDiff::NoChange => "noChange".to_string(),
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
    }
}

fn parse_semio_diff(line: &str) -> Result<SemioDiff, String> {
    if line == "noChange" {
        return Ok(SemioDiff::NoChange);
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("semio diff: missing ':' in {line:?}"))?;
    match tag {
        "replace" => Ok(SemioDiff::Replace(Box::new(dec_replace_snapshot(rest)?))),
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
        other => Err(format!("semio diff: unknown tag {other:?}")),
    }
}

impl protocol::DiffCodec for SemioDiff {
    fn print_diff(&self) -> String { print_semio_diff(self) }
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
            14 => SemioDiff::Replace(Box::new(<SemioSnapshot as store::ArtifactPack>::decode_pack(payload)?)),
            other => return Err(protocol::ProtocolError::Malformed { what: "diff tag", offset: 1, detail: format!("unknown tag {other}") }),
        })
    }
}
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioDiff` cases for this facet's conformance-law tests: `NoChange`, all 13
/// same-kind (empty-but-real-tagged) nested diffs, and one `Replace`. Single source of truth for
/// both this file's own round-trip test and `🎹️composer/🦀️component.rs`'s `diff_grammar_
/// conformance_law`/`protocol_walk_law`.
#[cfg(test)]
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
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::audio::schema::snapshot::{SemioAudioFormat, SemioAudioSnapshot};
    use crate::artifacts::semio::standards::v1::subsets::flow::schema::snapshot::{SemioFlowSnapshot, FlowNode};
    use crate::artifacts::semio::standards::v1::engine::geometry::SemioPoint2;

    fn audio_snapshot(sample_rate: u32) -> SemioSnapshot {
        SemioSnapshot {
            subset: SemioSubsetSnapshot::Audio(SemioAudioSnapshot { sample_rate, format: SemioAudioFormat::Pcm16, ..Default::default() }),
            ..Default::default()
        }
    }

    fn flow_snapshot(node_ids: &[&str]) -> SemioSnapshot {
        SemioSnapshot {
            subset: SemioSubsetSnapshot::Flow(SemioFlowSnapshot {
                nodes: node_ids.iter().map(|id| FlowNode {
                    id: (*id).into(), kind: "task".into(), label: (*id).into(), params: vec![],
                    position: SemioPoint2 { x: 0.0, y: 0.0 },
                }).collect(),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    /// 🧪️ between_roundtrip_law + field_sweep, real same-kind field change (audio's
    /// `sample_rate`, a genuinely mutable field — not the `schema` identity field every subset's
    /// own diff module explicitly excludes from diffing).
    #[test]
    fn between_roundtrip_law_same_kind_real_field_change() {
        let a = audio_snapshot(44_100);
        let b = audio_snapshot(48_000);
        let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
        assert!(matches!(d, SemioDiff::Audio(_)), "same-kind change must nest, not Replace: {d:?}");
        assert!(!d.is_empty());
        assert_eq!(d.apply(&a), b);
        assert!(<SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &a).is_empty());
    }

    /// 🧪️ field_sweep, second real same-kind field change (flow's id-keyed `nodes`
    /// collection) — sweeps a DIFFERENT subset and a DIFFERENT field shape (collection insert,
    /// not a scalar) than the audio case above.
    #[test]
    fn between_roundtrip_law_flow_node_insert() {
        let a = flow_snapshot(&["n1"]);
        let b = flow_snapshot(&["n1", "n2"]);
        let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
        assert!(matches!(d, SemioDiff::Flow(_)));
        assert_eq!(d.apply(&a), b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&d.apply(&a)), a);
    }

    /// 🧪️ between_roundtrip_law, cross-kind change: no sparse representation exists, must fall
    /// back to `Replace` — and still satisfy the law.
    #[test]
    fn between_roundtrip_law_cross_kind_replaces() {
        let a = audio_snapshot(44_100);
        let b = flow_snapshot(&["n1"]);
        let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
        assert!(matches!(d, SemioDiff::Replace(_)), "cross-kind change must Replace: {d:?}");
        assert_eq!(d.apply(&a), b);
    }

    /// 🧪️ inverse_law across all 3 shapes: same-kind nested, cross-kind Replace, and NoChange.
    #[test]
    fn inverse_law_covers_nested_replace_and_no_change() {
        for (a, b) in [
            (audio_snapshot(44_100), audio_snapshot(96_000)),
            (audio_snapshot(44_100), flow_snapshot(&["n1", "n2"])),
            (audio_snapshot(44_100), audio_snapshot(44_100)),
        ] {
            let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
            let applied = d.apply(&a);
            let inv = d.inverse(&a);
            assert_eq!(inv.apply(&applied), a, "inverse must restore base for {d:?}");
        }
    }

    /// 🧪️ absorb_law: same-kind sequential coalesce delegates to the nested subset's own
    /// (already-proven) `absorb`.
    #[test]
    fn absorb_law_same_kind_delegates_to_nested() {
        let a = audio_snapshot(44_100);
        let mid = audio_snapshot(48_000);
        let after = audio_snapshot(96_000);
        let mut d1 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &mid);
        let d2 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&mid, &after);
        let applied_before_absorb = d1.apply(&a);
        d1.absorb(d2.clone());
        assert_eq!(d1.apply(&a), d2.apply(&applied_before_absorb));
        assert_eq!(d1.apply(&a), after);
    }

    /// 🧪️ absorb_law: a later `Replace` always wins outright, whatever preceded it.
    #[test]
    fn absorb_law_later_replace_wins() {
        let a = audio_snapshot(44_100);
        let mid = audio_snapshot(48_000);
        let after = flow_snapshot(&["n1"]);
        let mut d1 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &mid);
        let d2 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&mid, &after);
        d1.absorb(d2);
        assert!(matches!(d1, SemioDiff::Replace(_)));
        assert_eq!(d1.apply(&a), after);
    }

    /// 🧪️ absorb_law: an earlier `Replace` absorbing a later same-kind diff folds it into the
    /// replacement snapshot rather than dropping it.
    #[test]
    fn absorb_law_replace_then_nested_folds_in() {
        let a = flow_snapshot(&["n1"]);
        let replaced = audio_snapshot(44_100);
        let after = audio_snapshot(48_000);
        let mut d1 = SemioDiff::Replace(Box::new(replaced.clone()));
        let d2 = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&replaced, &after);
        d1.absorb(d2);
        assert_eq!(d1.apply(&a), after);
    }

    /// 🧪️ diff_codec_text_binary_roundtrip_law across `NoChange`, a same-kind nested diff (one
    /// per subset kind, proving the dispatch table's all 13 tags), and `Replace`.
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        let a = audio_snapshot(44_100);
        let b = audio_snapshot(48_000);
        let nested = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&a, &b);
        let replace = SemioDiff::Replace(Box::new(flow_snapshot(&["n1"])));
        for d in [SemioDiff::NoChange, nested, replace] {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch for {d:?}");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch for {d:?}");
        }
    }

    /// 🧪️ Dispatch-table coverage: every one of the 13 same-kind tags round-trips through
    /// `print_diff`/`parse_diff` for the trivial `NoChange`-shaped nested diff (proves the 26-arm
    /// print/parse match is wired correctly for every subset, without re-deriving each subset's
    /// own deep field grammar here).
    #[test]
    fn all_thirteen_subset_tags_round_trip_empty_nested_diff() {
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
        ];
        for subset in subsets {
            let snap = SemioSnapshot { schema: "stdio.semio".into(), subset };
            let d = <SemioDiff as DiffAlgebra<SemioSnapshot>>::between(&snap, &snap);
            assert!(d.is_empty(), "identical snapshot must diff empty: {d:?}");
            let printed = d.print_diff();
            let parsed = SemioDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d);
        }
    }
}
//#endregion 🔖️Tests
