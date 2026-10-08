//! 📥️ S Home launcher app command — `import-space`: imports a studio into the local catalog from its `.os` DSL text.
//!
//! Without text the command asks the HOST to pick the file ([`Effect::RequestFileOpen`], host-owned file IO), which
//! re-dispatches `importSpace` with the picked text as `payload` (one import chunk; a manifest spanning more chunks is
//! refused as `s.home.import-space.oversized` when the action is decoded). With text the import is IO-owning retained work in two
//! stages (`HomeCatalogWork`, editor): [`validate`] parses the text as a studio manifest without touching the catalog,
//! [`commit`] admits it as a new event-sourced space document, publishes `change-catalog-generation` and asks the host to
//! keep it on this device (the host's local document catalog, route B). The job
//! checkpoints between the stages, so a cancellation before [`commit`] leaves the catalog untouched, and every refusal
//! is named (`s.home.import-space.*`) instead of answering an empty success.
//! @see ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs `HomeCatalogWork`

use crate::editor::home::config::{HomeConfig, HomeConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "import-space")]
pub struct ImportSpace {
    pub dsl: Option<String>,
}

/// 📤️ The host-owned file request that re-dispatches `importSpace` with the picked `.os` text.
pub fn file_request() -> Emit<SHomeMutation, HomeConfigMutation> {
    Emit::effect(Effect::RequestFileOpen { req: semio_framework_plugin::RequestId(124), accept: ".os".into(), read_as: None, import_action: "importSpace".into(), multiple: false, args: None })
}

pub fn handle(payload: &ImportSpace, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    match payload.dsl {
        None => Ok(file_request()),
        Some(_) => Err(Fault::new(FaultOrigin::App, "s.home.import-space.requires-retained-job", "importing a studio writes the local catalog and runs only as the retained import job")),
    }
}

/// 🔎️ Stage one — the text must be a named studio manifest; reads nothing but the text and writes nothing.
pub fn validate(dsl: &str) -> Result<(), Fault> {
    if dsl.trim().is_empty() {
        return Err(Fault::new(FaultOrigin::App, "s.home.import-space.empty", "the imported studio text is empty"));
    }
    let manifest = <semio_framework_artifact_space_space::SpaceSnapshot as store::ArtifactDsl>::parse_dsl(dsl)
        .map_err(|error| Fault::new(FaultOrigin::App, "s.home.import-space.not-a-studio", format!("the imported text is not a studio manifest: {}", error.message)))?;
    if manifest.name.trim().is_empty() {
        return Err(Fault::new(FaultOrigin::App, "s.home.import-space.unnamed", "the imported studio manifest has no name"));
    }
    Ok(())
}

/// 💾️ Stage two — the one catalog write: admits the studio as a new space document, bumps the catalog generation and asks
/// the host to keep it on this device (`os.local-catalog.admit` into the device's own data folder), so the import outlives
/// the session.
pub fn commit(dsl: &str, doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let entry = semio_framework_os::import_os_space_from_dsl(dsl, &::semio_framework_async::poll::resolve_ready(semio_s_space_core::catalog_port()))
        .map_err(|error| Fault::new(FaultOrigin::App, "s.home.import-space.catalog-refused", format!("the local catalog refused the imported studio: {error:?}")))?;
    Ok(Emit { artifact_mutations: vec![change_catalog_generation(doc.snapshot.catalog_generation + 1)], effects: vec![super::apply_local_catalog_document::keep_on_device(&entry.id, "folder", "")?], ..Default::default() })
}
