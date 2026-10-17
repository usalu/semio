mod tests {
    use super::*;
    use semio_framework::TopicContribution;
    use crate::workflow::{MediaContract, WorkflowEdge, WorkflowPosition, empty_workflow, placeholder_media_contract, validate_workflow};
    use {semio_framework::AppRole,semio_framework_artifact_reference::ArtifactDialect,semio_framework::MediaClass,semio_framework::MediaForm,semio_framework::MediaType,semio_framework::MediaWireFormat,semio_framework::ModeDefinition,semio_framework::PluginManifest,semio_framework::WindowKindDefinition};
    use std::sync::Arc;
    use store::{os_store::backbone::MemoryBackbone, os_store::backbone::MemoryBackbonePort};
    use semio_framework_ui_locale::LocalizedLabel;
    use ui_wgpu::wgpu::SurfaceKind;

    include!("🔌️plugin-registry-and-hot-swap/🦀️.rs");

    include!("🛟️rollback-contributions-and-recovery/🦀️.rs");

    include!("🧰️host-fixtures/🦀️.rs");

    include!("🎭️actor-context-and-store-laws/🦀️.rs");

    include!("🌱️instances-parameters-and-catalog/🦀️.rs");

    include!("📝️dsl-and-op-text-laws/🦀️.rs");

    include!("🧩️extension-install-laws/🦀️.rs");
}
