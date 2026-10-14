//! 🎨️ The `style` subset of the drawing artifact: the paint and text mutation leaves (fill, stroke, opacity, blend mode, fill rule, isolation, text, image).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
#[path = "."]
pub mod replace_layer_fill {
    #[path = "🧬️schema/🧬️mutations/🎨️replace-layer-fill/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🎨️replace-layer-fill/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🎨️replace-layer-fill/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🎨️replace-layer-fill/🧪️tests/🌈️solid/🦀️.rs"]
    mod tests_solid_to_linear_gradient;
}
#[path = "."]
pub mod replace_layer_stroke {
    #[path = "🧬️schema/🧬️mutations/🖊️replace-layer-stroke/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🖊️replace-layer-stroke/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🖊️replace-layer-stroke/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🖊️replace-layer-stroke/🧪️tests/🖊️adds/🦀️.rs"]
    mod tests_adds_a_dashed_stroke;
}
#[path = "."]
pub mod set_group_isolation {
    #[path = "🧬️schema/🧬️mutations/🧩️set-group-isolation/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🧩️set-group-isolation/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🧩️set-group-isolation/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🧩️set-group-isolation/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
#[path = "."]
pub mod set_layer_blend_mode {
    #[path = "🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🌓️set-layer-blend-mode/🧪️tests/✖️normal/🦀️.rs"]
    mod tests_normal_to_multiply;
}
#[path = "."]
pub mod set_layer_fill_rule {
    #[path = "🧬️schema/🧬️mutations/🌀️set-layer-fill-rule/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🌀️set-layer-fill-rule/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🌀️set-layer-fill-rule/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🌀️set-layer-fill-rule/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
#[path = "."]
pub mod set_layer_opacity {
    #[path = "🧬️schema/🧬️mutations/🌫️set-layer-opacity/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🌫️set-layer-opacity/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🌫️set-layer-opacity/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🌫️set-layer-opacity/🧪️tests/🌫️dims/🦀️.rs"]
    mod tests_dims_shape_a_to_half;
}
#[path = "."]
pub mod update_image {
    #[path = "🧬️schema/🧬️mutations/🖼️update-image/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/🖼️update-image/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/🖼️update-image/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/🖼️update-image/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
#[path = "."]
pub mod update_text {
    #[path = "🧬️schema/🧬️mutations/📝️update-text/🔺️diff/🦀️.rs"]
    pub mod diff;
    #[path = "🧬️schema/🧬️mutations/📝️update-text/↩️inverse/🦀️.rs"]
    pub mod inverse;
    #[path = "🧬️schema/🧬️mutations/📝️update-text/🦠️mutation/🦀️.rs"]
    pub mod mutation;
    #[cfg(test)]
    #[path = "🧬️schema/🧬️mutations/📝️update-text/🧪️tests/🔬️unit/🦀️.rs"]
    mod tests;
}
//#endregion 🪢️TaxonomyMounts
