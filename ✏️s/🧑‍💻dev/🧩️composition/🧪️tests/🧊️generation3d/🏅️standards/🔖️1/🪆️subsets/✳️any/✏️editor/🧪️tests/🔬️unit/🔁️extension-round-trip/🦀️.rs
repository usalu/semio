/// 🔁️ Extension round trip: hex column evaluation, contribution pages, the settled tick cycle and late installs.

//#region 🔁️ExtensionRoundTripTests

/// ⚖️ LAW: the served boot fixture `hexagonal-mushroom-column` evaluates END TO END across the
/// extension actor boundary. Every `brep`/`math` operator reaches this app as a CONTRIBUTED stub
/// (`🔬️flow-operators`, the browser's own wiring), so the only way the graph can finish is by
/// crossing the whole round trip once per node — `Emit::extension_invocations` → the SDK's minted
/// `req` and parked continuation → the host running the capability on the plugin the invocation
/// ADDRESSES → `Event::Completed` → `flowEvalResolve` seeding the node cache and re-arming the tick.
///
/// 🐛️ Regression guard for the served-app stall (`📓️runtime-verification-2026-09-09.md` boot #3):
/// the graph reached the renderer with `extrusion-axis: computing` and `data-meshes-json="[]"`
/// forever, because the invocation addressed the flow manifest's own id (`math`) while the host
/// resolves an extension actor by the CONTRIBUTING PLUGIN's id (`flow-extension-math`). Every
/// evaluated node here must end `ok` and the preview must paint real geometry
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn hex_column_evaluates_end_to_end_through_the_extension_round_trip() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;

    let graph = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).expect("node-graph scene decodes off the rendered flow surface");
    let status_json = scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status");
    for widget_id in ["height", "radius", "sides", "profile", "extrusion-axis", "extrude", "column-preview"] {
        assert_eq!(node_eval_status(&status_json, widget_id), "ok", "{widget_id} never finished evaluating: {status_json}");
    }

    let preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    assert!(mesh_count >= 1, "the extruded column must reach the preview as at least one mesh, got {mesh_count}: {}", world.meshes_json);
}

