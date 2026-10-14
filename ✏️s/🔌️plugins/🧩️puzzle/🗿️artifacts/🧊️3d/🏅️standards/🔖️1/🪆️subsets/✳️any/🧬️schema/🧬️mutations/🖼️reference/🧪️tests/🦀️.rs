//! 🎞️ Puzzle3d reference mutation scenes — each committed before/mutation/diff/after quartet of the reference leaves, applied through the dispatch union.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as protocol;

#[path = ".."]
mod change_reference_hidden {
    #[path = "👀change-reference-hidden/🧪️tests/🙈️hides/🦀️.rs"]
    mod tests_hides_reference_1;
}

#[path = ".."]
mod change_reference_locked {
    #[path = "🗝️change-reference-locked/🧪️tests/🔒️locks/🦀️.rs"]
    mod tests_locks_reference_1;
}

#[path = ".."]
mod create_reference {
    #[path = "🖼️create-reference/🧪️tests/🖼️appends-reference-2/🦀️.rs"]
    mod tests_appends_reference_2;
}

#[path = ".."]
mod delete_reference {
    #[path = "🚮delete-reference/🧪️tests/🚫️removes-reference-1/🦀️.rs"]
    mod tests_removes_reference_1;
    #[path = "🚮delete-reference/🧪️tests/🚮️removes-middle-reference/🦀️.rs"]
    mod tests_removes_the_middle_reference;
}

#[path = ".."]
mod move_reference {
    #[path = "🎯move-reference/🧪️tests/↔️slides-reference-1/🦀️.rs"]
    mod tests_slides_reference_1;
}

#[path = ".."]
mod replace_reference_source {
    #[path = "🖇️replace-reference-source/🧪️tests/🖇️repoints/🦀️.rs"]
    mod tests_repoints_reference_1_source;
}

#[path = ".."]
mod resize_reference {
    #[path = "📎resize-reference/🧪️tests/↔️widens-reference-1/🦀️.rs"]
    mod tests_widens_reference_1;
}
