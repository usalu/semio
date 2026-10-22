//! 🧪️ model tests (🔬️unit): compiled against the `testkit` surface of the world crates.

use semio_framework_os_infinite_world_model::{World3dBuildContext, WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY, WORLD_DYNAMIC_ID_BYTE_CAPACITY, WORLD_INTERACTION_BYTE_CAPACITY, WORLD_INTERACTION_ID_BYTE_CAPACITY, WORLD_INTERACTION_INTENT_CAPACITY, WORLD_INTERACTION_ITEM_CAPACITY, WORLD_INTERACTION_OBJECT_CAPACITY, world_pick_keys_match, WORLD_PICK_TARGET_CAPACITY, WORLD_PICK_TARGET_POINT_CAPACITY, world_pick_targets_from_json, world_scene_raster_pool, WorldCursorWakeAuthority, WorldCursorWakeFault, WorldDrawRegistry, WorldDynamicEntry, WorldDynamicFault, WorldDynamicRegistry, WorldFlatAction, WorldFlatActionKind, WorldInteractionIntent, WorldInteractionIntentQueue, WorldInteractionObjectKind, WorldInteractionObjectRegistry, WorldInteractionObjectSlot, WorldInteractionPhase, WorldInteractionPlan, WorldOpaqueOwner, WorldOpaqueQuarantine};
use semio_framework_test_fixtures::fixture_text;
use std::collections::BTreeMap;
use ui_scene::math::{Camera3d, Instance3d, Mat4, SceneDraw3d};
use semio_framework_ui_prepared::PreparedRenderUpload;
use semio_framework_ui_theme::geometry::Rect;
use semio_framework_ui_raster_owner::{SceneRasterBegin, SceneRasterDescriptor, SceneRasterLease, SceneRasterProfile, SceneRasterWriteMode};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress};
#[cfg(test)] use semio_framework_ui_draw_contract::gizmo;
#[cfg(test)] use semio_framework_ui_raster_owner::SceneRasterMeshSeal;

/// 📖️ Reads one test input at run time (relative to the package manifest) so it does not count as an embedded compile input.

/// 📖️ Reads several test inputs at run time and joins them into the one text the unsplit sources formed.
fn read_joined(paths: &[&str]) -> &'static str {
    let mut joined = String::new();
    for path in paths {
        joined.push_str(fixture_text!(path));
    }
    Box::leak(joined.into_boxed_str())
}

/// 🌳️ Reads one world crate root and splices every split-out child (a `#[path]` module re-exported with a glob) in at its mount, so the text keeps the item order of the unsplit file.
fn read_world_tree(path_from_manifest: &str) -> String {
    let directory = path_from_manifest.trim_end_matches("🦀️.rs");
    let lines: Vec<&str> = fixture_text!(path_from_manifest).lines().collect();
    let mut text = String::new();
    for (index, line) in lines.iter().enumerate() {
        text.push_str(line);
        text.push('\n');
        if index >= 2 && line.contains("use ") && line.ends_with("::*;") && lines[index - 2].starts_with("#[path = \"") {
            let child = lines[index - 2].trim_start_matches("#[path = \"").trim_end_matches("\"]");
            text.push_str(&read_world_tree(&format!("{directory}{child}")));
        }
    }
    text
}

#[allow(non_snake_case)]
fn WORLD_SOURCE() -> &'static str {
    let joined: String = ["../../🦀️.rs", "../../../🧩️components/🦀️.rs", "../../../🧲️cursors/🦀️.rs", "../../../🧱️scene/🦀️.rs", "../../../🌉️bridge/🦀️.rs", "../../../🧵️interaction/🦀️.rs", "../../../🎛️host/🦀️.rs"].iter().map(|root| read_world_tree(root)).collect();
    Box::leak(joined.into_boxed_str())
}

/// 🧊️ The wgpu renderer glue as the one text the unsplit file held: its root and every mounted child.
#[allow(non_snake_case)]
fn RENDERER_SOURCE() -> &'static str {
    read_joined(&["../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🗂️asset-authority/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🔤️json-lexer-and-schema-types/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🗃️schema-output-and-cursor-state/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🧮️schema-token-application/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🏁️schema-finishing-and-plan-types/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/📐️instance-plan-math-and-outline/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/🧵️materialize-cursor-step/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦴️glb-decode/📖️materialize-readers-and-component-helpers/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔁️asset-decode-jobs/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📊️scale-bench/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🛎️frame-actions/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🎨️presentation-raster/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🏗️mailbox-construction-and-authorities/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/📥️renderer-asset-steps/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🖼️asset-decode-pump-and-reference-images/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/🌐️native-asset-pumps/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📮️mailbox-impl/⏳️pending-work-deadlines-and-spawns/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧾️frame-build/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🎞️present-state-and-engine-closers/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🧯️retirement-and-present-steps/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🖥️frame-presentation/🎬️present-step-inner/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🧠️app-runtime/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🔬️native-socket-probe/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧠️kernel-runtime/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🔁️spawn-recovery/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🚫️request-refusal/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/🛂️admission-claim/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📬️kernel-replay-client/📄️typed-result/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/🏊️kernel-pool/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📡️kernel-protocol/📥️lifted/📦️kernel-runtime/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏊️kernel-pool/📥️lifted/🗃️kernel-retained-state/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/📥️lifted/🔁️asset-decode-jobs/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏛️host-base/📥️lifted/🏊️kernel-pool/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🧮️kernel-core/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/📬️kernel-replay-client/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🗃️kernel-retained-state/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🏗️pool-construction-and-product-replay/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/📼️job-replay-and-command-maintenance/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🧩️app-lifecycle-and-realm-close/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🔄️exchange-and-turn-settling/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/📦️kernel-runtime/🏊️kernel-pool/🚀️turn-dispatch-and-pool-runner/🦀️.rs"])
}

