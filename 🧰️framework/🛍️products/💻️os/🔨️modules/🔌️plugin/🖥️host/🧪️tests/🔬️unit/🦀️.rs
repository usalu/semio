use super::*;

//#region 🔖️IoRouterW1d
/// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `IoRouter` mechanism's own
/// route-resolution/determinism/reentrancy tests. Pure — `resolve_io_route`/
/// `route_reenters_calling_plugin` take a synthetic `BTreeMap<IoEntryKey, IoEntryRoute>`
/// directly, no `PluginInstanceHandle`/real wasm component needed — so these run on every
/// CI/dev machine, unlike `//#region 🔖️IoRouterPostTurnRelay` below. Unchanged from before the
/// `WasmPluginRuntime` deletion — never depended on it.
async fn io_dialect(kind: &str, standard: &str, subset: &str) -> semio_framework::io_schema::ArtifactDialect {
    semio_framework::io_schema::ArtifactDialect { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() }
}

/// 🎯️ The fixture EVERY test in this region shares — TWO mock plugins:
/// - `"stdio"` owns `s.stdio.binary@raw/*` (the binary carrier) `->` `s.stdio.gif@87a/*` at
///   `Exact` fidelity, and declares a sniff.
/// - `"gif"` owns TWO hops: `s.stdio.gif@87a/*` `->` `s.stdio.gif@89a/*` (the 87a-to-89a
///   migration, `Canonical` fidelity, no sniff) AND a DIRECT `s.stdio.binary@raw/*` `->`
///   `s.stdio.gif@89a/*` shortcut at `Lossy` fidelity (with a sniff) — a real alternate route
///   from the carrier straight to 89a, deliberately weaker so `route_prefers_higher_minimum_
///   fidelity` below has something genuine to prefer AGAINST.
///
/// This is also the literal fixture `🧪️w1d-io-router-parity.ts` (this ticket's folder) builds
/// through the TS `IoEntryGraph` — both sides must resolve `binary@raw/* -> gif@89a/*` to the
/// SAME 2-hop route via `stdio`'s `Exact` hop then `gif`'s `Canonical` migration hop, per
/// `📓️w1-d-report.md`.
async fn io_router_w1d_fixture_entries() -> Vec<(&'static str, semio_framework::io_schema::IoEntryDescriptor)> {
    let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
    let gif_87a = io_dialect("s.stdio.gif", "87a", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    vec![
        ("stdio", semio_framework::io_schema::IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }),
        ("gif", semio_framework::io_schema::IoEntryDescriptor { from: gif_87a, into: gif_89a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Canonical, sniffs: false }),
        ("gif", semio_framework::io_schema::IoEntryDescriptor { from: binary_raw, into: gif_89a, fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: true }),
    ]
}

/// 🏗️ `IoRouter::register_plugin`'s io-entries merge, without needing a real
/// `PluginInstanceHandle` — builds the SAME `BTreeMap<IoEntryKey, IoEntryRoute>` shape
/// directly from `(owner, descriptor)` rows, inserted in WHATEVER order `rows` lists them.
async fn build_io_entry_graph(rows: &[(&'static str, semio_framework::io_schema::IoEntryDescriptor)]) -> BTreeMap<IoEntryKey, IoEntryRoute> {
    let mut graph = BTreeMap::new();
    for (owner, descriptor) in rows {
        let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
        graph.entry(key).or_insert(IoEntryRoute { owner: (*owner).to_string(), fidelity: descriptor.fidelity, sniffs: descriptor.sniffs });
    }
    graph
}

/// 🎯️ "Register two mock plugins in both orders" — the ticket's own determinism proof
/// requirement. `fixture()` order is `stdio, gif, gif`; `reversed` is the exact reverse. Both
/// graphs, and both resolved routes, must be byte-identical.
#[semio_framework_async_macros::async_test]
async fn io_router_route_is_deterministic_across_load_order() {
    let forward = io_router_w1d_fixture_entries().await;
    let mut reversed = forward.clone();
    reversed.reverse();
    let graph_forward = build_io_entry_graph(&forward).await;
    let graph_reversed = build_io_entry_graph(&reversed).await;
    assert_eq!(graph_forward, graph_reversed, "the merged graph itself must not depend on registration order");

    let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    let route_forward = resolve_io_route(&graph_forward, &binary_raw, &gif_89a, 3).expect("forward-order route resolves");
    let route_reversed = resolve_io_route(&graph_reversed, &binary_raw, &gif_89a, 3).expect("reversed-order route resolves");
    assert_eq!(route_forward, route_reversed, "the resolved route must not depend on registration order");
    assert_eq!(route_forward.hops.len(), 2, "the winning route is the 2-hop stdio->gif87a->gif89a path, not the 1-hop lossy shortcut");
}

/// ⚖️ Proves the ranking rule's FIRST tie-break: highest minimum fidelity beats fewest hops.
/// The 1-hop `binary->gif89a` shortcut (Lossy) loses to the 2-hop `binary->gif87a->gif89a`
/// path (min fidelity Canonical) even though it has fewer hops.
#[semio_framework_async_macros::async_test]
async fn io_router_route_prefers_higher_minimum_fidelity_over_fewer_hops() {
    let graph = build_io_entry_graph(&io_router_w1d_fixture_entries().await).await;
    let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).expect("route resolves");
    assert_eq!(route.fidelity, semio_framework::io_schema::IoFidelity::Canonical);
    assert_eq!(route.hops.len(), 2);
    assert_eq!(route.hops[0].from, binary_raw);
    assert_eq!(route.hops[1].into, gif_89a);
}

/// 🌉️ `max_hops` bound is honored: clamped to 1, only the direct (Lossy) shortcut is reachable.
#[semio_framework_async_macros::async_test]
async fn io_router_route_respects_max_hops() {
    let graph = build_io_entry_graph(&io_router_w1d_fixture_entries().await).await;
    let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 1).expect("1-hop route resolves");
    assert_eq!(route.hops.len(), 1);
    assert_eq!(route.fidelity, semio_framework::io_schema::IoFidelity::Lossy);
}

