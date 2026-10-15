//! 🧭️ The routers of the plugin host: IO, inference, plugin graph, mutation, instance directory, transaction coordinator, app and opening resolver.

use super::*;

//#region 🔖️IoRouter
/// 🌉️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): the host
/// half of cross-plugin artifact reuse. Each `WasmPluginRuntime`'s own in-guest `IO_REGISTRY` only
/// ever sees composers registered inside ITS OWN wasm linear memory; this router is what makes a
/// key owned by plugin B actually reachable from plugin A's `host.io-compose` import — a single
/// shared table (keyed exactly like `semio_framework_os_kernel::io::IoKey`) built by calling `list-artifact-
/// dialects` on every plugin as it loads, mapping each key to the plugin id that owns it, plus a
/// handle to that plugin's own `WasmPluginRuntime` to actually forward the call.
pub struct IoRouter {
    state: Mutex<IoRouterState>,
}

struct IoRouterState {
    routes: HashMap<semio_framework_os_kernel::io::IoKey, String>,
    runtimes: HashMap<String, Arc<PluginInstanceHandle>>,
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW mechanism's merged cross-plugin
    /// graph — `(from, into) -> IoEntryRoute` — built from every plugin's `list-io-entries` roster,
    /// separate from `routes` above (OLD `IoKey`-keyed graph). `BTreeMap` (not `HashMap`) is load-
    /// bearing: `resolve_io_route`'s determinism proof depends on canonical iteration order, mirroring
    /// `io::io_mechanism`'s own `EntryMap`.
    io_entries: BTreeMap<IoEntryKey, IoEntryRoute>,
}

/// 🌉️ One edge of the NEW mechanism's merged graph: `(from, into)`.
pub(super) type IoEntryKey = (semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect);

/// 🌉️ One edge's owner + declared strength — erased from the owning plugin's `IoEntryDescriptor`,
/// plus which plugin registered it (needed by `run_io`'s reentrancy guard and `identify`'s fan-out).
#[derive(Clone, Debug, PartialEq)]
pub(super) struct IoEntryRoute {
    pub(super) owner: String,
    pub(super) fidelity: semio_framework::io_schema::IoFidelity,
    pub(super) sniffs: bool,
}

/// 🌉️ Inverse of `io_schema::Confidence::rank()` — the WIT `io-sniff` guest export returns a raw
/// `u8` rank byte, so the host reconstructs the typed `Confidence` from it before merging.
fn rank_to_io_confidence(rank: u8) -> semio_framework::io_schema::Confidence {
    match rank {
        3 => semio_framework::io_schema::Confidence::High,
        2 => semio_framework::io_schema::Confidence::Medium,
        1 => semio_framework::io_schema::Confidence::Low,
        _ => semio_framework::io_schema::Confidence::None,
    }
}

/// 🌉️ Inverse of `io_schema::IoFidelity::rank()` — mirrors `io::io_mechanism::rank_to_fidelity`
/// (this file cannot import that private fn from `🚪️io/**`, out of this wave's boundary, so the
/// tiny 4-arm match is duplicated here rather than requesting a visibility patch for one line).
fn rank_to_io_fidelity(rank: u8) -> semio_framework::io_schema::IoFidelity {
    match rank {
        3 => semio_framework::io_schema::IoFidelity::Exact,
        2 => semio_framework::io_schema::IoFidelity::Canonical,
        1 => semio_framework::io_schema::IoFidelity::Semantic,
        _ => semio_framework::io_schema::IoFidelity::Lossy,
    }
}

/// 🌉️ `(Reverse(min fidelity rank), hop count, joined into-coordinate string)` — the SAME ranking
/// tuple `io::io_mechanism::route_rank` uses, reimplemented here because the host's merged graph is
/// a DIFFERENT data structure (owner-aware, built from N plugins' wire rosters) than the guest-local
/// `io_mechanism`'s own `&'static IoEntry` registry; the algorithm is identical, only the storage
/// differs.
fn io_route_rank(hops: &[semio_framework::io_schema::IoEntryDescriptor]) -> (std::cmp::Reverse<u8>, usize, String) {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let mut min_fidelity: Option<u8> = None;
    for hop in hops {
        let rank = hop.fidelity.rank();
        min_fidelity = Some(min_fidelity.map_or(rank, |current| current.min(rank)));
    }
    let mut coordinates = Vec::with_capacity(hops.len());
    for hop in hops {
        coordinates.push(hop.into.to_coordinate());
    }
    let joined = coordinates.join(",");
    (std::cmp::Reverse(min_fidelity.unwrap_or(0)), hops.len(), joined)
}

/// 🌉️ Breadth-bounded, cycle-free DFS enumeration of every simple path `from -> into` up to
/// `remaining_hops`, mirroring `io::io_mechanism::walk_routes` exactly. `graph` is a `BTreeMap`, so
/// iteration order is a pure function of the KEY SET, never of insertion/registration order — this
/// plus sorting the FULL candidate set at the end in `resolve_io_route` (never short-circuiting on
/// the first hit) is what makes the result independent of plugin load order — proven by
/// `io_router_route_is_deterministic_across_load_order` below.
fn walk_io_routes(
    graph: &BTreeMap<IoEntryKey, IoEntryRoute>,
    current: &semio_framework_artifact_reference::ArtifactDialect,
    into: &semio_framework_artifact_reference::ArtifactDialect,
    remaining_hops: u8,
    path: &mut Vec<semio_framework::io_schema::IoEntryDescriptor>,
    visited: &mut BTreeSet<semio_framework_artifact_reference::ArtifactDialect>,
    candidates: &mut Vec<Vec<semio_framework::io_schema::IoEntryDescriptor>>,
) {
    if remaining_hops == 0 {
        return;
    }
    for ((from, hop_into), route) in graph.iter() {
        if from != current || visited.contains(hop_into) {
            continue;
        }
        let descriptor = semio_framework::io_schema::IoEntryDescriptor { from: from.clone(), into: hop_into.clone(), fidelity: route.fidelity, sniffs: route.sniffs };
        path.push(descriptor);
        if hop_into == into {
            candidates.push(path.clone());
        } else {
            visited.insert(hop_into.clone());
            walk_io_routes(graph, hop_into, into, remaining_hops - 1, path, visited, candidates);
            visited.remove(hop_into);
        }
        path.pop();
    }
}

/// 🌉️ `resolve_route`'s host-side twin (`io::io_mechanism::resolve_route`) over the merged
/// multi-plugin graph instead of one plugin's own local registry. Pure — no lock, no wasm call —
/// so it is directly unit-testable with a synthetic graph (`io_router_route_is_deterministic_
/// across_load_order`, `io_router_route_prefers_higher_minimum_fidelity`, below).
///
/// 🚫️async: R10 residue shape 1 — `io_route_rank` is async, so ranks are precomputed before
/// the sync `sort_by` comparator rather than called from inside it.
pub(super) fn resolve_io_route(graph: &BTreeMap<IoEntryKey, IoEntryRoute>, from: &semio_framework_artifact_reference::ArtifactDialect, into: &semio_framework_artifact_reference::ArtifactDialect, max_hops: u8) -> Result<semio_framework::io_schema::IoRoute, PluginHostError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let max_hops = max_hops.min(3);
    if max_hops == 0 {
        return Err(PluginHostError::Plugin(format!("io_routes {} -> {}: max_hops clamped to 0", from.to_coordinate(), into.to_coordinate())));
    }
    let mut candidates: Vec<Vec<semio_framework::io_schema::IoEntryDescriptor>> = Vec::new();
    let mut path: Vec<semio_framework::io_schema::IoEntryDescriptor> = Vec::new();
    let mut visited: BTreeSet<semio_framework_artifact_reference::ArtifactDialect> = BTreeSet::new();
    visited.insert(from.clone());
    walk_io_routes(graph, from, into, max_hops, &mut path, &mut visited, &mut candidates);
    if candidates.is_empty() {
        return Err(PluginHostError::Plugin(format!("no io route from {} to {} within {max_hops} hops", from.to_coordinate(), into.to_coordinate())));
    }
    let mut ranked: Vec<(_, Vec<semio_framework::io_schema::IoEntryDescriptor>)> = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        let rank = io_route_rank(&candidate);
        ranked.push((rank, candidate));
    }
    ranked.sort_by(|a, b| a.0.cmp(&b.0));
    let best = ranked.into_iter().next().expect("candidates checked non-empty above").1;
    let mut min_rank: Option<u8> = None;
    for hop in &best {
        let rank = hop.fidelity.rank();
        min_rank = Some(min_rank.map_or(rank, |current| current.min(rank)));
    }
    let fidelity = rank_to_io_fidelity(min_rank.expect("a route has at least one hop"));
    Ok(semio_framework::io_schema::IoRoute { hops: best, fidelity })
}

/// 🌉️ Pure preflight check behind `IoRouter::register_plugin`'s io-entries half: does merging
/// `plugin_id`'s `incoming` roster (its `list-io-entries` wire bytes, decoded) into `existing`
/// claim a `(from, into)` key a DIFFERENT plugin already owns? `None` means the merge is safe
/// (either a brand-new key, or `plugin_id` re-claiming its OWN key — idempotent). Extracted as its
/// own function so the conflict rule is unit-testable without a real `Arc<WasmPluginRuntime>`.
pub(super) fn io_entries_conflict(existing: &BTreeMap<IoEntryKey, IoEntryRoute>, plugin_id: &str, incoming: &[semio_framework::io_schema::IoEntryDescriptor]) -> Option<PluginHostError> {
    for descriptor in incoming {
        let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
        if let Some(current) = existing.get(&key) {
            if current.owner != plugin_id {
                return Some(PluginHostError::IoEntryRouteConflict(Box::new(IoEntryRouteConflict { from: key.0, into: key.1, existing_plugin: current.owner.clone(), incoming_plugin: plugin_id.to_string() })));
            }
        }
    }
    None
}

/// 🌉️ Pure predicate behind `IoRouter::run_io`'s reentrancy guard: does ANY hop of `route`, per
/// `graph`, belong to `calling_plugin_id`? Returns the FIRST such hop's `(from, into)` for the
/// error message, or `None` if the whole route is safe to execute. Extracted as its own function so
/// the guard is unit-testable without a real `Arc<WasmPluginRuntime>` (`run_io` needs one for every
/// OTHER hop it actually executes; this predicate needs none).
pub(super) fn route_reenters_calling_plugin<'route>(
    graph: &BTreeMap<IoEntryKey, IoEntryRoute>,
    route: &'route semio_framework::io_schema::IoRoute,
    calling_plugin_id: &str,
) -> Option<(&'route semio_framework_artifact_reference::ArtifactDialect, &'route semio_framework_artifact_reference::ArtifactDialect)> {
    route.hops.iter().find_map(|hop| {
        let owner = &graph.get(&(hop.from.clone(), hop.into.clone()))?.owner;
        (owner == calling_plugin_id).then_some((&hop.from, &hop.into))
    })
}

impl IoRouter {
    pub fn new() -> Self {
        Self { state: Mutex::new(IoRouterState { routes: HashMap::new(), runtimes: HashMap::new(), io_entries: BTreeMap::new() }) }
    }