/// 🐚️ The wgpu Shell as the one text the unsplit file held: its root and every child split out of it.
#[allow(non_snake_case)]
fn SHELL_SOURCE() -> &'static str {
    read_joined(&["../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⚓️dock-model-and-measures/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🧾️panel-records/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⚙️settings-and-dock-persistence/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/⌨️keyboard-and-palette/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🧱️chrome-paint-and-navbar/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🔑️storage-prefs-and-i18n/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🎨️theme-file-import-and-accessibility/🦀️.rs"])
}

fn test_scene_raster_lease(width: u32, height: u32, profile: SceneRasterProfile, mesh: Option<SceneRasterMeshSeal>, fill: u8) -> SceneRasterLease {
    static REVISION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let revision = REVISION.fetch_add(1, std::sync::atomic::Ordering::Relaxed).max(1);
    let descriptor = SceneRasterDescriptor { width, height, source_digest: [revision, revision.rotate_left(17)], source_revision: revision, profile, mesh };
    let pool = world_scene_raster_pool();
    let mut writer = None;
    for _ in 0..=semio_framework_ui_raster_owner::SCENE_RASTER_POOL_SLOTS {
        match pool.begin(descriptor, revision, SceneRasterWriteMode::Moved { capacity_bytes: descriptor.byte_len().unwrap() }) {
            SceneRasterBegin::Writer(candidate) => {
                writer = Some(candidate);
                break;
            }
            SceneRasterBegin::Reused(lease) => return lease,
            SceneRasterBegin::Backpressure(_) => {
                let _ = pool.maintenance_step();
            }
            SceneRasterBegin::Refused(fault) => panic!("test scene raster refused: {fault}"),
        }
    }
    let writer = writer.expect("test scene raster progresses within the fixed slot budget");
    let pixels = vec![fill; descriptor.byte_len().expect("test raster bytes")];
    pool.seal_moved(writer, pixels).map_err(|(_, fault)| fault).expect("test raster publishes")
}

#[test]
fn cursor_wake_coalesces_duplicates_and_rearms_after_exact_take() {
    let wake = WorldCursorWakeAuthority::new();
    let mut first_frame = World3dBuildContext::new(wake.clone());
    first_frame.request_cursor_wake();
    first_frame.request_cursor_wake();
    let first = first_frame.take_cursor_wake().unwrap().expect("first frame wake token");
    assert_eq!(first_frame.take_cursor_wake().unwrap(), None, "one frame carries one exact token");
    for _ in 0..128 {
        assert_eq!(wake.request().unwrap().generation(), first.generation(), "a pending wake storm coalesces onto one retained generation");
    }

    let mut consecutive_frame = World3dBuildContext::new(wake.clone());
    consecutive_frame.request_cursor_wake();
    let duplicate = consecutive_frame.take_cursor_wake().unwrap().expect("pending wake coalesces across frames");
    assert_eq!(duplicate.generation(), first.generation());
    assert!(wake.acknowledge(&first));
    assert!(!wake.acknowledge(&duplicate), "duplicate acknowledgement cannot consume another generation");

    let mut resumed_frame = World3dBuildContext::new(wake.clone());
    resumed_frame.request_cursor_wake();
    let resumed = resumed_frame.take_cursor_wake().unwrap().expect("resumed work receives another token");
    assert!(resumed.generation() > first.generation(), "consecutive presented frames retain an ABA-distinguishable generation");
    assert!(!wake.acknowledge(&duplicate), "stale token cannot consume the newer pending generation");
    assert_eq!(wake.pending_generation(), Some(resumed.generation()));
    assert!(wake.acknowledge(&resumed));
    assert_eq!(wake.pending_generation(), None);

    let pending = wake.request().expect("close fixture pending wake");
    assert_eq!(wake.pending_generation(), Some(pending.generation()));
    assert!(!wake.close_step(), "first close grant retires only the pending token scalar");
    assert!(!wake.close_step(), "second close grant retires only the generation scalar");
    assert!(!wake.close_step(), "third close grant retires only the acknowledgement scalar");
    assert!(wake.close_step());
    assert!(wake.terminal_is_empty());
    assert_eq!(wake.request(), Err(WorldCursorWakeFault::Closed));
}

