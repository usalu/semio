//! ♻️ Block2d document retirement — every semantic field retires through byte-bounded cursors, so a
//! replaced document root or a folded mutation is released in paid pages, never by a bare drop.
//!
//! The shared `Block*` rows are owned by this crate (`🧬️schema/🧱️shared`), so their retirement lives
//! here too and every sibling block artifact that embeds them reuses it.

use crate::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, Block2dSnapshot, BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;
use std::sync::Arc;
use store::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, SharedValueRetirementFactory};

//#region 🧱️SharedRows
store::artifact_retire_struct!(BlockKindIdentity { id, name, label, variant, description, icon, unit });
store::artifact_retire_struct!(BlockAttribute { key, value, definition });
store::artifact_retire_struct!(BlockAuthor { id, name, email });
store::artifact_retire_struct!(BlockCompatibilityRule { id, source, target, bidirectional });
store::artifact_retire_struct!(BlockCamera2d { x, y, zoom });
store::artifact_retire_struct!(BlockMeta { description });
store::artifact_retire_struct!(BlockRepresentation { id, name, mesh_url, tags, lod, description, attributes });

impl RetireOwned for BlockCamera3d {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockCamera3d { position, target, zoom } = self;
        store::artifact_retirement_sequence![Vec::from(position), Vec::from(target), zoom]
    }
}
//#endregion 🧱️SharedRows

//#region 🩻️Block2dRows
store::artifact_retire_struct!(Block2dPresentation { shape, radius, width, height, color, icon_kind });
store::artifact_retire_struct!(Block2dHandleKind { id, name, label, color, default_wire_kind });
store::artifact_retire_struct!(Block2dHandleTemplate { id, handle_kind, angle, radius });
store::artifact_retire_struct!(Block2dSnapshot { schema, node_kind, presentation, handle_kinds, handles, compatibility, attributes, authors, camera2d, meta });
//#endregion 🩻️Block2dRows

//#region 🧬️Mutations
impl RetireOwned for Block2dMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::RenameNodeKind(value) => value.new_name.retirement(),
            Self::ChangeNodeKindLabel(value) => value.new_label.retirement(),
            Self::ChangeNodeKindVariant(value) => value.new_variant.retirement(),
            Self::ChangeNodeKindDescription(value) => value.new_description.retirement(),
            Self::ChangeNodeKindIcon(value) => value.new_icon.retirement(),
            Self::ChangeNodeKindUnit(value) => value.new_unit.retirement(),
            Self::UpdatePresentation(value) => store::artifact_retirement_sequence![value.new_shape, value.new_radius, value.new_width, value.new_height, value.new_color, value.new_icon_kind],
            Self::CreateHandleKind(value) => value.handle_kind.retirement(),
            Self::DeleteHandleKind(value) => value.id.retirement(),
            Self::RenameHandleKind(value) => (value.id, value.new_name).retirement(),
            Self::ChangeHandleKindLabel(value) => (value.id, value.new_label).retirement(),
            Self::ChangeHandleKindColor(value) => (value.id, value.new_color).retirement(),
            Self::ChangeHandleKindDefaultWireKind(value) => (value.id, value.new_default_wire_kind).retirement(),
            Self::CreateHandle(value) => value.handle.retirement(),
            Self::DeleteHandle(value) => value.id.retirement(),
            Self::MoveHandle(value) => (value.id, value.new_angle, value.new_radius).retirement(),
            Self::ChangeHandleHandleKind(value) => (value.id, value.new_handle_kind).retirement(),
            Self::AddCompatibilityRule(value) => value.rule.retirement(),
            Self::RemoveCompatibilityRule(value) => value.id.retirement(),
            Self::AddAttribute(value) => value.attribute.retirement(),
            Self::RemoveAttribute(value) => value.key.retirement(),
            Self::AddAuthor(value) => value.author.retirement(),
            Self::RemoveAuthor(value) => value.id.retirement(),
            Self::MoveCamera2d(value) => (value.new_x, value.new_y).retirement(),
            Self::ScaleCamera2d(value) => value.new_zoom.retirement(),
            Self::ChangeMetaDescription(value) => value.new_description.retirement(),
        }
    }
}
//#endregion 🧬️Mutations

//#region 🗃️Owners
/// 🗃️ Block2d's exact document root, initial root and retained mutation retirement authorities — the
/// catalog a retained publication folds against (without it every batched fold faults with
/// `batched fold lacks exact snapshot or mutation retirement authority`) and a document replacement
/// or close retires through.
pub fn document_store_owners() -> store::DocumentStoreOwners<Block2dSnapshot, Block2dMutation> {
    store::DocumentStoreOwners::new(
        Arc::new(SharedValueRetirementFactory::<Block2dSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<Block2dSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<Block2dMutation>::default()),
        Box::new(store::ArtifactStoreCursorDisposer::<Block2dSnapshot, Block2dMutation>::new()),
    )
}
//#endregion 🗃️Owners
