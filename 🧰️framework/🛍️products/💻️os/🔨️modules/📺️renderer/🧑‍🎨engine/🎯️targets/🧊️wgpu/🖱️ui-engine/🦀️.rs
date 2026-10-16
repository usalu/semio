//! 🖱️ The retained UI engine: the retained engine cell users, scene interaction, the retained document consumer and the render plan validator.
//!
//! 🧭️ Native and wasm32: the dispatch of pointer, keyboard and accessibility events into the retained trees, the scene interaction step, the retained document consumer and the validation of component scene render plans. The module files keep their place in the interpreter tree and are mounted here as public modules; the `interpreter` module of the renderer is the facade over this crate and its two lower siblings.

extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_renderer_wgpu_contracts as element_contracts;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧩️retained-ui-engine/🦀️.rs"]
pub mod retained_ui_engine;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🖱️scene-interaction/🦀️.rs"]
pub mod scene_interaction;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/📄️retained-document-consumer/🦀️.rs"]
pub mod retained_document_consumer;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🧮️render-plan-validator/🦀️.rs"]
pub mod render_plan_validator;

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
