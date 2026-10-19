//! 🎞️ Puzzle3d object mutation scenes — each committed before/mutation/diff/after quartet of the object leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]



extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod change_object_anchor {
    #[path = "⚓change-object-anchor/🧪️tests/⚓️fixed-to-derived/🦀️.rs"]
    mod tests_fixed_to_derived;
}

#[path = ".."]
mod change_object_hidden {
    #[path = "👁️change-object-hidden/🧪️tests/🙈️hides-object-a/🦀️.rs"]
    mod tests_hides_object_a;
}

#[path = ".."]
mod change_object_kind {
    #[path = "🏗️change-object-kind/🧪️tests/🏷️reassigns/🦀️.rs"]
    mod tests_reassigns_object_a_kind;
}

#[path = ".."]
mod change_object_locked {
    #[path = "🔒change-object-locked/🧪️tests/🔒️locks-object-a/🦀️.rs"]
    mod tests_locks_object_a;
}

#[path = ".."]
mod change_object_mesh {
    #[path = "🧱change-object-mesh/🧪️tests/🕸️repoints/🦀️.rs"]
    mod tests_repoints_object_a_mesh;
}

#[path = ".."]
mod create_object {
    #[path = "🌱create-object/🧪️tests/🌱️appends-object-c/🦀️.rs"]
    mod tests_appends_object_c;
}

#[path = ".."]
mod delete_object {
    #[path = "🗑️delete-object/🧪️tests/🚫️removes/🦀️.rs"]
    mod tests_removes_object_a_and_severs_attraction;
    #[path = "🗑️delete-object/🧪️tests/🗑️removes-middle-object/🦀️.rs"]
    mod tests_removes_the_middle_object_and_its_attractions;
}

#[path = ".."]
mod edit_object_label {
    #[path = "🖋️edit-object-label/🧪️tests/🔤️relabels-object-a/🦀️.rs"]
    mod tests_relabels_object_a;
}