    /// 📌️ Registers one already-instantiated plugin's `PluginInstanceHandle` + merges its composer
    /// roster into the shared route table, AND (CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-D) its
    /// NEW `list-io-entries` roster into `state.io_entries`. Call once per plugin, after its
    /// `describe()` `PackageDescriptor` (packet E1) has been decoded — `world actor` has no runtime
    /// `list-artifact-dialects`/`list-io-entries` guest export anymore (`📜️.wit`'s own
    /// `describe` interface doc: "everything a plugin ... used to expose as separate `list-*` WIT
    /// exports ... is now static data inside this one descriptor instead"), so both rosters are the
    /// CALLER's job to decode and pass in — this function makes no wasm call at all. Both graphs
    /// preflight BEFORE either commits — a conflict in either leaves BOTH untouched, matching this
    /// file's existing all-or-nothing registration shape.
    pub async fn register_plugin(
        &self,
        plugin_id: &str,
        handle: Arc<PluginInstanceHandle>,
        artifact_dialect_entries: &[(semio_framework_artifact_reference::ArtifactDialect, Vec<semio_framework_artifact_reference::ArtifactDialect>)],
        io_entries: &[semio_framework::io_schema::IoEntryDescriptor],
    ) -> Result<(), PluginHostError> {
        let mut candidate_routes = Vec::new();
        for (writes, reads) in artifact_dialect_entries {
            for read in reads {
                candidate_routes.push(semio_framework_os_kernel::io::IoKey {
                    artifact_kind: writes.artifact_kind.clone(),
                    standard: writes.standard.clone(),
                    subset: writes.subset.clone(),
                    direction: semio_framework_os_kernel::io::IoDirection::Import,
                    format_kind: read.artifact_kind.clone(),
                    format_standard: read.standard.clone(),
                    format_subset: read.subset.clone(),
                });
                candidate_routes.push(semio_framework_os_kernel::io::IoKey {
                    artifact_kind: read.artifact_kind.clone(),
                    standard: read.standard.clone(),
                    subset: read.subset.clone(),
                    direction: semio_framework_os_kernel::io::IoDirection::Export,
                    format_kind: writes.artifact_kind.clone(),
                    format_standard: writes.standard.clone(),
                    format_subset: writes.subset.clone(),
                });
            }
        }
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        if let Some(existing) = state.runtimes.get(plugin_id) {
            if !Arc::ptr_eq(existing, &handle) {
                return Err(PluginHostError::PluginRuntimeConflict { plugin_id: plugin_id.to_owned() });
            }
        }
        for key in &candidate_routes {
            if let Some(existing_plugin) = state.routes.get(key) {
                if existing_plugin != plugin_id {
                    return Err(PluginHostError::IoRouteConflict(Box::new(IoRouteConflict { key: key.clone(), existing_plugin: existing_plugin.clone(), incoming_plugin: plugin_id.to_owned() })));
                }
            }
        }
        if let Some(conflict) = io_entries_conflict(&state.io_entries, plugin_id, io_entries) {
            return Err(conflict);
        }
        state.runtimes.entry(plugin_id.to_owned()).or_insert(handle);
        for key in candidate_routes {
            state.routes.entry(key).or_insert_with(|| plugin_id.to_owned());
        }
        for descriptor in io_entries {
            let key: IoEntryKey = (descriptor.from.clone(), descriptor.into.clone());
            state.io_entries.entry(key).or_insert(IoEntryRoute { owner: plugin_id.to_owned(), fidelity: descriptor.fidelity, sniffs: descriptor.sniffs });
        }
        Ok(())
    }

    /// 📊️ `N plugins / M keys` — logged at boot so a dev-boot smoke test can confirm the router
    /// actually picked up more than zero cross-plugin routes.
    pub async fn stats(&self) -> Result<(usize, usize), PluginHostError> {
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        Ok((state.runtimes.len(), state.routes.len()))
    }

    /// 🌉️ Resolves `key` (JSON wire bytes) to whichever OTHER plugin owns it. Refuses to route back
    /// into `calling_plugin_id` itself: the target plugin's `artifact-compose` guest handler is
    /// local-only by construction (see `io::wire_artifact_compose`'s own doc comment) and never calls
    /// `io-compose` again, so every route is exactly one hop — the self-route guard is what keeps a
    /// plugin from ever needing to reason about calling back into its own in-flight `Store` mutex
    /// (which would deadlock, since that mutex is already held by the caller of this very host call).
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): resolution (unchanged) now feeds a
    /// REAL dispatch — `PluginInstanceHandle::compose` driving the owner's own `semio.compose` cold
    /// job through `run_job_on_worker`, the same post-turn relay `run_io`/`identify`
    /// below already use. The guest body is the live `compose-await` packet's, not this file's —
    /// until it registers `"semio.compose"`, this surfaces the OWNER's own `job.unknown-kind` fault
    /// (a real, dynamic failure reflecting actual guest state) rather than the previous hand-written
    /// permanent host refusal. Once `compose-await` lands, this exact code starts succeeding with no
    /// further host change.
    pub async fn compose(&self, calling_plugin_id: &str, key_bytes: &[u8], sources_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let key_text = std::str::from_utf8(key_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let key: semio_framework_os_kernel::io::IoKey = semio_framework_pack_json::from_json_str(key_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let handle = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            let owner = state
                .routes
                .get(&key)
                .cloned()
                .ok_or_else(|| PluginHostError::Plugin(format!("no plugin registered for {}/{}/{} {:?} {}/{}/{}", key.artifact_kind, key.standard, key.subset, key.direction, key.format_kind, key.format_standard, key.format_subset)))?;
            if owner == calling_plugin_id {
                return Err(PluginHostError::Plugin(format!("io-compose refused: plugin `{calling_plugin_id}` would be routing to itself (should have resolved locally)")));
            }
            state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns this key but its instance handle is not registered with the router")))?
        };
        handle.compose(key_bytes, sources_bytes).await
    }

    /// 📚️ Every dialect ANY loaded plugin can move `artifact_kind` through in `direction`
    /// ("import"|"export"), JSON `Vec<ArtifactDialect>` bytes.
    pub async fn dialects(&self, artifact_kind: &str, direction: &str) -> Result<Vec<u8>, PluginHostError> {
        let direction = match direction {
            "import" => semio_framework_os_kernel::io::IoDirection::Import,
            "export" => semio_framework_os_kernel::io::IoDirection::Export,
            other => return Err(PluginHostError::Plugin(format!("unknown io direction `{other}` (expected \"import\" or \"export\")"))),
        };
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let dialects: Vec<semio_framework_artifact_reference::ArtifactDialect> = state
            .routes
            .keys()
            .filter(|key| key.artifact_kind == artifact_kind && key.direction == direction)
            .map(|key| semio_framework_artifact_reference::ArtifactDialect { artifact_kind: key.format_kind.clone(), standard: key.format_standard.clone(), subset: key.format_subset.clone() })
            .collect();
        Ok(semio_framework_pack_json::to_json_string(&dialects).into_bytes())
    }

    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): resolves the deterministic, ≤3-hop,
    /// cycle-free route `from -> into` over the merged `io_entries` graph — the WIT `io-routes`
    /// host import. JSON `io_schema::IoRoute` bytes.
    pub async fn io_routes(&self, from: &str, into: &str) -> Result<Vec<u8>, PluginHostError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let from = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(from).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let into = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(into).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        let route = resolve_io_route(&state.io_entries, &from, &into, 3)?;
        drop(state);
        Ok(semio_framework_pack_json::to_json_string(&route).into_bytes())
    }

    /// 🌉️ Executes the WHOLE resolved `from -> into` route — the WIT `io-run` host import. Resolves
    /// the route and every hop's owning runtime FIRST (holding the router's own lock only for that
    /// lookup, never across a guest call), THEN, before running anything, refuses the ENTIRE route
    /// if ANY hop is owned by `calling_plugin_id` itself: executing that hop would call back into
    /// the calling plugin's own `Store` mutex while it is still held by the in-flight outer call
    /// that reached this host import in the first place — a guaranteed deadlock on a non-reentrant
    /// `std::sync::Mutex`. This generalizes `compose`'s one-hop self-route refusal (above) to a
    /// resolved route of up to 3 hops; the guard is an up-front scan, not a per-hop check, so a
    /// route is either run in full or not run at all — no partial execution on a refusal.
    ///
    /// 🚫️async: R10 residue shape 1 — `to_coordinate` is external/async, hoisted out of
    /// the `ok_or_else` sync closures below.
    pub async fn run_io(&self, calling_plugin_id: &str, from: &str, into: &str, payload: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let from_dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(from).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let into_dialect = semio_framework_artifact_reference::ArtifactDialect::parse_coordinate(into).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
        let hops = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            let route = resolve_io_route(&state.io_entries, &from_dialect, &into_dialect, 3)?;
            if let Some(reentrant_hop) = route_reenters_calling_plugin(&state.io_entries, &route, calling_plugin_id) {
                return Err(PluginHostError::Plugin(format!(
                    "io-run refused: hop {} -> {} is owned by the calling plugin `{calling_plugin_id}` itself — executing it would re-enter that plugin's own in-flight, non-reentrant store lock",
                    reentrant_hop.0.to_coordinate(),
                    reentrant_hop.1.to_coordinate()
                )));
            }
            let mut hops = Vec::with_capacity(route.hops.len());
            for hop in &route.hops {
                let key: IoEntryKey = (hop.from.clone(), hop.into.clone());
                let from_coord = hop.from.to_coordinate();
                let into_coord = hop.into.to_coordinate();
                let owner = state.io_entries.get(&key).map(|entry| entry.owner.clone()).ok_or_else(|| PluginHostError::Plugin(format!("io-run: hop {from_coord} -> {into_coord} vanished from the router between resolve and execute")))?;
                let runtime = state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns hop {from_coord} -> {into_coord} but its runtime is not registered with the router")))?;
                hops.push((from_coord, into_coord, runtime));
            }
            hops
        };
        let mut current = payload;
        for (from_coordinate, into_coordinate, runtime) in hops {
            current = runtime.io_run(&from_coordinate, &into_coordinate, current).await?;
        }
        Ok(current)
    }

    /// 🔍️ Fans `io-sniff` out across every OTHER loaded plugin's carrier-`from` entries — the WIT
    /// `io-identify` host import. Skips the calling plugin's own carrier entries for the SAME
    /// reentrancy reason `run_io` refuses a self-owned hop (a fan-out is best-effort across
    /// multiple plugins, so this SKIPS rather than refuses the whole call). JSON `Vec<(ArtifactDialect,
    /// Confidence)>` bytes, sorted confidence descending then coordinate ascending — same shape and
    /// order `io::io_mechanism::io_identify` produces for the guest-local case.
    ///
    /// 🚫️async: R10 residue shape 1 — `Confidence::rank`/`ArtifactDialect::to_coordinate` are
    /// external async accessors, so the sort key is precomputed before the sync `sort_by`.
    pub async fn identify(&self, calling_plugin_id: &str, payload_bytes: Vec<u8>) -> Result<Vec<u8>, PluginHostError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let payload_text = std::str::from_utf8(&payload_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let payload: semio_framework::io_schema::IoPayload = semio_framework_pack_json::from_json_str(payload_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let carrier = semio_framework_artifact_reference::ArtifactDialect::from(match &payload {
            semio_framework::io_schema::IoPayload::Binary(_) => semio_framework::io_schema::CARRIER_BINARY,
            semio_framework::io_schema::IoPayload::Text(_) => semio_framework::io_schema::CARRIER_TEXT,
        });
        let candidates: Vec<(semio_framework_artifact_reference::ArtifactDialect, String)> = {
            let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
            state.io_entries.iter().filter(|((from, _into), route)| *from == carrier && route.sniffs && route.owner != calling_plugin_id).map(|((_from, into), route)| (into.clone(), route.owner.clone())).collect()
        };
        let mut found: Vec<(semio_framework_artifact_reference::ArtifactDialect, semio_framework::io_schema::Confidence)> = Vec::new();
        for (into, owner) in candidates {
            let runtime = {
                let state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
                state.runtimes.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("plugin `{owner}` owns a carrier io entry but its runtime is not registered with the router")))?
            };
            let carrier_coord = carrier.to_coordinate();
            let into_coord = into.to_coordinate();
            let rank = runtime.io_sniff(&carrier_coord, &into_coord, &payload_bytes).await?;
            let confidence = rank_to_io_confidence(rank);
            if confidence != semio_framework::io_schema::Confidence::None {
                found.push((into, confidence));
            }
        }
        let mut decorated = Vec::with_capacity(found.len());
        for (dialect, confidence) in found {
            let rank = confidence.rank();
            let coord = dialect.to_coordinate();
            decorated.push((rank, coord, dialect, confidence));
        }
        decorated.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        let found: Vec<(semio_framework_artifact_reference::ArtifactDialect, semio_framework::io_schema::Confidence)> = decorated.into_iter().map(|(_, _, dialect, confidence)| (dialect, confidence)).collect();
        Ok(semio_framework_pack_json::to_json_string(&found).into_bytes())
    }

    /// ✂️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): drops
    /// `plugin_id`'s runtime handle and every route it owns — required before a hot-reloaded
    /// plugin can re-register a FRESH `Arc<WasmPluginRuntime>` under the same id (`register_plugin`'s
    /// `PluginRuntimeConflict` check would otherwise reject the new `Arc` as a different pointer for
    /// an already-registered plugin id) and before an unload actually drops the runtime. Also drops
    /// every NEW-mechanism `io_entries` row `plugin_id` owns (W1-D).
    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("io router"))?;
        state.runtimes.remove(plugin_id);
        state.routes.retain(|_, owner| owner != plugin_id);
        state.io_entries.retain(|_, route| route.owner != plugin_id);
        Ok(())
    }
}

