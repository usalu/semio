//! 📂️ Routes one picked file to the dialect whose owner editor opens it: format resolution, revision pick by head sniff and the package content check, every dead end a named refusal. Rust twin of the TypeScript `routePickedFileV1`, the one entry the native shell's Open file calls.

use super::*;

/// 🪝️ What routing needs from its composition root: the registered format claims, the dialect coordinate whose owner opens each format, and the optional revision and package claims.
pub trait PickedFileCatalog {
    fn descriptors(&self) -> Vec<FormatDescriptor>;
    fn artifact_ref_of(&self, kind_id: &str) -> Option<String>;
    fn revisions_of(&self, _kind_id: &str) -> Vec<RevisionSniff> {
        Vec::new()
    }
    fn package_of(&self, _kind_id: &str) -> Option<PackageClaim> {
        None
    }
}

/// 🔬️ How many leading bytes of a picked file the revision sniff may read.
pub const PICKED_FILE_SNIFF_BYTES: usize = 4096;

/// 🚫️ Every way an Open file refuses a picked file before any document is created; the kebab names are the shared refusal vocabulary's keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickedFileRefusal {
    FormatUnresolved,
    FormatAmbiguous,
    FormatCatalogUnavailable,
    OwnerUnavailable,
    ContentMismatch,
}

impl PickedFileRefusal {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FormatUnresolved => "format-unresolved",
            Self::FormatAmbiguous => "format-ambiguous",
            Self::FormatCatalogUnavailable => "format-catalog-unavailable",
            Self::OwnerUnavailable => "owner-unavailable",
            Self::ContentMismatch => "content-mismatch",
        }
    }
}

/// 🧭️ A routed file with the resolution that decided it, or the refusal.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PickedFileRoute {
    Routed { artifact_ref: String, resolution: FormatResolution },
    Refused(PickedFileRefusal),
}

/// 📂️ Routes `file_name` (and its optional MIME claim); `content` is the package source of the picked bytes, absent when the host supplies none. Cancellation stops the check instead of refusing.
pub fn route_picked_file(catalog: Option<&dyn PickedFileCatalog>, file_name: &str, mime: Option<&str>, read_head: &dyn Fn(usize) -> Vec<u8>, content: Option<&dyn PackageSource>) -> Result<PickedFileRoute, PackageSniffCancelled> {
    let Some(catalog) = catalog else { return Ok(PickedFileRoute::Refused(PickedFileRefusal::FormatCatalogUnavailable)) };
    let query = FormatQuery { explicit: None, file_name: Some(file_name), mime: mime.filter(|mime| !mime.is_empty()) };
    let resolution = match resolve_format_in(&catalog.descriptors(), &query) {
        Ok(Some(resolution)) => resolution,
        Ok(None) | Err(FormatResolutionError::UnknownExplicit { .. } | FormatResolutionError::Unavailable(_)) => return Ok(PickedFileRoute::Refused(PickedFileRefusal::FormatUnresolved)),
        Err(FormatResolutionError::Ambiguous { .. }) => return Ok(PickedFileRoute::Refused(PickedFileRefusal::FormatAmbiguous)),
    };
    let owner = resolution.descriptor.kind_id.as_str();
    let revisions = catalog.revisions_of(owner);
    let kind_id = if revisions.is_empty() { owner.to_string() } else { pick_revision(owner, &revisions, &read_head(PICKED_FILE_SNIFF_BYTES)).to_string() };
    let verdict = match (catalog.package_of(&kind_id), content) {
        (Some(claim), Some(source)) => Some(inspect_package(source, &claim)?),
        _ => None,
    };
    if verdict == Some(PackageVerdict::Mismatch) {
        return Ok(PickedFileRoute::Refused(PickedFileRefusal::ContentMismatch));
    }
    let artifact_ref = match verdict {
        Some(PackageVerdict::Match { artifact_ref: Some(artifact_ref) }) => Some(artifact_ref),
        _ => catalog.artifact_ref_of(&kind_id),
    };
    Ok(artifact_ref.map_or(PickedFileRoute::Refused(PickedFileRefusal::OwnerUnavailable), |artifact_ref| PickedFileRoute::Routed { artifact_ref, resolution }))
}
