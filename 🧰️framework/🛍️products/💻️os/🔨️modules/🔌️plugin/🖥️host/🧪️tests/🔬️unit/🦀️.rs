
use std::sync::Arc;
use std::collections::BTreeMap;
use semio_framework::kernel::Budget;
use semio_plugin_host_guest_runtime::{GuestRuntimes, PackageRef, WasmtimeRuntime};
use semio_plugin_host_routers::{PluginRegistration, IoEntryKey, IoEntryRoute, IoRouter, io_entries_conflict, resolve_io_route, route_reenters_calling_plugin};
use semio_plugin_guest_contract::{PluginHostError, TurnFault};
use semio_framework_actor::{PackageHash, PackageId, ActorId as RuntimeActorId};
use semio_plugin_host_engine::SharedEngineConfig;
#[cfg(any(test, feature = "testkit"))]
use semio_plugin_host_guest_runtime::MockGuestRuntime;
#[cfg(any(test, feature = "testkit"))]
use semio_plugin_host_engine::minimal_component_without_actor_world;
use semio_plugin_host_guest_runtime::GuestRuntime as _;

//#region 🔖️IoRouterW1d
/// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW `IoRouter` mechanism's own
/// route-resolution/determinism/reentrancy tests. Pure — `resolve_io_route`/
/// `route_reenters_calling_plugin` take a synthetic `BTreeMap<IoEntryKey, IoEntryRoute>`
/// directly, no `PluginInstanceHandle`/real wasm component needed — so these run on every
/// CI/dev machine, unlike `//#region 🔖️IoRouterPostTurnRelay` below. Unchanged from before the
/// `WasmPluginRuntime` deletion — never depended on it.
async fn io_dialect(kind: &str, standard: &str, subset: &str) -> semio_framework_artifact_reference::ArtifactDialect {
    semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() }
}

/// 🎯️ The fixture EVERY test in this region shares — TWO mock plugins:
/// - `"stdio"` owns `framework.io.binary@raw/*` (the binary carrier) `->` `s.stdio.gif@87a/*` at
///   `Exact` fidelity, and declares a sniff.
/// - `"gif"` owns TWO hops: `s.stdio.gif@87a/*` `->` `s.stdio.gif@89a/*` (the 87a-to-89a
///   migration, `Canonical` fidelity, no sniff) AND a DIRECT `framework.io.binary@raw/*` `->`
///   `s.stdio.gif@89a/*` shortcut at `Lossy` fidelity (with a sniff) — a real alternate route
///   from the carrier straight to 89a, deliberately weaker so `route_prefers_higher_minimum_
///   fidelity` below has something genuine to prefer AGAINST.
///
/// This is also the literal fixture `🧪️w1d-io-router-parity.ts` (this ticket's folder) builds
/// through the TS `IoEntryGraph` — both sides must resolve `binary@raw/* -> gif@89a/*` to the
/// SAME 2-hop route via `stdio`'s `Exact` hop then `gif`'s `Canonical` migration hop, per
/// `📓️w1-d-report.md`.
async fn io_router_w1d_fixture_entries() -> Vec<(&'static str, semio_framework::io_schema::IoEntryDescriptor)> {
    let binary_raw = io_dialect("framework.io.binary", "raw", "*").await;
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

    let binary_raw = io_dialect("framework.io.binary", "raw", "*").await;
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
    let binary_raw = io_dialect("framework.io.binary", "raw", "*").await;
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
    let binary_raw = io_dialect("framework.io.binary", "raw", "*").await;
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
    let binary_raw = io_dialect("framework.io.binary", "raw", "*").await;
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
        semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("framework.io.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true },
    )])
    .await;

    let same_plugin_reclaim =
        vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("framework.io.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: true }];
    assert!(io_entries_conflict(&graph, "stdio", &same_plugin_reclaim).is_none(), "the SAME plugin reclaiming its own key must not conflict");

    let different_plugin_claim =
        vec![semio_framework::io_schema::IoEntryDescriptor { from: io_dialect("framework.io.binary", "raw", "*").await, into: io_dialect("s.stdio.gif", "87a", "*").await, fidelity: semio_framework::io_schema::IoFidelity::Lossy, sniffs: false }];
    let conflict = io_entries_conflict(&graph, "gif", &different_plugin_claim).expect("a second plugin claiming the same key must conflict");
    assert!(matches!(conflict, PluginHostError::IoEntryRouteConflict(ref detail) if detail.existing_plugin == "stdio" && detail.incoming_plugin == "gif"));
}
//#endregion 🔖️IoRouterW1d

//#region 🔖️IoRouterPostTurnRelay
#[semio_framework_async_macros::async_test]
async fn wasmtime_runtime_keeps_wrong_abi_components_isolated_by_package() {
    let runtime = WasmtimeRuntime::new(SharedEngineConfig::default(),semio_framework_job::default_now_us).await.expect("engine builds");
    let budget = Budget::original(semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 }, 1_000_000, semio_framework_job::default_now_us().unwrap_or(0), 4, 8, 4096, 1);
    for name in ["stdio", "cad"] {
        let package = PackageRef { package: PackageId(name.to_string()), hash: PackageHash([name.len() as u8; 32]) };
        let compiled = runtime.compile(&package, minimal_component_without_actor_world()).await.unwrap_or_else(|error| panic!("{name} component compiles: {error}"));
        let error = runtime.instantiate(&compiled, RuntimeActorId(1), &[], &budget).await.expect_err(&format!("{name} component does not export `reactor`/`jobs`/`checkpoint`/`describe`"));
        let _ = error;
    }
}

