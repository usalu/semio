//! 🧊️ Puzzle3d document commands — the action arms that edit the document or the selection: selection transforms,
//! deletion and duplication, target volumes, attractions and the snapshot import and export.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "➕️add-target-volume/🦀️.rs"]
pub mod add_target_volume;
#[path = "💔️delete-attraction/🦀️.rs"]
pub mod delete_attraction;
#[path = "🗑️delete-selection/🦀️.rs"]
pub mod delete_selection;
#[path = "🪦️delete-target-volume/🦀️.rs"]
pub mod delete_target_volume;
#[path = "👯️duplicate-selection/🦀️.rs"]
pub mod duplicate_selection;
#[path = "📤️export-snapshot/🦀️.rs"]
pub mod export_snapshot;
#[path = "🎯️focus-selection/🦀️.rs"]
pub mod focus_selection;
#[path = "📥️import-snapshot/🦀️.rs"]
pub mod import_snapshot;
#[path = "🗂️open-import-snapshot/🦀️.rs"]
pub mod open_import_snapshot;
#[path = "🚚️relocate-target-volume/🦀️.rs"]
pub mod relocate_target_volume;
#[path = "🔄️rotate-selection/🦀️.rs"]
pub mod rotate_selection;
#[path = "📏️scale-selection/🦀️.rs"]
pub mod scale_selection;
#[path = "🧬️select-same-kind/🦀️.rs"]
pub mod select_same_kind;
#[path = "🔖️set-selection-flag/🦀️.rs"]
pub mod set_selection_flag;
#[path = "🚩️set-target-volume-flag/🦀️.rs"]
pub mod set_target_volume_flag;
#[path = "👁️set-visible/🦀️.rs"]
pub mod set_visible;
#[path = "🚀️translate-selection/🦀️.rs"]
pub mod translate_selection;
#[path = "🌍️world-relocate/🦀️.rs"]
pub mod world_relocate;
//#endregion 🌳️Modules