/// ⚖️ LAW: the served machine. The host's `contributionsJson` — built here exactly the way
/// `buildContributionsJson` (`🎠️kernel/🧾️contributions-json/🟦️.ts`) builds it, from the staged extension crates' own
/// manifests — crosses as the page run `semio_framework::public_invocation_string_pages` cuts,
/// through the REAL `setContributions` retained route, and that alone is what makes `brep`/`math`
/// addressable and the whole hex-column chain finish with geometry.
///
/// The payload carries one extra witness manifest nothing else in this binary ever installs, so the
/// registry state this law reads is provably THIS run's delivery and not a leftover
/// `install_flow_extension_manifest` from `🔬️flow-operators` — the served plugin has neither
/// (`📓️extension-addressing-2026-09-10.md` §6.1). The witness is removed again at the end, because
/// the registry is process-wide and this law is a guest in it
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn host_pushed_contribution_pages_install_the_registry_the_served_chain_needs() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).is_err(),
        "the witness extension must not be addressable before the host pushes it"
    );

    let contributions = context::staged_flow_extension_contributions_json(&[(context::CONTRIBUTIONS_WITNESS_PLUGIN_ID, context::contributions_witness_manifest_json())]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    println!("[STATS] contributions payload chars={} pages={} page-bound={}", contributions.chars().count(), pages.len(), semio_framework::PUBLIC_INVOCATION_STRING_BYTES);
    assert!(pages.len() > 1, "the staged closure must exceed one public-invocation string page, else the paging law proves nothing");

    // 🌉️ The shell reaches this route through `AppCommand::Command` → `command_from_action`, not
    // through the typed binary channel, so the ARG DECODE is part of the delivery and is pinned here
    // on the real page the pager cut — including the envelope bound that sized it.
    for (index, page) in pages.iter().enumerate() {
        let counted: usize = page.chars().map(semio_framework::public_invocation_char_cost).sum();
        assert!(counted <= semio_framework::PUBLIC_INVOCATION_STRING_BYTES, "page {index} costs {counted} against the public-invocation string bound");
    }
    let decoded = <Generation3dPlayApp as ArtifactEditor>::command_from_action(
        "setContributions",
        Some(&semio_framework_value::DslValue::object([
            ("json".to_string(), semio_framework_value::DslValue::String(pages[0].clone())),
            ("page".to_string(), semio_framework_value::DslValue::uint(0)),
            ("pageCount".to_string(), semio_framework_value::DslValue::uint(pages.len() as u64)),
        ])),
    )
    .expect("the host's own argument shape decodes into the typed command");
    assert_eq!(decoded, Generation3dCommand::SetContributions(set_contributions::SetContributions { json: pages[0].clone(), page: 0, page_count: pages.len() as u64 }));

    semio_framework_trace::reset_heap_peak();
    let (before_boot, _) = context::heap_probe("before boot");
    let mut app = app_with_registry().await;
    let (after_boot, _) = context::heap_probe("after boot");
    for (index, page) in pages.iter().enumerate() {
        let receipt = context::dispatch(
            &mut app,
            Generation3dCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
        )
        .await;
        assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "contributions page {index} faulted in the retained job ladder");
        if index % 8 == 0 || index + 1 == pages.len() {
            context::heap_probe(&format!("after page {index}"));
        }
    }
    let (after_install, _) = context::heap_probe("after install");
    assert_eq!(semio_framework_os_flow::host_flow_extension_contributions_pending_bytes(), 0, "the assembler retains nothing once its last page lands");
    assert_eq!(
        semio_framework_os_flow::flow_extension_invocation_address(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness must be addressable only because this run's pages carried it"),
        context::CONTRIBUTIONS_WITNESS_PLUGIN_ID
    );
    for extension_id in [crate::flow_operators::BREP_EXTENSION_FLOW_ID, crate::flow_operators::MATH_EXTENSION_FLOW_ID] {
        let address = semio_framework_os_flow::flow_extension_invocation_address(extension_id).expect("a host-pushed contribution makes its extension addressable");
        assert!(address.starts_with("flow-extension-"), "{extension_id} resolved to {address:?}, not to a contributing plugin id");
    }

    drain_flow_eval_ticks(&mut app).await;
    let (after_eval, _) = context::heap_probe("after first eval");
    let graph = context::render(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN).await;
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(&graph).expect("node-graph scene decodes off the rendered flow surface");
    let status_json = scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status");
    for widget_id in ["profile", "extrusion-axis", "extrude", "column-preview"] {
        assert_eq!(node_eval_status(&status_json, widget_id), "ok", "{widget_id} never finished evaluating under a host-pushed registry: {status_json}");
    }
    let preview = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(&preview).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    let mesh_count = meshes.as_array().map_or(0, Vec::len);
    assert!(mesh_count >= 1, "a host-pushed registry must reach a painted preview, got {mesh_count}: {}", world.meshes_json);
    println!("[STATS] host-pushed chain finished: meshes={mesh_count}");
    let (after_mesh, peak) = context::heap_probe("after first tessellation");
    println!(
        "[MEMORY] deltas: boot={} install={} eval={} mesh={} peak={peak} ceiling={} budget={}",
        after_boot - before_boot,
        after_install - after_boot,
        after_eval - after_install,
        after_mesh - after_eval,
        semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes(),
        semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES
    );
    // ⚖️ LAW: the whole boot + contributions-install + first-mesh sequence peaks under the schema's
    // declared share of the guest's linear-memory budget — the bound that turns a browser-only
    // `rust_oom` trap into a native failure (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    let ceiling = semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes() as isize;
    assert!(
        peak < ceiling,
        "the boot + install + first-mesh sequence peaked at {peak} B, over {}% of the {} B guest budget ({ceiling} B)",
        semio_framework_trace::GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT,
        semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES
    );
    semio_framework_os_flow::uninstall_flow_extension(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
}

/// 🪟️ The thirteen surfaces the SERVED shell mounts in one `surface-visible` burst, in the order the
/// browser sends them (measured on 6018, `🗑️generated/fix-forward-contributions/bridge-trace4`): the
/// eight app bodies, the framework History body and the four framework sections. A guest turn that
/// cannot render all thirteen never hands its turn back, and the shard watchdog reads that as a dead
/// worker (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const GENERATION3D_SHELL_MOUNTED_BODY_KEYS: [&str; 13] = [
    flow_window::GENERATION_3D_PLAY_BODY_MAIN,
    edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW,
    generations::GENERATION_3D_PLAY_BODY_GENERATIONS,
    form::GENERATION_3D_PLAY_BODY_GENERATE_FORM,
    generate_preview::GENERATION_3D_PLAY_BODY_GENERATE_PREVIEW,
    artifact_panel::GENERATION_3D_PLAY_BODY_ARTIFACT,
    catalogue_panel::GENERATION_3D_PLAY_BODY_CATALOGUE,
    inspection_panel::GENERATION_3D_PLAY_BODY_INSPECTION,
    "framework.body.history",
    "framework.section.engagements",
    "framework.section.measures",
    "framework.section.tools",
    "framework.section.catalogue",
];