/// 🪪️ Registers one mock-backed package with the process origin registry and returns the router's immutable identity of it. The mock backs registration and routing laws only: Host startup requires the real owned backend, so dispatch laws run against built components.
async fn mock_registration(mock: &Arc<MockGuestRuntime>, actor: RuntimeActorId, package: &str, hash: u8) -> PluginRegistration {
    use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller};
    use semio_plugin_host_post_turn_relay::origin_registry;
    let compiled = mock.compile(&PackageRef { package: PackageId(package.to_string()), hash: PackageHash([hash; 32]) }, &[]).await.expect("mock compile");
    let runtime = Arc::new(GuestRuntimes::Mock(mock.clone()));
    let stock = semio_framework_value::RetainedCloneGrant { maximum_items: 100_000, maximum_copy_bytes: 4_000_000, maximum_capacity_bytes: 16_000_000, maximum_release_bytes: 16_000_000, maximum_depth: 64 };
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(98), Generation(1), stock, || Some(1000), 100_000_000, 1_000_000);
    let identity = origin_registry::next_identity();
    let mut origin = None;
    for _ in 0..4096 {
        if caller.turn(stock, |cx| origin_registry::register_step(identity, &runtime, &compiled, &mut origin, cx)).unwrap() {
            break;
        }
    }
    caller.retire(stock);
    PluginRegistration { actor, origin: origin.expect("fixture registry admits the original compiled parent") }
}

/// 🌉️ Two mock-backed plugins (`stdio` owns `binary->gif87a`, `gif` owns `gif87a->gif89a`) register through the production `register_plugin` path into one shared router.
#[semio_framework_async_macros::async_test]
async fn io_router_registers_two_plugin_registrations() {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let router = IoRouter::new();
    let stdio_registration = mock_registration(&Arc::new(MockGuestRuntime::new().await), RuntimeActorId(200), "stdio", 3).await;
    let gif_registration = mock_registration(&Arc::new(MockGuestRuntime::new().await), RuntimeActorId(201), "gif", 4).await;
    let binary_raw = io_dialect("framework.io.binary", "raw", "*").await;
    let gif_87a = io_dialect("s.stdio.gif", "87a", "*").await;
    let gif_89a = io_dialect("s.stdio.gif", "89a", "*").await;
    router
        .register_plugin("stdio", stdio_registration, &[], &[semio_framework::io_schema::IoEntryDescriptor { from: binary_raw, into: gif_87a.clone(), fidelity: semio_framework::io_schema::IoFidelity::Exact, sniffs: false }])
        .await
        .expect("register stdio");
    router.register_plugin("gif", gif_registration, &[], &[semio_framework::io_schema::IoEntryDescriptor { from: gif_87a, into: gif_89a, fidelity: semio_framework::io_schema::IoFidelity::Canonical, sniffs: false }]).await.expect("register gif");

    let (plugins, _keys) = router.stats().await.expect("router stats");
    assert_eq!(plugins, 2, "both plugin registrations must be registered with the shared router");
}

fn refusing_driver() -> Result<semio_plugin_host_wake::GuestRelayWakeDriver, semio_framework_value::ValueError> {
    Err(semio_framework_value::ValueError::invariant("self-routed compose never reaches its driver"))
}

fn compose_key() -> Vec<u8> {
    let key = semio_os_kernel::io::IoKey {
        artifact_kind: "s.stdio.step".to_string(),
        standard: "ap214".to_string(),
        subset: "*".to_string(),
        direction: semio_os_kernel::io::IoDirection::Export,
        format_kind: "s.cad".to_string(),
        format_standard: "1".to_string(),
        format_subset: "*".to_string(),
    };
    semio_framework_pack_json::to_json_string(&key).into_bytes()
}

fn compose_dialects() -> Vec<(semio_framework_artifact_reference::ArtifactDialect, Vec<semio_framework_artifact_reference::ArtifactDialect>)> {
    vec![(
        semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() },
        vec![semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.step".to_string(), standard: "ap214".to_string(), subset: "*".to_string() }],
    )]
}

/// 🧬️ `IoRouter::compose` refuses to route back into the calling plugin itself, before any dispatch.
#[semio_framework_async_macros::async_test]
async fn io_router_compose_still_refuses_to_route_back_into_the_calling_plugin() {
    let router = IoRouter::new();
    let mock = Arc::new(MockGuestRuntime::new().await);
    let registration = mock_registration(&mock, RuntimeActorId(203), "cad", 6).await;
    router.register_plugin("cad", registration, &compose_dialects(), &[]).await.expect("register cad");

    let error = router.compose("cad", &compose_key(), b"sources", &refusing_driver).await.expect_err("a plugin routing to its own key must be refused, not dispatched");
    assert!(error.to_string().contains("routing to itself"), "unexpected message: {error}");
}
//#endregion 🔖️IoRouterPostTurnRelay

#[test]
fn host_error_layout_matches_language_neutral_budget() {
    let oracle: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/⚠️error-layout/🔣️.json")).expect("language-neutral error layout budget");
    let host_bytes = size_of::<PluginHostError>();
    let turn_bytes = size_of::<TurnFault>();
    assert!(host_bytes <= oracle["maximumHostErrorInlineBytes"].as_u64().expect("host budget") as usize);
    assert!(turn_bytes <= oracle["maximumTurnFaultInlineBytes"].as_u64().expect("turn budget") as usize);
}