impl Default for IoRouter {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🔖️IoRouter

//#region 💡️InferenceRouter
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct GuestArtifactInferenceMetadata {
    pub owner: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub artifact_schema_version: u32,
    pub inference_schema: String,
    pub inference_schema_version: u32,
    pub algorithm_version: u32,
    pub policy_version: u32,
    /// 🎯️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS: `Some(plugin_id)` when
    /// this row is a CONTRIBUTED inference (contract §3/§4); absent/`None`, or equal to `owner`,
    /// for an ordinary owner-authored inference. Additive+defaulted so today's guest wire (which
    /// never sends this field) still decodes unchanged.
    #[serde(default)]
    #[value(default)]
    pub contributor: Option<String>,
    /// 🕸️ Other `inference_schema` ids (same `artifact_kind`) this inference's own computation
    /// consumes the RESULT of — the per-artifact `depends_on` DAG `ArtifactInferenceRouter` toposorts
    /// at registration and injects into `artifact-inference-request.dependencies` at call time.
    #[serde(default)]
    #[value(default)]
    pub depends_on: Vec<String>,
    /// 📜️ The inference's own PUBLISHED payload contract (`semio_framework::
    /// InferencePayloadContract`), carried on the roster so this router accepts the committed
    /// descriptor verbatim. The router itself never reads it — the contract is a CLIENT-facing
    /// declaration the gateway enforces before dispatch — but the roster this router decodes IS
    /// `contributions.inferenceServices`, so a row it cannot decode is a row it cannot route
    /// (`inference route registration: json: 0.unknown field 'payload'`, measured 2026-09-22 21:08
    /// the moment the first contract was committed). Additive+defaulted, so a descriptor from
    /// before contracts existed still decodes unchanged.
    #[serde(default)]
    #[value(default)]
    pub payload: Option<semio_framework::InferencePayloadContract>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceRouteRequest {
    wire_version: u32,
    owner: String,
    artifact_kind: String,
    artifact_schema: String,
    artifact_schema_version: u32,
    inference_schema: String,
    inference_schema_version: u32,
    algorithm_version: u32,
    policy_version: u32,
    revision: u64,
    generation: u64,
    source_dialect: String,
    policy: Vec<u8>,
    budgets: InferenceRouteBudget,
    retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    cancellation_id: String,
    previous_state: Option<Vec<u8>>,
    requested_cache_mode: InferenceRouteCacheMode,
    canonical_payload: Vec<u8>,
    dependencies: Vec<(String, Vec<u8>)>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceRouteBudget {
    allocation_bytes: u64,
    work_units: u64,
    recursion_depth: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
enum InferenceRouteCacheMode {
    Cold,
    Incremental,
    Bypass,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct InferenceRouteResult {
    wire_version: u32,
    owner: String,
    artifact_kind: String,
    artifact_schema: String,
    artifact_schema_version: u32,
    inference_schema: String,
    inference_schema_version: u32,
    algorithm_version: u32,
    policy_version: u32,
    revision: u64,
    generation: u64,
    source_dialect: String,
    policy: Vec<u8>,
    budgets: InferenceRouteBudget,
    retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    cancellation_id: String,
    previous_state: Option<Vec<u8>>,
    requested_cache_mode: InferenceRouteCacheMode,
    canonical_payload: Vec<u8>,
    dependencies: Vec<(String, Vec<u8>)>,
    complete: bool,
    actual_cache_mode: InferenceRouteCacheMode,
}

pub struct ArtifactInferenceRouter {
    routes: Mutex<BTreeMap<(String, String), (String, GuestArtifactInferenceMetadata)>>,
    runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>,
    live_commits: Mutex<BTreeMap<String, (u64, u64)>>,
}

impl ArtifactInferenceRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(BTreeMap::new()), runtimes: Mutex::new(HashMap::new()), live_commits: Mutex::new(BTreeMap::new()) }
    }

    /// 📌️ `dependencies` is the reporting plugin's OWN declared `PluginManifest.dependencies` —
    /// required to gate a CONTRIBUTED row (contract §4 rule 1, same rule `ArtifactMutationRouter`
    /// applies): its `contributor` must equal `plugin_id`, and `owner` must be a direct entry of
    /// `dependencies`. Also toposorts every registered row's `depends_on` DAG (existing rows plus
    /// this registration's own) and rejects a cycle before committing anything. `roster_wire_bytes`
    /// is JSON `Vec<GuestArtifactInferenceMetadata>` — mirrors `ArtifactMutationRouter::register_
    /// plugin`'s already-decoupled idiom (`roster_wire_bytes: &[u8]`, no runtime call inside this
    /// function) rather than `list_artifact_inferences()`, `contributor.list-artifact-inferences`
    /// not being a `world actor` export any more than the old `plugin` interface's own list-* exports
    /// were (`describe.wit`'s doc comment) — the caller decodes `describe()`'s `PackageDescriptor`
    /// and passes the roster in; `handle` is kept only for later `infer()` dispatch.
    pub async fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], handle: Arc<PluginInstanceHandle>, roster_wire_bytes: &[u8]) -> Result<(), PluginHostError> {
        let roster_text = std::str::from_utf8(roster_wire_bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let metadata: Vec<GuestArtifactInferenceMetadata> = semio_framework_pack_json::from_json_str(roster_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        for item in &metadata {
            if let Some(contributor) = &item.contributor {
                if contributor != &item.owner {
                    if contributor != plugin_id {
                        return Err(PluginHostError::Plugin(format!("inference metadata for {}/{} claims contributor `{contributor}` but was reported by plugin `{plugin_id}`", item.artifact_kind, item.inference_schema)));
                    }
                    if !dependencies.iter().any(|dependency| dependency.plugin_id == item.owner) {
                        return Err(PluginHostError::Plugin(format!("plugin `{plugin_id}` contributes an inference on `{}` (owner `{}`) without declaring `{}` as a dependency (contract §4 rule 1)", item.artifact_kind, item.owner, item.owner)));
                    }
                }
            }
        }

        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?;
        let mut candidate = routes.clone();
        for item in &metadata {
            let key = (item.artifact_kind.clone(), item.inference_schema.clone());
            if let Some((existing_plugin, existing)) = candidate.get(&key) {
                if existing_plugin == plugin_id && existing == item {
                    continue;
                }
                return Err(PluginHostError::Plugin(format!("conflicting artifact inference owner for {}/{}: {} {:?}, incoming {} {:?}", key.0, key.1, existing_plugin, existing, plugin_id, item)));
            }
            candidate.insert(key, (plugin_id.to_string(), item.clone()));
        }
        validate_inference_dependency_graph(&candidate)?;
        *routes = candidate;
        drop(routes);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.insert(plugin_id.to_string(), handle);
        Ok(())
    }

    pub async fn metadata(&self) -> Result<Vec<GuestArtifactInferenceMetadata>, PluginHostError> {
        Ok(self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?.values().map(|(_, item)| item.clone()).collect())
    }

    pub async fn infer(&self, request: &[u8], cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        let request_text = std::str::from_utf8(request).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let identity: InferenceRouteRequest = semio_framework_pack_json::from_json_str(request_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        {
            let mut live = self.live_commits.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference live commits"))?;
            if live.insert(identity.cancellation_id.clone(), (identity.revision, identity.generation)).is_some() {
                return Err(PluginHostError::Plugin(format!("inference cancellation identity {:?} is already active", identity.cancellation_id)));
            }
        }
        let result = self.infer_with_visited(request, &mut Vec::new(), cancel).await;
        let live = self
            .live_commits
            .lock()
            .map_err(|_| PluginHostError::LockPoisoned("artifact inference live commits"))?
            .remove(&identity.cancellation_id)
            .ok_or_else(|| PluginHostError::Plugin("inference live commit authority disappeared before admission".into()))?;
        let result = result?;
        let operation = semio_framework_job::Operation::new(semio_framework_job::allocate_operation_id(), semio_framework_job::RevisionId(identity.revision), semio_framework_job::Generation(identity.generation), 0);
        match semio_framework_job::validate_commit(&operation, semio_framework_job::RevisionId(live.0), semio_framework_job::Generation(live.1)) {
            semio_framework_job::CommitValidation::Accepted => Ok(result),
            semio_framework_job::CommitValidation::Stale { .. } => {
                Err(PluginHostError::Plugin(format!("inference commit for {:?} is stale: based on revision/generation {}/{}, live authority is {}/{}", identity.cancellation_id, identity.revision, identity.generation, live.0, live.1)))
            }
        }
    }

    /// 🪪️ Model-actor freshness handoff for an active inference. The final route admission reads this
    /// value only after the guest's terminal candidate returns, then calls
    /// `semio_framework_job::validate_commit` immediately before exposing the result.
    pub async fn set_live_revision_generation(&self, cancellation_id: &str, revision: u64, generation: u64) -> Result<(), PluginHostError> {
        let mut live = self.live_commits.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference live commits"))?;
        let authority = live.get_mut(cancellation_id).ok_or_else(|| PluginHostError::Plugin(format!("no active inference has cancellation identity {cancellation_id:?}")))?;
        *authority = (revision, generation);
        Ok(())
    }

    /// 🕸️ Contract §6: before dispatching to the owner/contributor's own `artifact-infer`, resolves
    /// this route's `depends_on` (same `artifact_kind`, per `GuestArtifactInferenceMetadata`'s own
    /// doc) by recursively computing each dependency's OWN result first, appending the pairs to
    /// `dependencies: Vec<(inference_schema, result_wire_bytes)>` on the outgoing request — exactly
    /// the WIT `artifact-inference-request.dependencies: list<tuple<string, list<u8>>>` shape. The
    /// requester's own entries — a composed document's owned children (design §20.15,
    /// `semio_framework_plugin::inference_child_dependency`) — stay first and reach every dependency
    /// request too ([`routed_inference_dependencies`]).
    /// `visited` is the runtime counterpart of `register_plugin`'s registration-time toposort — a
    /// registered graph can still recurse infinitely if two rows' `depends_on` disagree with what
    /// was toposorted (e.g. a hot-reloaded plugin), so this is real defense-in-depth, not
    /// redundant.
    ///
    /// 🚫️async: R10 residue shape 3 — genuinely self-recursive (this fn really does
    /// await real plugin-runtime I/O via `handle.infer` below, so R9 does not apply);
    /// `Box::pin` breaks the otherwise-infinite future size, per rustc's own E0733 hint.
    async fn infer_with_visited(&self, request: &[u8], visited: &mut Vec<String>, cancel: &semio_framework_async::CancelToken) -> Result<Vec<u8>, PluginHostError> {
        let request_text = std::str::from_utf8(request).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let mut route: InferenceRouteRequest = semio_framework_pack_json::from_json_str(request_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        if visited.contains(&route.inference_schema) {
            return Err(PluginHostError::Plugin(format!("inference dependency cycle at call time: {} -> {}", visited.join(" -> "), route.inference_schema)));
        }
        let (owner, own_metadata) = {
            let routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?;
            routes
                .get(&(route.artifact_kind.clone(), route.inference_schema.clone()))
                .map(|(plugin_id, metadata)| (plugin_id.clone(), metadata.clone()))
                .ok_or_else(|| PluginHostError::Plugin(format!("no guest inference route for {}/{}", route.artifact_kind, route.inference_schema)))?
        };

        if !own_metadata.depends_on.is_empty() {
            visited.push(route.inference_schema.clone());
            let mut dependencies = Vec::with_capacity(own_metadata.depends_on.len());
            for dependency_schema in &own_metadata.depends_on {
                let dependency_metadata = {
                    let routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?;
                    routes
                        .get(&(route.artifact_kind.clone(), dependency_schema.clone()))
                        .map(|(_, metadata)| metadata.clone())
                        .ok_or_else(|| PluginHostError::Plugin(format!("inference `{}` declares depends_on `{dependency_schema}` which is not registered for artifact kind `{}`", route.inference_schema, route.artifact_kind)))?
                };
                let dependency_request = build_dependency_inference_request(&route, &dependency_metadata).await;
                let dependency_request_bytes = semio_framework_pack_json::to_json_string(&dependency_request).into_bytes();
                let dependency_result_bytes = Box::pin(self.infer_with_visited(&dependency_request_bytes, visited, cancel)).await?;
                dependencies.push((dependency_schema.clone(), dependency_result_bytes));
            }
            visited.pop();
            route.dependencies = routed_inference_dependencies(std::mem::take(&mut route.dependencies), dependencies);
        }

        let request = semio_framework_pack_json::to_json_string(&route).into_bytes();
        let handle = self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.get(&owner).cloned().ok_or_else(|| PluginHostError::Plugin(format!("inference owner `{owner}` is not loaded")))?;
        let result = handle.infer(&request, cancel).await?;
        let result_text = std::str::from_utf8(&result).map_err(|error| PluginHostError::Json(error.to_string()))?;
        let echoed: InferenceRouteResult = semio_framework_pack_json::from_json_str(result_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| PluginHostError::Json(error.to_string()))?;
        validate_inference_echo(&route, &echoed).await?;
        Ok(result)
    }

    /// ✂️ Drops every route reported by `plugin_id` — called on unload/hot-reload.
    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference routes"))?.retain(|_, (owner, _)| owner != plugin_id);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("artifact inference runtimes"))?.remove(plugin_id);
        Ok(())
    }
}

/// 🕸️ DFS cycle detector over the `depends_on` DAG among every row of `routes`, scoped by
/// `inference_schema` id (`depends_on` entries name a sibling `inference_schema` within the SAME
/// `artifact_kind`, per `GuestArtifactInferenceMetadata`'s own field doc). Genuinely new logic —
/// distinct from W0-C's plugin-manifest toposort, a different domain (inference schemas within one
/// artifact kind, not plugins).
///
/// 🚫️async: R10 residue shape 3 — self-recursive, and per E1's own principle: pure in-memory
/// BTreeMap/Vec recursion with no suspension point anywhere in the body, so async here would
/// ALSO need `Box::pin` at the recursive call just to compile, for zero behavioural benefit.
/// Reverted to sync rather than boxed — R9.
fn validate_inference_dependency_graph(routes: &BTreeMap<(String, String), (String, GuestArtifactInferenceMetadata)>) -> Result<(), PluginHostError> {
    let mut adjacency: BTreeMap<(&str, &str), Vec<&str>> = BTreeMap::new();
    for ((artifact_kind, inference_schema), (_, item)) in routes {
        adjacency.entry((artifact_kind.as_str(), inference_schema.as_str())).or_default().extend(item.depends_on.iter().map(|dep| dep.as_str()));
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Mark {
        Visiting,
        Done,
    }

    fn visit<'a>(artifact_kind: &'a str, node: &'a str, adjacency: &BTreeMap<(&'a str, &'a str), Vec<&'a str>>, marks: &mut BTreeMap<(&'a str, &'a str), Mark>, stack: &mut Vec<&'a str>) -> Result<(), PluginHostError> {
        match marks.get(&(artifact_kind, node)) {
            Some(Mark::Done) => return Ok(()),
            Some(Mark::Visiting) => return Err(PluginHostError::Plugin(format!("inference dependency cycle on `{artifact_kind}`: {} -> {node}", stack.join(" -> ")))),
            None => {}
        }
        marks.insert((artifact_kind, node), Mark::Visiting);
        stack.push(node);
        if let Some(deps) = adjacency.get(&(artifact_kind, node)) {
            for dep in deps {
                visit(artifact_kind, dep, adjacency, marks, stack)?;
            }
        }
        stack.pop();
        marks.insert((artifact_kind, node), Mark::Done);
        Ok(())
    }

    let mut marks = BTreeMap::new();
    let mut stack = Vec::new();
    for (artifact_kind, node) in adjacency.keys() {
        visit(artifact_kind, node, &adjacency, &mut marks, &mut stack)?;
    }
    Ok(())
}

/// 🎞️ Builds the outgoing request for one `depends_on` entry: identity fields (`owner`/`artifact_schema*`/
/// `artifact_schema*`/`inference_schema*`/`algorithm_version`/`policy_version`) come from the
/// DEPENDENCY's own registered metadata; caller-context fields (`revision`/`generation`/
/// `source_dialect`/`policy`/`budgets`/`cancellation_id`/`requested_cache_mode`/`canonical_payload`)
/// are inherited from the parent request — the same underlying artifact source, viewed through a
/// different inference facet. `previous_state` always starts fresh (`None`): each dependency
/// resolves its own current result, not a cached delta. The requester's own `dependencies` (a composed
/// document's owned children, design §20.15) are caller context too, so they travel unchanged.
async fn build_dependency_inference_request(base: &InferenceRouteRequest, dependency: &GuestArtifactInferenceMetadata) -> InferenceRouteRequest {
    InferenceRouteRequest {
        wire_version: base.wire_version,
        owner: dependency.owner.clone(),
        artifact_kind: base.artifact_kind.clone(),
        artifact_schema: dependency.artifact_schema.clone(),
        artifact_schema_version: dependency.artifact_schema_version,
        inference_schema: dependency.inference_schema.clone(),
        inference_schema_version: dependency.inference_schema_version,
        algorithm_version: dependency.algorithm_version,
        policy_version: dependency.policy_version,
        revision: base.revision,
        generation: base.generation,
        source_dialect: base.source_dialect.clone(),
        policy: base.policy.clone(),
        budgets: base.budgets.clone(),
        retained: base.retained,
        cancellation_id: base.cancellation_id.clone(),
        previous_state: None,
        requested_cache_mode: base.requested_cache_mode.clone(),
        canonical_payload: base.canonical_payload.clone(),
        dependencies: base.dependencies.clone(),
    }
}

/// 🪆️ The dependencies one routed request carries: the requester's own entries (a composed document's owned children, design
/// §20.15), first and unchanged, then the `depends_on` results the router resolved for it.
fn routed_inference_dependencies(requested: Vec<(String, Vec<u8>)>, resolved: Vec<(String, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
    requested.into_iter().chain(resolved).collect()
}

async fn validate_inference_echo(request: &InferenceRouteRequest, result: &InferenceRouteResult) -> Result<(), PluginHostError> {
    if result.wire_version != request.wire_version
        || result.owner != request.owner
        || result.artifact_kind != request.artifact_kind
        || result.artifact_schema != request.artifact_schema
        || result.artifact_schema_version != request.artifact_schema_version
        || result.artifact_schema != request.artifact_schema
        || result.artifact_schema_version != request.artifact_schema_version
        || result.inference_schema != request.inference_schema
        || result.inference_schema_version != request.inference_schema_version
        || result.algorithm_version != request.algorithm_version
        || result.policy_version != request.policy_version
        || result.revision != request.revision
        || result.generation != request.generation
        || result.source_dialect != request.source_dialect
        || result.policy != request.policy
        || result.budgets != request.budgets
        || result.retained != request.retained
        || result.cancellation_id != request.cancellation_id
        || result.previous_state != request.previous_state
        || result.requested_cache_mode != request.requested_cache_mode
        || result.dependencies != request.dependencies
        || result.actual_cache_mode != request.requested_cache_mode
    {
        return Err(PluginHostError::Plugin("guest inference result did not exactly echo its request metadata".into()));
    }
    Ok(())
}

impl Default for ArtifactInferenceRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️artifact-inference-router/🦀️.rs"]
mod artifact_inference_router_tests;
//#endregion 💡️InferenceRouter

//#region 🔖️PluginGraph
/// 🕸️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A host wave): the
/// registered set of every loaded plugin's/extension's `PluginManifest`, validated against
/// contract freeze §4 rule 5 by calling straight through to W0-C's PURE `semio_framework::
/// {validate_dependency_graph, resolve_load_order, dependents}` — this type owns none of that
/// graph logic itself, only the registration/lifecycle bookkeeping `runtime_for`
/// (`🏃️run/🦀️.rs`) needs to load a dependency before its dependent and to gate
/// hot-reload/unload against a plugin's live dependents (scout-2 §3/§5: neither existed before
/// this ticket).
pub struct PluginGraph {
    state: Mutex<BTreeMap<String, PluginManifest>>,
}

#[derive(Debug)]
pub enum PluginGraphError {
    Graph(semio_framework::DependencyGraphError),
    UnloadBlocked { plugin_id: String, dependents: Vec<String> },
    Unknown { plugin_id: String },
    LockPoisoned,
}

impl std::fmt::Display for PluginGraphError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Graph(error) => std::fmt::Display::fmt(error, formatter),
            Self::UnloadBlocked { plugin_id, dependents } => write!(formatter, "plugin `{plugin_id}` cannot be unloaded: still depended on by {dependents:?}"),
            Self::Unknown { plugin_id } => write!(formatter, "plugin `{plugin_id}` is not registered"),
            Self::LockPoisoned => formatter.write_str("plugin graph lock poisoned"),
        }
    }
}

impl std::error::Error for PluginGraphError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Graph(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework::DependencyGraphError> for PluginGraphError {
    fn from(error: semio_framework::DependencyGraphError) -> Self {
        Self::Graph(error)
    }
}

impl PluginGraph {
    pub fn new() -> Self {
        Self { state: Mutex::new(BTreeMap::new()) }
    }

    pub(super) fn lock(&self) -> Result<std::sync::MutexGuard<'_, BTreeMap<String, PluginManifest>>, PluginGraphError> {
        self.state.lock().map_err(|_| PluginGraphError::LockPoisoned)
    }

    /// 🔗️ Why a contributor may not act on `owner`'s artifact right now, if anything: `None` means
    /// the dependency is declared, the owner is loaded, and its version satisfies the requirement.
    /// Load-time registration already rejects all three, but a transaction can still meet them at
    /// dispatch time — an owner unloaded after registration, or a contributor loaded against a
    /// different build — so the transaction path asks again rather than assuming.
    pub async fn contribution_block(&self, contributor: &str, owner: &str) -> Result<Option<(&'static str, String)>, PluginGraphError> {
        let state = self.lock()?;
        let Some(contributor_manifest) = state.get(contributor) else {
            return Ok(Some(("transaction.dependency-missing", format!("contributor `{contributor}` is not loaded"))));
        };
        let Some(dependency) = contributor_manifest.dependencies.iter().find(|dependency| dependency.plugin_id == owner) else {
            return Ok(Some(("transaction.contribution-not-permitted", format!("`{contributor}` does not declare `{owner}` as a dependency"))));
        };
        let Some(owner_manifest) = state.get(owner) else {
            return Ok(Some(("transaction.dependency-missing", format!("`{contributor}` depends on `{owner}`, which is not loaded"))));
        };
        match semio_framework::Version::parse(&owner_manifest.version) {
            Ok(version) if dependency.version.matches(&version) => Ok(None),
            Ok(version) => Ok(Some(("transaction.version-mismatch", format!("`{contributor}` requires `{owner}` {}, but the loaded build is {version}", dependency.version)))),
            Err(error) => Ok(Some(("transaction.version-mismatch", format!("`{owner}`'s version {:?} is unparseable: {error}", owner_manifest.version)))),
        }
    }

    /// ✅️ Registers (or replaces) `manifest`'s entry, re-validating the WHOLE resulting graph
    /// before committing — an invalid addition (missing dependency, version mismatch, cycle)
    /// leaves the previously-registered set untouched.
    ///
    /// 🪪️ `resolve_load_order` (not `validate_dependency_graph` alone) is what actually detects a
    /// CYCLE — per W0-C's own report: "a real cycle among present plugins passes validation and
    /// is caught by the toposort leftover-set walk". `validate_dependency_graph` alone only
    /// catches missing-dependency/version-mismatch.
    pub async fn register(&self, manifest: PluginManifest) -> Result<(), PluginGraphError> {
        let mut state = self.lock()?;
        let mut candidate = state.clone();
        candidate.insert(manifest.plugin_id.clone(), manifest);
        let list: Vec<PluginManifest> = candidate.values().cloned().collect();
        semio_framework::resolve_load_order(&list)?;
        *state = candidate;
        Ok(())
    }

    /// 🩹️ Contract §4.5 hot-reload half: re-validates the graph as it would look with `plugin_id`
    /// replaced by `new_manifest` — in particular every OTHER plugin's dependency ON `plugin_id`
    /// is re-checked against `new_manifest.version`, so a reload that would break a live
    /// dependent's contribution is rejected before the swap. Does not mutate on failure.
    pub async fn prepare_hot_reload(&self, new_manifest: &PluginManifest) -> Result<(), PluginGraphError> {
        let state = self.lock()?;
        let mut candidate = state.clone();
        candidate.insert(new_manifest.plugin_id.clone(), new_manifest.clone());
        let list: Vec<PluginManifest> = candidate.values().cloned().collect();
        semio_framework::resolve_load_order(&list)?;
        Ok(())
    }

    /// ✅️ Commits `new_manifest` after `prepare_hot_reload` has already validated it.
    pub async fn commit_hot_reload(&self, new_manifest: PluginManifest) -> Result<(), PluginGraphError> {
        self.register(new_manifest).await
    }

    /// 🔒️ Contract §4.5 unload half: refuses while any OTHER registered plugin still depends on
    /// `plugin_id` (typed `UnloadBlocked`, names every blocking dependent).
    pub async fn guard_unload(&self, plugin_id: &str) -> Result<(), PluginGraphError> {
        let blockers = self.dependents(plugin_id).await?;
        if !blockers.is_empty() {
            return Err(PluginGraphError::UnloadBlocked { plugin_id: plugin_id.to_string(), dependents: blockers });
        }
        Ok(())
    }

    /// ✂️ Removes `plugin_id`'s registration — callers MUST call `guard_unload` first.
    pub async fn unregister(&self, plugin_id: &str) -> Result<(), PluginGraphError> {
        let mut state = self.lock()?;
        state.remove(plugin_id).ok_or_else(|| PluginGraphError::Unknown { plugin_id: plugin_id.to_string() })?;
        Ok(())
    }

    /// 🔢️ Deterministic dependency-respecting load order over every currently registered plugin.
    pub async fn load_order(&self) -> Result<Vec<String>, PluginGraphError> {
        let state = self.lock()?;
        let list: Vec<PluginManifest> = state.values().cloned().collect();
        Ok(semio_framework::resolve_load_order(&list)?)
    }

    /// 👥️ Direct dependents of `plugin_id`, sorted.
    pub async fn dependents(&self, plugin_id: &str) -> Result<Vec<String>, PluginGraphError> {
        let state = self.lock()?;
        let list: Vec<PluginManifest> = state.values().cloned().collect();
        Ok(semio_framework::dependents(&list, plugin_id))
    }

    pub async fn manifest(&self, plugin_id: &str) -> Result<Option<PluginManifest>, PluginGraphError> {
        Ok(self.lock()?.get(plugin_id).cloned())
    }

    pub async fn is_registered(&self, plugin_id: &str) -> Result<bool, PluginGraphError> {
        Ok(self.lock()?.contains_key(plugin_id))
    }
}

impl Default for PluginGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️plugin-graph/🦀️.rs"]
mod plugin_graph_tests;
//#endregion 🔖️PluginGraph

//#region 🎯️MutationRouter
/// 🗂️ Host mirror of the guest `WireMutationRosterEntry` (contract §6's `contributor.
/// list-artifact-mutations` roster row) — `semio-framework-plugin-host` does not depend on the
/// guest SDK crate (same reasoning `ExtensionManifest` below already documents), so this is a
/// field-for-field shape-identical local copy, decoded off the same `store::pack_rt::
/// encode_wire_value`/`semio_framework_value::ToValue::to_value` wire `list_artifact_mutations()` returns, `inputs` included.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct HostMutationRosterEntry {
    pub mutation_id: String,
    pub verb: String,
    pub entity: String,
    pub kind: String,
    pub record: String,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub contributor: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub artifact_kind: Option<String>,
    pub inputs: HostMutationInputs,
}

