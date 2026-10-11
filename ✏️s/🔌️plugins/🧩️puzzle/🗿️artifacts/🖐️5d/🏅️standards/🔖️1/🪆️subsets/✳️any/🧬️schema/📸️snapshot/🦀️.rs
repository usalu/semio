//! 🧬️ Puzzle5d snapshot schema — artifact-lane fields only.

use crate::{Puzzle5dFastener, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume, PUZZLE_5D_SCHEMA};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;


/// 🚪️ Owns the codec used by the actual subset's native declaration.
pub(crate) fn native_codec()->store::ArtifactCodec {
    store::ArtifactCodec::bare::<Puzzle5dSnapshot,crate::Puzzle5dMutation>(PUZZLE_5D_SCHEMA)
}

//#region 🔖️Snapshot
/// 📸️ Persisted puzzle5d document snapshot (persistent fields of the artifact).
///
/// 🔣️ Artifact JSON uses the first-party value codec, including the composed kit child.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
#[dsl(id = "puzzle.puzzle5d", layout = "lines")]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[value(default)]
    #[state(artifact)]
    pub domain: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[state(artifact)]
    pub label: Option<String>,
    #[value(default)]
    #[dsl(block)]
    #[state(artifact)]
    pub meta: Puzzle5dMeta,
    /// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM W4d: composed `s.stdio.semio.kit`
    /// child handle — the shared (`SemioKitType` id/name/category) half of what was the inline
    /// `Puzzle5dKindCatalogs` field. See `🗿️artifacts/🖐️5d/🦀️.rs`'s `🔖️KindCatalogComposition`
    /// region for the split/join contract and `kind_catalogs_of` accessor.
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub kind_catalogs: Option<store::ArtifactChild<SemioKitSnapshot>>,
    /// 🧩️ The puzzle5d-owned overflow half `SemioKitType` cannot represent — sibling to
    /// `kind_catalogs`, id-joined back together by `kind_catalogs_of`.
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Puzzle5dKindCatalogsExtra>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub kind_compatibility: Vec<Puzzle5dKindCompatibility>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub parts: Vec<Puzzle5dPart>,
    #[value(default)]
    #[dsl(table)]
    #[state(artifact)]
    pub fasteners: Vec<Puzzle5dFastener>,
    /// 🧊️ Oriented boxes constraining where the fill planner may place — the 3d-projection half of
    /// the document; the board pane paints their derived flat rectangles.
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    #[dsl(table)]
    #[state(artifact)]
    pub target_volumes: Vec<Puzzle5dTargetVolume>,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedArtifactCodecs



//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Puzzle5dSnapshot {
    fn default() -> Self {
        Self { schema: PUZZLE_5D_SCHEMA.to_string(), domain: "architecture".to_string(), label: None, meta: Default::default(), kind_catalogs: None, kind_catalogs_extra: None, kind_compatibility: Vec::new(), parts: Vec::new(), fasteners: Vec::new(), target_volumes: Vec::new() }
    }
}

//#region 🔁️Re-exports
/// 🔁️ Entities this module's schema exports and its crate declares elsewhere.
pub use crate::Puzzle5dKindCatalogsExtra;
//#endregion 🔁️Re-exports


