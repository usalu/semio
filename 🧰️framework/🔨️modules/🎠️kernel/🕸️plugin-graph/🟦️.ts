/** 🕸️ The plugin dependency graph: version pins, validation, load order, dependents, the graph class and the localized dependency fault message.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import type { ShellLocale } from "../../🛂️manifest/🤖️generated/🎚️ui-axes/🟦️.ts";
import { type PluginDependency, type PluginRegistryEntry, type VersionPin } from "../🗂️plugin-registry/🟦️.ts";

// #region 🎠️Kernel
//#region 🔖️PluginGraph
/** 🔗️ One node of the plugin dependency graph — a plugin's own id/version plus the dependencies its
 * manifest declares. Mirrors Rust `PluginManifest`'s `pluginId`/`version`/`dependencies` triple
 * (`🛂️manifest/🦀️.rs`), narrowed to exactly what {@link resolvePluginLoadOrder}/
 * {@link validatePluginDependencyGraph} need — a caller with only a `PluginRegistryEntry` (no
 * `version` yet, contract freeze §3-era catalogs) still validates presence/cycles correctly. */
export type PluginGraphNode = {
  readonly pluginId: string;
  readonly version?: string;
  readonly dependencies?: readonly PluginDependency[];
};

/** 🧯 One dependency-graph fault — reuses the frozen transaction rejection codes (contract freeze §5
 * rejection taxonomy, ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS)
 * since plugin-load rejection and transaction contribution resolution share the same three failure
 * shapes (contract freeze §4 rule 5: "Dependency graph: missing dependency, version mismatch, or
 * cycle ⇒ plugin load rejected with a typed error"). */
export type PluginGraphError =
  | { readonly code: "transaction.dependency-missing"; readonly pluginId: string; readonly dependsOn: string }
  | { readonly code: "transaction.version-mismatch"; readonly pluginId: string; readonly dependsOn: string; readonly required: VersionPin; readonly actual: string }
  | { readonly code: "transaction.cycle"; readonly members: readonly string[] };

type ParsedVersion = { readonly major: number; readonly minor: number; readonly patch: number };

function parseVersion(raw: string | undefined): ParsedVersion | null {
  if (!raw) return null;
  const match = /^(\d+)\.(\d+)\.(\d+)$/.exec(raw.trim());
  if (!match) return null;
  return { major: Number(match[1]), minor: Number(match[2]), patch: Number(match[3]) };
}

/** 🔢️ Parses the one pin form, `=X.Y.Z`. Mirrors Rust `VersionPin::parse`'s accepted syntax exactly: every
 * range (`*`, `^`, `~`, `>=`, a bare triple) is refused. */
function parseVersionPin(raw: VersionPin): ParsedVersion | null {
  const match = /^=(\d+\.\d+\.\d+)$/.exec(raw.trim());
  return match ? parseVersion(match[1]) : null;
}

/** ✅️ True when `actual` (a plain `major.minor.patch` string) is exactly the pinned version. An
 * unparseable `actual`/`pin` — a range included — is unsatisfied and never throws, matching the "typed
 * error, never a panic" law the Rust planner's law tests hold `PlanError` to (contract freeze §1 law 4). */
export function versionSatisfies(actual: string, pin: VersionPin): boolean {
  const required = parseVersionPin(pin);
  const version = parseVersion(actual);
  return required !== null && version !== null && version.major === required.major && version.minor === required.minor && version.patch === required.patch;
}

/** 🧯 Validates every node's declared `dependencies` resolve (present, version-satisfying) — does
 * NOT detect cycles (see {@link resolvePluginLoadOrder}, which layers cycle detection on top only
 * once every missing/mismatched edge has already been reported). A node with no `version`, or an edge
 * with no pin (the registry's pre-build view), skips the version check (nothing to compare) rather than
 * failing closed. Mirrors Rust `validate_dependency_graph`. */
export function validatePluginDependencyGraph(nodes: readonly PluginGraphNode[]): readonly PluginGraphError[] {
  const byId = new Map(nodes.map((node) => [node.pluginId, node] as const));
  const errors: PluginGraphError[] = [];
  for (const node of nodes) {
    for (const dependency of node.dependencies ?? []) {
      const target = byId.get(dependency.pluginId);
      if (!target) {
        errors.push({ code: "transaction.dependency-missing", pluginId: node.pluginId, dependsOn: dependency.pluginId });
        continue;
      }
      if (dependency.version !== undefined && target.version !== undefined && !versionSatisfies(target.version, dependency.version)) {
        errors.push({ code: "transaction.version-mismatch", pluginId: node.pluginId, dependsOn: dependency.pluginId, required: dependency.version, actual: target.version });
      }
    }
  }
  return errors;
}