#[test]
fn live_renderer_retains_generation_wake_and_rejects_recreation_erasure_loss_and_duplicate_consumption() {
    const TOKEN_FIELD: &str = "cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>";
    const HOST_TOKEN_FIELD: &str = "cursor_wake_requested: Option<infinite_world::world::WorldCursorWakeToken>";

    fn region<'a>(source: &'a str, start: &str, end: &str) -> Option<&'a str> {
        let start = source.find(start)?;
        let end = source[start..].find(end)?.checked_add(start)?;
        Some(&source[start..end])
    }

    fn replace_region(source: &str, start: &str, end: &str, from: &str, to: &str) -> Option<String> {
        let start = source.find(start)?;
        let end = source[start..].find(end)?.checked_add(start)?;
        let owned = source[start..end].replacen(from, to, 1);
        let mut output = String::with_capacity(source.len().saturating_add(to.len()).saturating_sub(from.len()));
        output.push_str(&source[..start]);
        output.push_str(&owned);
        output.push_str(&source[end..]);
        Some(output)
    }

    fn exact(glue: &str, host: &str, native: &str, browser: &str) -> bool {
        let typed_handoffs = [
            ("AppFrameBuild", "pub(crate) struct AppFrameBuild {", "struct AppFrameAfterChrome {"),
            ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameBuildCursor {"),
            ("FrameBuildCursor", "struct FrameBuildCursor {", "enum FrameBuildPhase {"),
            ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
            ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
            ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
        ];
        let host_handoffs = [("OsHost", "pub struct OsHost {", "struct OsHostRetirementState {"), ("OsHostRetirementState", "struct OsHostRetirementState {", "pub(crate) struct OsHostRetirement {")];
        glue.contains("world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority")
            && glue.contains("World3dBuildContext::new(runtime.world_cursor_wake_authority())")
            && glue.matches(TOKEN_FIELD).count() == typed_handoffs.len()
            && typed_handoffs.iter().all(|(_, start, end)| region(glue, start, end).is_some_and(|handoff| handoff.matches(TOKEN_FIELD).count() == 1))
            && region(glue, "pub(crate) enum AppPresentStep {", "impl AppPresenter {").is_some_and(|handoff| {
                region(handoff, "Complete {", "\n    },").is_some_and(|complete| {
                    let fields = complete.split_whitespace().collect::<String>();
                    ["generation:semio_framework_trace::Generation,", "cursor:SemioCursor,", "fullscreen:Option<bool>,", "cursor_wake:Option<infinite_world::world::WorldCursorWakeToken>,"].iter().all(|field| fields.contains(field))
                }) && handoff.matches(TOKEN_FIELD).count() == 1
            })
            && region(glue, "pub(crate) struct AppPresenter {", "#[derive(Clone, Copy, Debug, PartialEq, Eq)]").is_some_and(|presenter| presenter.matches("pending: Option<AppPresentCursor>").count() == 1)
            && region(glue, "struct AppPresentCursor {", "pub(crate) enum AppPresentStep {").is_some_and(|presenter| presenter.matches("frame: AppFramePresentation").count() == 1)
            && !glue.contains("cursor_wake: bool")
            && !glue.contains("cursor_wake: Option<bool>")
            && !glue.contains("request_frame: bool")
            && glue.contains("fn close_world_cursor_wake_step(&self) -> bool")
            && glue.contains("pub(crate) fn close_cursor_wake_step(&mut self) -> bool")
            && !glue.contains("World3dBuildContext::default()")
            && host.matches(HOST_TOKEN_FIELD).count() == host_handoffs.len()
            && host_handoffs.iter().all(|(_, start, end)| region(host, start, end).is_some_and(|handoff| handoff.matches(HOST_TOKEN_FIELD).count() == 1))
            && !host.contains("cursor_wake_requested: bool")
            && !host.contains("cursor_wake_requested: Option<bool>")
            && host.contains("token.generation() > pending.generation()")
            && host.contains("if self.cursor_wake_requested.take().is_some()")
            && native.contains("self.runtime.acknowledge_world_cursor_wake(&token)")
            && native.contains("self.retain_cursor_wake_directive(token)")
            && native.matches("host.take_cursor_wake_directive()").count() == 1
            && browser.contains("host.take_cursor_wake_directive().is_some()")
    }

    let glue = RENDERER_SOURCE();
    let host = fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏠️os-host/🦀️.rs");
    let native = read_joined(&["../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🎞️frame-generation-and-os-host/🦀️.rs", "../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🪟️winit-app/🚀️native-winit-app/🦀️.rs"]);
    let browser = fixture_text!("../../../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs");
    assert!(exact(glue, host, native, browser));
    assert!(!exact(&glue.replace("World3dBuildContext::new(runtime.world_cursor_wake_authority())", "World3dBuildContext::default()"), host, native, browser));
    for (name, start, end) in [
        ("AppFrameBuild", "pub(crate) struct AppFrameBuild {", "struct AppFrameAfterChrome {"),
        ("AppFrameAfterChrome", "struct AppFrameAfterChrome {", "struct FrameBuildCursor {"),
        ("FrameBuildCursor", "struct FrameBuildCursor {", "enum FrameBuildPhase {"),
        ("AppFramePresentation", "pub(crate) struct AppFramePresentation {", "impl AppFrameBuild {"),
        ("AppFramePreparation", "pub(crate) struct AppFramePreparation {", "impl AppFramePreparation {"),
        ("AppPresentStep::Complete", "pub(crate) enum AppPresentStep {", "impl AppPresenter {"),
    ] {
        let erased = replace_region(glue, start, end, TOKEN_FIELD, "request_frame: bool").expect("enumerated typed wake handoff");
        assert!(!exact(&erased, host, native, browser), "erasing {name} must violate the exact typed-token handoff census");
    }
    let erased_all = glue.replace(TOKEN_FIELD, "request_frame: bool");
    assert!(!exact(&erased_all, host, native, browser), "erasing every internal typed wake handoff must be rejected");
    let extra_bool = glue.replacen("pub(crate) struct AppFrameBuild {", "pub(crate) struct AppFrameBuild {\n    request_frame: bool,", 1);
    assert!(!exact(&extra_bool, host, native, browser), "an extra internal boolean wake channel must be rejected");
    let presenter_erased = glue.replacen("frame: AppFramePresentation", "request_frame: bool", 1);
    assert!(!exact(&presenter_erased, host, native, browser), "presenter ownership must retain the typed presentation handoff");
    let pending_presenter_erased = glue.replacen("pending: Option<AppPresentCursor>", "request_frame: bool", 1);
    assert!(!exact(&pending_presenter_erased, host, native, browser), "AppPresenter must retain its typed pending cursor");
    for (name, start, end) in [("OsHost", "pub struct OsHost {", "struct OsHostRetirementState {"), ("OsHostRetirementState", "struct OsHostRetirementState {", "pub(crate) struct OsHostRetirement {")] {
        let erased = replace_region(host, start, end, HOST_TOKEN_FIELD, "cursor_wake_requested: bool").expect("enumerated host token field");
        assert!(!exact(glue, &erased, native, browser), "erasing {name}'s retained host token must be rejected");
    }
    assert!(!exact(glue, host, &native.replace("self.runtime.acknowledge_world_cursor_wake(&token)", "true"), browser));
    assert!(!exact(glue, host, &native.replace("self.retain_cursor_wake_directive(token)", "drop(token)"), browser));
    assert!(!exact(glue, &host.replace("token.generation() > pending.generation()", "true"), native, browser));
    assert!(!exact(glue, &host.replace("if self.cursor_wake_requested.take().is_some()", "if false"), native, browser));
    assert!(!exact(glue, host, native, &browser.replace("host.take_cursor_wake_directive().is_some()", "host.cursor_wake_requested.is_some()")));
}

