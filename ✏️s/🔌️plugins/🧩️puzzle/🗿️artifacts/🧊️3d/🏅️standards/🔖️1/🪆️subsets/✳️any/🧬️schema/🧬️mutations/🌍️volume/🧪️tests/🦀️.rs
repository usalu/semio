//! 🎞️ Puzzle3d target-volume mutation scenes — each committed before/mutation/diff/after quartet of the target-volume leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod change_target_volume_hidden {
    #[path = "🙈change-target-volume-hidden/🧪️tests/🙈️hides/🦀️.rs"]
    mod tests_hides_volume_1;
}

#[path = ".."]
mod change_target_volume_locked {
    #[path = "🔐change-target-volume-locked/🧪️tests/🔒️locks/🦀️.rs"]
    mod tests_locks_volume_1;
}

#[path = ".."]
mod create_target_volume {
    #[path = "🌍create-target-volume/🧪️tests/🧊️appends-volume-2/🦀️.rs"]
    mod tests_appends_volume_2;
}

#[path = ".."]
mod delete_target_volume {
    #[path = "🪦delete-target-volume/🧪️tests/🪦️removes-middle-volume/🦀️.rs"]
    mod tests_removes_the_middle_volume;
    #[path = "🪦delete-target-volume/🧪️tests/🚫️removes-volume-1/🦀️.rs"]
    mod tests_removes_volume_1;
}

#[path = ".."]
mod move_target_volume {
    #[path = "🚀move-target-volume/🧪️tests/⬆️lifts-volume-1/🦀️.rs"]
    mod tests_lifts_volume_1;
}

#[path = ".."]
mod rotate_target_volume {
    #[path = "🌀rotate-target-volume/🧪️tests/🔄️half-turn-about-z/🦀️.rs"]
    mod tests_half_turn_about_z;
}

#[path = ".."]
mod scale_target_volume {
    #[path = "📐scale-target-volume/🧪️tests/📏️per-axis-to-uniform/🦀️.rs"]
    mod tests_per_axis_to_uniform;
}
