//! 🔺️ SemioDiff — the envelope union's own diff: the same-kind nested diff of the subset the snapshot carries.
//! The diff nests that subset's own REAL `DiffAlgebra`-driven diff (`SemioBrepDiff`, `SemioAudioDiff`, …) unchanged —
//! zero reinvention of any subset's own sparse-diff algebra. A genuine cross-kind change has no diff at all: it is a
//! document load, never a mutation.

use crate::standards::v1::subsets::animation::schema::{diff::SemioAnimationDiff, snapshot::SemioAnimationSnapshot};
use crate::standards::v1::subsets::audio::schema::{diff::SemioAudioDiff, snapshot::SemioAudioSnapshot};
use crate::standards::v1::subsets::base::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
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
use protocol::{DiffCodec};
use protocol::MutationApplyError;
use protocol::MutationDiff;

//#region 🔖️Diff
/// 🔺️ `NoChange` and the same-kind wrappers; a subset-kind change has no sparse representation and is a document load.
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
}

impl MutationDiff<SemioSnapshot> for SemioDiff {
    fn apply(&self, base: &SemioSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioSnapshot> {
        use SemioSubsetSnapshot as S;
        let subset = match (self, &base.subset) {
            (SemioDiff::NoChange, s) => s.clone(),
            (SemioDiff::Rejected(error), _) => return Err(error.clone()),
            (SemioDiff::Brep(d), S::Brep(b)) => S::Brep(d.apply(b, capability).map_err(|error| error.under(["subset", "brep"]))?),
            (SemioDiff::Mesh(d), S::Mesh(b)) => S::Mesh(d.apply(b, capability).map_err(|error| error.under(["subset", "mesh"]))?),
            (SemioDiff::Model(d), S::Model(b)) => S::Model(d.apply(b, capability).map_err(|error| error.under(["subset", "model"]))?),
            (SemioDiff::Value(d), S::Value(b)) => S::Value(d.apply(b, capability).map_err(|error| error.under(["subset", "value"]))?),
            (SemioDiff::Document(d), S::Document(b)) => S::Document(d.apply(b, capability).map_err(|error| error.under(["subset", "document"]))?),
            (SemioDiff::Cad(d), S::Cad(b)) => S::Cad(d.apply(b, capability).map_err(|error| error.under(["subset", "cad"]))?),
            (SemioDiff::Drawing(d), S::Drawing(b)) => S::Drawing(d.apply(b, capability).map_err(|error| error.under(["subset", "drawing"]))?),
            (SemioDiff::Image(d), S::Image(b)) => S::Image(d.apply(b, capability).map_err(|error| error.under(["subset", "image"]))?),
            (SemioDiff::Video(d), S::Video(b)) => S::Video(d.apply(b, capability).map_err(|error| error.under(["subset", "video"]))?),
            (SemioDiff::Audio(d), S::Audio(b)) => S::Audio(d.apply(b, capability).map_err(|error| error.under(["subset", "audio"]))?),
            (SemioDiff::Animation(d), S::Animation(b)) => S::Animation(d.apply(b, capability).map_err(|error| error.under(["subset", "animation"]))?),
            (SemioDiff::Presentation(d), S::Presentation(b)) => S::Presentation(d.apply(b, capability).map_err(|error| error.under(["subset", "presentation"]))?),
            (SemioDiff::Flow(d), S::Flow(b)) => S::Flow(d.apply(b, capability).map_err(|error| error.under(["subset", "flow"]))?),
            (SemioDiff::Text(d), S::Text(b)) => S::Text(d.apply(b, capability).map_err(|error| error.under(["subset", "text"]))?),
            (SemioDiff::Table(d), S::Table(b)) => S::Table(d.apply(b, capability).map_err(|error| error.under(["subset", "table"]))?),
            (SemioDiff::Graph(d), S::Graph(b)) => S::Graph(d.apply(b, capability).map_err(|error| error.under(["subset", "graph"]))?),
            (SemioDiff::Object(d), S::Object(b)) => S::Object(d.apply(b, capability).map_err(|error| error.under(["subset", "object"]))?),
            (SemioDiff::Kit(d), S::Kit(b)) => S::Kit(d.apply(b, capability).map_err(|error| error.under(["subset", "kit"]))?),
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
            _ => Rejected(MutationApplyError::new("mutation.apply.kind-mismatch", "Semio subset diffs of different kinds cannot be composed").at(["subset"])),
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
            // 🧭 Different kinds have no sparse representation: a subset-kind change is a document load, never a diff.
            _ => {
                if base == other {
                    SemioDiff::NoChange
                } else {
                    SemioDiff::Rejected(MutationApplyError::new("mutation.apply.kind-mismatch", "Semio subset kinds differ; a subset-kind change is a document load, not a diff").at(["subset"]))
                }
            }
        }
    }

    fn inverse(&self, base: &SemioSnapshot) -> Self {
        use SemioSubsetSnapshot as S;
        match (self, &base.subset) {
            (SemioDiff::NoChange, _) => SemioDiff::NoChange,
            (SemioDiff::Rejected(error), _) => SemioDiff::Rejected(error.clone()),
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
            _ => SemioDiff::Rejected(MutationApplyError::new("mutation.apply.kind-mismatch", "Semio subset diff kind does not match the base snapshot kind").at(["subset"])),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            SemioDiff::NoChange => true,
            SemioDiff::Rejected(_) => false,
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

//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec















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
    cases
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