#[test]
fn world_flat_plan_enforces_exact_item_and_byte_caps() {
    let mut bytes = WorldInteractionPlan::new(1, 1);
    assert!(bytes.push_string(&"x".repeat(WORLD_INTERACTION_BYTE_CAPACITY)).is_some());
    assert!(bytes.push_string("x").is_none());
    assert!(bytes.faulted);

    let mut items = WorldInteractionPlan::new(1, 1);
    let action = WorldFlatAction { kind: WorldFlatActionKind::Camera, strings: [None; 8], numbers: [0.0; 10] };
    for _ in 0..WORLD_INTERACTION_ITEM_CAPACITY {
        assert!(items.push_action(action));
    }
    assert!(!items.push_action(action));
    assert!(items.faulted);
}

#[test]
fn world_object_registry_enforces_capacity_revision_and_aba() {
    let mut registry = WorldInteractionObjectRegistry::default();
    let first = registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [0.0; 8], None).expect("first object token");
    assert_eq!(registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [0.0; 8], None), Some(first));
    let replacement = registry.admit(1, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [1.0; 8], None).expect("same-revision replacement");
    assert_ne!(replacement, first);
    assert!(registry.resolve(first).is_none());
    let next_revision = registry.admit(2, WorldInteractionObjectKind::Instance, "object-0", None, Mat4::identity(), [1.0; 8], None).expect("new revision replacement");
    assert_ne!(next_revision, replacement);
    assert!(registry.resolve(replacement).is_none());
    for index in 1..WORLD_INTERACTION_OBJECT_CAPACITY {
        assert!(registry.admit(2, WorldInteractionObjectKind::Instance, &format!("object-{index}"), None, Mat4::identity(), [index as f32; 8], None).is_some());
    }
    assert!(registry.admit(2, WorldInteractionObjectKind::Instance, "object-overflow", None, Mat4::identity(), [0.0; 8], None).is_none());
    assert!(registry.faulted);

    let mut oversized = WorldInteractionObjectRegistry::default();
    assert!(oversized.admit(1, WorldInteractionObjectKind::Reference, &"x".repeat(WORLD_INTERACTION_ID_BYTE_CAPACITY + 1), None, Mat4::identity(), [0.0; 8], None).is_none());
    assert!(oversized.faulted);
}

fn world_intent(generation: u64, delta: f32) -> WorldInteractionIntent {
    WorldInteractionIntent { phase: WorldInteractionPhase::Wheel, generation, x: 0.0, y: 0.0, dx: 0.0, dy: 0.0, delta, button: 0, down: false, shift: false, ctrl: false, alt: false, meta: false }
}

#[test]
fn world_intent_queue_retains_exact_fifo_owner_on_saturation_and_closes_one_per_turn() {
    let mut queue = WorldInteractionIntentQueue::default();
    for index in 0..WORLD_INTERACTION_INTENT_CAPACITY {
        queue.push(world_intent(index as u64, index as f32)).expect("fixed intent slot");
    }
    let rejected = queue.push(world_intent(99, 99.0)).expect_err("capacity plus one retains intent");
    assert_eq!(rejected.generation, 99);
    assert_eq!(queue.front().map(|intent| intent.generation), Some(0));
    assert!(!queue.retire_front(1));
    assert_eq!(queue.front().map(|intent| intent.generation), Some(0));
    assert!(queue.retire_front(0));
    assert_eq!(queue.front().map(|intent| intent.generation), Some(1));
    queue.begin_close();
    let mut turns = 0;
    while !queue.close_step() {
        turns += 1;
        assert!(turns < WORLD_INTERACTION_INTENT_CAPACITY);
    }
    assert_eq!(turns, WORLD_INTERACTION_INTENT_CAPACITY - 1);
    assert!(queue.terminal_is_empty());
    assert_eq!(queue.push(rejected).expect_err("closed authority retains late intent").generation, 99);
}

