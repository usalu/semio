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
    register_child_store_factories();
}
//#endregion 🔖️Register

//#region 🔖️ChildStoreFactories
/// 🧸️ Dispatches child-store construction across `semio`'s subsets.
///
/// 🎯️ Why one factory and not eighteen: `ChildStoreFactory` is registered per `ArtifactKindId`, and
/// that grammar is exactly three segments (`s.<plugin>.<artifact>`) — every semio subset shares the
/// ONE kind `s.stdio.semio`, with the subset carried in the `ArtifactDialect` instead. So the
/// per-subset `(Snapshot, Mutation)` choice has to happen inside a single registered factory, by
/// switching on `dialect.subset`.
///
/// `open` gets no dialect argument, so it recovers the subset from the persisted envelope itself —
/// which only works because the `.spr` composition overlay now carries `dialect` (see this ticket's
/// `REC_COMPOSITION`; before that a reopened child had no way to know what it was).
struct SemioChildStoreFactory;

/// 🧸️ The subset table, written once and expanded into both the `create`/`open` dispatch and the
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

impl dsl::ChildStoreFactory for SemioChildStoreFactory {
    async fn create(&self, id: &str, dialect: &dsl::os_io::ArtifactDialect, initial_pack: &[u8]) -> Result<Box<dyn dsl::SpaceMember>, dsl::VcsError> {
        use crate::artifacts::semio::standards::v1::subsets;
        macro_rules! create_arm {
            ($($name:ident => $module:ident, $snapshot:ident, $mutation:ident);* $(;)?) => {
                match dialect.subset.as_str() {
                    $(stringify!($name) => crate::dsl::TypedChildStoreFactory::<subsets::$module::schema::snapshot::$snapshot, subsets::$module::schema::mutations::$mutation>::new(STDIO_SEMIO_DOCUMENT_SCHEMA).create(id, dialect, initial_pack),)*
                    other => Err(crate::dsl::VcsError::Deserialize(format!("semio child store: no composable subset {other:?}"))),
                }
            };
        }
        semio_subset_table!(create_arm)
    }

    async fn open(&self, envelope_pack: &[u8]) -> Result<Box<dyn dsl::SpaceMember>, dsl::VcsError> {
        use crate::artifacts::semio::standards::v1::subsets;
        let subset = subset_of_persisted_envelope(envelope_pack)?;
        macro_rules! open_arm {
            ($($name:ident => $module:ident, $snapshot:ident, $mutation:ident);* $(;)?) => {
                match subset.as_str() {
                    $(stringify!($name) => crate::dsl::TypedChildStoreFactory::<subsets::$module::schema::snapshot::$snapshot, subsets::$module::schema::mutations::$mutation>::new(STDIO_SEMIO_DOCUMENT_SCHEMA).open(envelope_pack),)*
                    other => Err(crate::dsl::VcsError::Deserialize(format!("semio child store: no composable subset {other:?}"))),
                }
            };
        }
        semio_subset_table!(open_arm)
    }
}

/// 🎯️ Reads a persisted child's subset out of its own `.spr` composition overlay — deliberately
/// snapshot-type-agnostic (it decodes only the history log, never the document body), because
/// choosing the snapshot type is exactly what this answer is needed FOR.
async fn subset_of_persisted_envelope(envelope_pack: &[u8]) -> Result<String, dsl::VcsError> {
    let (_, spr) = dsl::decode_document_pack_bytes(envelope_pack)?;
    let log = dsl::decode_history(&spr, &dsl::os_spr::DecodeOptions::default()).map_err(|error| dsl::VcsError::Deserialize(error.to_string()))?;
    log.composition.and_then(|composition| composition.dialect).map(|(_, _, subset)| subset).ok_or_else(|| dsl::VcsError::Deserialize("semio child store: persisted child carries no dialect, so its subset is unknowable".to_string()))
}