/** 🔁️ DFS cycle extraction restricted to `leftover` (the toposort leftover set) — names every plugin
 * actually on a cycle rather than the whole leftover set (which may include acyclic nodes downstream
 * of the real cycle). Falls back to the sorted leftover set only if no back-edge is found (should not
 * happen given `leftover` is non-empty and the full graph already passed structural validation). */
function findCycleMembers(byId: ReadonlyMap<string, PluginGraphNode>, leftover: ReadonlySet<string>): readonly string[] {
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const stack: string[] = [];
  let cycle: string[] | null = null;

  function visit(id: string): void {
    if (cycle || !leftover.has(id) || visited.has(id)) return;
    if (visiting.has(id)) {
      const start = stack.indexOf(id);
      cycle = stack.slice(start);
      return;
    }
    visiting.add(id);
    stack.push(id);
    for (const dependency of byId.get(id)?.dependencies ?? []) {
      if (leftover.has(dependency.pluginId)) visit(dependency.pluginId);
      if (cycle) return;
    }
    stack.pop();
    visiting.delete(id);
    visited.add(id);
  }

  for (const id of [...leftover].sort()) {
    visit(id);
    if (cycle) break;
  }
  return cycle ?? [...leftover].sort();
}

/** 🔁️ Kahn toposort with lexicographically-smallest-id tie-breaking — mirrors Rust
 * `resolve_load_order` exactly, including the deterministic tie-break. `errors` is non-empty and
 * `order` empty on any missing dependency or version mismatch (reported before a cycle would be,
 * matching the Rust validate-then-sort order) or on a real cycle (members individually named via
 * {@link findCycleMembers}, not just the toposort leftover set). */
export function resolvePluginLoadOrder(nodes: readonly PluginGraphNode[]): { readonly order: readonly string[]; readonly errors: readonly PluginGraphError[] } {
  const structural = validatePluginDependencyGraph(nodes);
  if (structural.length > 0) return { order: [], errors: structural };

  const byId = new Map(nodes.map((node) => [node.pluginId, node] as const));
  const indegree = new Map<string, number>();
  const dependents = new Map<string, string[]>();
  for (const node of nodes) {
    indegree.set(node.pluginId, indegree.get(node.pluginId) ?? 0);
    for (const dependency of node.dependencies ?? []) {
      indegree.set(node.pluginId, (indegree.get(node.pluginId) ?? 0) + 1);
      const list = dependents.get(dependency.pluginId) ?? [];
      list.push(node.pluginId);
      dependents.set(dependency.pluginId, list);
    }
  }

  const order: string[] = [];
  const remaining = new Map(indegree);
  const queue = [...indegree.entries()].filter(([, count]) => count === 0).map(([id]) => id);
  while (queue.length > 0) {
    queue.sort();
    const id = queue.shift()!;
    order.push(id);
    for (const dependent of dependents.get(id) ?? []) {
      const next = (remaining.get(dependent) ?? 0) - 1;
      remaining.set(dependent, next);
      if (next === 0) queue.push(dependent);
    }
  }

  if (order.length === nodes.length) return { order, errors: [] };
  const leftover = new Set(nodes.map((node) => node.pluginId).filter((id) => !order.includes(id)));
  return { order: [], errors: [{ code: "transaction.cycle", members: findCycleMembers(byId, leftover) }] };
}

/** 🔎️ Direct dependents of `pluginId` — every node that declares `pluginId` in its own `dependencies`
 * — sorted. Mirrors Rust `dependents`. */
export function pluginDependents(nodes: readonly PluginGraphNode[], pluginId: string): readonly string[] {
  return nodes
    .filter((node) => (node.dependencies ?? []).some((dependency) => dependency.pluginId === pluginId))
    .map((node) => node.pluginId)
    .sort();
}

/** 🕸️ Convenience wrapper over the pure {@link validatePluginDependencyGraph}/
 * {@link resolvePluginLoadOrder}/{@link pluginDependents} functions for a caller that wants to hold
 * one graph instance across several queries (boot ordering, then later a hot-reload/unload guard). */