/// ⚖️ LAW: every surface the shell mounts in its boot burst renders, and the whole burst costs less
/// than one interactive turn. The reactor renders every dirty surface inside ONE `reactor::poll`
/// (`⚛️reactor/🔄️turn/🦀️.rs`, the `dirty.surfaces` loop between the `presence-clock` and `render`
/// phases), so a single body that cannot finish takes the whole turn — and with it the worker's
/// progress ticker, which the host then reads as `shard 0 terminated by the host watchdog`.
#[semio_framework_async_macros::async_test]
async fn the_shell_boot_surface_burst_renders_inside_one_turn() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow, preview) = context::shell_views("procedural-main", "procedural-preview-test");
    let burst = std::time::Instant::now();
    for body_key in GENERATION3D_SHELL_MOUNTED_BODY_KEYS {
        let view = if body_key == flow_window::GENERATION_3D_PLAY_BODY_MAIN { flow.clone() } else { preview.clone() };
        let started = std::time::Instant::now();
        eprintln!("[STATS] shell burst rendering {body_key}");
        let rendered = app.render(body_key, None, &view).await;
        let elapsed = started.elapsed();
        println!("[STATS] shell burst {body_key} ok={} elapsed_ms={}", rendered.is_ok(), elapsed.as_millis());
        assert!(elapsed < std::time::Duration::from_secs(2), "surface {body_key} took {elapsed:?} to render, more than one interactive turn can afford");
        if let Ok(tree) = rendered {
            semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(tree).expect("a rendered shell surface projects");
        }
    }
    let burst = burst.elapsed();
    println!("[STATS] shell boot burst total_ms={}", burst.as_millis());
    assert!(burst < std::time::Duration::from_secs(4), "the whole shell boot surface burst took {burst:?}, over the one-turn budget the reactor renders it in");
}

/// ⏱️ Native wall budget for ONE served `setContributions` crossing. The host's shard watchdog kills
/// a worker that is silent for `SHARD_LIVENESS_POLICY`'s budget (18 s observed), and a debug wasm
/// guest runs one to two orders of magnitude slower than this debug native build — so a native
/// install that needs seconds is already a dead shard in the browser. Two seconds is the widest
/// figure that still fails fast (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const SERVED_CONTRIBUTIONS_INSTALL_BUDGET: std::time::Duration = std::time::Duration::from_secs(2);

