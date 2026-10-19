/** 🎮️ Playground resolution: the registry id and default app of a playground, the boot planner and the host config.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { dependsOnToPluginDependencies, expandPluginRegistry, type PluginRegistryEntry } from "../🗂️plugin-registry/🟦️.ts";
import { type PlaygroundCatalogTarget, type PluginCatalog, type PluginCatalogTarget, type PluginHostConfig } from "../📚️plugin-catalog/🟦️.ts";
import { orderPluginRegistryEntries, type PluginGraphError } from "../🕸️plugin-graph/🟦️.ts";

// #region 🎠️Kernel
// #region 🎮️PlaygroundResolution
/** 🎮️ Finds the injected catalog's playground row for a variant id or one of its aliases. */
function findPlaygroundVariant(catalog: PluginCatalog, playgroundPluginId: string): PlaygroundCatalogTarget | undefined {
  return catalog.playgrounds.find((entry) => entry.variant === playgroundPluginId || entry.aliases.includes(playgroundPluginId));
}

/** 🎯️ Resolves a playground filter/alias (e.g. "3d", "sourcing") to its underlying wasm component registry id. */
export function resolvePluginRegistryId(catalog: PluginCatalog, playgroundPluginId: string): string {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.pluginId ?? playgroundPluginId;
}

/** 🎯️ Resolves a playground filter/alias to the app id that should be instantiated by default within its plugin's manifest. */
export function resolvePlaygroundDefaultAppId(catalog: PluginCatalog, playgroundPluginId: string): string | undefined {
  return findPlaygroundVariant(catalog, playgroundPluginId)?.app;
}

export type PlaygroundBootSession = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
};

export type PlaygroundBoot = {
  readonly variant: string;
  readonly defaultAppId?: string;
  readonly plugins: readonly PluginRegistryEntry[];
  /** 🧯 Any {@link PluginGraphError}s that kept an entry out of `plugins` — empty for the
   * `session`-reuse fast path (nothing was recomputed). A caller renders these through
   * {@link pluginGraphErrorMessage} instead of leaving the gap silent. */
  readonly dependencyErrors: readonly PluginGraphError[];
};

/** 🧱️ How many catalog rows one {@link PlaygroundBootPlanner} chunk projects. Sized so a chunk
 * stays an order of magnitude under the frame Worker's 8 ms step ceiling even on the slowest target: the
 * whole 59-row projection executes in 46 µs natively, so a 16-row chunk is ~12 µs. */
export const PLUGIN_GRAPH_CHUNK_ROWS = 16;

/** ⏳️ The playground boot plan as a RESUMABLE unit of work, so a caller that owns an interactive
 * budget — the wgpu frame Worker's `plugin-graph` boot step — can hand its isolate back between chunks
 * instead of holding it for the whole graph. {@link resolvePlaygroundBoot} is this planner driven to
 * completion in one turn; there is exactly one implementation of the graph.
 *
 * 🧮️ Chunks, in order: `rows` ({@link PLUGIN_GRAPH_CHUNK_ROWS} catalog rows per step), `closure`
 * (dependency expansion), `order` (activation order). Each `step()` performs one and answers whether
 * more remains. */
export class PlaygroundBootPlanner {
  private readonly catalog: PluginCatalog;
  private readonly variant: string;
  private readonly targets: readonly PluginCatalogTarget[];
  private readonly defaultAppId: string | undefined;
  private readonly registryPluginId: string;
  private readonly hostMode: boolean;
  private readonly rows: PluginRegistryEntry[] = [];
  private readonly reused: PlaygroundBoot | undefined;
  private cursor = 0;
  private phase: "rows" | "closure" | "order" | "done" = "rows";
  private expanded: readonly PluginRegistryEntry[] = [];
  private order: readonly PluginRegistryEntry[] = [];
  private errors: readonly PluginGraphError[] = [];

  constructor(catalog: PluginCatalog, variant: string, session?: PlaygroundBootSession, registrySelection: "variant" | "all" = "variant") {
    this.catalog = catalog;
    this.variant = variant;
    this.defaultAppId = resolvePlaygroundDefaultAppId(catalog, variant);
    this.registryPluginId = resolvePluginRegistryId(catalog, variant);
    this.hostMode = registrySelection === "all" || resolvePluginHostConfig(catalog, variant) !== undefined;
    this.targets = [...catalog.plugins, ...catalog.extensions];
    if (session?.variant === variant) {
      this.reused = { variant, defaultAppId: session.defaultAppId ?? this.defaultAppId, plugins: session.plugins, dependencyErrors: [] };
      this.phase = "done";
    }
  }

