//! 🧪️ Fixture laws of the semio text edit mutations (change-run-language, edit-run, reorder-runs), over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../✏️edit-run/🧪️tests/✍️rewrites/🦀️.rs"]
mod tests_edit_run_rewrites_the_marked_runs_content;
#[path = "../../🌐change-run-language/🧪️tests/🏷️retags/🦀️.rs"]
mod tests_change_run_language_retags_the_second_run_as_german;
#[path = "../../🔀reorder-runs/🧪️tests/🧭️moves/🦀️.rs"]
mod tests_reorder_runs_moves_the_first_run_to_the_end;
