//! 🧪️ Fixture laws of the semio table row and cell mutations (insert, remove, reorder, edit-cell), over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../✏️edit-cell/🧪️tests/👥️rewrites/🦀️.rs"]
mod tests_edit_cell_rewrites_the_population_cell_of_the_second_row;
#[path = "../../➖remove-row/🧪️tests/🚫️removes-the-leading-row/🦀️.rs"]
mod tests_remove_row_removes_the_leading_row;
#[path = "../../📥insert-row/🧪️tests/📥️inserts/🦀️.rs"]
mod tests_insert_row_inserts_a_row_between_the_two_existing_rows;
#[path = "../../🔃reorder-rows/🧪️tests/🔃️moves/🦀️.rs"]
mod tests_reorder_rows_moves_the_last_row_to_the_front;