export class PluginGraph {
  private readonly nodes: readonly PluginGraphNode[];
  constructor(nodes: readonly PluginGraphNode[]) {
    this.nodes = nodes;
  }
  validate(): readonly PluginGraphError[] {
    return validatePluginDependencyGraph(this.nodes);
  }
  loadOrder(): { readonly order: readonly string[]; readonly errors: readonly PluginGraphError[] } {
    return resolvePluginLoadOrder(this.nodes);
  }
  dependents(pluginId: string): readonly string[] {
    return pluginDependents(this.nodes, pluginId);
  }
  /** 🚫️ Contract freeze §4 rule 5's "unload refused while dependents are loaded" — `loadedIds` is
   * every plugin id currently resident (not merely declared in the graph), so a dependent that was
   * never actually loaded doesn't block `pluginId`'s unload. */
  canUnload(pluginId: string, loadedIds: ReadonlySet<string>): boolean {
    return this.dependents(pluginId).every((dependent) => !loadedIds.has(dependent));
  }
}

/** 🎯️ Orders `entries` by {@link PluginGraph.loadOrder}; entries the graph can't place (missing
 * dependency, version mismatch, or on a cycle) are dropped from the returned order and reported in
 * `errors` instead of silently keeping their original array position — contract freeze §4 rule 5's
 * "plugin load rejected with a typed error". Every other entry still boots, in dependency order
 * (best-effort degrade, matching the shell's existing fail-soft posture toward a single unavailable
 * plugin module). An entry with no declared `dependencies` at all always validates trivially, so a
 * registry that hasn't adopted `dependsOn` yet round-trips through this function unchanged (array
 * order in, array order out). */
export function orderPluginRegistryEntries(entries: readonly PluginRegistryEntry[]): { readonly order: readonly PluginRegistryEntry[]; readonly errors: readonly PluginGraphError[] } {
  const nodes: PluginGraphNode[] = entries.map((entry) => ({ pluginId: entry.pluginId, dependencies: entry.dependencies }));
  const { order, errors } = new PluginGraph(nodes).loadOrder();
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry] as const));
  if (errors.length === 0) {
    return { order: order.map((id) => byId.get(id)).filter((entry): entry is PluginRegistryEntry => entry !== undefined), errors: [] };
  }
  // 🔁 Retry on the remaining (non-blocked) subset — a single missing/mismatched/cyclic entry must
  // not degrade every OTHER entry back to plain array order; it only takes itself (and, for a cycle,
  // its fellow cycle members) out of the graph. Each retry strictly shrinks `entries`, so this always
  // terminates.
  const blocked = new Set(errors.flatMap((error) => (error.code === "transaction.cycle" ? error.members : [error.pluginId])));
  const remaining = entries.filter((entry) => !blocked.has(entry.pluginId));
  const retried = orderPluginRegistryEntries(remaining);
  return { order: retried.order, errors: [...errors, ...retried.errors] };
}
//#endregion 🔖️PluginGraph

//#region 🌐️DependencyFault
/** 🌐️ Picks the best string out of a {@link LocalizedLabel}-shaped `{en, de}` record for `locale` —
 * falls back to English, then to whatever key exists, since this repo supports multiple languages
 * with no default language but a fault MUST still render something rather than nothing. */
function resolveLocalizedLabel(label: Record<string, string>, locale: ShellLocale): string {
  return label[locale] ?? label.en ?? Object.values(label)[0] ?? "";
}

/** 🌐️ Turns a {@link PluginGraphError} into a real, localized (English + German) message — the
 * dependency-fault UI this ticket requires instead of a bare console error. Callers needing a
 * console-safe fallback can still log the same string; this is the single source of the wording so
 * a boot banner and an in-shell notification never drift apart. */
export function pluginGraphErrorMessage(error: PluginGraphError, locale: ShellLocale): string {
  switch (error.code) {
    case "transaction.dependency-missing":
      return resolveLocalizedLabel(
        {
          en: `Plugin "${error.pluginId}" needs "${error.dependsOn}", which is not installed.`,
          de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“, welches nicht installiert ist.`,
        },
        locale,
      );
    case "transaction.version-mismatch":
      return resolveLocalizedLabel(
        {
          en: `Plugin "${error.pluginId}" needs "${error.dependsOn}" ${error.required}, but ${error.actual} is installed.`,
          de: `Das Plugin „${error.pluginId}“ benötigt „${error.dependsOn}“ ${error.required}, installiert ist jedoch ${error.actual}.`,
        },
        locale,
      );
    case "transaction.cycle":
      return resolveLocalizedLabel(
        {
          en: `Plugin dependency cycle: ${error.members.join(" → ")}.`,
          de: `Zyklische Plugin-Abhängigkeit: ${error.members.join(" → ")}.`,
        },
        locale,
      );
  }
}
//#endregion 🌐️DependencyFault
// #endregion 🎠️Kernel
