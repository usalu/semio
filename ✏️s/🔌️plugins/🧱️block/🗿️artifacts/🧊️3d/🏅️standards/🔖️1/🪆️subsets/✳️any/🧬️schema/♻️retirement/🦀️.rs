//! ♻️ Block3d document retirement: every semantic field derives its owner; only the materialized kit catalog behind the child handle needs this snapshot to lift it into a leased shared owner.

use crate::Block3dSnapshot;
use semio_framework_value::retirement::{deferred, deferred_birth_bytes, deferred_birth_bytes_for, sequence, sequence_birth_bytes, shared::SharedControlledRetirement, RetireOwned, RetirementCursor};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

type CatalogOwner = Option<SharedControlledRetirement<SemioKitSnapshot>>;

/// 📚️ A document retires every field as a paid child and lifts its materialized kit catalog into a leased shared owner, so the last alias pays the catalog release and every other alias only drops its count.
impl RetireOwned for Block3dSnapshot {
    fn retirement(mut self) -> Box<dyn RetirementCursor> {
        let catalog_owner: CatalogOwner = self.catalog.take_local_owner::<SemioKitSnapshot>().ok().flatten().map(SharedControlledRetirement::lease);
        let Self { schema, object_kind, representations, catalog, vortex_kind_extra, vortices, compatibility, attributes, authors, camera3d, meta } = self;
        sequence(vec![deferred(schema), deferred(object_kind), deferred(representations), deferred(catalog), deferred(vortex_kind_extra), deferred(vortices), deferred(compatibility), deferred(attributes), deferred(authors), deferred(camera3d), deferred(meta), deferred(catalog_owner)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> {
        sequence_birth_bytes(&[
            deferred_birth_bytes_for(&self.schema),
            deferred_birth_bytes_for(&self.object_kind),
            deferred_birth_bytes_for(&self.representations),
            deferred_birth_bytes_for(&self.catalog),
            deferred_birth_bytes_for(&self.vortex_kind_extra),
            deferred_birth_bytes_for(&self.vortices),
            deferred_birth_bytes_for(&self.compatibility),
            deferred_birth_bytes_for(&self.attributes),
            deferred_birth_bytes_for(&self.authors),
            deferred_birth_bytes_for(&self.camera3d),
            deferred_birth_bytes_for(&self.meta),
            deferred_birth_bytes::<CatalogOwner>(),
        ])
    }
    fn controlled_retirement_supported() -> bool { <CatalogOwner as RetireOwned>::controlled_retirement_supported() }
}
