//! ⚙️ Headless imperative engine: ordered path of side-effect steps.

extern crate semio_framework_os_kernel as protocol;
#[path = "../../⚙️engine/🦀️component.rs"]
pub mod engine;

#[path = "../../📝️compiler/🦀️component.rs"]
pub mod compiler;

#[path = "../../📇️registry/🦀️component.rs"]
pub mod registry;

pub use compiler::compile_to_text;
pub use engine::{EffectLogEntry, Executor, Path, RunResult, Step};
pub use imperative_extension_sdk::{
    build_manifest_json, evaluate_invoke, evaluate_json, imperative_module_contribution, ImperativeExtensionManifest, IMPERATIVE_MODULE_EVALUATE_CAPABILITY, IMPERATIVE_PLAY_APP_ID,
};
pub use registry::{
    contributions_json_from_entries, imperative_catalogue_json, imperative_module_registry, register_default_imperative_contributions, register_native_imperative_module,
    sync_imperative_module_contributions,
};
