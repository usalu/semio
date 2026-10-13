//! ♻️ Block5d document retirement — every semantic field retires through byte-bounded cursors, so a
//! replaced document root or a folded mutation is released in paid pages, never by a bare drop. The
//! shared `Block*` rows retire through their plugin owner `semio-s-plugin-block`.

use crate::{Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d, Block5dSnapshot};

//#region 🖐️Block5dRows
semio_framework_value::artifact_retire_struct!(Block5dPart2d { shape, radius, width, height, color, icon_kind });
semio_framework_value::artifact_retire_struct!(Block5dGripKind { id, name, label, color, default_rope_kind });
semio_framework_value::artifact_retire_struct!(Block5dSnapshot { schema, part_kind, part_2d, part_3d, representations, grip_kinds, grips, compatibility, attributes, authors, camera2d, camera3d, meta });

semio_framework_value::artifact_retire_struct!(Block5dPart3d { orientation, scale });
semio_framework_value::artifact_retire_struct!(Block5dGripTemplate { id, grip_kind, angle, radius_2d, position, direction, radius_3d });
//#endregion 🖐️Block5dRows