/// 🎛️ Host mirror of the guest `WireMutationInputs`: the input descriptors a roster row's leaf payload schema declares
/// (`semio_framework::mutation_input_defs`), the reason it declares none a user could edit, or `opaque` when the row
/// publishes no payload schema (a contributed plan edits its target's own operations).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum HostMutationInputs {
    Declared { inputs: Vec<semio_framework::ActionArgDef> },
    Refused { error: semio_framework::InputSchemaError },
    Opaque,
}

/// 🎞️ Host mirror of `WireArtifactMutationPlanRequest`/`Result` (contract §5.3's `contributor.
/// artifact-mutation-plan` call) — same "cannot depend on the guest SDK crate" reasoning.
#[derive(Clone, Debug, PartialEq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostArtifactMutationPlanRequest {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub snapshot_pack: Vec<u8>,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct HostArtifactMutationPlanResult {
    pub artifact_kind: String,
    pub mutation_id: String,
    pub revision: u64,
    pub generation: u64,
    pub owner_ops: Vec<Vec<u8>>,
    pub label: String,
    pub foreign: Vec<protocol::ForeignStep>,
}

/// 🪶️ `store::pack_rt::decode_wire_value` + `semio_framework_value::FromValue::from_value` in one step — the exact decode
/// idiom `WasmPluginRuntime::read_manifest` already uses, mirrored here for the two new
/// `contributor` wire calls (contract §6): the guest's own `encode_wire_serialized` (`🔌️plugin/
/// 🦀️.rs`) is `store::pack_rt::encode_wire_value(&to_dsl_value(value))`, NOT plain JSON.
async fn decode_wire_dsl<T: semio_framework_value::FromValue>(bytes: &[u8]) -> Result<T, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    let value = store::pack_rt::renormalize_whole_number_floats(value);
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