  /** 🏷️ The chunk `step()` will perform next, as a boot-progress stage id. */
  stage(): string {
    return this.phase === "rows" ? `plugin-graph:rows ${Math.min(this.cursor + PLUGIN_GRAPH_CHUNK_ROWS, this.targets.length)}/${this.targets.length}` : `plugin-graph:${this.phase}`;
  }

  /** 🧮️ How far the plan is, in `[0, 1]` — a boot-progress share, not a fuel reading. */
  completion(): number {
    if (this.phase === "done") return 1;
    if (this.phase === "rows") return this.targets.length === 0 ? 0.8 : (this.cursor / this.targets.length) * 0.8;
    return this.phase === "closure" ? 0.85 : 0.95;
  }

  /** ⏭️ Performs ONE chunk and answers whether the plan needs more. Never throws: a dependency
   * fault leaves its entry out of the plan and is reported through `dependencyErrors`. */
  step(): boolean {
    if (this.phase === "rows") {
      const end = Math.min(this.cursor + PLUGIN_GRAPH_CHUNK_ROWS, this.targets.length);
      for (; this.cursor < end; this.cursor++) {
        const target = this.targets[this.cursor]!;
        this.rows.push({
          pluginId: target.pluginId,
          moduleUrl: target.role === "extension" ? this.catalog.extensionModuleUrl(target.pluginId) : this.catalog.moduleUrl(target.pluginId),
          contributes: target.contributes,
          consumes: target.consumes,
          dependencies: dependsOnToPluginDependencies(target.dependsOn),
        });
      }
      if (this.cursor >= this.targets.length) this.phase = "closure";
      return true;
    }
    if (this.phase === "closure") {
      this.expanded = expandPluginRegistry(this.rows, this.hostMode ? undefined : this.registryPluginId, this.hostMode);
      this.phase = "order";
      return true;
    }
    if (this.phase === "order") {
      const resolved = orderPluginRegistryEntries(this.expanded);
      this.order = resolved.order;
      this.errors = resolved.errors;
      this.phase = "done";
      return false;
    }
    return false;
  }

  /** 🏁️ The finished plan. Drives any remaining chunks itself, so a caller that stops slicing
   * still gets a complete plan. */
  finish(): PlaygroundBoot {
    while (this.phase !== "done") this.step();
    return this.reused ?? { variant: this.variant, defaultAppId: this.defaultAppId, plugins: this.order, dependencyErrors: this.errors };
  }
}

/** 🎮️ Resolves the wasm plugin list and default app for one playground variant; when a caller injects
 * session rows for a different variant, rebuilds from the authoritative {@link PluginCatalog}. One-turn drive of
 * {@link PlaygroundBootPlanner} for callers that own no interactive budget. */
export function resolvePlaygroundBoot(catalog: PluginCatalog, variant: string, session?: PlaygroundBootSession): PlaygroundBoot {
  return new PlaygroundBootPlanner(catalog, variant, session).finish();
}

//#region 🏠️🧳️PluginHostConfig
/** 🎯️ Resolves a playground filter/alias to its plugin's host config, or `undefined` when that program
 * doesn't offer a host-style multi-app experience. A playground row that names one `app` boots THAT app
 * standalone — the host crate's own artifact apps (`🪐️space`'s Home and Space) each have such a row, and
 * booting one of them is a single-app session, never the launcher. */
export function resolvePluginHostConfig(catalog: PluginCatalog, playgroundPluginId: string): PluginHostConfig | undefined {
  const variant = findPlaygroundVariant(catalog, playgroundPluginId);
  if (variant?.app !== undefined) return undefined;
  const registryId = variant?.pluginId ?? playgroundPluginId;
  return catalog.hosts.find((entry) => entry.pluginId === registryId);
}
//#endregion 🏠️🧳️PluginHostConfig
// #endregion 🎮️PlaygroundResolution
// #endregion 🎠️Kernel
