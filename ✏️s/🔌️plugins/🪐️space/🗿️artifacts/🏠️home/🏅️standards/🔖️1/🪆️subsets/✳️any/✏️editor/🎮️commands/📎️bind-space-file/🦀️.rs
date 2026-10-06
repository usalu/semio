//! 📎️ S Home launcher app command — `bind-space-file`: binds a local studio to a file on disk.
//!
//! IO-owning retained work in two stages (`HomeCatalogWork`, editor): [`validate`] resolves the studio and the path
//! without writing anything, [`commit`] opens the file backbone, writes the studio's event-sourced document into it,
//! admits the file-backed studio into the local catalog (so Home keeps listing it once its ephemeral draft is gone) and
//! publishes `change-catalog-generation`. The job checkpoints between the stages, so a
//! cancellation before [`commit`] writes nothing, and every refusal is named (`s.home.bind-space-file.*`). On a guest
//! without a filesystem (every `wasm32` shell) the commit asks the host's local document catalog to bind the studio instead
//! (`os.local-catalog.admit`, storage `file`) and the host hands the kept studio back.
//! @see ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs `HomeCatalogWork`

use crate::editor::home::config::{local_studio_id_is_admissible, HomeConfig, HomeConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::SHomeMutation;
use crate::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "bind-space-file")]
pub struct BindSpaceFile {
    pub space_id: String,
    pub file_path: String,
}

pub fn handle(_payload: &BindSpaceFile, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App, "s.home.bind-space-file.requires-retained-job", "binding a studio file writes the filesystem and the local catalog and runs only as the retained bind job"))
}

/// 🔎️ Stage one — the studio must exist and the path must name a file on a host with a filesystem; writes nothing.
pub fn validate(payload: &BindSpaceFile) -> Result<(), Fault> {
    if !local_studio_id_is_admissible(&payload.space_id) {
        return Err(Fault::new(FaultOrigin::App, "s.home.bind-space-file.studio-invalid", "the studio id is empty, oversized or carries control characters"));
    }
    if payload.file_path.trim().is_empty() || payload.file_path.chars().any(char::is_control) {
        return Err(Fault::new(FaultOrigin::App, "s.home.bind-space-file.path-invalid", "the file path is empty or carries control characters"));
    }
    ::semio_framework_async::poll::resolve_ready(crate::resolve_studio_document(&payload.space_id))
        .map(|_| ())
        .ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.bind-space-file.unknown-studio", format!("no local studio {} exists", payload.space_id)))
}

/// 💾️ Stage two — the one filesystem + catalog write, then the catalog generation bump.
#[cfg(not(target_arch = "wasm32"))]
pub fn commit(payload: &BindSpaceFile, doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    use semio_framework_os::{document_backbone_ref, encode_backbone_payload, OsBackbonePort as _};
    let refused = |error: semio_framework_os::VcsError| Fault::new(FaultOrigin::App, "s.home.bind-space-file.io-failed", format!("binding {} to {} failed: {error:?}", payload.space_id, payload.file_path));
    let uri = format!("file://{}", payload.file_path);
    let port = semio_framework_os::open_file_space_backbone(&payload.file_path).map_err(refused)?;
    let mut document = ::semio_framework_async::poll::resolve_ready(crate::resolve_studio_document(&payload.space_id)).ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.bind-space-file.unknown-studio", format!("no local studio {} exists", payload.space_id)))?;
    document.backbone = Some(::semio_framework_async::poll::resolve_ready(document_backbone_ref(&uri)));
    port.write(&uri, &encode_backbone_payload(&document).map_err(refused)?).map_err(refused)?;
    ::semio_framework_async::poll::resolve_ready(crate::register_studio_port(&payload.space_id, port));
    semio_framework_os::host::admit_os_space_document(document, &::semio_framework_async::poll::resolve_ready(crate::catalog_port())).map_err(refused)?;
    let draft_port = ::semio_framework_async::poll::resolve_ready(crate::draft_backbone_port());
    ::semio_framework_async::poll::resolve_ready(crate::ephemeral_draft_catalog()).discard_draft(&draft_port, &payload.space_id);
    Ok(Emit::mutations(vec![crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation(doc.snapshot.catalog_generation + 1)]))
}

/// 📮️ Stage two on a guest without a filesystem — the host binds the studio to the file: one `os.local-catalog.admit`
/// request with the studio's own pack/spr pair; the host writes it, records it in its local catalog and hands it back
/// through `applyLocalCatalogDocument`, which lists it as persisted.
#[cfg(target_arch = "wasm32")]
pub fn commit(payload: &BindSpaceFile, _doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit::effect(super::apply_local_catalog_document::keep_on_device(&payload.space_id, "file", &payload.file_path)?))
}
