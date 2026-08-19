//! 🌐️ `stdio.semio` artifact root — the inbuilt semio semantic artifact (standard `v1`, 13
//! schema-owning domain subsets + the `✳️any` envelope union). See the master plan's
//! "Architecture > The semio artifact" section.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::semio::standards::v1::subsets::any::schema::diff::SemioDiff;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::mutations::SemioMutation;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::snapshot::SemioSnapshot;
pub use crate::artifacts::semio::standards::v1::subsets::any::schema::SemioArtifact;

/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_SEMIO_DOCUMENT_SCHEMA: &str = "stdio.semio";

/// 🧬️ Artifact schema descriptor id.
pub const SEMIO_ARTIFACT_SCHEMA_ID: &str = "s.stdio.semio";

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
pub async fn assembly(definition: semio_framework_plugin::ArtifactDefinition) -> Result<crate::registry::ArtifactAssembly, semio_framework_plugin::PluginAssemblyError> {
    crate::registry::definition_only_assembly("semio", definition)
}

pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "stdio.semio".into(),
        name: "Semio".into(),
        source_format: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_SEMIO_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🗂️ Registers all 19 of `v1`'s subsets' IO composers (14 domain subsets + `text` + `✳️any`
/// itself) — dissolved out of the former standard-level `⚙️engine` (ticket
/// 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES). `semio` is one of stdio's 10
/// deliberate imperative-`register()` artifacts (never converted to the `ArtifactDeclaration`
/// builder pattern, per `crate::plugin()`'s own call — unchanged in call order/behavior, only
/// the function's file moved with the deleted directory).
pub async fn register() {
    crate::artifacts::semio::standards::v1::subsets::brep::io::register();
    crate::artifacts::semio::standards::v1::subsets::mesh::io::register();
    crate::artifacts::semio::standards::v1::subsets::model::io::register();
    crate::artifacts::semio::standards::v1::subsets::value::io::register();
    crate::artifacts::semio::standards::v1::subsets::document::io::register();
    crate::artifacts::semio::standards::v1::subsets::cad::io::register();
    crate::artifacts::semio::standards::v1::subsets::drawing::io::register();
    crate::artifacts::semio::standards::v1::subsets::image::io::register();
    crate::artifacts::semio::standards::v1::subsets::video::io::register();
    crate::artifacts::semio::standards::v1::subsets::audio::io::register();
    crate::artifacts::semio::standards::v1::subsets::animation::io::register();
    crate::artifacts::semio::standards::v1::subsets::presentation::io::register();
    crate::artifacts::semio::standards::v1::subsets::flow::io::register();
    crate::artifacts::semio::standards::v1::subsets::text::io::register();
    crate::artifacts::semio::standards::v1::subsets::table::io::register();
    crate::artifacts::semio::standards::v1::subsets::graph::io::register();
    crate::artifacts::semio::standards::v1::subsets::object::io::register();
    crate::artifacts::semio::standards::v1::subsets::kit::io::register();
    crate::artifacts::semio::standards::v1::subsets::any::io::register();
}
//#endregion 🔖️Register

//#region 🔖️Members
/// 🧸️ The subset table, written once and expanded into both the `SemioMembers` enum and the
/// subset-name list, so a new subset cannot be added to one and forgotten in the other.
macro_rules! semio_subset_table {
    ($macro_name:ident) => {
        $macro_name! {
            animation => animation, SemioAnimationSnapshot, SemioAnimationMutation;
            audio => audio, SemioAudioSnapshot, SemioAudioMutation;
            brep => brep, SemioBrepSnapshot, SemioBrepMutation;
            cad => cad, SemioCadSnapshot, SemioCadMutation;
            document => document, SemioDocumentSnapshot, SemioDocumentMutation;
            drawing => drawing, SemioDrawingSnapshot, SemioDrawingMutation;
            flow => flow, SemioFlowSnapshot, SemioFlowMutation;
            graph => graph, SemioGraphSnapshot, SemioGraphMutation;
            image => image, SemioImageSnapshot, SemioImageMutation;
            kit => kit, SemioKitSnapshot, SemioKitMutation;
            mesh => mesh, SemioMeshSnapshot, SemioMeshMutation;
            model => model, SemioModelSnapshot, SemioModelMutation;
            object => object, SemioObjectSnapshot, SemioObjectMutation;
            presentation => presentation, SemioPresentationSnapshot, SemioPresentationMutation;
            table => table, SemioTableSnapshot, SemioTableMutation;
            text => text, SemioTextSnapshot, SemioTextMutation;
            value => value, SemioValueSnapshot, SemioValueMutation;
            video => video, SemioVideoSnapshot, SemioVideoMutation;
        }
    };
}

/// 🧸️ Every subset name this artifact can materialize a child as.
pub async fn composable_subsets() -> Vec<&'static str> {
    macro_rules! subset_names {
        ($($name:ident => $module:ident, $snapshot:ident, $mutation:ident);* $(;)?) => { vec![$(stringify!($name)),*] };
    }
    semio_subset_table!(subset_names)
}

