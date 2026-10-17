//! 🥣️ Companion test crate of `semio-s-artifact-raster-raster-mutations-layers` and `semio-s-artifact-raster-raster-mutations-pixels`: the leaf laws that apply a mutation through the closed vocabulary. The test files stay at their taxonomy nodes and are mounted here.

#![cfg(test)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

#[path = "../../↔️move-layer/🧪️tests/📍️slides/🦀️.rs"]
mod move_layer_tests_slides_the_stamp_layer_off_the_origin;
#[path = "../../✏️rename-layer/🧪️tests/✏️renames/🦀️.rs"]
mod rename_layer_tests_renames_the_sketch_layer_to_final_linework;
#[path = "../../🌫️change-layer-opacity/🧪️tests/🌫️fades/🦀️.rs"]
mod change_layer_opacity_tests_fades_the_highlight_layer_to_a_quarter;
#[path = "../../🌱create-layer/🧪️tests/🖋️creates-an-ink-8a1bf9/🦀️.rs"]
mod create_layer_tests_creates_an_ink_layer_inside_the_artwork_group;
#[path = "../../🎚️change-layer-adjustment/🧪️tests/📈️switches/🦀️.rs"]
mod change_layer_adjustment_kind_tests_switches_the_tone_layer_from_levels_to_curves;
#[path = "../../🎨change-layer-blend-mode/🧪️tests/💡️switches/🦀️.rs"]
mod change_layer_blend_mode_tests_switches_the_glow_layer_to_screen;
#[path = "../../👁️change-layer-visible/🧪️tests/🙈️hides/🦀️.rs"]
mod change_layer_visible_tests_hides_the_overlay_layer;
#[path = "../../📐resize-layer/🧪️tests/📐️resizes/🦀️.rs"]
mod resize_layer_tests_resizes_the_canvas_layer_to_256_by_128;
#[path = "../../🔀reorder-layers/🧪️tests/⤴️lifts/🦀️.rs"]
mod reorder_layers_tests_lifts_the_caption_layer_out_of_the_frame_group;
#[path = "../../🗑️delete-layer/🧪️tests/🚫️deletes/🦀️.rs"]
mod delete_layer_tests_deletes_the_frame_group_and_its_nested_children;
#[path = "../../🎨️change-layer-pixels/🧪️tests/🦀️.rs"]
mod change_layer_pixels_tests;
#[path = "../../🖇️add-layer-asset/🧪️tests/🖼️declines/🦀️.rs"]
mod add_layer_asset_tests_declines_to_reattach_an_asset_already_on_the_document;
#[path = "../../🗂️remove-layer-asset/🧪️tests/🖼️rejects/🦀️.rs"]
mod remove_layer_asset_tests_rejects_removing_an_asset_the_document_never_attached;
#[path = "../../🎛️change-layer/🧪️tests/🦀️.rs"]
mod change_layer_adjustment_parameter_tests;
#[path = "../../📐️change-layer-transform/🧪️tests/🦀️.rs"]
mod change_layer_transform_tests;
#[path = "../../🔲write-pixel-region/🧪️tests/🦀️.rs"]
mod write_pixel_region_tests;