/// 🔒️ `route_reenters_calling_plugin` — the pure predicate behind `run_io`'s guard. A route
/// with NO hop owned by the caller is safe (`None`); a route where the caller owns even ONE
/// hop is refused, naming that hop.
#[semio_framework_async_macros::async_test]
async fn io_router_run_io_reentrancy_guard_predicate() {
    let graph = build_io_entry_graph(&io_router_w1d_fixture_entries().await).await;
    let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    let route = resolve_io_route(&graph, &binary_raw, &gif_89a, 3).expect("route resolves");
    assert_eq!(route_reenters_calling_plugin(&graph, &route, "norm"), None, "a plugin owning neither hop is safe");
    let hop = route_reenters_calling_plugin(&graph, &route, "stdio").expect("stdio owns the first hop of this route");
    assert_eq!(hop.0, &binary_raw);
    assert_eq!(hop.1, &io_dialect("s.stdio.gif", "87a", "*").await);
    let hop = route_reenters_calling_plugin(&graph, &route, "gif").expect("gif owns the second hop of this route");
    assert_eq!(hop.1, &gif_89a);
}

/// 🧯️ A duplicate `(from, into)` claimed by a DIFFERENT plugin than the first registration is a
/// typed conflict — mirrors `io::io_mechanism`'s own `duplicate_entry_is_a_typed_error` law for
/// the OLD graph's `IoRouteConflict`, generalized to the NEW `IoEntryRouteConflict`. Exercises
/// `io_entries_conflict` directly — the SAME function `register_plugin` calls — so this proves
/// the real preflight rule, not a re-derivation of it, without needing a live wasm component.
#[semio_framework_async_macros::async_test]
async fn io_router_register_plugin_rejects_conflicting_io_entry_ownership() {
    let graph = build_io_entry_graph(&[(
        "stdio",
        semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true },
    )])
    .await;

    let same_plugin_reclaim =
        vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }];
    assert!(io_entries_conflict(&graph, "stdio", &same_plugin_reclaim).is_none(), "the SAME plugin reclaiming its own key must not conflict");

    let different_plugin_claim =
        vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("s.stdio.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: false }];
    let conflict = io_entries_conflict(&graph, "gif", &different_plugin_claim).expect("a second plugin claiming the same key must conflict");
    assert!(matches!(conflict, PluginHostError::IoEntryRouteConflict(ref detail) if detail.existing_plugin == "stdio" && detail.incoming_plugin == "gif"));
}
//#endregion 🔖️IoRouterW1d

//#region 🔖️IoRouterPostTurnRelay
#[semio_framework_async_macros::async_test]
async fn wasmtime_runtime_keeps_wrong_abi_components_isolated_by_package() {
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default()).await.expect("engine builds");
    let budget = Budget { fuel: 1_000_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    for name in ["stdio", "cad"] {
        let package = PackageRef { package: PackageId(name.to_string()), hash: PackageHash([name.len() as u8; 32]) };
        let compiled = runtime.compile(&package, minimal_component_without_actor_world()).await.unwrap_or_else(|error| panic!("{name} component compiles: {error}"));
        let error = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect_err(&format!("{name} component does not export `reactor`/`jobs`/`checkpoint`/`describe`"));
        let _ = error;
    }
}

