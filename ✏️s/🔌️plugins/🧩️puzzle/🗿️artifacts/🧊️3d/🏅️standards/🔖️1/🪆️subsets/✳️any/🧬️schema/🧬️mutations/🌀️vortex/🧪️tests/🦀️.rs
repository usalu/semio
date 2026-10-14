//! 🎞️ Puzzle3d vortex mutation scenes — each committed before/mutation/diff/after quartet of the vortex and attraction leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod add_object_vortex {
    #[path = "➕add-object-vortex/🧪️tests/🌀️appends/🦀️.rs"]
    mod tests_appends_vortex_3_to_object_b;
}

#[path = ".."]
mod connect_vortices {
    #[path = "🪢️connect-vortices/🧪️tests/🧲️adds/🦀️.rs"]
    mod tests_adds_second_attraction;
}

#[path = ".."]
mod disconnect_vortices {
    #[path = "✂️disconnect-vortices/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_attraction_1;
    #[path = "✂️disconnect-vortices/🧪️tests/✂️severs-middle-attraction/🦀️.rs"]
    mod tests_severs_the_middle_attraction;
}

#[path = ".."]
mod remove_object_vortex {
    #[path = "➖remove-object-vortex/🧪️tests/➖removes-middle-vortex/🦀️.rs"]
    mod tests_removes_the_middle_vortex_and_its_attraction;
    #[path = "➖remove-object-vortex/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_vortex_2_and_severs_attraction;
}

#[path = ".."]
mod replace_attraction_geometry {
    #[path = "🧮replace-attraction-geometry/🧪️tests/📍️repositions/🦀️.rs"]
    mod tests_repositions_attraction_1;
}

#[path = ".."]
mod replace_object_vortex {
    #[path = "🔌replace-object-vortex/🧪️tests/⏸️rekind/🦀️.rs"]
    mod tests_rekind_vortex_1_is_noop;
}