async fn encode_wire_dsl<T: semio_framework_value::ToValue>(value: &T) -> Result<Vec<u8>, PluginHostError> {
    let dsl_value = semio_framework_value::ToValue::to_value(value);
    Ok(store::pack_rt::encode_wire_value(&dsl_value))
}

/// 🎯️ Contract §5.3: who answers a `(artifact_kind, mutation_id)` mutation — the artifact kind's
/// own owning plugin, or a contributor plugin that declared it as a direct dependency.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationOwnership {
    Owner { plugin_id: String },
    Contributed { plugin_id: String },
}

/// 🎯️ Merges every loaded plugin's `list-artifact-mutations` roster into one
/// `(artifact_kind, mutation_id) -> Owner | Contributed{plugin}` table (contract §4 rules 1/3).
/// Conflict rule mirrors `ArtifactInferenceRouter::register_plugin` EXACTLY: two sources claiming
/// the same key is an error unless byte-identical.
pub struct ArtifactMutationRouter {
    routes: Mutex<BTreeMap<(String, String), (String, HostMutationRosterEntry)>>,
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): mirrors `ArtifactInferenceRouter`'s
    /// own `runtimes` field EXACTLY (same shape, same purpose) — kept by `register_plugin`/
    /// `unregister_plugin` for `plan`'s dispatch. Production today calls `register_roster` directly
    /// (see `🏃️run/🦀️.rs`, outside this packet's owned paths), which never touches this
    /// field — see this packet's report `## lease-requests` for the one-line change that wires it.
    runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>,
}

impl ArtifactMutationRouter {
    pub fn new() -> Self {
        Self { routes: Mutex::new(BTreeMap::new()), runtimes: Mutex::new(HashMap::new()) }
    }

