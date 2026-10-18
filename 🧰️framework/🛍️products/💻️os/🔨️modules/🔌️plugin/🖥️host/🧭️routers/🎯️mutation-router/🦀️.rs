//! 🎯️ Mutation router: the host mutation roster, the mutation plan request and result, the wire helpers, the mutation ownership and the artifact mutation router. See [`super`].

use super::*;

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
pub(super) async fn decode_wire_dsl<T: semio_framework_value::FromValue>(bytes: &[u8]) -> Result<T, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    let value = store::pack_rt::renormalize_whole_number_floats(value);
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

pub(super) async fn encode_wire_dsl<T: semio_framework_value::ToValue>(value: &T) -> Result<Vec<u8>, PluginHostError> {
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
    pub(super) routes: Mutex<BTreeMap<(String, String), (String, HostMutationRosterEntry)>>,
    /// 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (cold-kinds): mirrors `ArtifactInferenceRouter`'s
    /// own `runtimes` field EXACTLY (same shape, same purpose) — kept by `register_plugin`/
    /// `unregister_plugin` for `plan`'s dispatch. Production today calls `register_roster` directly
    /// (see `🏃️run/🦀️.rs`, outside this packet's owned paths), which never touches this
    /// field — see this packet's report `## lease-requests` for the one-line change that wires it.
    pub(super) runtimes: Mutex<HashMap<String, Arc<PluginInstanceHandle>>>,
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
//#endregion 🎯️MutationRouter
