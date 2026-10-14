//! 🧪️ Fixture laws of the semio text lifecycle mutations (add-mark, insert-run, remove-mark, remove-run), over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../➕add-mark/🧪️tests/🔗️adds/🦀️.rs"]
mod tests_add_mark_adds_a_link_mark_ahead_of_the_bold_mark;
#[path = "../../➖remove-mark/🧪️tests/➖️detaches/🦀️.rs"]
mod tests_remove_mark_detaches_the_italic_mark_from_the_run;
#[path = "../../📥insert-run/🧪️tests/📥️inserts/🦀️.rs"]
mod tests_insert_run_inserts_a_german_run_between_two_english_runs;
#[path = "../../🗑️remove-run/🧪️tests/🚫️removes/🦀️.rs"]
mod tests_remove_run_removes_the_middle_run;
