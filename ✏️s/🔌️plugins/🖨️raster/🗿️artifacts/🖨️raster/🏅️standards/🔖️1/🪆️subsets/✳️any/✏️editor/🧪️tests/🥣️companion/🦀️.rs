//! 🥣️ Companion test crate of the raster editor crates: the laws that drive the whole editor app through its command channel, panels, modes and media export. The test files stay at their taxonomy nodes and are mounted here.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../🖱️selection/🧪️tests/🦀️.rs"]
mod layer_selection_tests;
#[path = "../../🎮️commands/🌈️apply-filter/🧪️tests/🦀️.rs"]
mod commands_apply_filter_tests;
#[path = "../../🎮️commands/🎬️set-active-example/🧪️tests/🔬️unit/🦀️.rs"]
mod commands_set_active_example_tests;
#[path = "../../🎮️commands/🎭️mask-from-selection/🧪️tests/🦀️.rs"]
mod commands_mask_from_selection_tests;
#[path = "../../🎮️commands/📋️duplicate-layer/🧪️tests/🦀️.rs"]
mod commands_duplicate_layer_tests;
#[path = "../../🎮️commands/🔄️transform-image/🧪️tests/🦀️.rs"]
mod commands_transform_image_tests;
#[path = "../../🎮️commands/🖌️paint-stroke/🧪️tests/🦀️.rs"]
mod commands_paint_stroke_tests;
#[path = "../../🎮️commands/🥞️flatten-layers/🧪️tests/🦀️.rs"]
mod commands_flatten_layers_tests;
#[path = "../../🎮️commands/🩹️patch-layer/🧪️tests/🦀️.rs"]
mod commands_patch_layer_tests;
#[path = "../../🎮️commands/🪣️fill-region/🧪️tests/🦀️.rs"]
mod commands_fill_region_tests;
#[path = "../../🎮️commands/🫗️fill-selection/🧪️tests/🦀️.rs"]
mod commands_fill_selection_tests;
#[path = "../../🎮️commands/🫳️merge-down/🧪️tests/🦀️.rs"]
mod commands_merge_down_tests;
#[path = "../../🎮️commands/🚚️move-layer/🧪️tests/🦀️.rs"]
mod commands_move_layer_tests;
#[path = "../../💾️document/🧪️tests/🦀️.rs"]
mod editor_document_tests;
#[path = "../../📤️export/🧪️tests/🦀️.rs"]
mod media_export_tests;
#[path = "../🔬️unit/🦀️.rs"]
mod unit_tests;
#[path = "../🧪️fill-tool-transactions/🦀️.rs"]
mod fill_tool_transactions;
#[path = "../🧪️stroke-stream-transactions/🦀️.rs"]
mod stroke_stream_transactions;