/// 🎬️ `PluginInstanceHandle`'s `run_job_on_worker` (`start-job`, then one `step-job` per pool turn)
/// against `MockGuestRuntime` — the FIRST real coverage of the post-turn job-dispatch mechanism
/// itself (design-runtime.md §2), independent of whether any real `.wasm` exports `world actor`
/// yet. `step_job`'s FIRST scripted outcome is `Running`, forcing the persistent relay to
/// resubmit before the scripted `Done` — proves this is not a single batch call.
#[semio_framework_async_macros::async_test]
async fn plugin_instance_handle_drives_io_run_job_on_worker_through_a_running_step() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(100);
    let compiled = mock.compile(&PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([1u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    let io_payload = semio_framework::io_schema::IoPayload::Text("87a-bytes".to_string());
    mock.script_job_step(actor, JobStep::Done { output: dsl::os_pack::json::to_json_string(&io_payload).into_bytes() }).await;
    let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

    let payload_bytes = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Text("raw-bytes".to_string())).into_bytes();
    let result = handle.io_run("s.stdio.gif@87a/*", "s.stdio.gif@89a/*", payload_bytes).await.expect("job-backed io_run must drive start-job + step-job to Done");
    let decoded: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(std::str::from_utf8(&result).expect("decode io_run result utf8")).expect("decode io_run result");
    assert_eq!(decoded, io_payload);
}

/// 🔍️ `PluginInstanceHandle::io_sniff` decodes the single confidence-rank byte `semio.io-sniff`
/// returns — mirrors the deleted `WasmPluginRuntime::io_sniff`'s exact return contract.
#[semio_framework_async_macros::async_test]
async fn plugin_instance_handle_io_sniff_decodes_the_confidence_byte() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(101);
    let compiled = mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([2u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    mock.script_job_step(actor, JobStep::Done { output: vec![3u8] }).await;
    let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

    let payload_bytes = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Binary(vec![0xFF])).into_bytes();
    let rank = handle.io_sniff("s.stdio.binary@raw/*", "s.stdio.gif@87a/*", &payload_bytes).await.expect("job-backed io_sniff must decode a Done result");
    assert_eq!(rank, 3);
}

/// 🔀️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `PluginInstanceHandle::migrate`
/// drives `semio.migrate` through a `Running` slice before `Done`, exactly like the `io_run`
/// test above — proves this is a real `start-job`/`step-job` loop, not a single call.
#[semio_framework_async_macros::async_test]
async fn plugin_instance_handle_migrate_drives_the_semio_migrate_job_to_completion() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(102);
    let compiled = mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([9u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    mock.script_job_step(actor, JobStep::Done { output: vec![1, 2, 3, 0xAB] }).await;
    let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

    let result = handle.migrate("s.stdio.gif@87a/*", "s.stdio.gif@89a/*", vec![1, 2, 3]).await.expect("job-backed migrate must drive start-job + step-job to Done");
    assert_eq!(result, vec![1, 2, 3, 0xAB]);
}

/// 🧬️ `PluginInstanceHandle::mutation_plan` passes DSL wire-pack bytes straight through, both
/// directions — no re-encoding at this layer (that's `ArtifactMutationRouter::plan`'s job).
#[semio_framework_async_macros::async_test]
async fn plugin_instance_handle_mutation_plan_passes_wire_bytes_through_to_done() {
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(103);
    let compiled = mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([10u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 }).await.expect("mock instantiate");
    mock.script_job_step(actor, JobStep::Done { output: b"planned".to_vec() }).await;
    let handle = PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock.clone())), instance).await;

    let result = handle.mutation_plan(b"request-wire-bytes").await.expect("job-backed mutation_plan must drive start-job + step-job to Done");
    assert_eq!(result, b"planned");
}

