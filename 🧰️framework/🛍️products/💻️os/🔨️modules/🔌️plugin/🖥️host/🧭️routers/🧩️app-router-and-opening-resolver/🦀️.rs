//! 🧩️ App router and opening resolver: the app router with its state and the opening resolver. See [`super`].

use super::*;

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
    pub(super) state: Mutex<AppRouterState>,
}

#[derive(Default)]
struct AppRouterState {
    /// 🗂️ `artifact_kind -> owning plugin_id`.
    owners: HashMap<String, String>,
    /// 🗂️ `plugin_id -> its declared dependency plugin ids` (contract §3's contribution gate).
    pub(super) dependencies: HashMap<String, BTreeSet<String>>,
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
//#endregion 🔖️OpeningResolver
