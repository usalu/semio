//! 🧪️ Fixture laws of the semio table column mutations (create, delete, rename, reorder), over the mutation crate's public API.

extern crate semio_framework_os_kernel as protocol;

#[path = "../../🏗️create-column/🧪️tests/🏗️appends/🦀️.rs"]
mod tests_create_column_appends_a_float_column_and_null_pads_every_row;
#[path = "../../🏷️rename-column/🧪️tests/✏️renames/🦀️.rs"]
mod tests_rename_column_renames_city_to_town_without_touching_any_row;
#[path = "../../🔀reorder-columns/🧪️tests/🔀️moves/🦀️.rs"]
mod tests_reorder_columns_moves_the_area_column_to_the_front_and_realigns_every_row;
#[path = "../../🗑️delete-column/🧪️tests/🗑️drops/🦀️.rs"]
mod tests_delete_column_drops_the_middle_column_and_cascades_into_every_row;