    /// 📌️ Decodes `roster_wire_bytes` (the exact `contributor.list-artifact-mutations` wire
    /// payload), registers every row (see `register_roster` for the gating rules), and keeps
    /// `handle` for `plan`'s later dispatch — mirrors `ArtifactInferenceRouter::register_plugin`'s
    /// exact 4-argument shape (`plugin_id, dependencies, handle, roster_wire_bytes`).
    pub async fn register_plugin(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], handle: Arc<PluginInstanceHandle>, roster_wire_bytes: &[u8]) -> Result<(), PluginHostError> {
        let roster: Vec<HostMutationRosterEntry> = decode_wire_dsl(roster_wire_bytes).await?;
        self.register_roster_with_runtime(plugin_id, dependencies, handle, roster).await
    }

    /// 🔌️ The PRODUCTION registration entry point: typed roster rows plus the instance handle that
    /// `plan()` will dispatch through. `🏃️run` builds `HostMutationRosterEntry` values directly off
    /// the descriptor and never holds the wire bytes `register_plugin` decodes, so routing it
    /// through that method would mean encoding typed rows to JSON purely to decode them again.
    ///
    /// 🧪️ Kept SEPARATE from the pure `register_roster` below rather than folded into it: that
    /// function is deliberately wasm-free so 15 routing tests can exercise the contract §4 gating
    /// rules without constructing a `PluginInstanceHandle`, and requiring a handle would destroy
    /// that. Registering the runtime is therefore a wrapper around the pure core, never a second
    /// call a caller could forget — which is exactly how `runtimes` sat empty in production while
    /// every routing test passed.
    pub async fn register_roster_with_runtime(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], handle: Arc<PluginInstanceHandle>, roster: Vec<HostMutationRosterEntry>) -> Result<(), PluginHostError> {
        self.register_roster(plugin_id, dependencies, roster).await?;
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router runtimes"))?.insert(plugin_id.to_string(), handle);
        Ok(())
    }

    /// 🧪️ Pure half of `register_plugin`, split out for deterministic testing without any wasm —
    /// mirrors `crate::app::register_contributions`'s own split in the guest SDK (W1-A). A
    /// CONTRIBUTED row (`contributor`/`artifact_kind` both `Some`) is gated against contract §4
    /// rule 1: `entry.contributor` must equal the reporting `plugin_id`, and `artifact_kind`'s
    /// owning plugin must be a DIRECT entry of `dependencies`. An OWNER row (both `None`) is keyed
    /// by the reporting plugin's own id and gated only by the ordinary conflict rule.
    pub async fn register_roster(&self, plugin_id: &str, dependencies: &[semio_framework::PluginDependency], roster: Vec<HostMutationRosterEntry>) -> Result<(), PluginHostError> {
        let mut candidate_inserts = Vec::new();
        for entry in roster {
            let owner_of_kind = match (&entry.contributor, &entry.artifact_kind) {
                (Some(contributor), Some(artifact_kind)) => {
                    if contributor != plugin_id {
                        return Err(PluginHostError::Plugin(format!("mutation roster row {:?} claims contributor `{contributor}` but was reported by plugin `{plugin_id}`", entry.mutation_id)));
                    }
                    let owner_plugin = semio_framework_artifact_reference::ArtifactKindId::parse(artifact_kind).map_err(|error| PluginHostError::Plugin(error.to_string()))?.plugin().to_string();
                    if !dependencies.iter().any(|dependency| dependency.plugin_id == owner_plugin) {
                        return Err(PluginHostError::Plugin(format!("plugin `{plugin_id}` contributes a mutation on `{artifact_kind}` (owner `{owner_plugin}`) without declaring `{owner_plugin}` as a dependency (contract §4 rule 1)")));
                    }
                    artifact_kind.clone()
                }
                (None, None) => plugin_id.to_string(),
                _ => return Err(PluginHostError::Plugin(format!("mutation roster row {:?} has exactly one of contributor/artifact_kind set — both or neither are required", entry.mutation_id))),
            };
            candidate_inserts.push(((owner_of_kind, entry.mutation_id.clone()), entry));
        }

        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        for (key, entry) in &candidate_inserts {
            if let Some((existing_plugin, existing_entry)) = routes.get(key) {
                let reporting_plugin = entry.contributor.as_deref().unwrap_or(plugin_id);
                if existing_plugin != reporting_plugin || existing_entry != entry {
                    return Err(PluginHostError::Plugin(format!("conflicting artifact mutation owner for {}/{}: {} {:?}, incoming {} {:?}", key.0, key.1, existing_plugin, existing_entry, reporting_plugin, entry)));
                }
            }
        }
        for (key, entry) in candidate_inserts {
            let reporting_plugin = entry.contributor.clone().unwrap_or_else(|| plugin_id.to_string());
            routes.insert(key, (reporting_plugin, entry));
        }
        Ok(())
    }

    /// 🔎️ Contract §5.3 resolution: who answers `(artifact_kind, mutation_id)`.
    /// 🔎️ Two-tier lookup: a CONTRIBUTED row is keyed by the exact `(target_artifact_kind,
    /// mutation_id)` (the wire row carries `artifact_kind`), but an OWNER row's wire shape carries
    /// no `artifact_kind` at all — it is keyed by `(reporting_plugin_id, mutation_id)` at
    /// registration (see `register_roster`). So an owner lookup first tries the exact
    /// `(artifact_kind, mutation_id)` key (a contributed row on this exact kind), then falls back
    /// to `(ArtifactKindId::parse(artifact_kind).plugin(), mutation_id)` — the artifact kind's own
    /// owning plugin's roster.
    pub async fn resolve(&self, artifact_kind: &str, mutation_id: &str) -> Result<MutationOwnership, PluginHostError> {
        let routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        if let Some((plugin_id, entry)) = routes.get(&(artifact_kind.to_string(), mutation_id.to_string())) {
            return Ok(match &entry.contributor {
                Some(_) => MutationOwnership::Contributed { plugin_id: plugin_id.clone() },
                None => MutationOwnership::Owner { plugin_id: plugin_id.clone() },
            });
        }
        if let Ok(parsed) = semio_framework_artifact_reference::ArtifactKindId::parse(artifact_kind) {
            if let Some((plugin_id, _entry)) = routes.get(&(parsed.plugin().to_string(), mutation_id.to_string())) {
                return Ok(MutationOwnership::Owner { plugin_id: plugin_id.clone() });
            }
        }
        Err(PluginHostError::Plugin(format!("no mutation route for {artifact_kind}/{mutation_id}")))
    }

    /// 📚️ Every registered roster row — surfaced for the dev-boot smoke line and for tests
    /// ("assert the contributed roster is visible").
    pub async fn roster(&self) -> Result<Vec<HostMutationRosterEntry>, PluginHostError> {
        Ok(self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?.values().map(|(_, entry)| entry.clone()).collect())
    }

    /// 🎯️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): contract §5.3 dispatch — resolves
    /// `(artifact_kind, mutation_id)` (reusing `resolve` above, never duplicating that lookup) then
    /// drives the owning/contributing plugin's own `semio.mutation-plan` cold job to completion.
    /// Mirrors `ArtifactInferenceRouter::infer`'s exact shape: raw wire bytes in, raw wire bytes out
    /// (`request_bytes`/the return are both the DSL wire form `HostArtifactMutationPlanRequest`/
    /// `Result` already use elsewhere in this region — `PluginInstanceHandle::mutation_plan` passes
    /// them straight through, no re-encoding).
    pub async fn plan(&self, request_bytes: &[u8]) -> Result<Vec<u8>, PluginHostError> {
        let request: HostArtifactMutationPlanRequest = decode_wire_dsl(request_bytes).await?;
        let plugin_id = match self.resolve(&request.artifact_kind, &request.mutation_id).await? {
            MutationOwnership::Contributed { plugin_id } | MutationOwnership::Owner { plugin_id } => plugin_id,
        };
        let handle = self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router runtimes"))?.get(&plugin_id).cloned().ok_or_else(|| PluginHostError::Plugin(format!("mutation plan owner `{plugin_id}` is not loaded")))?;
        handle.mutation_plan(request_bytes).await
    }

    /// ✂️ Drops every route reported by `plugin_id` (owner rows keyed under its own id, contributed
    /// rows it reported under an owner artifact kind) and its runtime handle — called on
    /// unload/hot-reload.
    pub async fn unregister_plugin(&self, plugin_id: &str) -> Result<(), PluginHostError> {
        let mut routes = self.routes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router"))?;
        routes.retain(|_, (owner_plugin, entry)| {
            let reporter = entry.contributor.as_deref().unwrap_or(owner_plugin.as_str());
            reporter != plugin_id
        });
        drop(routes);
        self.runtimes.lock().map_err(|_| PluginHostError::LockPoisoned("mutation router runtimes"))?.remove(plugin_id);
        Ok(())
    }
}

impl Default for ArtifactMutationRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️artifact-mutation-router/🦀️.rs"]
mod artifact_mutation_router_tests;
//#endregion 🎯️MutationRouter

//#region 🔖️InstanceDirectory
/// 🗺️ `ArtifactRef ↔ (plugin_id, instance_id, artifact_kind)` — genuinely new (scout-2 §3: "there
/// is no instance directory" before this ticket). Populated at `instantiate-app` and at every
/// document load (see `WasmPluginRuntime::create_app` and `HostState::pre_adopt_command_packs`'s
/// hooks below) and consulted by `HostTransactionCoordinator` to resolve a `ForeignStep.target`
/// into a live instance. Keyed by plain artifact-id strings (not the full `semio_framework_artifact_reference::ArtifactRef`, which
/// requires an `ArtifactDialect` not always resolvable at bind time) — callers that have a real
/// `semio_framework_artifact_reference::ArtifactRef` pass `.artifact_id` through.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceLocation {
    pub plugin_id: String,
    pub instance_id: u32,
    pub artifact_kind: String,
}

pub struct InstanceDirectory {
    state: Mutex<InstanceDirectoryState>,
}

#[derive(Default)]
struct InstanceDirectoryState {
    by_artifact_id: HashMap<String, InstanceLocation>,
    by_instance: HashMap<(String, u32), Vec<String>>,
}

impl InstanceDirectory {
    pub fn new() -> Self {
        Self { state: Mutex::new(InstanceDirectoryState::default()) }
    }

    /// 📌️ Binds `artifact_id` to `(plugin_id, instance_id, artifact_kind)`, replacing any prior
    /// binding for the SAME `artifact_id` — a re-bind after a document load on the same instance
    /// is expected to be idempotent, not an error (unlike the mutation/inference routers' conflict
    /// rule, this is a live pointer table, not a registration ledger).
    pub async fn bind(&self, artifact_id: &str, plugin_id: &str, instance_id: u32, artifact_kind: &str) -> Result<(), PluginHostError> {
        let mut state = self.state.lock().map_err(|_| PluginHostError::LockPoisoned("instance directory"))?;
        if let Some(previous) = state.by_artifact_id.get(artifact_id) {
            if previous.plugin_id != plugin_id || previous.instance_id != instance_id {
                let previous_key = (previous.plugin_id.clone(), previous.instance_id);
                if let Some(previous_refs) = state.by_instance.get_mut(&previous_key) {
                    previous_refs.retain(|existing| existing != artifact_id);
                }
            }
        }
        state.by_artifact_id.insert(artifact_id.to_string(), InstanceLocation { plugin_id: plugin_id.to_string(), instance_id, artifact_kind: artifact_kind.to_string() });
        let refs = state.by_instance.entry((plugin_id.to_string(), instance_id)).or_default();
        if !refs.iter().any(|existing| existing == artifact_id) {
            refs.push(artifact_id.to_string());
        }
        Ok(())
    }

    pub async fn resolve(&self, artifact_id: &str) -> Option<InstanceLocation> {
        self.state.lock().ok()?.by_artifact_id.get(artifact_id).cloned()
    }

    /// ✂️ Drops every binding for `(plugin_id, instance_id)` — called on `destroy_app`.
    pub async fn unbind_instance(&self, plugin_id: &str, instance_id: u32) {
        let Ok(mut state) = self.state.lock() else { return };
        if let Some(refs) = state.by_instance.remove(&(plugin_id.to_string(), instance_id)) {
            for artifact_id in refs {
                state.by_artifact_id.remove(&artifact_id);
            }
        }
    }

    pub async fn artifact_ids_for_instance(&self, plugin_id: &str, instance_id: u32) -> Vec<String> {
        self.state.lock().ok().and_then(|state| state.by_instance.get(&(plugin_id.to_string(), instance_id)).cloned()).unwrap_or_default()
    }
}

impl Default for InstanceDirectory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️instance-directory/🦀️.rs"]
mod instance_directory_tests;
//#endregion 🔖️InstanceDirectory

//#region 🎯️TransactionCoordinator
/// 👥️ One transaction participant: which plugin, which live instance.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionMember {
    pub plugin_id: String,
    pub instance_id: u32,
}

/// ✅️ A completed (committed) transaction — `members` is discovery order (member 0 = initiator);
/// `edit_ids[i]` is `members[i]`'s own edit id, from the actual (reverse-order) commit phase.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionOutcome {
    pub txn_id: String,
    pub members: Vec<TransactionMember>,
    pub edit_ids: Vec<String>,
}

/// 🧯 Contract freeze §5's frozen rejection taxonomy, typed.
#[derive(Debug)]
pub enum TransactionError {
    Rejected { code: String, message: String },
    Host(PluginHostError),
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Rejected { code, message } => write!(formatter, "{code}: {message}"),
            Self::Host(error) => write!(formatter, "plugin host error: {error}"),
        }
    }
}

impl std::error::Error for TransactionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Host(error) => Some(error),
            Self::Rejected { .. } => None,
        }
    }
}