#[test]
fn renderer_world_consumers_use_only_fixed_intent_ingress() {
    let glue = RENDERER_SOURCE();
    let shell = SHELL_SOURCE();
    for source in [glue, shell] {
        assert!(source.contains("enqueue_world3d_event"));
        assert!(!source.contains("handle_world3d_pointer_button"));
        assert!(!source.contains("handle_world3d_pointer_move"));
        assert!(!source.contains("handle_world3d_paint_actions"));
        assert!(!source.contains("handle_world3d_pointer_drag"));
        assert!(!source.contains("handle_world3d_wheel"));
        assert!(!source.contains(concat!("let mut world_actions", " = Vec")));
    }
    let world = WORLD_SOURCE();
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_button")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_move")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_paint_actions")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_pointer_drag")));
    assert!(!world.contains(concat!("pub fn ", "handle_world3d_wheel")));
}

/// 📐️ The world reference underlay is the only raster producer this lane publishes, and a producer it
/// appends UNBOUND is refused by the prepared render job with `raster producer generation is stale` —
/// the silent refusal that held the wgpu shell at exactly one presented frame per boot
/// (`📓️w7b-presenter-one-frame-per-boot.md` §1).
#[test]
fn an_appended_world_scene_raster_keeps_its_exact_pool_lease() {
    use semio_framework_job::InteractiveJob;
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    let lease = test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1);
    let identity = lease.identity();
    resources.ensure_world_plane_texture("plan", lease);
    let mut input = semio_framework_ui_prepared::PreparedRenderInput::try_new(1, 2, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).ok().expect("prepared input admitted");
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    assert!(matches!(input.uploads.get(0), Some(PreparedRenderUpload::SceneRaster { lease, .. }) if lease.identity() == identity));
    for _ in 0..4096 {
        let grant = semio_framework_job::RetainedCloneGrant { maximum_copy_bytes: 16 * 1024, ..input.next_close_demands(16 * 1024).expect("prepared input close demand") };
        if matches!(input.close_step(grant).expect("prepared input close step"), semio_framework_job::InteractiveJobCloseStep::Complete { .. }) {
            break;
        }
    }
    assert!(input.terminal_is_empty());

    let mut input = semio_framework_ui_prepared::PreparedRenderInput::try_new(1, 2, semio_framework_ui_draw_contract::DrawList::default(), None, 0.0).ok().expect("second prepared input admitted");
    let mut resources = World3dBuildContext::new(WorldCursorWakeAuthority::new());
    resources.ensure_world_plane_texture("plan", test_scene_raster_lease(1, 1, SceneRasterProfile::ReferenceImageMapNoColorSpace, None, 1));
    assert_eq!(resources.append_step(&mut input).ok(), Some(false));
    let mut job = semio_framework_ui_prepared::PreparedRenderJob::try_new(input).ok().expect("prepared job admitted");
    let mut preview = 0;
    let (outcome,receipt) = semio_framework_job::drive_step(
        &mut job,
        "world.prepare",
        semio_framework_job::OperationId(1),
        semio_framework_job::Generation(2),
        semio_framework_job::InteractiveStage::BackgroundStep,
        semio_framework_job::StepBudget::new(4, u64::MAX,world_original_fixture_grant()),
        semio_framework_job::root_cancel_token(),
        semio_framework_job::default_now_us,
        &mut preview,
        &mut None,
    );
    assert!(receipt.fits(world_original_fixture_grant()));
    assert!(matches!(outcome, semio_framework_job::StepOutcome::Yield), "a bound producer keeps the preparation alive");
    assert_eq!(job.fault(), None, "the prepared job never refuses this lane's own producer");
    job.begin_close();
    while !matches!(semio_framework_job::close_with_self_funded_context(world_original_fixture_grant(), |cx| InteractiveJob::close_step(&mut job, cx)).expect("original fixture close turn"), semio_framework_job::InteractiveJobCloseStep::Complete{..}) {}
}

/// 🧭️ React's own `resolveSceneGizmoViewportPlacement` oracle — the block margin clears the folded
/// projection pane so the cube sits ABOVE the pane's bottom chip row, never on it.
#[test]
fn world_orbit_view_gizmo_placement_matches_react_bottom_right_insets() {
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 }), (32.0, 58.0));
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 120.0, h: 160.0 }), (32.0, 53.0));
    assert_eq!(gizmo::orbit_view_gizmo_placement(Rect { x: 0.0, y: 0.0, w: 40.0, h: 48.0 }), (22.0, 22.0));
}

#[test]
fn world_orbit_view_gizmo_preserves_label_free_hit_targets() {
    let tips = gizmo::orbit_view_gizmo_tips(&Camera3d::default(), Rect { x: 0.0, y: 0.0, w: 1280.0, h: 720.0 });
    assert_eq!(tips.len(), 15);
    assert_eq!(tips.iter().filter(|tip| tip.prominent).count(), 8);
    assert!(tips.iter().all(|tip| tip.pick_radius >= 7.0));
}

