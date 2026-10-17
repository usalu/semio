//! 🧪️ Laws of the wgpu shell's time-travel chrome and its staged editors (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING,
//! packet W2-C).
//!
//! The band law is language-agnostic and shared with the React shell: `🛠️ShellHelpers/🧫️fixtures/🧫️time-travel-band`
//! (validated by Ajv against its schema and the kernel's `HistoryTimeTravel` schema in the React suite) is read here per
//! session and locale — the band's lines, its controls with the refusal that disables one, the dispatch each sends,
//! the chords and the window indicator. The rest pins the wgpu behaviour around it: the keyboard (remappable chords,
//! Escape never discards), the reveal of the History tab, the indicator, the history-edit refusals, the finalize
//! prompt operated by keyboard alone, every staged editor kind in the Actions form and in a chrome dialog, and the
//! history body the guest's REAL producer builds (paged transaction rows, Edit refused with its reason).

use semio_framework_os_renderer_wgpu_fixtures::shell::{host_test_shell, test_app, tree_pointer_record};
use super::*;
use semio_framework_os_renderer_wgpu_program_bridge::OperationPublication;
use semio_framework::kernel::{HistoryEntry, HistoryPatch, HistoryReprojection, HistoryTimeTravel, HistoryTimeTravelStage, InvocationResult};
use semio_framework::{ActionArgDef, ActionArgOption, ArgPresentation, ArgSchema, DialogDefinition, DomainSelection};
use semio_framework_plugin::app::time_travel::{history_row_window_path, TimeTravelInputRow, TimeTravelList, TimeTravelListItem};
use semio_framework_plugin::app::{ui_history_panel, CommandView, HistoryMutationPage, HistoryMutationPages, HistoryView, MutationView, TimeTravelEditorPanel, TimeTravelPanel};

/// 🎟️ Unbounded retained grant: the pre-migration step budget carried no retained limit.
const UNBOUNDED_STEP_GRANT: semio_framework_job::RetainedCloneGrant = semio_framework_job::RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };

include!("🧰️harness-and-recorders/🦀️.rs");

include!("⏪️band-law-and-chords/🦀️.rs");

include!("📐️band-layout-and-focus/🦀️.rs");

include!("🪟️reveal-indicator-and-refusals/🦀️.rs");

include!("👥️notices-peer-edits-and-finalize/🦀️.rs");

include!("🎛️staged-editors/🦀️.rs");

include!("📚️history-body/🦀️.rs");

include!("📡️reprojection-blur-import-and-context-menu/🦀️.rs");