impl From<PluginHostError> for TransactionError {
    fn from(error: PluginHostError) -> Self {
        Self::Host(error)
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️owned-error/🦀️.rs"]
mod owned_error_tests;

impl TransactionError {
    pub async fn rejected(code: &str, message: impl Into<String>) -> Self {
        Self::Rejected { code: code.to_string(), message: message.into() }
    }

    pub async fn code(&self) -> &str {
        match self {
            TransactionError::Rejected { code, .. } => code,
            TransactionError::Host(_) => "transaction.commit-failed",
        }
    }
}

async fn payload_hash_of(bytes: &[u8]) -> protocol::PayloadHash {
    protocol::PayloadHash(*semio_framework_hash::hash(bytes).as_bytes())
}

/// 🧮️ One member's accumulated prepare state, built up while the resolution loop walks the
/// foreign-step graph — never sent to the guest until every step targeting this member AT THE
/// CURRENT DEPTH has been folded in, so a member is `TransactionPrepare`d AT MOST ONCE per
/// transaction (contract §5.9: a second concurrent prepare on one instance is a hard
/// `instance-busy` reject at the guest, so the host must never issue two). Always sent in the
/// PRE-PLANNED wire form (never the owner-mutation form), per W1-B's own recommendation
/// (`📓️w1-b-report.md` §5): the owner-mutation form carries no `origin` on the wire, so preferring
/// pre-planned form for every foreign target closes that provenance gap and lets N accumulated ops
/// ride in one `prepared_ops` list regardless of how many `ForeignStep`s targeted this member.
struct MemberDraft {
    prepared_ops: Vec<Vec<u8>>,
    origin: protocol::MutationOrigin,
}

pub struct HostTransactionCoordinator {
    seq: std::sync::atomic::AtomicU64,
}

impl HostTransactionCoordinator {
    pub fn new() -> Self {
        Self { seq: std::sync::atomic::AtomicU64::new(1) }
    }

    async fn next_seq(&self) -> u64 {
        self.seq.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    }

    async fn mint_txn_id(&self, initiator: &TransactionMember) -> String {
        format!("txn-{}-{}-{}", initiator.plugin_id, initiator.instance_id, self.next_seq().await)
    }

    /// 🎯️ Contract §5 steps 1-6: resolves every `foreign` step to a live instance via
    /// `instances`/`mutation_router`, recurses with depth/cycle guards (§5.4), issues ONE
    /// `TransactionPrepare` per member (always pre-planned form — see `MemberDraft`'s doc),
    /// requires every member's `TransactionPrepared.rejection` to be empty (§5.5, else rolls back
    /// every already-prepared member), then commits in REVERSE discovery order (§5.6,
    /// compensating already-committed members via `TransactionUndo` on a commit failure).
    ///
    /// `exchange`/`plan_contributed` are closures so this can be driven over real
    /// `WasmPluginRuntime`s (production, see `🏃️run/🦀️.rs`) or an in-process fake
    /// (tests) without either depending on the other.
    ///
    /// 🚫️async: R10 residue shape 1 — `Option::and_then` takes a sync closure, so the
    /// `resolve(...).await` that used to live inside it is hoisted out here instead.
    ///
    /// 🚫️async: R10 residue shape 1 — `TransactionError::rejected` is async, hoisted
    /// out of `ok_or_else`/`map_err`'s sync closures via explicit matches.
    ///
    /// Phase 1 (§5.5): prepare every member exactly once, in discovery order; any rejection
    /// rolls back every member already prepared before it.
    ///
    /// Phase 2 (§5.6): commit in REVERSE discovery order.
    #[allow(clippy::too_many_arguments)]
    pub async fn run_transaction(
        &self,
        instances: &InstanceDirectory,
        mutation_router: &ArtifactMutationRouter,
        mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>,
        mut plan_contributed: impl FnMut(&str, &str, &str, &TransactionMember, &[u8]) -> Result<HostArtifactMutationPlanResult, TransactionError>,
        initiator: TransactionMember,
        local_ops: Vec<Vec<u8>>,
        foreign: Vec<protocol::ForeignStep>,
    ) -> Result<TransactionOutcome, TransactionError> {
        let txn_id = self.mint_txn_id(&initiator).await;

        let initiator_artifact_id = instances.artifact_ids_for_instance(&initiator.plugin_id, initiator.instance_id).await.into_iter().next();
        let initiator_target = match initiator_artifact_id {
            Some(artifact_id) => match instances.resolve(&artifact_id).await {
                Some(location) => protocol::ForeignTarget { artifact_id, artifact_kind: location.artifact_kind, dialect: None },
                None => protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None },
            },
            None => protocol::ForeignTarget { artifact_id: String::new(), artifact_kind: String::new(), dialect: None },
        };

        let mut discovery_order: Vec<TransactionMember> = vec![initiator.clone()];
        let mut drafts: BTreeMap<(String, u32), MemberDraft> = BTreeMap::new();
        drafts.insert((initiator.plugin_id.clone(), initiator.instance_id), MemberDraft { prepared_ops: local_ops, origin: protocol::MutationOrigin::Owner });

        let mut visited: std::collections::HashSet<(String, String, [u8; 32])> = std::collections::HashSet::new();
        let mut frontier: Vec<protocol::ForeignStep> = foreign;
        let mut depth: u8 = 0;

        while !frontier.is_empty() {
            depth += 1;
            if depth > protocol::MAX_PLAN_DEPTH {
                return Err(TransactionError::rejected("transaction.depth-exceeded", format!("transaction `{txn_id}` exceeded MAX_PLAN_DEPTH ({})", protocol::MAX_PLAN_DEPTH)).await);
            }
            let mut next_frontier: Vec<protocol::ForeignStep> = Vec::new();
            for step in frontier {
                let cycle_key = (step.target.artifact_id.clone(), step.mutation_id.0.clone(), *semio_framework_hash::hash(&step.payload).as_bytes());
                if !visited.insert(cycle_key) {
                    return Err(TransactionError::rejected("transaction.cycle", format!("transaction `{txn_id}` revisited {}/{}", step.target.artifact_id, step.mutation_id.0)).await);
                }

                let location = match instances.resolve(&step.target.artifact_id).await {
                    Some(location) => location,
                    None => return Err(TransactionError::rejected("transaction.unknown-target", format!("no live instance bound to artifact id `{}`", step.target.artifact_id)).await),
                };
                let ownership = match mutation_router.resolve(&location.artifact_kind, &step.mutation_id.0).await {
                    Ok(ownership) => ownership,
                    Err(error) => return Err(TransactionError::rejected("transaction.unknown-mutation", error.to_string()).await),
                };

                let member = TransactionMember { plugin_id: location.plugin_id.clone(), instance_id: location.instance_id };
                let key = (member.plugin_id.clone(), member.instance_id);
                if !drafts.contains_key(&key) {
                    discovery_order.push(member.clone());
                }

                match ownership {
                    MutationOwnership::Owner { .. } => {
                        let draft = drafts.entry(key).or_insert_with(|| MemberDraft { prepared_ops: Vec::new(), origin: protocol::MutationOrigin::Transaction { initiator: initiator_target.clone() } });
                        draft.prepared_ops.push(step.payload.clone());
                    }
                    MutationOwnership::Contributed { plugin_id: contributor } => {
                        let plan = plan_contributed(&contributor, &location.artifact_kind, &step.mutation_id.0, &member, &step.payload)?;
                        let origin = protocol::MutationOrigin::Contributed { plugin_id: contributor.clone(), mutation_id: step.mutation_id.clone(), payload_hash: payload_hash_of(&step.payload).await };
                        let draft = drafts.entry(key).or_insert_with(|| MemberDraft { prepared_ops: Vec::new(), origin: origin.clone() });
                        draft.prepared_ops.extend(plan.owner_ops);
                        draft.origin = origin;
                        next_frontier.extend(plan.foreign);
                    }
                }
            }
            frontier = next_frontier;
        }

        let mut rejection: Option<TransactionError> = None;
        let mut prepared: Vec<TransactionMember> = Vec::new();
        for member in &discovery_order {
            let key = (member.plugin_id.clone(), member.instance_id);
            let draft = drafts.get(&key).expect("every discovered member has a draft");
            let origin_bytes = match encode_wire_dsl(&draft.origin).await {
                Ok(bytes) => bytes,
                Err(error) => {
                    rejection = Some(TransactionError::Host(error));
                    break;
                }
            };
            let command = protocol::AppCommand::TransactionPrepare {
                seq: self.next_seq().await,
                txn_id: txn_id.clone(),
                mutation_id: String::new(),
                payload: Vec::new(),
                prepared_ops: draft.prepared_ops.clone(),
                origin: origin_bytes,
                prepared_child_ops: Vec::new(),
            };
            let frames = match exchange(&member.plugin_id, member.instance_id, command) {
                Ok(frames) => frames,
                Err(error) => {
                    rejection = Some(error);
                    break;
                }
            };
            let outcome = frames.iter().find_map(|frame| match frame {
                protocol::AppFrame::TransactionPrepared { txn_id: reply_txn, rejection, .. } if reply_txn == &txn_id => Some(rejection.clone()),
                protocol::AppFrame::Error { fault, .. } => Some(fault.clone()),
                _ => None,
            });
            match outcome {
                Some(rejection_bytes) if rejection_bytes.is_empty() => {
                    prepared.push(member.clone());
                }
                Some(rejection_bytes) => {
                    let fault = semio_framework_diagnostic::decode_fault_bytes(&rejection_bytes);
                    rejection = Some(TransactionError::rejected("transaction.member-rejected", format!("{}/{} rejected prepare (`{}`): {}", member.plugin_id, member.instance_id, fault.code.0, fault.message)).await);
                    break;
                }
                None => {
                    rejection = Some(TransactionError::rejected("transaction.member-rejected", format!("{}/{} sent no TransactionPrepared reply", member.plugin_id, member.instance_id)).await);
                    break;
                }
            }
        }

        if let Some(error) = rejection {
            for member in prepared.iter().rev() {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRollback { seq: self.next_seq().await, txn_id: txn_id.clone() });
            }
            return Err(error);
        }

        let mut committed: Vec<TransactionMember> = Vec::new();
        let mut edit_id_by_member: BTreeMap<(String, u32), String> = BTreeMap::new();
        let mut commit_error: Option<TransactionError> = None;
        for member in discovery_order.iter().rev() {
            let command = protocol::AppCommand::TransactionCommit { seq: self.next_seq().await, txn_id: txn_id.clone() };
            let outcome = exchange(&member.plugin_id, member.instance_id, command).ok().and_then(|frames| {
                frames.into_iter().find_map(|frame| match frame {
                    protocol::AppFrame::TransactionCommitted { txn_id: reply_txn, edit_id } if reply_txn == txn_id => Some(edit_id),
                    _ => None,
                })
            });
            match outcome {
                Some(edit_id) => {
                    edit_id_by_member.insert((member.plugin_id.clone(), member.instance_id), edit_id);
                    committed.push(member.clone());
                }
                None => {
                    commit_error = Some(TransactionError::rejected("transaction.commit-failed", format!("{}/{} failed to commit txn `{txn_id}`", member.plugin_id, member.instance_id)).await);
                    break;
                }
            }
        }

        if let Some(error) = commit_error {
            for member in &committed {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionUndo { seq: self.next_seq().await, group_id: txn_id.clone() });
            }
            for member in discovery_order.iter().filter(|member| !committed.contains(member)) {
                let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRollback { seq: self.next_seq().await, txn_id: txn_id.clone() });
            }
            return Err(error);
        }

        let edit_ids: Vec<String> = discovery_order.iter().map(|member| edit_id_by_member.get(&(member.plugin_id.clone(), member.instance_id)).cloned().unwrap_or_default()).collect();
        Ok(TransactionOutcome { txn_id, members: discovery_order, edit_ids })
    }

    /// 🔁️ Contract §5.7: fans `TransactionUndo{group_id}` out to every member, best-effort (a
    /// member whose tail has since moved on independently errors on ITS side, not here — a host
    /// must not assume success for every member).
    pub async fn undo_group(&self, mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>, members: &[TransactionMember], group_id: &str) {
        for member in members {
            let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionUndo { seq: self.next_seq().await, group_id: group_id.to_string() });
        }
    }

    pub async fn redo_group(&self, mut exchange: impl FnMut(&str, u32, protocol::AppCommand) -> Result<Vec<protocol::AppFrame>, TransactionError>, members: &[TransactionMember], group_id: &str) {
        for member in members {
            let _ = exchange(&member.plugin_id, member.instance_id, protocol::AppCommand::TransactionRedo { seq: self.next_seq().await, group_id: group_id.to_string() });
        }
    }
}

impl Default for HostTransactionCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️host-transaction-coordinator/🦀️.rs"]
mod host_transaction_coordinator_tests;
//#endregion 🎯️TransactionCoordinator

//#region 🔖️AppRouter
/// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §3: `(ArtifactDialect,
/// AppRole) -> Vec<AppRef>`, built by walking every loaded plugin's `PluginManifest.apps`.
/// Registration idiom mirrors `ArtifactInferenceRouter` above: one `Mutex`-guarded table,
/// `register_plugin` called once per loaded `WasmPluginRuntime` (reads its own already-resident
/// `.manifest` directly — no extra guest call needed, unlike `IoRouter`/`ArtifactInferenceRouter`,
/// which must query a composer/inference roster over the wasm ABI because the host has no other way
/// to see inside a guest's linear memory; a `PluginManifest` is already host-resident). Ownership of
/// a dialect's `artifact_kind` is derived from `PluginManifest.artifact_kinds` (plugin-level
/// declarations only — matches that field's own doc: "library plugins with zero apps declare kinds
/// here"); the first plugin to claim a kind wins. Nothing here consults per-app `artifact_kinds` —
/// that is a different vocabulary (OS resource/media catalog kinds, see `register_artifact_descriptors`
/// in `semio-framework-os`), not dialect ownership. Deliberately independent of the peer `PluginGraph`
/// region above (dependency-graph load-order/version validation) — this router only needs each
/// manifest's OWN `dependencies` list for the contribution gate, not global load order.
pub struct AppRouter {
    state: Mutex<AppRouterState>,
}