/// ⚖️ LAW: the crossing the SERVED shell actually performs. Since the pack-encoded command ingress
/// replaced the 4 KiB string pager (`🏛️ShellHost/🟦️.tsx`, "One pack crossing"), the whole scoped
/// closure arrives as page 0 of 1 — a single ~250 000-character `json` argument in ONE retained
/// command — and the guest must fold it, rebuild the process-wide registry and hand its turn back
/// inside the shard watchdog's silence budget.
///
/// The paged twin above proves delivery; this one proves the served SHAPE and its COST. A guest that
/// installs the same closure correctly but takes 19 s to do it reads to the host as a worker that
/// died (`shard 0 terminated by the host watchdog`, boot check #12), and every later boot repeats the
/// loop for ever because the re-created actor is handed the same command again
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn the_served_one_page_contributions_crossing_installs_inside_the_watchdog_budget() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let contributions = context::staged_flow_extension_contributions_json(&[(context::CONTRIBUTIONS_WITNESS_PLUGIN_ID, context::contributions_witness_manifest_json())]);
    assert!(contributions.len() > semio_framework::PUBLIC_INVOCATION_STRING_BYTES, "a one-page law proves nothing on a payload that would have fitted one 4 KiB string page");
    let mut app = app_with_registry().await;
    let started = std::time::Instant::now();
    let receipt = context::dispatch(&mut app, Generation3dCommand::SetContributions(set_contributions::SetContributions { json: contributions.clone(), page: 0, page_count: 1 })).await;
    let elapsed = started.elapsed();
    println!("[STATS] served one-page contributions install chars={} elapsed_ms={}", contributions.len(), elapsed.as_millis());
    assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "the served one-page crossing faulted in the retained job ladder");
    assert_eq!(
        semio_framework_os_flow::flow_extension_invocation_address(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness must be addressable only because this run's single page carried it"),
        context::CONTRIBUTIONS_WITNESS_PLUGIN_ID
    );
    assert_eq!(semio_framework_os_flow::host_flow_extension_contributions_pending_bytes(), 0, "a one-page run retains nothing once its only page lands");
    semio_framework_os_flow::uninstall_flow_extension(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
    assert!(
        elapsed < SERVED_CONTRIBUTIONS_INSTALL_BUDGET,
        "the served one-page contributions crossing took {elapsed:?} of native debug time, over the {SERVED_CONTRIBUTIONS_INSTALL_BUDGET:?} budget — the wasm guest pays this many times over and the shard watchdog kills it mid-turn"
    );
}

/// ⚖️ LAW: the STEADY STATE holds. A served boot does not stop at the first mesh — the preview
/// window re-arms `flowEvalTick` for as long as the tab is open, and the guest it runs in has ONE
/// fixed linear memory ([`semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES`]) with no
/// process to restart. Boot #9b trapped `rust_oom` ~60 s after boot, so the bound this law states is
/// exactly that horizon: whatever one settled tick+render cycle retains, `STEADY_STATE_CYCLES` of
/// them must still fit in the headroom the install-peak ceiling leaves
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn the_settled_evaluation_tick_cycle_retains_nothing_that_would_exhaust_the_guest() {
    /// ⏱️ Tick+render cycles a 60 s browser boot runs at the preview window's re-arm cadence.
    const STEADY_STATE_CYCLES: isize = 600;
    /// 📐️ Cycles actually run here — the growth is linear in the leak, so a short run measured and
    /// extrapolated is the same law at a fraction of the suite's time.
    const MEASURED_CYCLES: isize = 60;
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    drain_flow_eval_ticks(&mut app).await;
    let _ = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    let (settled, _) = context::heap_probe("steady state armed");
    for _ in 0..MEASURED_CYCLES {
        drain_flow_eval_ticks(&mut app).await;
        let _ = context::render(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW).await;
    }
    let (after, _) = context::heap_probe("steady state settled");
    let per_cycle = (after - settled) / MEASURED_CYCLES;
    let headroom = (semio_framework_trace::GUEST_LINEAR_MEMORY_MAXIMUM_BYTES - semio_framework_trace::guest_linear_memory_install_peak_ceiling_bytes()) as isize;
    println!("[MEMORY] steady state: per-cycle={per_cycle} horizon={} headroom={headroom}", per_cycle * STEADY_STATE_CYCLES);
    assert!(
        per_cycle * STEADY_STATE_CYCLES < headroom,
        "a settled tick+render cycle retains {per_cycle} B, so {STEADY_STATE_CYCLES} of them claim {} B of the {headroom} B the install-peak ceiling leaves in the guest budget",
        per_cycle * STEADY_STATE_CYCLES
    );
}

/// ⚖️ LAW: every `Effect::InvokeExtension` this app emits carries the CONTRIBUTING PLUGIN's id,
/// resolved through the single translation surface
/// [`semio_framework_os_flow::flow_extension_invocation_address`] — the flow manifest's own id
/// (`brep`, `math`) is never an address, and the tessellate hop in particular must reach
/// `flow-extension-brep`.
///
/// 🪪️ And its inverse: with the geometry kernel's CONTRIBUTION removed (the served app's actual
/// state — nothing installs contributed manifests into a browser plugin instance, see
/// `📓️extension-addressing-2026-09-10.md` §3), the preview window must publish a typed
/// `phase: "faulted"` carrying both languages and the miss's own `fault` object, instead of the
/// per-tick `eprintln!` boot #7 counted 46 times (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn extension_invocations_address_the_contributing_plugin_and_a_missing_contribution_faults_the_preview() {
    use crate::flow_operators::{BREP_EXTENSION_FLOW_ID, BREP_EXTENSION_PLUGIN_ID, MATH_EXTENSION_PLUGIN_ID};
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (view, _) = preview_views("procedural-address-test", "procedural-address-test-other");
    let mut addresses: Vec<(String, String)> = Vec::new();
    context::drive_preview_run_serving(&mut app, &view, &[], &mut |pending| {
        addresses.push((pending.extension_id.clone(), pending.capability.clone()));
        crate::brep_extension::serve(pending)
    })
    .await;
    assert!(!addresses.is_empty(), "the hex column boot must cross the extension boundary at least once");
    for (extension_id, capability) in &addresses {
        assert!(extension_id == BREP_EXTENSION_PLUGIN_ID || extension_id == MATH_EXTENSION_PLUGIN_ID, "invocation address {extension_id:?} (capability {capability:?}) is not a contributing plugin id");
    }
    assert!(
        addresses.iter().any(|(extension_id, capability)| extension_id == BREP_EXTENSION_PLUGIN_ID && capability == "tessellate"),
        "the tessellate hop must address the contributing brep plugin: {addresses:?}"
    );

    let miss = semio_framework_os_flow::flow_extension_invocation_address("not-contributed-by-anything").expect_err("an uncontributed flow extension id has no invocation address");
    let mut owner = Generation3dInstanceOperationOwner::new();
    let faulted: serde_json::Value = owner
        .with_session_waking(|session| serde_json::from_str(&semio_s_artifact_procedural_3d_preview_eval::preview_progress_status_json_for(Some(session), None, Err(miss))).expect("preview status json"))
        .expect("the retained evaluation-session owner lends its session");
    for _ in 0..1_000_000 {
        if matches!(semio_framework_plugin::ArtifactInstanceOperationOwner::close_step(&mut owner, usize::MAX, usize::MAX), Ok(semio_framework_plugin::PluginCloseStep::Complete)) {
            break;
        }
    }
    assert!(semio_framework_plugin::ArtifactInstanceOperationOwner::terminal_is_empty(&owner), "the projection's own session owner must close terminal-empty");
    assert_eq!(faulted.get("phase").and_then(serde_json::Value::as_str), Some("faulted"), "{faulted}");
    let label = faulted.get("phaseLabel").expect("the faulted phase carries its own label pair");
    assert!(label.get("en").and_then(serde_json::Value::as_str).is_some_and(|text| !text.is_empty()));
    assert!(label.get("de").and_then(serde_json::Value::as_str).is_some_and(|text| !text.is_empty()));
    assert_eq!(faulted.get("cancellable").and_then(serde_json::Value::as_bool), Some(false), "a preview that never invoked anything offers no abort");
    let fault = faulted.get("fault").expect("the faulted status names the miss");
    assert_eq!(fault.get("code").and_then(serde_json::Value::as_str), Some(semio_framework_os_flow::FlowExtensionAddressMiss::CODE));
    assert_eq!(fault.get("extensionId").and_then(serde_json::Value::as_str), Some("not-contributed-by-anything"));
    for language in ["en", "de"] {
        let message = fault.get("message").and_then(|value| value.get(language)).and_then(serde_json::Value::as_str).unwrap_or_default();
        assert!(message.contains("not-contributed-by-anything"), "the {language} message must name the unresolved flow extension id: {message}");
    }
    let contributed = fault.get("contributed").and_then(serde_json::Value::as_array).expect("the miss names every translation the session DOES carry");
    for (extension_id, plugin_id) in [(BREP_EXTENSION_FLOW_ID, BREP_EXTENSION_PLUGIN_ID), ("math", MATH_EXTENSION_PLUGIN_ID)] {
        assert!(
            contributed.iter().any(|entry| entry.get("extensionId").and_then(serde_json::Value::as_str) == Some(extension_id) && entry.get("pluginId").and_then(serde_json::Value::as_str) == Some(plugin_id)),
            "{extension_id} -> {plugin_id} missing from {contributed:?}"
        );
    }
}

