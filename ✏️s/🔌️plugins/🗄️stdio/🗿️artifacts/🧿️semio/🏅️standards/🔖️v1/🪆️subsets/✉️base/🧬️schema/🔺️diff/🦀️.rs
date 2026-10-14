//! 🔺️ SemioDiff — the envelope union's own diff: the same-kind nested diff of the subset the snapshot carries.
//! The diff nests that subset's own REAL `DiffAlgebra`-driven diff (`SemioBrepDiff`, `SemioAudioDiff`, …) unchanged —
//! zero reinvention of any subset's own sparse-diff algebra. A genuine cross-kind change has no diff at all: it is a
//! document load, never a mutation.

use {semio_s_artifact_stdio_semio_animation::schema::diff::SemioAnimationDiff, semio_s_artifact_stdio_semio_animation::schema::snapshot::SemioAnimationSnapshot};
use {semio_s_artifact_stdio_semio_audio_model::schema::diff::SemioAudioDiff, semio_s_artifact_stdio_semio_audio_model::schema::snapshot::SemioAudioSnapshot};
use crate::schema::snapshot::{SemioSnapshot, SemioSubsetSnapshot};
use {semio_s_artifact_stdio_semio_brep::schema::diff::SemioBrepDiff, semio_s_artifact_stdio_semio_brep::schema::snapshot::SemioBrepSnapshot};
use {semio_s_artifact_stdio_semio_cad::schema::diff::SemioCadDiff, semio_s_artifact_stdio_semio_cad::schema::snapshot::SemioCadSnapshot};
use {semio_s_artifact_stdio_semio_document::schema::diff::SemioDocumentDiff, semio_s_artifact_stdio_semio_document::schema::snapshot::SemioDocumentSnapshot};
use {semio_s_artifact_stdio_semio_drawing::schema::diff::SemioDrawingDiff, semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot};
use {semio_s_artifact_stdio_semio_flow::schema::diff::SemioFlowDiff, semio_s_artifact_stdio_semio_flow::schema::snapshot::SemioFlowSnapshot};
use {semio_s_artifact_stdio_semio_graph::schema::diff::SemioGraphDiff, semio_s_artifact_stdio_semio_graph::schema::snapshot::SemioGraphSnapshot};
use {semio_s_artifact_stdio_semio_image::schema::diff::SemioImageDiff, semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageSnapshot};
use {semio_s_artifact_stdio_semio_kit::schema::diff::SemioKitDiff, semio_s_artifact_stdio_semio_kit::schema::snapshot::SemioKitSnapshot};
use {semio_s_artifact_stdio_semio_mesh::schema::diff::SemioMeshDiff, semio_s_artifact_stdio_semio_mesh::schema::snapshot::SemioMeshSnapshot};
use {semio_s_artifact_stdio_semio_model::schema::diff::SemioModelDiff, semio_s_artifact_stdio_semio_model::schema::snapshot::SemioModelSnapshot};
use {semio_s_artifact_stdio_semio_object::schema::diff::SemioObjectDiff, semio_s_artifact_stdio_semio_object::schema::snapshot::SemioObjectSnapshot};
use {semio_s_artifact_stdio_semio_presentation::schema::diff::SemioPresentationDiff, semio_s_artifact_stdio_semio_presentation::schema::snapshot::SemioPresentationSnapshot};
use {semio_s_artifact_stdio_semio_table_model::schema::diff::SemioTableDiff, semio_s_artifact_stdio_semio_table_model::schema::snapshot::SemioTableSnapshot};
use {semio_s_artifact_stdio_semio_text_model::schema::diff::SemioTextDiff, semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot};
use {semio_s_artifact_stdio_semio_value_model::schema::diff::SemioValueTreeDiff, semio_s_artifact_stdio_semio_value_model::schema::snapshot::SemioValueSnapshot};
use {semio_s_artifact_stdio_semio_video_model::schema::diff::SemioVideoDiff, semio_s_artifact_stdio_semio_video_model::schema::snapshot::SemioVideoSnapshot};
use protocol::command::DiffAlgebra;
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
/// 🌱 Representative `SemioDiff` cases for this facet's conformance-law tests: `NoChange` and one empty-but-real-tagged nested diff per
/// subset. Single source of truth for both this file's own round-trip test and `🎹️composer/🦀️.rs`'s `diff_grammar_conformance_law`/
/// `protocol_walk_law`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioDiff> {
    vec![
        SemioDiff::NoChange,
        SemioDiff::Brep(Default::default()),
        SemioDiff::Mesh(Default::default()),
        SemioDiff::Model(Default::default()),
        SemioDiff::Value(Default::default()),
        SemioDiff::Document(Default::default()),
        SemioDiff::Cad(Default::default()),
        SemioDiff::Drawing(Default::default()),
        SemioDiff::Image(Default::default()),
        SemioDiff::Video(Default::default()),
        SemioDiff::Audio(Default::default()),
        SemioDiff::Animation(Default::default()),
        SemioDiff::Presentation(Default::default()),
        SemioDiff::Flow(Default::default()),
        SemioDiff::Text(Default::default()),
        SemioDiff::Table(Default::default()),
        SemioDiff::Graph(Default::default()),
        SemioDiff::Object(Default::default()),
        SemioDiff::Kit(Default::default()),
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests

#[cfg(test)]
use protocol::{DiffBinary,DiffText};
