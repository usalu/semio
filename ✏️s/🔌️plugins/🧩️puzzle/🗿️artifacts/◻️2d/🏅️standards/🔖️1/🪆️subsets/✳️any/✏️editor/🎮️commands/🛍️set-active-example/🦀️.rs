//! 🛍️ `set-active-example` example sources. The load itself is `Puzzle2dActiveExampleWork`
//! (`✏️editor/🦀️.rs`), the single resumable state machine both the retained job and the batch
//! `handle` path drive — this module only owns the warmed example snapshots it reads.

use crate::artifacts::puzzle2d::Puzzle2dSnapshot;
use crate::editor::puzzle2d::{PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID, PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID};
use std::sync::LazyLock;

static EMPTY: LazyLock<Puzzle2dSnapshot> = LazyLock::new(Puzzle2dSnapshot::default);
static CONCRETE_FOREST: LazyLock<Puzzle2dSnapshot> = LazyLock::new(|| dsl::json::from_json_str(crate::examples::puzzle2d::concrete_forest::SOURCE.document_json()).expect("concrete forest example json must match Puzzle2dSnapshot"));
static NAKAGIN: LazyLock<Puzzle2dSnapshot> = LazyLock::new(|| dsl::json::from_json_str(crate::examples::puzzle2d::nakagin_capsule_tower::SOURCE.document_json()).expect("nakagin example json must match Puzzle2dSnapshot"));

pub fn warm_examples() {
    LazyLock::force(&EMPTY);
    LazyLock::force(&CONCRETE_FOREST);
    LazyLock::force(&NAKAGIN);
}

pub(crate) fn canonical_example_id(example_id: &str) -> &'static str {
    match example_id {
        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID | "concrete" => PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID,
        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID | "nakagin" => PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID,
        _ => "",
    }
}

pub(crate) fn target(example_id: &str) -> &'static Puzzle2dSnapshot {
    match example_id {
        PUZZLE2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID => &CONCRETE_FOREST,
        PUZZLE2D_PLAY_EXAMPLE_NAKAGIN_ID => &NAKAGIN,
        _ => &EMPTY,
    }
}
