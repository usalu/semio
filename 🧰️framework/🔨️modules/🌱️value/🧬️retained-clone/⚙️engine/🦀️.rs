//! ⚙️ Concrete native clone algorithms expand beside their defining trait and original tests.
#[path="🎁️option/🦀️.rs"]mod option_cursor_engine;
#[path="🧱️tuple/🦀️.rs"]mod tuple_cursor_engine;
#[path="🗃️box/🦀️.rs"]mod box_cursor_engine;
pub use semio_framework_value_native_dynamic_clone::dynamic_cursor_engine;
#[path="🧺️collection/🦀️.rs"]mod collection_cursor_engine;

#[path="📋️copy/🦀️.rs"]mod native_copy;
pub use native_copy::RetainedCopy;

#[path="♻️release/🦀️.rs"]mod native_release;
pub use native_release::RetainedCursorRelease;

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;

#[path="📋️prefix/🦀️.rs"]mod native_prefix;
pub use native_prefix::NativeArrayPrefix;

#[path="📋️field/🦀️.rs"]mod field_cursor_engine;

#[path="🔢️scalar/🦀️.rs"] mod scalar;
#[path="🔤️string/🦀️.rs"] mod string;
#[path="🔗️projection/🦀️.rs"] mod projection_admission;


#[path="🔗️source-owner/🦀️.rs"]mod source_owner_engine;

pub use semio_framework_value_native_array_backing::NativeArrayBacking;

#[path="📦️allocation/🦀️.rs"]mod native_allocation;
pub use native_allocation::allocate_clone_storage;
