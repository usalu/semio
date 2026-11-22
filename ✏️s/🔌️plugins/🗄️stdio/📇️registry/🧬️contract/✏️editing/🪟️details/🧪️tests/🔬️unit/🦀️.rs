use super::*;
use semio_plugin::{TreeWindowRequest, ViewModel, TREE_WINDOW_PATH_SEPARATOR};
use semio_s_artifact_stdio_contract_editing::{snapshot_edit_event_from_action, SnapshotEditEvent, REPLACE_SNAPSHOT_SOURCE_ACTION_ID, SET_SNAPSHOT_VALUE_ACTION_ID, SNAPSHOT_EDIT_ACTION_IDS};
use semio_s_artifact_stdio_contract_wire::snapshot_edit_source;

/// 🏟️ Holds the real UI value arena Source for one rendering law; see [`semio_ui_contract_values::testkit::UiArenaFixture`].
struct DetailsUiArena(Option<semio_ui_contract_values::testkit::UiArenaFixture>);

/// 🚪️ Opens the arena for one rendering law.
fn ui_arena() -> DetailsUiArena {
    DetailsUiArena(Some(semio_ui_contract_values::testkit::UiArenaFixture::new()))
}

/// 🧹️ Retires every built-child page the law dropped, so the arena closes with no borrowed descendants; see [`ui_contract::close_built_node_page_one`].
impl Drop for DetailsUiArena {
    fn drop(&mut self) {
        for _ in 0..1_000_000 {
            if ui_contract::close_built_node_page_one() {
                break;
            }
        }
        drop(self.0.take());
    }
}

include!("🧱️lazy-and-bounded-laws/🦀️.rs");

include!("🌳️collection-and-table-laws/🦀️.rs");

include!("🧬️schema-template-laws/🦀️.rs");

include!("🔀️union-source-and-window-laws/🦀️.rs");