/// 🧲️ LAW: the lane is BOUNDED on the wire — the retained set and every row's polyline are capped at
/// parse time, so no pointer move can carry an unbounded `Vec` behind it.
#[test]
fn the_pick_target_lane_is_capped_at_parse_time_however_large_it_arrives() {
    let rows: Vec<String> = (0..WORLD_PICK_TARGET_CAPACITY + 64)
        .map(|index| {
            let points: Vec<String> = (0..WORLD_PICK_TARGET_POINT_CAPACITY + 32).map(|sample| format!("[{sample}.0,0.0,0.0]")).collect();
            format!(r#"{{"kind":"edge","id":"e-{index}","point":[0,0,0],"points":[{}]}}"#, points.join(","))
        })
        .collect();
    let parsed = world_pick_targets_from_json(&format!("[{}]", rows.join(",")));
    assert_eq!(parsed.len(), WORLD_PICK_TARGET_CAPACITY);
    assert!(parsed.iter().all(|target| target.points.len() == WORLD_PICK_TARGET_POINT_CAPACITY));
    assert!(world_pick_targets_from_json("not json").is_empty(), "a broken lane leaves the retained set empty, never faults");
}

/// 🪪️ LAW: hover keys match across the kernel/pick ALIASES (`solid:` ↔ `object:`, `shell:` → `face:`,
/// `wire:` → `edge:`, `anchor:` → `vertex:`) — the framework twin of the CAD engine's
/// `spatial_hover_key_aliases`, and what stops a guest echo from re-publishing the same hover.
#[test]
fn pick_hover_keys_match_across_the_kernel_and_pick_aliases() {
    assert!(world_pick_keys_match(Some("object:s1"), Some("solid:s1")));
    assert!(world_pick_keys_match(Some("solid:s1"), Some("object:s1")));
    assert!(world_pick_keys_match(Some("shell:h1"), Some("face:h1")));
    assert!(world_pick_keys_match(Some("wire:w1"), Some("edge:w1")));
    assert!(world_pick_keys_match(Some("anchor:a1"), Some("vertex:a1")));
    assert!(!world_pick_keys_match(Some("object:s1"), Some("object:s2")));
    assert!(!world_pick_keys_match(None, Some("object:s1")));
    assert!(!world_pick_keys_match(Some(""), Some("")));
}
//#endregion 🧲️PickTargetLane

#[test]
fn dynamic_registry_returns_capacity_and_identifier_owners_exactly() {
    let mut registry = WorldDynamicRegistry::<u32, 2>::default();
    assert!(registry.insert("a".into(), 1).is_ok());
    assert!(registry.insert("b".into(), 2).is_ok());
    let rejected = registry.insert("c".into(), 3).expect_err("capacity owner");
    assert_eq!(rejected.fault, WorldDynamicFault::RegistryCapacity);
    assert_eq!((rejected.id.as_str(), rejected.value), ("c", 3));
    let rejected = registry.insert("x".repeat(WORLD_DYNAMIC_ID_BYTE_CAPACITY + 1), 4).expect_err("identifier owner");
    assert_eq!(rejected.fault, WorldDynamicFault::IdCapacity);
    assert_eq!(rejected.value, 4);
    assert_eq!(registry.len(), 2);
}

#[test]
fn dynamic_registry_replacement_invalidates_aba_token_and_retains_previous_owner() {
    let mut registry = WorldDynamicRegistry::<u32, 1>::default();
    let (first, _) = registry.insert("mesh".into(), 1).unwrap();
    let (second, previous) = registry.insert("mesh".into(), 2).unwrap();
    let previous = previous.expect("replacement returns previous owner");
    assert_eq!((previous.id.as_str(), previous.value), ("mesh", 1));
    assert_eq!(registry.remove_token(first).unwrap_err(), WorldDynamicFault::StaleToken);
    assert_eq!(registry.remove_token(second).unwrap().value, 2);
    assert!(registry.is_empty());
}

#[test]
fn opaque_quarantine_saturation_returns_the_exact_rejected_owner() {
    let mut quarantine = WorldOpaqueQuarantine::<1>::default();
    let first = WorldOpaqueOwner::Draw(WorldDynamicEntry { id: "first".into(), epoch: 1, value: SceneDraw3d { mesh_key: "first-mesh".into(), mesh_version: 1, instances: Vec::new(), shadow_role: Default::default() } });
    assert!(quarantine.admit(first).is_ok());
    let second = WorldOpaqueOwner::Draw(WorldDynamicEntry { id: "second".into(), epoch: 2, value: SceneDraw3d { mesh_key: "second-mesh".into(), mesh_version: 2, instances: Vec::new(), shadow_role: Default::default() } });
    let rejected = quarantine.admit(second).expect_err("quarantine returns saturated owner");
    let WorldOpaqueOwner::Draw(rejected) = rejected;
    assert_eq!((rejected.id.as_str(), rejected.epoch, rejected.value.mesh_key.as_str(), rejected.value.mesh_version), ("second", 2, "second-mesh", 2));
    assert_eq!(quarantine.saturated, 1);
    assert!(quarantine.take_one().is_some());
    assert!(quarantine.take_one().is_none());
}

#[test]
fn dynamic_close_is_interruptible_and_rejects_late_insertions() {
    let mut registry = WorldDynamicRegistry::<u32, 2>::default();
    registry.insert("a".into(), 1).unwrap();
    registry.insert("b".into(), 2).unwrap();
    registry.begin_close();
    assert_eq!(registry.take_one().expect("one owner per close grant").value, 1);
    assert_eq!(registry.len(), 1);
    let rejected = registry.insert("late".into(), 3).expect_err("closing registry rejects late owner");
    assert_eq!((rejected.fault, rejected.id.as_str(), rejected.value), (WorldDynamicFault::Closing, "late", 3));
    assert_eq!(registry.take_one().expect("resumed close owner").value, 2);
    assert!(registry.is_empty());
}

#[test]
fn draw_registry_returns_the_exact_instance_capacity_owner() {
    let mut draws = WorldDrawRegistry::default();
    let instances = (0..=WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY).map(|index| Instance3d { component_source: None, id: format!("instance-{index}"), model: Mat4::identity(), color: [1.0; 4], selected: false, hovered: false, material: Default::default() }).collect();
    let rejected = draws.push(SceneDraw3d { mesh_key: "mesh".into(), mesh_version: 1, instances, shadow_role: Default::default() }).expect_err("draw instance capacity owner");
    assert_eq!(rejected.fault, WorldDynamicFault::InstanceCapacity);
    assert_eq!(rejected.value.instances.len(), WORLD_DYNAMIC_DRAW_INSTANCE_CAPACITY + 1);
    assert!(draws.is_empty());
}

#[test]
fn production_dynamic_owners_have_no_hash_map_vec_or_direct_pixel_mutation_bypass() {
    let source = WORLD_SOURCE();
    let production = source.split("#[cfg(test)]\nmod tests").next().expect("production source");
    for forbidden in [
        concat!("pub meshes: HashMap<String, Mesh", "3d>"),
        "pub draws: Vec<SceneDraw3d>",
        "reference_pixels: HashMap<String, (u32, u32, Vec<u8>)>",
        "mesh_paint_textures: HashMap<String, (u32, u32, Vec<u8>)>",
        "state.reference_pixels.insert(",
        "state.mesh_paint_textures.insert(",
        "state.draws =",
        "fn rebuild_instance_draws(state:",
    ] {
        assert!(!production.contains(forbidden), "production dynamic owner bypass returned: {forbidden}");
    }
    assert!(production.contains("state.meshes.plan_insert(&id)"), "mesh publication remains centralized at the observed-slot replacement authority");
    assert!(production.contains("step_world3d_dynamic_retirement"), "World3d exposes the one-grant retained close pump");
    assert!(production.contains("struct WorldPlaceholderMeshCursor"));
    assert!(production.contains("mesh3d_allocate_capacity_byte_demand(token)?"));
    assert!(production.contains("struct WorldFaceOverlayMeshCursor"));
    assert!(production.contains("mesh3d_begin_capacity_byte_demand()"));
    let face_route = production.split("fn append_component_face_translucent_overlays").nth(1).and_then(|source| source.split("fn selection_centroid").next()).expect("face overlay production route");
    for forbidden in [concat!("Mesh", "3d::from_buffers"), "FaceOverlayBucket", "HashSet<String>", "Vec<f32>"] {
        assert!(!face_route.contains(forbidden), "face overlay recursive/contiguous constructor returned: {forbidden}");
    }
    let mounted_root = fixture_text!("../../../../🦀️.rs");
    assert!(mounted_root.contains("pub use crate::world::*;"));
    assert!(!mounted_root.contains(concat!("Mesh", "3d")));
}

/// 🧱️ The `boxed_fixed_slots` law for this module's fixed slot tables, against the one committed
/// budget every implementation of it reads (`the committed fixed-slot fixture`).
///
/// Asserts the measured shape of each table (capacity, one slot's bytes, the owner's own bytes)
/// against that record, that each owner is smaller than the table it owns — the structural proof the
/// slots are heap-first rather than an inline `[T; N]` field — and then constructs them on a thread
/// holding only the fixture's `boundedThreadStackBytes`. `Builder::stack_size` overrides
/// `RUST_MIN_STACK`, so the repo runner's 128 MiB floor cannot hide a re-inflated frame here.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn world_interaction_object_slot_table_is_heap_first_and_fits_a_bounded_thread_stack() {
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../🧪️tests/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json")).expect("🧱️ the committed fixed-slot-table budget parses");
    let declared: Vec<semio_framework_async_slots::FixedSlotTableBudget> = fixture["tables"]
        .as_array()
        .expect("🧱️ the budget lists its tables")
        .iter()
        .filter(|table| table["guard"] == "infinite::world")
        .map(|table| {
            semio_framework_async_slots::FixedSlotTableBudget::new(
                table["owner"].as_str().expect("owner"),
                table["capacity"].as_u64().expect("capacity") as usize,
                table["elementSizeBytes"].as_u64().expect("element bytes") as usize,
                table["ownerSizeBytes"].as_u64().expect("owner bytes") as usize,
            )
        })
        .collect();
    let measured = vec![semio_framework_async_slots::FixedSlotTableBudget::new("world::WorldInteractionObjectRegistry", WORLD_INTERACTION_OBJECT_CAPACITY, size_of::<Option<WorldInteractionObjectSlot>>(), size_of::<WorldInteractionObjectRegistry>())];
    semio_framework_async_slots::assert_fixed_slot_tables(
        "infinite::world",
        fixture["boundedThreadStackBytes"].as_u64().expect("bounded stack budget") as usize,
        fixture["conversionThresholdBytes"].as_u64().expect("conversion threshold") as usize,
        &declared,
        &measured,
        || {
            drop(WorldInteractionObjectRegistry::default());
        },
    );
}

