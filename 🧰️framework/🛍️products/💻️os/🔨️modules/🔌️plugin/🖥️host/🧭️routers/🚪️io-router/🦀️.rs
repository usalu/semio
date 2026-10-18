//! 🚪️ Io router: the io router with its route keys, ranking, route walk, conflict and re-entry checks and the router implementation. See [`super`].

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
    pub(super) state: Mutex<IoRouterState>,
}

struct IoRouterState {
    pub(super) routes: HashMap<semio_framework_os_kernel::io::IoKey, String>,
    pub(super) runtimes: HashMap<String, Arc<PluginInstanceHandle>>,
    /// 🌉️ CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM (W1-D): the NEW mechanism's merged cross-plugin
    /// graph — `(from, into) -> IoEntryRoute` — built from every plugin's `list-io-entries` roster,
    /// separate from `routes` above (OLD `IoKey`-keyed graph). `BTreeMap` (not `HashMap`) is load-
    /// bearing: `resolve_io_route`'s determinism proof depends on canonical iteration order, mirroring
    /// `io::io_mechanism`'s own `EntryMap`.
    io_entries: BTreeMap<IoEntryKey, IoEntryRoute>,
}

/// 🌉️ One edge of the NEW mechanism's merged graph: `(from, into)`.
pub type IoEntryKey = (semio_framework_artifact_reference::ArtifactDialect, semio_framework_artifact_reference::ArtifactDialect);

/// 🌉️ One edge's owner + declared strength — erased from the owning plugin's `IoEntryDescriptor`,
/// plus which plugin registered it (needed by `run_io`'s reentrancy guard and `identify`'s fan-out).
#[derive(Clone, Debug, PartialEq)]
pub struct IoEntryRoute {
    pub owner: String,
    pub fidelity: semio_framework::io_schema::IoFidelity,
    pub sniffs: bool,
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
pub fn resolve_io_route(graph: &BTreeMap<IoEntryKey, IoEntryRoute>, from: &semio_framework_artifact_reference::ArtifactDialect, into: &semio_framework_artifact_reference::ArtifactDialect, max_hops: u8) -> Result<semio_framework::io_schema::IoRoute, PluginHostError> {
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
pub fn io_entries_conflict(existing: &BTreeMap<IoEntryKey, IoEntryRoute>, plugin_id: &str, incoming: &[semio_framework::io_schema::IoEntryDescriptor]) -> Option<PluginHostError> {
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
pub fn route_reenters_calling_plugin<'route>(
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
