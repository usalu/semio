//! 💡️ Inference router: the guest artifact inference metadata, the route request, budget, cache mode and result, the artifact inference router and its dependency graph validation. See [`super`].

use super::*;

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
pub(super) struct InferenceRouteRequest {
    pub(super) wire_version: u32,
    pub(super) owner: String,
    pub(super) artifact_kind: String,
    pub(super) artifact_schema: String,
    pub(super) artifact_schema_version: u32,
    pub(super) inference_schema: String,
    pub(super) inference_schema_version: u32,
    pub(super) algorithm_version: u32,
    pub(super) policy_version: u32,
    pub(super) revision: u64,
    pub(super) generation: u64,
    pub(super) source_dialect: String,
    pub(super) policy: Vec<u8>,
    pub(super) budgets: InferenceRouteBudget,
    pub(super) retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    pub(super) cancellation_id: String,
    pub(super) previous_state: Option<Vec<u8>>,
    pub(super) requested_cache_mode: InferenceRouteCacheMode,
    pub(super) canonical_payload: Vec<u8>,
    pub(super) dependencies: Vec<(String, Vec<u8>)>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) struct InferenceRouteBudget {
    pub(super) allocation_bytes: u64,
    pub(super) work_units: u64,
    pub(super) recursion_depth: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, ToValue, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) enum InferenceRouteCacheMode {
    Cold,
    Incremental,
    Bypass,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub(super) struct InferenceRouteResult {
    pub(super) wire_version: u32,
    pub(super) owner: String,
    pub(super) artifact_kind: String,
    pub(super) artifact_schema: String,
    pub(super) artifact_schema_version: u32,
    pub(super) inference_schema: String,
    pub(super) inference_schema_version: u32,
    pub(super) algorithm_version: u32,
    pub(super) policy_version: u32,
    pub(super) revision: u64,
    pub(super) generation: u64,
    pub(super) source_dialect: String,
    pub(super) policy: Vec<u8>,
    pub(super) budgets: InferenceRouteBudget,
    pub(super) retained: semio_framework_value::retained_clone::RetainedCloneGrant,
    pub(super) cancellation_id: String,
    pub(super) previous_state: Option<Vec<u8>>,
    pub(super) requested_cache_mode: InferenceRouteCacheMode,
    pub(super) canonical_payload: Vec<u8>,
    pub(super) dependencies: Vec<(String, Vec<u8>)>,
    pub(super) complete: bool,
    pub(super) actual_cache_mode: InferenceRouteCacheMode,
}

pub struct ArtifactInferenceRouter {
    pub(super) routes: Mutex<BTreeMap<(String, String), (String, GuestArtifactInferenceMetadata)>>,
    pub(super) runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>,
    pub(super) live_commits: Mutex<BTreeMap<String, (u64, u64)>>,
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
pub(super) async fn build_dependency_inference_request(base: &InferenceRouteRequest, dependency: &GuestArtifactInferenceMetadata) -> InferenceRouteRequest {
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
pub(super) fn routed_inference_dependencies(requested: Vec<(String, Vec<u8>)>, resolved: Vec<(String, Vec<u8>)>) -> Vec<(String, Vec<u8>)> {
    requested.into_iter().chain(resolved).collect()
}

pub(super) async fn validate_inference_echo(request: &InferenceRouteRequest, result: &InferenceRouteResult) -> Result<(), PluginHostError> {
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
//#endregion 💡️InferenceRouter
