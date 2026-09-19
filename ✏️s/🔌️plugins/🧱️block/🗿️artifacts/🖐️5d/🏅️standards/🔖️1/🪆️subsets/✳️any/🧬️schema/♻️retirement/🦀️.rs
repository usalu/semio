//! ♻️ Block5d document retirement — every semantic field retires through byte-bounded cursors, so a
//! replaced document root or a folded mutation is released in paid pages, never by a bare drop. The
//! shared `Block*` rows retire through `semio-s-artifact-block-2d`, which owns them.

use crate::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, Block5dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;
use std::sync::Arc;
use store::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, SharedValueRetirementFactory};

//#region 🖐️Block5dRows
store::artifact_retire_struct!(Block5dPart2d { shape, radius, width, height, color, icon_kind });
store::artifact_retire_struct!(Block5dGripKind { id, name, label, color, default_rope_kind });
store::artifact_retire_struct!(Block5dSnapshot { schema, part_kind, part_2d, part_3d, representations, grip_kinds, grips, compatibility, attributes, authors, camera2d, camera3d, meta });

impl RetireOwned for Block5dPart3d {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Block5dPart3d { orientation, scale } = self;
        store::artifact_retirement_sequence![orientation.map(Vec::from), scale.map(Vec::from)]
    }
}

impl RetireOwned for Block5dGripTemplate {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Block5dGripTemplate { id, grip_kind, angle, radius_2d, position, direction, radius_3d } = self;
        store::artifact_retirement_sequence![id, grip_kind, angle, radius_2d, Vec::from(position), Vec::from(direction), radius_3d]
    }
}
//#endregion 🖐️Block5dRows

//#region 🧬️Mutations
impl RetireOwned for Block5dMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::RenamePartKind(value) => value.new_name.retirement(),
            Self::ChangePartKindLabel(value) => value.new_label.retirement(),
            Self::ChangePartKindVariant(value) => value.new_variant.retirement(),
            Self::ChangePartKindDescription(value) => value.new_description.retirement(),
            Self::ChangePartKindIcon(value) => value.new_icon.retirement(),
            Self::ChangePartKindUnit(value) => value.new_unit.retirement(),
            Self::UpdatePart2d(value) => store::artifact_retirement_sequence![value.new_shape, value.new_radius, value.new_width, value.new_height, value.new_color, value.new_icon_kind],
            Self::UpdatePart3d(value) => store::artifact_retirement_sequence![value.new_orientation.map(Vec::from), value.new_scale.map(Vec::from)],
            Self::CreateRepresentation(value) => value.representation.retirement(),
            Self::DeleteRepresentation(value) => value.id.retirement(),
            Self::RenameRepresentation(value) => (value.id, value.new_name).retirement(),
            Self::ChangeRepresentationMeshUrl(value) => (value.id, value.new_mesh_url).retirement(),
            Self::ChangeRepresentationLod(value) => (value.id, value.new_lod).retirement(),
            Self::ChangeRepresentationDescription(value) => (value.id, value.new_description).retirement(),
            Self::AddRepresentationTag(value) => (value.id, value.tag).retirement(),
            Self::RemoveRepresentationTag(value) => (value.id, value.tag).retirement(),
            Self::AddRepresentationAttribute(value) => (value.id, value.attribute).retirement(),
            Self::RemoveRepresentationAttribute(value) => (value.id, value.key).retirement(),
            Self::CreateGripKind(value) => value.grip_kind.retirement(),
            Self::DeleteGripKind(value) => value.id.retirement(),
            Self::RenameGripKind(value) => (value.id, value.new_name).retirement(),
            Self::ChangeGripKindLabel(value) => (value.id, value.new_label).retirement(),
            Self::ChangeGripKindColor(value) => (value.id, value.new_color).retirement(),
            Self::ChangeGripKindDefaultRopeKind(value) => (value.id, value.new_default_rope_kind).retirement(),
            Self::CreateGrip(value) => value.grip.retirement(),
            Self::DeleteGrip(value) => value.id.retirement(),
            Self::MoveGrip2d(value) => (value.id, value.new_angle, value.new_radius_2d).retirement(),
            Self::MoveGrip3d(value) => (value.id, Vec::from(value.new_position), Vec::from(value.new_direction)).retirement(),
            Self::ResizeGrip3d(value) => (value.id, value.new_radius_3d).retirement(),
            Self::ChangeGripGripKind(value) => (value.id, value.new_grip_kind).retirement(),
            Self::AddCompatibilityRule(value) => value.rule.retirement(),
            Self::RemoveCompatibilityRule(value) => value.id.retirement(),
            Self::AddAttribute(value) => value.attribute.retirement(),
            Self::RemoveAttribute(value) => value.key.retirement(),
            Self::AddAuthor(value) => value.author.retirement(),
            Self::RemoveAuthor(value) => value.id.retirement(),
            Self::MoveCamera2d(value) => (value.new_x, value.new_y).retirement(),
            Self::ScaleCamera2d(value) => value.new_zoom.retirement(),
            Self::MoveCamera3d(value) => (Vec::from(value.new_position), Vec::from(value.new_target)).retirement(),
            Self::ScaleCamera3d(value) => value.new_zoom.retirement(),
            Self::ChangeMetaDescription(value) => value.new_description.retirement(),
        }
    }
}
//#endregion 🧬️Mutations

//#region 🗃️Owners
/// 🗃️ Block5d's exact document root, initial root and retained mutation retirement authorities — the
/// catalog a retained publication folds against (without it every batched fold faults with
/// `batched fold lacks exact snapshot or mutation retirement authority`) and a document replacement
/// or close retires through.
pub fn document_store_owners() -> store::DocumentStoreOwners<Block5dSnapshot, Block5dMutation> {
    store::DocumentStoreOwners::new(
        Arc::new(SharedValueRetirementFactory::<Block5dSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<Block5dSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<Block5dMutation>::default()),
        Box::new(store::ArtifactStoreCursorDisposer::<Block5dSnapshot, Block5dMutation>::new()),
    )
}
//#endregion 🗃️Owners
