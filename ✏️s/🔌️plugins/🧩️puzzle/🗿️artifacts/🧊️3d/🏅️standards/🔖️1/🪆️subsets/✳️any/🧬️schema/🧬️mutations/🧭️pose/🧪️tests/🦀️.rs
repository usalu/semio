//! 🎞️ Puzzle3d pose mutation scenes — each committed before/mutation/diff/after quartet of the move, rotate and scale leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod move_object {
    #[path = "📍move-object/🧪️tests/📍️moves-object-a/🦀️.rs"]
    mod tests_moves_object_a;
}

#[path = ".."]
mod rotate_object {
    #[path = "🔃rotate-object/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
    mod tests_half_turn_about_z;
}

#[path = ".."]
mod scale_object {
    #[path = "📏scale-object/🧪️tests/📐️uniform-to-per-axis/🦀️.rs"]
    mod tests_uniform_to_per_axis;
}