/// 🧸️ Registers the `semio` child-store factory so any plugin declaring
/// `#[child(kind = "s.stdio.semio")]` can actually MINT and REOPEN that child as its own envelope
/// with its own `ArtifactVcs` history — the thing `CompositionCoordinator::dispatch_group`'s phase 1
/// validates for and refuses to proceed without.
///
/// ⚠️ Must be called from every composing plugin's own app-creation path, not only from stdio's: the
/// factory registry is a process-global inside each WASM component, and a plugin that composes
/// stdio children is a DIFFERENT component from stdio itself. Idempotent, so calling it from N
/// plugins is harmless.
pub async fn register_child_store_factories() {
    let _ = dsl::register_child_store_factory(dsl::os_io::ArtifactKindId::parse(SEMIO_ARTIFACT_SCHEMA_ID).expect("canonical semio artifact kind"), std::sync::Arc::new(SemioChildStoreFactory));
}
//#endregion 🔖️ChildStoreFactories

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
    use crate::dsl::{
        child_store_factory,
        os_io::{ArtifactDialect, ArtifactKindId},
        ArtifactPack,
    };

    async fn subset_dialect(subset: &str) -> ArtifactDialect {
        ArtifactDialect { artifact_kind: SEMIO_ARTIFACT_SCHEMA_ID.into(), standard: "v1".into(), subset: subset.into() }
    }

    /// 🧸️ The artifact must have a live factory after `register()`, or a parent declaring
    /// `#[child(kind = "s.stdio.semio")]` fails phase-1 validation in
    /// `CompositionCoordinator::dispatch_group` with "no ChildStoreFactory registered".
    #[test]
    async fn the_semio_artifact_has_a_registered_child_store_factory() {
        register_child_store_factories();
        let kind = ArtifactKindId::parse(SEMIO_ARTIFACT_SCHEMA_ID).expect("canonical kind");
        assert!(child_store_factory(&kind).expect("child store factory registry available").is_some(), "no ChildStoreFactory registered for {SEMIO_ARTIFACT_SCHEMA_ID}");
    }

    /// 🧸️ Every composable subset must be reachable through that one factory — an unlisted subset
    /// would fail at genesis time with an unhelpful error rather than at registration time.
    #[test]
    async fn every_composable_subset_dispatches_to_a_real_child_store() {
        register_child_store_factories();
        let factory = child_store_factory(&ArtifactKindId::parse(SEMIO_ARTIFACT_SCHEMA_ID).expect("canonical kind")).expect("child store factory registry available").expect("factory registered");
        for subset in composable_subsets() {
            // An empty pack is rejected by the production factory, so this asserts the DISPATCH
            // reached a real typed factory rather than falling through to "no composable subset".
            let error = match factory.create("probe", &subset_dialect(subset), &[]) {
                Ok(_) => panic!("empty genesis pack must be rejected"),
                Err(error) => error,
            };
            assert!(!error.to_string().contains("no composable subset"), "subset {subset} is not wired into the child-store dispatch");
        }
        let unknown = match factory.create("probe", &subset_dialect("not-a-subset"), &[]) {
            Ok(_) => panic!("unknown subset must be rejected"),
            Err(error) => error,
        };
        assert!(unknown.to_string().contains("no composable subset"));
    }

    /// 🧸️ A registered factory must MINT a real child store and REOPEN it from its own persisted
    /// envelope — the whole point of "children have their own version history". The reopen half only
    /// works because the persisted `.spr` now carries the dialect the subset is recovered from.
    #[test]
    async fn a_registered_factory_mints_and_reopens_a_real_child_envelope() {
        register_child_store_factories();
        let factory = child_store_factory(&ArtifactKindId::parse(SEMIO_ARTIFACT_SCHEMA_ID).expect("canonical kind")).expect("child store factory registry available").expect("factory registered");
        let dialect = subset_dialect("mesh");

        let seed = SemioMeshSnapshot::default();
        let child = factory.create("mesh-child-1", &dialect, &seed.encode_pack()).expect("create child");
        assert_eq!(child.document_id(), "mesh-child-1");

        let reopened = factory.open(&child.envelope_pack_bytes().expect("envelope pack")).expect("reopen child");
        assert_eq!(reopened.document_pack_bytes().expect("head pack"), child.document_pack_bytes().expect("head pack"), "the reopened child diverged from the persisted one");
    }
}
//#endregion 🧪️Tests
