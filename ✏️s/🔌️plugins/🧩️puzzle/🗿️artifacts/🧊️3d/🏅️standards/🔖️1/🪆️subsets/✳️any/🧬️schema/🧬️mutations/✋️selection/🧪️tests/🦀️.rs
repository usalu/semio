//! 🎞️ Puzzle3d selection mutation scenes — each committed before/mutation/diff/after quartet of the drag, rotate and scale selection leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]



extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod drag_selection {
    #[path = "✋️drag-selection/🧪️tests/🪢️cross/🦀️.rs"]
    mod tests_cross;
    #[path = "✋️drag-selection/🧪️tests/🎯️drags/🦀️.rs"]
    mod tests_drags_object_and_volume;
    #[path = "✋️drag-selection/🧪️tests/✋️drags-two-objects/🦀️.rs"]
    mod tests_drags_two_objects;
    #[path = "✋️drag-selection/🧪️tests/⛓️follow/🦀️.rs"]
    mod tests_follow;
    #[path = "✋️drag-selection/🧪️tests/⏸️keeps-a-zero-offset/🦀️.rs"]
    mod tests_keeps_a_zero_offset;
    #[path = "✋️drag-selection/🧪️tests/🧱️no-targets/🦀️.rs"]
    mod tests_no_targets;
    #[path = "✋️drag-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
    mod tests_rejects_ghosts;
    #[path = "✋️drag-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
    mod tests_skips_locked_ghost;
}

#[path = ".."]
mod rotate_selection {
    #[path = "🔄️rotate-selection/🧪️tests/⏸️keeps-a-zero-angle/🦀️.rs"]
    mod tests_keeps_a_zero_angle;
    #[path = "🔄️rotate-selection/🧪️tests/⛓️orbits/🦀️.rs"]
    mod tests_orbits;
    #[path = "🔄️rotate-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
    mod tests_rejects_ghosts;
    #[path = "🔄️rotate-selection/🧪️tests/🧱️repeated-targets/🦀️.rs"]
    mod tests_repeated_targets;
    #[path = "🔄️rotate-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
    mod tests_skips_locked_ghost;
    #[path = "🔄️rotate-selection/🧪️tests/🎯️turns/🦀️.rs"]
    mod tests_turns_object_and_volume;
    #[path = "🔄️rotate-selection/🧪️tests/🔄️turns-two-objects/🦀️.rs"]
    mod tests_turns_two_objects;
}

#[path = ".."]
mod scale_selection {
    #[path = "🔍️scale-selection/🧪️tests/⏸️keeps-unit-factors/🦀️.rs"]
    mod tests_keeps_unit_factors;
    #[path = "🔍️scale-selection/🧪️tests/⛔️negative-factor/🦀️.rs"]
    mod tests_negative_factor;
    #[path = "🔍️scale-selection/🧪️tests/🚫️rejects-ghosts/🦀️.rs"]
    mod tests_rejects_ghosts;
    #[path = "🔍️scale-selection/🧪️tests/🎯️scales/🦀️.rs"]
    mod tests_scales_object_and_volume;
    #[path = "🔍️scale-selection/🧪️tests/🔍️scales-two-objects/🦀️.rs"]
    mod tests_scales_two_objects;
    #[path = "🔍️scale-selection/🧪️tests/⚠️skips-locked-ghost/🦀️.rs"]
    mod tests_skips_locked_ghost;
    #[path = "🔍️scale-selection/🧪️tests/⛓️stays/🦀️.rs"]
    mod tests_stays;
    #[path = "🔍️scale-selection/🧪️tests/🧱️zero-factor/🦀️.rs"]
    mod tests_zero_factor;
}