use crate::artifacts::semio::standards::v1::subsets;

/// 🧬️ Closed set spanning `semio`'s 18 composable subsets — the O1 replacement for the deleted
/// `Box<dyn SpaceMember>` `ChildStoreFactory` registry (`store::MemberFactory`'s own doc explains the
/// general mechanism; `store::space_members!` generates the `SpaceMember`/`MemberFactory` delegation
/// below). Generated exactly as any other family would be, EXCEPT the string fed to `MemberFactory::
/// create`/`open` at the two call sites below is `dialect.subset` — never `dialect.artifact_kind` —
/// because all 18 variants share the SAME kind (`s.stdio.semio`, `SEMIO_ARTIFACT_SCHEMA_ID` above) and
/// differ only by subset. Nothing in `space_members!` requires its `kind: &str` parameter to actually
/// BE an `ArtifactKindId`; it only ever compares it against string literals, so this reuse is exact,
/// not approximate.
dsl::space_members! {
    pub enum SemioMembers {
        Animation("animation", "stdio.semio") => dsl::ArtifactStore<subsets::animation::schema::snapshot::SemioAnimationSnapshot, subsets::animation::schema::mutations::SemioAnimationMutation>,
        Audio("audio", "stdio.semio") => dsl::ArtifactStore<subsets::audio::schema::snapshot::SemioAudioSnapshot, subsets::audio::schema::mutations::SemioAudioMutation>,
        Brep("brep", "stdio.semio") => dsl::ArtifactStore<subsets::brep::schema::snapshot::SemioBrepSnapshot, subsets::brep::schema::mutations::SemioBrepMutation>,
        Cad("cad", "stdio.semio") => dsl::ArtifactStore<subsets::cad::schema::snapshot::SemioCadSnapshot, subsets::cad::schema::mutations::SemioCadMutation>,
        Document("document", "stdio.semio") => dsl::ArtifactStore<subsets::document::schema::snapshot::SemioDocumentSnapshot, subsets::document::schema::mutations::SemioDocumentMutation>,
        Drawing("drawing", "stdio.semio") => dsl::ArtifactStore<subsets::drawing::schema::snapshot::SemioDrawingSnapshot, subsets::drawing::schema::mutations::SemioDrawingMutation>,
        Flow("flow", "stdio.semio") => dsl::ArtifactStore<subsets::flow::schema::snapshot::SemioFlowSnapshot, subsets::flow::schema::mutations::SemioFlowMutation>,
        Graph("graph", "stdio.semio") => dsl::ArtifactStore<subsets::graph::schema::snapshot::SemioGraphSnapshot, subsets::graph::schema::mutations::SemioGraphMutation>,
        Image("image", "stdio.semio") => dsl::ArtifactStore<subsets::image::schema::snapshot::SemioImageSnapshot, subsets::image::schema::mutations::SemioImageMutation>,
        Kit("kit", "stdio.semio") => dsl::ArtifactStore<subsets::kit::schema::snapshot::SemioKitSnapshot, subsets::kit::schema::mutations::SemioKitMutation>,
        Mesh("mesh", "stdio.semio") => dsl::ArtifactStore<subsets::mesh::schema::snapshot::SemioMeshSnapshot, subsets::mesh::schema::mutations::SemioMeshMutation>,
        Model("model", "stdio.semio") => dsl::ArtifactStore<subsets::model::schema::snapshot::SemioModelSnapshot, subsets::model::schema::mutations::SemioModelMutation>,
        Object("object", "stdio.semio") => dsl::ArtifactStore<subsets::object::schema::snapshot::SemioObjectSnapshot, subsets::object::schema::mutations::SemioObjectMutation>,
        Presentation("presentation", "stdio.semio") => dsl::ArtifactStore<subsets::presentation::schema::snapshot::SemioPresentationSnapshot, subsets::presentation::schema::mutations::SemioPresentationMutation>,
        Table("table", "stdio.semio") => dsl::ArtifactStore<subsets::table::schema::snapshot::SemioTableSnapshot, subsets::table::schema::mutations::SemioTableMutation>,
        Text("text", "stdio.semio") => dsl::ArtifactStore<subsets::text::schema::snapshot::SemioTextSnapshot, subsets::text::schema::mutations::SemioTextMutation>,
        Value("value", "stdio.semio") => dsl::ArtifactStore<subsets::value::schema::snapshot::SemioValueSnapshot, subsets::value::schema::mutations::SemioValueMutation>,
        Video("video", "stdio.semio") => dsl::ArtifactStore<subsets::video::schema::snapshot::SemioVideoSnapshot, subsets::video::schema::mutations::SemioVideoMutation>,
    }
}

