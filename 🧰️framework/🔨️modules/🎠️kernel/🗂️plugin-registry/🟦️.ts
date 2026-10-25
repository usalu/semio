/** 🗂️ The plugin registry entries and their dependency pins, and `expandPluginRegistry`, which adds a primary plugin's dependencies and host-mode entries.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
/**
 * 🧩️ Expands a plugin registry for a primary plugin: `primaryPluginId` is matched directly
 * against entry `pluginId` (no registry-id indirection), then every other entry whose
 * `contributes` intersects the primary entry's `consumes` is appended. Host mode (a launch that
 * hosts every plugin at once, e.g. a shell/studio session), or the absence of a primary id, passes
 * the full registry through unchanged.
 */
export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, hostMode = false): readonly PluginRegistryEntry[] {
  if (hostMode || !primaryPluginId) return plugins;
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry] as const));
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  const selected = new Map<string, PluginRegistryEntry>();
  const queue: PluginRegistryEntry[] = [...primaryEntries, ...contributorEntries];
  for (const entry of queue) selected.set(entry.pluginId, entry);
  for (let index = 0; index < queue.length; index++) {
    const entry = queue[index]!;
    for (const dependency of entry.dependencies ?? []) {
      if (selected.has(dependency.pluginId)) continue;
      const dependencyEntry = byId.get(dependency.pluginId);
      if (!dependencyEntry) continue;
      selected.set(dependency.pluginId, dependencyEntry);
      queue.push(dependencyEntry);
    }
  }
  return [...selected.values()];
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
  /** 🔗️ Direct plugin dependencies this entry's manifest declares — mirrors Rust
   * `PluginManifest.dependencies` (`🛂️manifest/🦀️.rs`), ticket
   * 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS §3. */
  readonly dependencies?: readonly PluginDependency[];
};

/** ⛓️ Widens a {@link PluginCatalogTarget.dependsOn} plugin-id list (the crate's declared runtime
 * dependencies, carried without version info) into `PluginRegistryEntry.dependencies` — each edge
 * carries only its id, so {@link resolvePluginLoadOrder}/{@link validatePluginDependencyGraph}
 * validate presence and detect cycles from the registry's pre-build view, which has no pin to read;
 * the exact pin travels on the loaded manifest's own `dependencies` once the plugin's descriptor is
 * available. */
export function dependsOnToPluginDependencies(dependsOn: readonly string[] | undefined): readonly PluginDependency[] | undefined {
  return dependsOn?.map((pluginId) => ({ pluginId }));
}

//#region 🔖️PluginDependency
/** 📌️ An exact dependency pin string, `=X.Y.Z` — the only form a manifest declares, because a trusted
 * catalog admits only exact pins inside its closure. Mirrors Rust `VersionPin`'s `Display`/`Serialize`
 * wire form exactly. */
export type VersionPin = string;

/** 🕸️ One direct plugin dependency — mirrors Rust `PluginDependency` (`🛂️manifest/🦀️.rs`). A
 * registry-derived edge carries no `version` (the pre-build view has none to read); a manifest-derived
 * edge carries its exact pin. */
export type PluginDependency = {
  readonly pluginId: string;
  readonly version?: VersionPin;
};
//#endregion 🔖️PluginDependency
// #endregion 🎠️Kernel
