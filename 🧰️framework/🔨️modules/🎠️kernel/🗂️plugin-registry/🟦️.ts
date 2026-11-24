/** 🗂️ The plugin registry entries and their dependency pins, and `expandPluginRegistry`, which adds a primary plugin's dependencies and host-mode entries.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

// #region 🎠️Kernel
/** 🔗️ The edges of `entry` a boot must satisfy before its actor starts: every edge not declared `on-demand`. */
export function requiredDependencies(entry: { readonly dependencies?: readonly PluginDependency[] }): readonly PluginDependency[] {
  return (entry.dependencies ?? []).filter((dependency) => dependency.activation !== "on-demand");
}

/** ⏳️ True for an entry its own `activationEvents` defer: it is activated when one of them fires, never at boot. */
export function isActivatedOnDemand(entry: { readonly activationEvents?: readonly string[] }): boolean {
  return (entry.activationEvents ?? []).length > 0;
}

/**
 * 🧩️ Expands a plugin registry for a primary plugin: `primaryPluginId` is matched directly
 * against entry `pluginId` (no registry-id indirection), then every other entry whose
 * `contributes` intersects the primary entry's `consumes` is appended, unless that entry declares
 * activation events ({@link isActivatedOnDemand}) — such a contributor is installed but activated on
 * demand ({@link onDemandPluginEntries}). Only `required` dependency edges are followed. Host mode
 * (a launch that hosts every plugin at once, e.g. a shell/studio session), or the absence of a primary
 * id, passes the full registry through unchanged.
 */
export function expandPluginRegistry(plugins: readonly PluginRegistryEntry[], primaryPluginId?: string, hostMode = false): readonly PluginRegistryEntry[] {
  if (hostMode || !primaryPluginId) return plugins;
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry] as const));
  const primaryEntries = plugins.filter((entry) => entry.pluginId === primaryPluginId);
  const consumes = new Set(primaryEntries.flatMap((entry) => entry.consumes ?? []));
  const contributorEntries = plugins.filter((entry) => entry.pluginId !== primaryPluginId && !isActivatedOnDemand(entry) && (entry.contributes ?? []).some((tag) => consumes.has(tag)));
  const selected = new Map<string, PluginRegistryEntry>();
  const queue: PluginRegistryEntry[] = [...primaryEntries, ...contributorEntries];
  for (const entry of queue) selected.set(entry.pluginId, entry);
  for (let index = 0; index < queue.length; index++) {
    const entry = queue[index]!;
    for (const dependency of requiredDependencies(entry)) {
      if (selected.has(dependency.pluginId)) continue;
      const dependencyEntry = byId.get(dependency.pluginId);
      if (!dependencyEntry) continue;
      selected.set(dependency.pluginId, dependencyEntry);
      queue.push(dependencyEntry);
    }
  }
  return [...selected.values()];
}

/**
 * ⏳️ The entries a boot plan leaves out and a request may activate later: every entry of `plugins` outside `selected`
 * that a selected entry reaches through an `on-demand` edge, or that contributes a tag a selected entry consumes.
 * Order follows `plugins`; an entry appears once.
 */
export function onDemandPluginEntries(plugins: readonly PluginRegistryEntry[], selected: readonly { readonly pluginId: string; readonly consumes?: readonly string[]; readonly dependencies?: readonly PluginDependency[] }[]): readonly PluginRegistryEntry[] {
  const active = new Set(selected.map((entry) => entry.pluginId));
  const consumed = new Set(selected.flatMap((entry) => entry.consumes ?? []));
  const named = new Set(selected.flatMap((entry) => (entry.dependencies ?? []).filter((dependency) => dependency.activation === "on-demand").map((dependency) => dependency.pluginId)));
  return plugins.filter((entry) => !active.has(entry.pluginId) && (named.has(entry.pluginId) || (entry.contributes ?? []).some((tag) => consumed.has(tag))));
}

/** 🧮️ What starting `pluginId` needs started first: its not yet active `required` dependency closure in start order, itself last; `undefined` when it or a required dependency is not in `plugins`. */
export function onDemandActivationClosure(plugins: readonly PluginRegistryEntry[], activeIds: ReadonlySet<string>, pluginId: string): readonly PluginRegistryEntry[] | undefined {
  const byId = new Map(plugins.map((entry) => [entry.pluginId, entry] as const));
  const order: PluginRegistryEntry[] = [];
  const visiting = new Set<string>();
  const visit = (id: string): boolean => {
    if (activeIds.has(id) || order.some((entry) => entry.pluginId === id)) return true;
    const entry = byId.get(id);
    if (!entry || visiting.has(id)) return false;
    visiting.add(id);
    for (const dependency of requiredDependencies(entry)) if (!visit(dependency.pluginId)) return false;
    visiting.delete(id);
    order.push(entry);
    return true;
  };
  return visit(pluginId) ? order : undefined;
}

export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes?: readonly string[];
  readonly consumes?: readonly string[];
  /** 🎬️ The declared activation events (`on-artifact-kind:<kind>`, `on-node-kind:<namespace>`, …): a non-empty list defers the entry to {@link onDemandPluginEntries}. */
  readonly activationEvents?: readonly string[];
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
  /** ⏳️ `required` (the default) must be started before the dependent; `on-demand` is started by its own activation events. */
  readonly activation?: PluginDependencyActivation;
};

/** ⏳️ How a dependency edge is satisfied — the vocabulary of `🧬️schema/🧩️plugin-modules/🔣️.json` `$defs.dependency.activation`. */
export type PluginDependencyActivation = "required" | "on-demand";
//#endregion 🔖️PluginDependency
// #endregion 🎠️Kernel