/// 🏭️ Mints a new subset-typed `semio` child — the `create` half of the removed `ChildStoreFactory`.
/// Dispatch key is `dialect.subset` (see [`SemioMembers`]'s doc).
pub async fn create_semio_member(id: &str, dialect: &dsl::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<SemioMembers, dsl::VcsError> {
    <SemioMembers as dsl::MemberFactory>::create(dialect.subset.as_str(), id, dialect, initial_pack).await
}

/// 📤️ Reopens a persisted subset-typed `semio` child — the `open` half. The subset is recovered from
/// the envelope itself (`subset_of_persisted_envelope`), exactly as the removed `ChildStoreFactory::
/// open` did — `open` gets no dialect argument, so it has to; this only works because the `.spr`
/// composition overlay carries `dialect` (see this ticket's `REC_COMPOSITION`).
pub async fn open_semio_member(envelope_pack: &[u8]) -> Result<SemioMembers, dsl::VcsError> {
    let subset = subset_of_persisted_envelope(envelope_pack).await?;
    <SemioMembers as dsl::MemberFactory>::open(subset.as_str(), envelope_pack).await
}

/// 🎯️ Reads a persisted child's subset out of its own `.spr` composition overlay — deliberately
/// snapshot-type-agnostic (it decodes only the history log, never the document body), because
/// choosing the snapshot type is exactly what this answer is needed FOR.
async fn subset_of_persisted_envelope(envelope_pack: &[u8]) -> Result<String, dsl::VcsError> {
    let (_, spr) = dsl::decode_document_pack_bytes(envelope_pack).await?;
    let log = dsl::decode_history(&spr, &dsl::os_spr::DecodeOptions::default()).await.map_err(|error| dsl::VcsError::Deserialize(error.to_string()))?;
    log.composition.and_then(|composition| composition.dialect).map(|(_, _, subset)| subset).ok_or_else(|| dsl::VcsError::Deserialize("semio child store: persisted child carries no dialect, so its subset is unknowable".to_string()))
}
//#endregion 🔖️Members

//#region 🚪️DerivedIoRegistry
pub mod io_registry {
    use crate::artifacts::semio::standards::v1::subsets::any::io::io_registry as v1;
    use semio_framework_plugin::{register_composer_entries, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<&'static ComposerEntry>> = OnceLock::new();

    pub async fn entries() -> &'static [&'static ComposerEntry] {
        ENTRIES.get_or_init(|| v1::entries().iter().collect()).as_slice()
    }

    pub async fn compose(target: Dialect, sources: &[ErasedComposeSource]) -> Result<ComposedArtifact, ComposeError> {
        let entry = entries().iter().find(|e| e.writes == target).ok_or_else(|| ComposeError { message: format!("SemioComposer: no entry writes {:?}", target), diagnostics: Vec::new() })?;
        semio_framework_plugin::resolve_ready((entry.compose)(sources))
    }

    pub async fn register() {
        let _ = register_composer_entries(v1::entries());
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot;
    use crate::dsl::{os_io::ArtifactDialect, ArtifactPack, SpaceMember};

    async fn subset_dialect(subset: &str) -> ArtifactDialect {
        ArtifactDialect { artifact_kind: SEMIO_ARTIFACT_SCHEMA_ID.into(), standard: "v1".into(), subset: subset.into() }
    }

    /// 🧸️ Every composable subset must be reachable through `create_semio_member` — an unlisted
    /// subset would fail with an unhelpful error rather than a named one.
    #[semio_framework_async_macros::async_test]
    async fn every_composable_subset_dispatches_to_a_real_child_store() {
        for subset in composable_subsets().await {
            let dialect = subset_dialect(subset).await;
            // An empty pack is rejected by the production member, so this asserts the DISPATCH
            // reached a real typed variant rather than falling through to "no member kind".
            let error = match create_semio_member("probe", &dialect, &[]).await {
                Ok(_) => panic!("empty genesis pack must be rejected"),
                Err(error) => error,
            };
            assert!(!error.to_string().contains("no member kind"), "subset {subset} is not wired into the child-store dispatch");
        }
        let unknown = match create_semio_member("probe", &subset_dialect("not-a-subset").await, &[]).await {
            Ok(_) => panic!("unknown subset must be rejected"),
            Err(error) => error,
        };
        assert!(unknown.to_string().contains("no member kind"));
    }

    /// 🧸️ `create_semio_member` must MINT a real child store and `open_semio_member` must REOPEN it
    /// from its own persisted envelope — the whole point of "children have their own version
    /// history". The reopen half only works because the persisted `.spr` now carries the dialect the
    /// subset is recovered from.
    #[semio_framework_async_macros::async_test]
    async fn a_semio_member_mints_and_reopens_a_real_child_envelope() {
        let dialect = subset_dialect("mesh").await;

        let seed = SemioMeshSnapshot::default();
        let child = create_semio_member("mesh-child-1", &dialect, &seed.encode_pack().await).await.expect("create child");
        assert_eq!(child.document_id().await, "mesh-child-1");

        let reopened = open_semio_member(&child.envelope_pack_bytes().await.expect("envelope pack")).await.expect("reopen child");
        assert_eq!(reopened.document_pack_bytes().await.expect("head pack"), child.document_pack_bytes().await.expect("head pack"), "the reopened child diverged from the persisted one");
    }
}
//#endregion 🧪️Tests
