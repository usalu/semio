//! 🏷️ The `metadata` subset of the drawing artifact: the naming, visibility and lock mutation leaves.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod rename_layer {
    #[path = "🧬️schema/🧬️mutations/✏️rename-layer/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/✏️rename-layer/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/✏️rename-layer/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/✏️renames/🦀️.rs"]
    mod tests_renames_shape_a_without_touching_its_id;
}
#[path = "."]
pub mod set_layer_locked {
    #[path = "🧬️schema/🧬️mutations/🔒️set-layer-locked/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🔒️set-layer-locked/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🔒️set-layer-locked/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/🔒️locks/🦀️.rs"]
    mod tests_locks_shape_a;
}
#[path = "."]
pub mod set_layer_visible {
    #[path = "🧬️schema/🧬️mutations/👁️set-layer-visible/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/👁️set-layer-visible/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/👁️set-layer-visible/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/🙈️hides/🦀️.rs"]
    mod tests_hides_shape_a;
}
//#endregion 🪢️TaxonomyMounts

#[path="."]pub mod set_artboard{#[path="🧬️schema/🧬️mutations/🖼️set-artboard/🦠️mutation/🦀️.rs"]pub mod mutation;}
