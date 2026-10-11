//! ♻️ Block2d document retirement — every semantic field retires through byte-bounded cursors, so a
//! replaced document root or a folded mutation is released in paid pages, never by a bare drop.
//!
//! Shared Block records use the plugin-owned retirement implementations.

use crate::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, Block2dSnapshot};
use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};



//#region 🩻️Block2dRows
semio_framework_value::artifact_retire_struct!(Block2dPresentation { shape, radius, width, height, color, icon_kind });
semio_framework_value::artifact_retire_struct!(Block2dHandleKind { id, name, label, color, default_wire_kind });
semio_framework_value::artifact_retire_struct!(Block2dHandleTemplate { id, handle_kind, angle, radius });
semio_framework_value::artifact_retire_struct!(Block2dSnapshot { schema, node_kind, presentation, handle_kinds, handles, compatibility, attributes, authors, camera2d, meta });
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
            Self::UpdatePresentation(value) => semio_framework_value::artifact_retirement_sequence![value.new_shape, value.new_radius, value.new_width, value.new_height, value.new_color, value.new_icon_kind],
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