/// 🌉️ End-to-end through the REAL `IoRouter`, not just `PluginInstanceHandle` directly: registers
/// two mock-backed plugins (`stdio` owns `binary->gif87a`, `gif` owns `gif87a->gif89a`) using the
/// SAME `register_plugin`/`run_io` production code path a live `🏃️run` boot would use, then
/// drives a real 2-hop `run_io` call where each hop is answered by a DIFFERENT `PluginInstanceHandle`
/// backed by a DIFFERENT `MockGuestRuntime` instance (proving hop-to-hop chaining actually crosses
/// plugin boundaries, not just calls the same handle twice). This is the direct replacement for
/// the deleted `io_router_routes_a_real_cross_plugin_compose_between_two_loaded_wasm_plugins` —
/// same shape (register two plugins into one shared router, route a call that can only be
/// answered by crossing into the OTHER plugin's instance), narrowed to the NEW `io_entries`
/// mechanism (`run_io`, which has a real `jobs.wit` job kind) rather than the OLD `IoKey`-keyed
/// `compose` (which does not — see `IoRouter::compose`'s own doc comment on why that dispatch is
/// not yet wired).
#[semio_framework_async_macros::async_test]
async fn io_router_run_io_crosses_two_real_plugin_instance_handles() {
    let router = IoRouter::new();
    let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };

    let stdio_mock = Arc::new(MockGuestRuntime::new().await);
    let stdio_actor = RuntimeActorId(200);
    let stdio_compiled = stdio_mock.compile(&PackageRef { package: PackageId("stdio".to_string()), hash: PackageHash([3u8; 32]) }, &[]).await.expect("stdio mock compile");
    let stdio_instance = stdio_mock.instantiate(&stdio_compiled, stdio_actor, &[], &budget).await.expect("stdio mock instantiate");
    let midpoint = semio_framework::io_schema::IoPayload::Text("midpoint".to_string());
    stdio_mock.script_job_step(stdio_actor, JobStep::Done { output: dsl::os_pack::json::to_json_string(&midpoint).into_bytes() }).await;
    let stdio_handle = Arc::new(PluginInstanceHandle::new(stdio_actor, Arc::new(GuestRuntimes::Mock(stdio_mock)), stdio_instance).await);

    let gif_mock = Arc::new(MockGuestRuntime::new().await);
    let gif_actor = RuntimeActorId(201);
    let gif_compiled = gif_mock.compile(&PackageRef { package: PackageId("gif".to_string()), hash: PackageHash([4u8; 32]) }, &[]).await.expect("gif mock compile");
    let gif_instance = gif_mock.instantiate(&gif_compiled, gif_actor, &[], &budget).await.expect("gif mock instantiate");
    let final_payload = semio_framework::io_schema::IoPayload::Text("final".to_string());
    gif_mock.script_job_step(gif_actor, JobStep::Done { output: dsl::os_pack::json::to_json_string(&final_payload).into_bytes() }).await;
    let gif_handle = Arc::new(PluginInstanceHandle::new(gif_actor, Arc::new(GuestRuntimes::Mock(gif_mock)), gif_instance).await);

    let binary_raw = io_dialect("s.stdio.binary", "raw", "*").await;
    let gif_87a = io_dialect("s.stdio.gif", "87a", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    router
        .register_plugin("stdio", stdio_handle, &[], &[semio_framework::io_schema::IoEntryDescriptor { from: binary_raw.clone(), into: gif_87a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: false }])
        .await
        .expect("register stdio");
    router.register_plugin("gif", gif_handle, &[], &[semio_framework::io_schema::IoEntryDescriptor { from: gif_87a, into: gif_89a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Canonical, sniffs: false }]).await.expect("register gif");

    let (plugins, _keys) = router.stats().await.expect("router stats");
    assert_eq!(plugins, 2, "both plugin instance handles must be registered with the shared router");

    let start_payload = dsl::os_pack::json::to_json_string(&semio_framework::io_schema::IoPayload::Text("start".to_string())).into_bytes();
    let result_bytes = router.run_io("norm", &binary_raw.to_coordinate(), &gif_89a.to_coordinate(), start_payload).await.expect("2-hop run_io crossing stdio then gif must succeed");
    let decoded: semio_framework::io_schema::IoPayload = dsl::os_pack::json::from_json_str(std::str::from_utf8(&result_bytes).expect("decode final run_io result utf8")).expect("decode final run_io result");
    assert_eq!(decoded, final_payload, "the SECOND hop's (gif's) scripted result must be what comes out — proves the chain really crossed both instance handles in order");
}

/// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): `IoRouter::compose` resolution
/// (unchanged pure algorithm) now feeds a REAL dispatch through `PluginInstanceHandle::compose`
/// — replaces the retired `io_router_compose_resolves_ownership_but_dispatch_is_not_yet_wired`,
/// which pinned down the OLD hand-written host refusal this packet deleted. `MockGuestRuntime`'s
/// `start_job`/`step_job` don't inspect `kind` at all (only whatever is scripted), so scripting a
/// `Done` here proves the host-side plumbing — resolve ownership, find the handle,
/// `start-job`/`step-job` to completion — is fully real end to end; the real guest kind
/// `"semio.compose"` itself is `compose-await`'s to register.
#[semio_framework_async_macros::async_test]
async fn io_router_compose_resolves_ownership_and_drives_the_semio_compose_job_to_completion() {
    let router = IoRouter::new();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(202);
    let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let compiled = mock.compile(&PackageRef { package: PackageId("cad".to_string()), hash: PackageHash([5u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &budget).await.expect("mock instantiate");
    mock.script_job_step(actor, JobStep::Running { progress: None }).await;
    mock.script_job_step(actor, JobStep::Done { output: b"composed".to_vec() }).await;
    let handle = Arc::new(PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance).await);

    let dialects = vec![(
        semio_framework::ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() },
        vec![semio_framework::ArtifactDialect { artifact_kind: "s.stdio.step".to_string(), standard: "ap214".to_string(), subset: "*".to_string() }],
    )];
    router.register_plugin("cad", handle, &dialects, &[]).await.expect("register cad");

    // 🧭️ Key orientation matches what `register_plugin` actually derives from a `(writes, reads)`
    // pair: the Export route is keyed on the READ dialect with the WRITE dialect as its format
    // (see the `candidate_routes` loop above). Asserting the inverse orientation here would fail
    // on route resolution before ever reaching dispatch.
    let key = semio_framework::IoKey {
        artifact_kind: "s.stdio.step".to_string(),
        standard: "ap214".to_string(),
        subset: "*".to_string(),
        direction: semio_framework::IoDirection::Export,
        format_kind: "s.cad".to_string(),
        format_standard: "1".to_string(),
        format_subset: "*".to_string(),
    };
    let key_bytes = dsl::os_pack::json::to_json_string(&key).into_bytes();
    let result = router.compose("stdio", &key_bytes, b"sources").await.expect("compose must resolve ownership AND drive the job to completion, not hard-error");
    assert_eq!(result, b"composed", "the SCRIPTED job outcome must be what comes out, proving real start-job/step-job dispatch reached the resolved owner's handle");
}

/// 🧬️ `IoRouter::compose` still refuses to route back into the calling plugin itself — that
/// guard runs BEFORE dispatch, so it must fire even though dispatch is now real.
#[semio_framework_async_macros::async_test]
async fn io_router_compose_still_refuses_to_route_back_into_the_calling_plugin() {
    let router = IoRouter::new();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let actor = RuntimeActorId(203);
    let budget = Budget { fuel: 1_000, deadline_ms: 4, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
    let compiled = mock.compile(&PackageRef { package: PackageId("cad".to_string()), hash: PackageHash([6u8; 32]) }, &[]).await.expect("mock compile");
    let instance = mock.instantiate(&compiled, actor, &[], &budget).await.expect("mock instantiate");
    let handle = Arc::new(PluginInstanceHandle::new(actor, Arc::new(GuestRuntimes::Mock(mock)), instance).await);

    let dialects = vec![(
        semio_framework::ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() },
        vec![semio_framework::ArtifactDialect { artifact_kind: "s.stdio.step".to_string(), standard: "ap214".to_string(), subset: "*".to_string() }],
    )];
    router.register_plugin("cad", handle, &dialects, &[]).await.expect("register cad");

    let key = semio_framework::IoKey {
        artifact_kind: "s.stdio.step".to_string(),
        standard: "ap214".to_string(),
        subset: "*".to_string(),
        direction: semio_framework::IoDirection::Export,
        format_kind: "s.cad".to_string(),
        format_standard: "1".to_string(),
        format_subset: "*".to_string(),
    };
    let key_bytes = dsl::os_pack::json::to_json_string(&key).into_bytes();
    let error = router.compose("cad", &key_bytes, b"sources").await.expect_err("a plugin routing to its own key must be refused, not dispatched");
    assert!(error.to_string().contains("routing to itself"), "unexpected message: {error}");
}
//#endregion 🔖️IoRouterPostTurnRelay

#[test]
fn host_error_layout_matches_language_neutral_budget() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/⚠️error-layout/🔣️.json")).expect("language-neutral error layout budget");
    let host_bytes = size_of::<PluginHostError>();
    let turn_bytes = size_of::<TurnFault>();
    eprintln!("[DEBUG] Host error inline bytes={host_bytes}, turn fault inline bytes={turn_bytes}");
    assert!(host_bytes <= oracle["maximumHostErrorInlineBytes"].as_u64().expect("host budget") as usize);
    assert!(turn_bytes <= oracle["maximumTurnFaultInlineBytes"].as_u64().expect("turn budget") as usize);
}