/// 🧷️ LAW: every scene pass the world publishes runs its mesh-backed draw lists through an exact
/// residency filter first. The filter is the structural guarantee that a sixth
/// ghost site cannot reintroduce the fatal submit fault.
#[test]
fn the_scene_pass_is_published_behind_the_residency_filter() {
    let world = WORLD_SOURCE();
    assert_eq!(world.matches("ctx.draw.push_scene_pass(ScenePass3d {").count(), 1, "the world publishes exactly one scene pass");
    assert!(
        world.contains("retain_ensured_world_draws(gpu, &mut shadow_draws);\n    retain_ensured_world_draws(gpu, &mut culled_draws);\n    retain_ensured_world_draws(gpu, &mut translucent_draws);\n    retain_ensured_world_material_draws(gpu, &mut material_draws);\n    ctx.draw.push_scene_pass(ScenePass3d {"),
        "the caster, standard color and material draw lists are filtered immediately before the pass is pushed"
    );
}

#[test]
fn scalar_attribute_slot_retirement_keeps_original_backing_until_mesh_owner_close(){
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,retirement::controlled::ControlledRetirement};
    use semio_framework_os_kernel::os_spr::causal::transition::HistoryFoldIndex;
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();
    let reference:Vec<(String,String)>=serde_json::from_value(law["input"].clone()).unwrap();
    let(mut original,birth)=observe(||HistoryFoldIndex::<String,String>::from_iter(reference.iter().cloned()));
    assert_eq!(birth.released_bytes,0);
    assert!(birth.requested_bytes<=world_original_fixture_grant().maximum_capacity_bytes);
    let pointers:Vec<_>=original.slot_entries_mut().map(|(key,value)|(key.as_ptr(),value.as_ptr())).collect();
    let mut independent=std::collections::BTreeMap::from_iter(reference);
    independent.retain(|_,value|value.starts_with("keep"));
    let grant=world_original_fixture_grant();
    for slot in 0..original.slot_count(){
        let remove=!original.slot_entry(slot).unwrap().1.starts_with("keep");
        let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}];
        if remove{denied.push(RetainedCloneGrant{maximum_copy_bytes:std::mem::size_of::<(String,String)>()-1,..grant});}
        for denied in denied{let(receipt,heap)=observe(||original.retire_slot_if(slot,|_,value|value.starts_with("keep"),denied).unwrap());assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
        let(receipt,heap)=observe(||original.retire_slot_if(slot,|_,value|value.starts_with("keep"),grant).unwrap());assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,if remove{std::mem::size_of::<(String,String)>()}else{0});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        if remove{let(receipt,heap)=observe(||original.retire_slot_if(slot,|_,_|false,grant).unwrap());assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    }
    let active:Vec<_>=original.iter().map(|(key,value)|(key,value)).collect();
    assert_eq!(serde_json::to_value(&active).unwrap(),law["active"]);
    assert_eq!(serde_json::to_value(&active).unwrap(),serde_json::to_value(independent.into_iter().collect::<Vec<_>>()).unwrap());
    assert_eq!((0..original.slot_count()).map(|slot|{let(key,value)=original.retained_slot_entry(slot).unwrap();(key.as_ptr(),value.as_ptr())}).collect::<Vec<_>>(),pointers);
    let slots=original.slot_count();
    let((key,value),incoming)=observe(||(String::from("e"),String::from("keep-e")));
    let(reservation,backing)=observe(||original.reserve_insert_step(&key,grant).unwrap());assert!(reservation.fits(grant));assert_eq!((backing.requested_bytes,backing.released_bytes),(reservation.retained_capacity_bytes,reservation.released_bytes));
    let((previous,receipt),movement)=observe(||original.insert_reserved(key,value,grant).unwrap_or_else(|_|panic!("original admitted append")));assert!(previous.is_none());assert!(receipt.fits(grant));assert_eq!((movement.requested_bytes,movement.released_bytes),(0,0));assert_eq!(original.slot_count(),slots+1);
    assert_eq!((0..slots).map(|slot|{let(key,value)=original.retained_slot_entry(slot).unwrap();(key.as_ptr(),value.as_ptr())}).collect::<Vec<_>>(),pointers);
    let(mut owner,handoff)=observe(||ControlledRetirement::new(original).unwrap_or_else(|_|panic!("original native attribute index has controlled authority")));
    assert_eq!((handoff.requested_bytes,handoff.released_bytes),(law["handoffBirthBytes"].as_u64().unwrap()as usize,law["handoffReleaseBytes"].as_u64().unwrap()as usize));
    let(mut born,mut released,mut turns)=(birth.requested_bytes+incoming.requested_bytes+backing.requested_bytes,incoming.released_bytes+backing.released_bytes,0);
    while !owner.terminal_is_empty(){let(step,heap)=observe(||owner.step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<16384);}
    assert_eq!(born,released);let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}

fn world_original_fixture_grant()->semio_framework_value::RetainedCloneGrant{
    let law:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🔬️unit/🧫️retained-attributes.json")).unwrap();let axes=&law["grant"];
    semio_framework_value::RetainedCloneGrant{maximum_items:axes["items"].as_u64().unwrap()as usize,maximum_copy_bytes:axes["copyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:axes["capacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:axes["releaseBytes"].as_u64().unwrap()as usize,maximum_depth:axes["depth"].as_u64().unwrap()as usize}
}