/// ⚖️ LAW: the SERVED ORDER. `ShellHost` loads the example and only THEN pushes the contributions
/// closure, so the first evaluation always runs against a registry that cannot address the geometry
/// kernel — and the miss is retained, not merely late: it sits in the session's neural cache, in its
/// incremental baseline, in the published `eval_json`/`status_json` and in the tessellation ledger a
/// surface projects as `phase: "faulted"`. Installing operators into a process-wide registry
/// publishes nothing, so boot #11 sat at `faulted` for 2.5 minutes with no user action able to move
/// it (`📓️runtime-verification-2026-09-09.md`).
///
/// This law drives that exact order and asserts the recovery is AUTOMATIC: the page run's last page
/// invalidates the retained session against the new
/// `semio_framework_os_flow::flow_extension_registry_generation` and owes every attached preview
/// window an evaluation, which the surface's own `pending_effects` turns into a `previewEval` run.
///
/// 🧪️ A `--lib` binary LINKS both extension packs (`🔬️flow-operators`), and a linked pack shadows
/// the contributed stub — so an empty contribution table alone still evaluates, and would prove
/// nothing. [`UnlinkedFlowExtensions`] therefore retires the linked installers for the length of
/// this law, which is the served guest's actual shape: nothing linked, everything contributed. The
/// evaluate hop then crosses the real extension wire, answered in-process by `🔬️brep-extension`
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn a_late_contributions_install_re_arms_the_evaluation_the_empty_registry_faulted() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let _unlinked = crate::flow_operators::UnlinkedFlowExtensions::take();
    semio_framework_os_flow::sync_host_flow_extension_contributions("[]".to_string()).expect("an empty closure is a legal registry state — it is the state every served boot starts in");
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_err(),
        "the geometry kernel must be unaddressable before the host pushes anything, or this law proves nothing"
    );

    let mut app = app_with_registry().await;
    let (flow_view, preview_view) = context::shell_views("procedural-rearm-main", "procedural-rearm-preview");
    context::dispatch_with_view(
        &mut app,
        Generation3dCommand::SetActiveExample(set_active_example::SetActiveExample { example_id: semio_s_artifact_procedural_3d_model::PROCEDURAL_EXAMPLE_HEX_COLUMN.to_string() }),
        flow_view.clone(),
    )
    .await
    .expect("the example loads before any contribution arrives, exactly as ShellHost orders it");
    context::drain_armed_flow_eval_ticks(&mut app, &flow_view).await;
    let mut install_effects = Vec::new();

    let faulted_preview = context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await;
    assert_eq!(preview_phase(&faulted_preview), "faulted", "an unaddressable geometry kernel must fault the preview");
    assert_eq!(preview_mesh_count(&faulted_preview), 0, "a faulted preview paints nothing");
    let faulted_status = flow_status_json(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    assert_ne!(node_eval_status(&faulted_status, "extrude"), "ok", "the extrusion cannot have evaluated against a registry with no operators: {faulted_status}");
    println!("[STATS] before the install: {faulted_status}");

    let contributions = context::staged_flow_extension_contributions_json(&[]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    println!("[STATS] late install: pages={} page-bound={}", pages.len(), semio_framework::PUBLIC_INVOCATION_STRING_BYTES);
    for (index, page) in pages.iter().enumerate() {
        let receipt = context::dispatch_with_view(
            &mut app,
            Generation3dCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
            flow_view.clone(),
        )
        .await
        .expect("contributions page");
        assert!(!receipt.lanes.contains(&semio_framework_plugin::app::TypedOperationResultLane::Fault), "contributions page {index} faulted in the retained job ladder");
        let carried = context::run_actions(&receipt.effects);
        if index + 1 < pages.len() {
            assert!(carried.is_empty(), "page {index} installs nothing yet, so it carries nothing: {carried:?}");
        } else {
            assert_eq!(carried, vec![semio_framework_tool_run::TOOL_RUN_START_ACTION_ID.to_string()], "the last page carries the attached preview exactly one run start");
            install_effects = receipt.effects.clone();
        }
    }
    assert!(
        semio_framework_os_flow::flow_extension_invocation_address(crate::flow_operators::BREP_EXTENSION_FLOW_ID).is_ok(),
        "the pushed closure must make the geometry kernel addressable"
    );

    let ticks = context::drain_armed_flow_eval_ticks_from(&mut app, &flow_view, &install_effects).await;
    assert!(ticks > 0, "the run the install carried must evaluate the graph again");
    let status_json = flow_status_json(&context::render_with_view(&mut app, flow_window::GENERATION_3D_PLAY_BODY_MAIN, &flow_view).await);
    for widget_id in ["profile", "extrusion-axis", "extrude", "column-preview"] {
        assert_eq!(node_eval_status(&status_json, widget_id), "ok", "{widget_id} never re-evaluated after the late install: {status_json}");
    }
    let preview = context::render_with_view(&mut app, edit_preview::GENERATION_3D_PLAY_BODY_PREVIEW, &preview_view).await;
    assert_ne!(preview_phase(&preview), "faulted", "the install must clear the preview fault without any user action");
    let meshes = preview_mesh_count(&preview);
    assert!(meshes >= 1, "the re-armed chain must reach a painted preview, got {meshes}");
    println!("[STATS] late install ran {ticks} hops and painted meshes={meshes}");
}

/// ⚖️ LAW: the invalidation KEY is the registry generation, not "a page run finished". A re-push of
/// an unchanged closure leaves the generation where it was, invalidates nothing and owes the settled
/// run nothing — otherwise every host refresh would restart a settled evaluation
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
#[semio_framework_async_macros::async_test]
async fn re_pushing_an_unchanged_closure_owes_the_settled_run_nothing() {
    let _serial = crate::editor_domain::editor_laws::serial_execution::lock();
    let mut app = app_with_registry().await;
    let (flow_view, _preview_view) = context::shell_views("procedural-repush-main", "procedural-repush-preview");
    // 🪪️ A closure the registry does NOT already hold, so run 0 is a real control: it MUST move the
    // generation and re-arm, which is the only thing that makes run 1's silence meaningful.
    let contributions = context::staged_flow_extension_contributions_json(&[(context::CONTRIBUTIONS_WITNESS_PLUGIN_ID, context::contributions_witness_manifest_json())]);
    let pages = semio_framework::public_invocation_string_pages(&contributions);
    let before = semio_framework_os_flow::flow_extension_registry_generation();
    let mut generations = Vec::new();
    let mut owed_per_run = Vec::new();
    for _ in 0..2 {
        let settled = context::drive_preview_run(&mut app, &flow_view, &[]).await;
        assert_eq!(settled.state.as_deref(), Some("finalized"), "the preview settles before each push: {settled:?}");
        let mut carried = Vec::new();
        for (index, page) in pages.iter().enumerate() {
            let receipt = context::dispatch_with_view(
                &mut app,
                Generation3dCommand::SetContributions(set_contributions::SetContributions { json: page.clone(), page: index as u64, page_count: pages.len() as u64 }),
                flow_view.clone(),
            )
            .await
            .expect("contributions page");
            carried.extend(receipt.effects);
        }
        generations.push(semio_framework_os_flow::flow_extension_registry_generation());
        owed_per_run.push(context::run_actions(&carried));
        context::drive_preview_run(&mut app, &flow_view, &carried).await;
    }
    assert!(generations[0] > before, "a closure the registry did not hold must burn a registry replacement");
    assert_eq!(owed_per_run[0], vec![semio_framework_tool_run::TOOL_RUN_START_ACTION_ID.to_string()], "a changed closure carries the settled preview one restart");
    assert_eq!(generations[0], generations[1], "an unchanged closure must not burn a registry replacement");
    assert!(owed_per_run[1].is_empty(), "an unchanged closure must owe the settled run nothing on the second push: {:?}", owed_per_run[1]);
    println!("[STATS] re-push generations={before} -> {generations:?} owed={owed_per_run:?}");
    semio_framework_os_flow::uninstall_flow_extension(context::CONTRIBUTIONS_WITNESS_EXTENSION_ID).expect("the witness leaves the process-wide registry as it found it");
}


/// 📈 The preview window's published tessellation phase.
fn preview_phase(preview_body_json: &str) -> String {
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(preview_body_json).expect("projected preview body must decode as an assembled world-3d scene");
    let status: serde_json::Value = serde_json::from_str(world.status_json.as_deref().unwrap_or("{}")).expect("preview status json");
    status.get("phase").and_then(serde_json::Value::as_str).unwrap_or("<absent>").to_string()
}

/// 🧊 How many meshes the preview window actually painted.
fn preview_mesh_count(preview_body_json: &str) -> usize {
    let world: semio_framework_ui::wgpu::World3dScene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes(preview_body_json).expect("projected preview body must decode as an assembled world-3d scene");
    let meshes: serde_json::Value = serde_json::from_str(&world.meshes_json).expect("preview meshes json");
    meshes.as_array().map_or(0, Vec::len)
}

/// 🚦 The flow window's per-widget evaluation status object.
fn flow_status_json(flow_body_json: &str) -> String {
    let scene = semio_framework_plugin::artifact_app_laws::decode_fixture_scene_with_lanes::<semio_framework_plugin::NodeGraphScene>(flow_body_json).expect("node-graph scene decodes off the rendered flow surface");
    scene.status_json.clone().expect("the flow window publishes a per-widget evaluation status")
}
//#endregion 🔁️ExtensionRoundTripTests