#[derive(Default)]
struct AppRouterState {
    /// 🗂️ `artifact_kind -> owning plugin_id`.
    owners: HashMap<String, String>,
    /// 🗂️ `plugin_id -> its declared dependency plugin ids` (contract §3's contribution gate).
    dependencies: HashMap<String, BTreeSet<String>>,
    /// 🗂️ `(dialect, role) -> registered AppRefs`, unsorted insertion order — `surfaces_for` sorts
    /// lazily against the CURRENT `owners` snapshot (never stale, since ownership never changes once
    /// claimed).
    surfaces: HashMap<(semio_framework_artifact_reference::ArtifactDialect, semio_framework::AppRole), Vec<semio_framework::AppRef>>,
    /// 🚧️ Every `(plugin_id, app_id)` seen so far, for O(1) `surface.conflict` detection.
    registered_refs: std::collections::HashSet<(String, String)>,
    /// 🧯️ `plugin_id -> the fault that excluded it` (ticket 26/09/05/S-END-TO-END lane H). A
    /// breaching manifest registers NO surface at all — never a partial prefix of its apps — and is
    /// remembered here so a shell can show the plugin as excluded instead of silently unroutable.
    plugin_faults: BTreeMap<String, semio_framework::Fault>,
}

impl AppRouter {
    pub fn new() -> Self {
        Self { state: Mutex::new(AppRouterState::default()) }
    }


    /// 🧪️ `register_plugin` split out for direct manifest-driven testing (no wasmtime component
    /// needed to exercise the two frozen conflict/gate faults) — pure aside from the `Mutex` lock.
    /// All-or-nothing per plugin: a breaching manifest leaves no surface behind and its fault is
    /// remembered under its plugin id (`plugin_faults`), so one bad manifest costs exactly that
    /// plugin's surfaces. Its `artifact_kinds` ownership claims survive for the same reason
    /// `unregister_plugin` keeps them — a later contributor must never inherit an excluded plugin's
    /// kind. TS twin: `AppRouter.build` (`🎠️kernel/🟦️.ts`), shared vectors
    /// `🎠️kernel/🧫️fixtures/🧫️app-router-plugin-faults/🔣️.json`.
    ///
    /// 🗑️ The old `register_plugin(&self, plugin_id, runtime: &WasmPluginRuntime)` thin wrapper is
    /// gone with `WasmPluginRuntime` itself — it only ever read `runtime.manifest` (already
    /// host-resident, no wasm call), so every caller now calls `register_manifest` directly with
    /// whatever `PluginManifest` it already has on hand (mirrors `IoRouter`/`ArtifactInferenceRouter`'s
    /// own post-`WasmPluginRuntime` registration idiom: pre-decoded data in, no runtime dependency).
    pub async fn register_manifest(&self, plugin_id: &str, manifest: &PluginManifest) -> Result<(), semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let dependencies: BTreeSet<String> = manifest.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect();
        state.dependencies.insert(plugin_id.to_string(), dependencies);
        for spec in &manifest.artifact_kinds {
            state.owners.entry(spec.id.clone()).or_insert_with(|| plugin_id.to_string());
        }
        let mut staged: Vec<((semio_framework_artifact_reference::ArtifactDialect, semio_framework::AppRole), semio_framework::AppRef)> = Vec::with_capacity(manifest.apps.len());
        let mut staged_refs: BTreeSet<String> = BTreeSet::new();
        let mut breach: Option<semio_framework::Fault> = None;
        for app in &manifest.apps {
            let owner = state.owners.entry(app.dialect.artifact_kind.clone()).or_insert_with(|| plugin_id.to_string()).clone();
            if owner != plugin_id && !state.dependencies.get(plugin_id).is_some_and(|deps| deps.contains(&owner)) {
                breach = Some(
                    semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Framework,
                        semio_framework::FaultCode::new("surface.contribution-not-permitted"),
                        format!("plugin `{plugin_id}` declares a surface for `{}` (owned by `{owner}`) without listing `{owner}` in its dependencies", app.dialect.to_coordinate()),
                    )
                    .with_scope(semio_framework::FaultScope { plugin_id: Some(plugin_id.to_string()), app_id: Some(app.id.clone()), ..Default::default() }),
                );
                break;
            }
            let app_ref = semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app.id.clone() };
            if state.registered_refs.contains(&(app_ref.plugin_id.clone(), app_ref.app_id.clone())) || !staged_refs.insert(app_ref.app_id.clone()) {
                breach = Some(
                    semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("surface.conflict"), format!("surface `{}` is already registered for plugin `{}`", app_ref.app_id, app_ref.plugin_id))
                        .with_scope(semio_framework::FaultScope { plugin_id: Some(app_ref.plugin_id), app_id: Some(app_ref.app_id), ..Default::default() }),
                );
                break;
            }
            staged.push(((app.dialect.clone(), app.role), app_ref));
        }
        if let Some(fault) = breach {
            state.plugin_faults.insert(plugin_id.to_string(), fault.clone());
            return Err(fault);
        }
        state.plugin_faults.remove(plugin_id);
        for (key, app_ref) in staged {
            state.registered_refs.insert((app_ref.plugin_id.clone(), app_ref.app_id.clone()));
            state.surfaces.entry(key).or_default().push(app_ref);
        }
        Ok(())
    }

    /// 🏗️ Registers a whole catalogue in load order and returns every excluded plugin's fault,
    /// sorted by plugin id — the total twin of TS `AppRouter.build`. Never aborts on the first
    /// breach: the plugins that pass still route.
    pub async fn register_manifests(&self, manifests: &[(String, PluginManifest)]) -> Vec<semio_framework::Fault> {
        for (plugin_id, manifest) in manifests {
            let _ = self.register_manifest(plugin_id, manifest).await;
        }
        self.plugin_faults().await
    }

    /// 🧯️ Every excluded plugin's fault, sorted by plugin id — mirrors TS `AppRouter.pluginFaults`.
    pub async fn plugin_faults(&self) -> Vec<semio_framework::Fault> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).plugin_faults.values().cloned().collect()
    }

    /// 🧯️ The fault that excluded `plugin_id`, or `None` when its surfaces route.
    pub async fn fault_for(&self, plugin_id: &str) -> Option<semio_framework::Fault> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).plugin_faults.get(plugin_id).cloned()
    }

    /// 📚️ Every `AppRef` serving `(dialect, role)`, deterministically ordered: the dialect's owner
    /// plugin's surface first (if it has one), then the rest sorted `plugin_id` asc / `app_id` asc
    /// (contract §3).
    pub async fn surfaces_for(&self, dialect: &semio_framework_artifact_reference::ArtifactDialect, role: semio_framework::AppRole) -> Vec<semio_framework::AppRef> {
        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let owner = state.owners.get(&dialect.artifact_kind).cloned();
        let mut refs = state.surfaces.get(&(dialect.clone(), role)).cloned().unwrap_or_default();
        refs.sort_by(|a, b| {
            let a_owner = owner.as_deref() == Some(a.plugin_id.as_str());
            let b_owner = owner.as_deref() == Some(b.plugin_id.as_str());
            match (a_owner, b_owner) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => (a.plugin_id.as_str(), a.app_id.as_str()).cmp(&(b.plugin_id.as_str(), b.app_id.as_str())),
            }
        });
        refs
    }

    /// 🎯️ The dialect's owner plugin id, if any plugin has claimed `artifact_kind` (via its own
    /// `PluginManifest.artifact_kinds`, or by being first to declare ANY surface for it).
    pub async fn owner_of(&self, artifact_kind: &str) -> Option<String> {
        self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).owners.get(artifact_kind).cloned()
    }

    /// ✂️ Drops every surface AND dependency record `plugin_id` registered — mirrors `IoRouter`/
    /// `ArtifactMutationRouter`/`ArtifactInferenceRouter`'s own `unregister_plugin` (called from
    /// `WasmtimeNodeHost::unload_plugin`/`hot_reload_plugin`). Deliberately does NOT clear `owners`:
    /// a hot-reload of the owner plugin re-registers the same `artifact_kinds` and re-claims
    /// ownership on its own; clearing the entry here would let whichever OTHER plugin happens to be
    /// registered next silently inherit ownership mid-reload — worse than a momentarily-stale owner
    /// pointing at a plugin with zero live surfaces (which `owned_surface_gaps` would then flag).
    pub async fn unregister_plugin(&self, plugin_id: &str) {
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        state.dependencies.remove(plugin_id);
        state.plugin_faults.remove(plugin_id);
        for refs in state.surfaces.values_mut() {
            refs.retain(|app_ref| app_ref.plugin_id != plugin_id);
        }
        state.registered_refs.retain(|(owner, _)| owner != plugin_id);
    }

    /// 🩺️ Contract §3: every dialect with AT LEAST ONE registered surface, whose `artifact_kind` has a
    /// known owner, must resolve for BOTH roles. Pure and total, never panics — the caller decides
    /// whether to log or hard-fail. W1 logged this as a soft diagnostic while every plugin still had
    /// zero surfaces; W3 (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET, now that
    /// `policySubsetSurfaceCompletenessBreaches` + the W2 scaffolder have populated all 286 real
    /// surfaces) flips `load_runtime_recursive`'s caller to hard-fail the load instead. Deliberately
    /// scoped to what the router can actually see: full taxonomy-disk subset completeness (all 143
    /// subsets, including ones with zero surfaces at all) is `policySubsetSurfaceCompletenessBreaches`'s
    /// job in `📜️script.ts`, not a host runtime concern — a wasm plugin host cannot walk the repo
    /// filesystem.
    pub async fn owned_surface_gaps(&self) -> Vec<semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let mut dialects: Vec<semio_framework_artifact_reference::ArtifactDialect> = state.surfaces.keys().map(|(dialect, _)| dialect.clone()).collect();
        dialects.sort();
        dialects.dedup();
        let mut gaps = Vec::new();
        for dialect in dialects {
            let Some(owner) = state.owners.get(&dialect.artifact_kind) else { continue };
            for role in [semio_framework::AppRole::Viewer, semio_framework::AppRole::Editor] {
                let has_surface = state.surfaces.get(&(dialect.clone(), role)).is_some_and(|refs| !refs.is_empty());
                if !has_surface {
                    gaps.push(semio_framework::Fault::new(
                        semio_framework::FaultOrigin::Framework,
                        semio_framework::FaultCode::new("surface.missing-owner-surface"),
                        format!("`{}` (owned by `{owner}`) has no registered {} surface", dialect.to_coordinate(), role.as_str()),
                    ));
                }
            }
        }
        gaps
    }
}

impl Default for AppRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️app-router/🦀️.rs"]
mod app_router_tests;
//#endregion 🔖️AppRouter

//#region 🔖️OpeningResolver
/// 🧭️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §3: the frozen four-step
/// precedence — (1) an explicit user default, if it is still present in the router; (2) the
/// dialect's owner plugin's surface; (3) the router's first entry; (4) `surface.unknown-dialect`.
/// Steps 2 and 3 collapse into ONE branch below by construction: `AppRouter::surfaces_for` already
/// sorts the owner's surface first (when the owner has one), so "the first entry" and "the owner's
/// surface" are the same value whenever an owner surface exists — there is no code path where they
/// would disagree. `user_default` is a single already-resolved lookup, not the whole
/// `OpeningPreferences` value: folding the config op log into that lookup
/// (`OpeningPreferences`'s `MutationDiff` impl / `apply_opening_config_mutation`, `🎚️config/🧬️schema`)
/// is the CALLER's job — this function only ever reads, never mutates, matching the ticket's "never a
/// mutable map" instruction for `prefs`.
pub struct OpeningResolver;

impl OpeningResolver {
    /// 🚫️async: R10 residue shape 2 — a future is consumed by one `.await`; awaited once here
    /// instead of once inside the `if let` and again (moved) at `.into_iter()` below.
    pub async fn resolve(router: &AppRouter, dialect: &semio_framework_artifact_reference::ArtifactDialect, role: semio_framework::AppRole, user_default: Option<&semio_framework::AppRef>) -> Result<semio_framework::AppRef, semio_framework::Fault> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

        let candidates = router.surfaces_for(dialect, role).await;
        if let Some(default_ref) = user_default {
            if candidates.contains(default_ref) {
                return Ok(default_ref.clone());
            }
        }
        if let Some(first) = candidates.into_iter().next() {
            return Ok(first);
        }
        Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("surface.unknown-dialect"), format!("no {} surface registered for `{}`", role.as_str(), dialect.to_coordinate())))
    }
}

#[cfg(test)]
#[path = "../🧪️tests/🔬️opening-resolver/🦀️.rs"]
mod opening_resolver_tests;
//#endregion 🔖️OpeningResolver
