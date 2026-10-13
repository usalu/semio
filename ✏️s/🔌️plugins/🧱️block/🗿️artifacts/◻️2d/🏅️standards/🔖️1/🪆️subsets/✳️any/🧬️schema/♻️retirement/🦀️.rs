//! ♻️ Block2d document retirement — every semantic field retires through byte-bounded cursors, so a
//! replaced document root or a folded mutation is released in paid pages, never by a bare drop.
//!
//! Shared Block records use the plugin-owned retirement implementations.

use crate::{Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation, Block2dSnapshot};



//#region 🩻️Block2dRows
semio_framework_value::artifact_retire_struct!(Block2dPresentation { shape, radius, width, height, color, icon_kind });
semio_framework_value::artifact_retire_struct!(Block2dHandleKind { id, name, label, color, default_wire_kind });
semio_framework_value::artifact_retire_struct!(Block2dHandleTemplate { id, handle_kind, angle, radius });
semio_framework_value::artifact_retire_struct!(Block2dSnapshot { schema, node_kind, presentation, handle_kinds, handles, compatibility, attributes, authors, camera2d, meta });
//#endregion 🩻️Block2dRows
