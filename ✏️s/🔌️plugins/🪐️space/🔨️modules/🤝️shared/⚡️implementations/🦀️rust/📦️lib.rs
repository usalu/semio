//! 🌱️ S Studio plugin — fixtures + document helpers shared by the `home` and `space` apps
//! (non-constitutional: neither app owns this content alone, see the constitutional split recipe's
//! "shared code used by ≥2 apps of the plugin" rule).

use semio_framework_os::{create_backbone_document, register_os_fixture_json, OsWorkflowArtifactDocument, WorkflowDocument, S_WORKFLOW_SCHEMA};
use std::sync::LazyLock;

//#region 🔖️Constants
pub const DEMO_STUDIO_ID: &str = "demo-studio";
pub const DEMO_STUDIO_NAME: &str = "Demo Studio";
/// 📜️ the demo studio is handcrafted `.s` DSL text (a `WorkflowDocument`, see `🔖️DocumentHelpers` —
/// the dissolved `OsProjection`'s successor, see `## The inversion`), not JSON — it is compiled into
/// the binary, so a parse failure here is a bug in the bundled fixture.
pub const DEMO_STUDIO_DSL: &str = include_str!("../../../../../../../✏️s/🔌️plugins/🪐️space/📚️examples/✏️demo.s");
//#endregion 🔖️Constants

//#region 🔖️DocumentHelpers
/// 🧵️ Registers the draw/writer fixture documents referenced by the demo space's app instances —
/// shared by the Home launcher's catalog seed ({@link app_home}) and the Studio app's media export
/// path ({@link app_space}), both of which need these fixtures resolvable before they touch a studio
/// document that references them.
pub fn ensure_space_fixtures_registered() {
    static FIXTURES: LazyLock<()> = LazyLock::new(|| {
        // 🩹️ draw/writer migrated their fixtures from JSON to a handcrafted DSL (`store::DocumentDsl`);
        // this registry is still JSON-shaped (framework/product/os hasn't migrated yet — tracked for
        // the Wave 6 lock step), so `materialize_os_app_instance_document_json`'s `serde_json::from_str`
        // will fall back to `json!({})` for these two slugs until then. Non-fatal: seed content is a
        // convenience default, not required for correctness.
        register_os_fixture_json("🖍️semio.draw.json", include_str!("../../../../../../../✏️s/🔌️plugins/🖍️draw/📚️examples/🖍️semio.draw"));
        register_os_fixture_json("✒️jack.writer.json", include_str!("../../../../../../../✏️s/🔌️plugins/✒️writer/📚️examples/✒️jack.writer"));
    });
    let _ = &*FIXTURES;
}

/// 🌱️ Parses the packaged demo studio fixture into a full `OsWorkflowArtifactDocument` envelope —
/// shared by the Home launcher's catalog seed ({@link app_home}) and the Studio app's
/// `initial_projection` ({@link app_space}). The fixture holds only the `WorkflowDocument` payload
/// (`DEMO_STUDIO_DSL`); the envelope metadata (schema/id/name, freshly-minted history) is built via
/// `create_backbone_document`.
pub fn parse_demo_space_document() -> OsWorkflowArtifactDocument {
    let initial_projection = <WorkflowDocument as store::DocumentDsl>::parse_dsl(DEMO_STUDIO_DSL).expect("bundled example/✏️demo.s is valid WorkflowDocument DSL text");
    create_backbone_document(S_WORKFLOW_SCHEMA, DEMO_STUDIO_ID, DEMO_STUDIO_NAME, initial_projection)
}

pub fn demo_os_document() -> OsWorkflowArtifactDocument {
    parse_demo_space_document()
}

/// @emoji 🌱️ The demo space's bare `WorkflowDocument` — the studio app's `initial_projection`, parsed
/// straight out of the packaged fixture (no envelope/runtime wrapper).
pub fn demo_space_projection() -> WorkflowDocument {
    demo_os_document().vcs.initial_projection
}
//#endregion 🔖️DocumentHelpers
