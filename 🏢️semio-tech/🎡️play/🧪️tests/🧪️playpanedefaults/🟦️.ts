import { existsSync, readdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";

/** @emoji 🛂️ The committed descriptor of the plugin that owns `cratePath`, or `null` when the plugin ships
 * none. Read once per plugin: a descriptor carries every example's whole `artifactJson`, so puzzle's
 * alone is 4.8 MB and the 58 panes share 31 plugins. */
function pluginDescriptorReader(repoRoot: string): (cratePath: string) => any | null {
  const parsed = new Map<string, any | null>();
  return (cratePath) => {
    const file = join(repoRoot, dirname(dirname(cratePath)), "🔣️.json");
    if (!parsed.has(file)) parsed.set(file, existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : null);
    return parsed.get(file)!;
  };
}

/** @emoji 📚️ Plugins whose examples never reach a navbar picker, so a pane of theirs boots its app's own
 * default document: the framework's own `NAVBAR_EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS`, minus `stdio` — that
 * one is exempt there because only one of its 18 shipped document apps publishes an example, while play
 * gives each of those apps its OWN pane, so the single publishing app (`s.stdio.md@commonmark/*#editor`)
 * is the only stdio pane that can name a curated example and the other eight name none. */
const EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS: readonly string[] = ["demonstrator", "flow", "norm"];

/** @emoji 🕳️ The only plugins that commit no descriptor at all, so no manifest states which examples
 * their apps publish and a pane of theirs can only be checked against the example DIRECTORIES on disk —
 * plugin-wide, never per dialect. Named so that ANY OTHER plugin losing its descriptor fails this gate
 * instead of silently dropping out of it; which of these two actually has a pane is the pane catalog's
 * business, so the assertion is a subset, not an equality. */
const PLUGINS_WITHOUT_A_COMMITTED_DESCRIPTOR: readonly string[] = ["playbook", "stdio"];

/** @emoji 📚️ Every authored example id under a plugin, read from the `📚️examples/<emoji-id>/🟦️.ts`
 * directories its artifacts own — the fallback source for a plugin that commits no descriptor.
 * A `📚️examples` node under a SURFACE (`✏️editor`, `👁️viewer`) holds recorded interaction sessions,
 * not documents: the subset declaration's `examples()` only ever names the nodes directly under the
 * subset, so the walk stops at a surface and this fallback states exactly what the manifest would. */
function diskExampleIds(repoRoot: string, pluginRoot: string): readonly string[] {
  const found = new Set<string>();
  const walk = (dir: string, depth: number): void => {
    if (depth > 6 || !existsSync(dir)) return;
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (!entry.isDirectory()) continue;
      if (entry.name === "📚️examples") {
        for (const example of readdirSync(join(dir, entry.name), { withFileTypes: true })) {
          if (example.isDirectory() && existsSync(join(dir, entry.name, example.name, "🟦️.ts"))) found.add(example.name.replace(/^[^a-z0-9]+/i, ""));
        }
        continue;
      }
      if (entry.name === "✏️editor" || entry.name === "👁️viewer" || entry.name === "🧪️tests" || entry.name === "🧫️fixtures" || entry.name === "📦️packages" || entry.name === "node_modules") continue;
      walk(join(dir, entry.name), depth + 1);
    }
  };
  walk(join(repoRoot, pluginRoot, "🗿️artifacts"), 0);
  return [...found];
}

/** @emoji 🦀️ Whether a plugin's editor Rust declares the navbar picker's action at all — the only
 * source left for a plugin that commits no descriptor, read from the handwritten
 * `ActionDefinition::new("setActiveExample", …)` every declaring editor spells out verbatim. */
function declaresSetActiveExampleInRust(repoRoot: string, pluginRoot: string): boolean {
  const walk = (dir: string, depth: number): boolean => {
    if (depth > 8 || !existsSync(dir)) return false;
    for (const entry of readdirSync(dir, { withFileTypes: true })) {
      if (entry.isFile() && entry.name === "🦀️.rs" && readFileSync(join(dir, entry.name), "utf8").includes('ActionDefinition::new("setActiveExample"')) return true;
      if (entry.isDirectory() && entry.name !== "🧪️tests" && entry.name !== "🧫️fixtures" && entry.name !== "📦️packages" && entry.name !== "node_modules" && walk(join(dir, entry.name), depth + 1)) return true;
    }
    return false;
  };
  return walk(join(repoRoot, pluginRoot, "🗿️artifacts"), 0);
}

/** @emoji 🚧️ Panes whose boot app publishes examples but declares no `setActiveExample`, so
 * `appSwitchesExamples` gates `exampleOptions` to `[]`, the navbar picker is hidden and
 * `resolveBootExampleId` never announces a boot example: the pane opens the app's genesis document
 * and the curated `example` stays INERT until the plugin declares the action. Each entry names the
 * plugin that owns the defect; the evidence per pane (and the six declaration sites one fix needs)
 * is in ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP `🗑️generated/play-defaults/EMPTY-PANES.md`.
 * Asserted as an UPPER BOUND, never an equality, so a plugin declaring the action shrinks the set
 * without breaking this gate — while a pane newly losing it fails here. */
const PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES: Readonly<Record<string, string>> = {
  generation2d: "procedural — generation2d editor declares no setActiveExample",
  lowpoly: "lowpoly — lowpoly editor declares no setActiveExample",
  layout: "layout — layout editor declares no setActiveExample",
  gis3d: "gis — gisterrain editor declares no setActiveExample",
  mathematical: "mathematical — equation editor declares no setActiveExample",
  sequence: "sequence — sequence editor declares no setActiveExample",
  vcs: "vcs — vcs editor declares no setActiveExample",
  home: "space — home editor declares no setActiveExample",
  space: "space — space editor declares no setActiveExample",
  architect: "architect — committed descriptor predates the action its editor Rust already declares",
  dag: "dag — committed descriptor predates the action its editor Rust already declares",
  imperative: "imperative — committed descriptor predates the action its editor Rust already declares",
  "trinity-rewriting": "trinity — committed descriptor predates the action its editor Rust already declares",
  stdio: "stdio — md editor declares no setActiveExample",
};

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, repoRoot: string): Promise<void> {
  const { PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_PANES } = dependencies;
  const { describe, expect, it } = vitest;
  const { dialectCoordinate } = await import("../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts");
  const readDescriptor = pluginDescriptorReader(repoRoot);
  type Published = { readonly source: "descriptor" | "disk"; readonly appId: string | null; readonly ids: readonly string[]; readonly switches: boolean };
  const resolved = new Map<number, Published>();

  /** @emoji 📚️ Exactly what the pane's navbar example picker would offer, restated against the ONE
   * coordinate function both sides share (`dialectCoordinate`) so this gate needs neither a wasm boot
   * nor the renderer's module graph:
   * — the app the shell boots is `resolveBootPrimaryAppV1(apps, undefined, row.app, undefined)`, which with
   *   no env pin (`VITE_SEMIO_APP_ID`) and no `appRole` is the registry `app` column, else `apps[0]`;
   * — the rows it offers are `examplesForApp` = `examplesForDialect(examples, app.dialect)`: manifest
   *   order, same dialect coordinate, deduplicated by id;
   * — it offers them at all only when `appSwitchesExamples(app.id, app.windowKinds, app.actions)` holds,
   *   i.e. `undeclaredActionDiagnostic` finds `setActiveExample` on a window kind or on the app itself
   *   (`🛠️ShellHelpers/🟦️.tsx`) — restated here rather than imported, because that module is the
   *   renderer's React surface and this gate is a static read of committed JSON. */
  const publishedExamples = (index: number): Published => {
    if (resolved.has(index)) return resolved.get(index)!;
    const row = PLAY_RUNTIME_TARGETS[index];
    const pluginRoot = dirname(dirname(row.cratePath));
    const manifest = readDescriptor(row.cratePath)?.manifest;
    const answer: Published = manifest
      ? (() => {
          const apps: any[] = manifest.apps ?? [];
          const app = apps.find((candidate) => candidate.id === row.app) ?? apps[0];
          const wanted = app?.dialect ? dialectCoordinate(app.dialect) : null;
          const ids = wanted === null ? [] : [...new Set((manifest.examples ?? []).filter((example: any) => example.dialect && dialectCoordinate(example.dialect) === wanted).map((example: any) => example.id as string))];
          const declares = (actions: readonly any[] | undefined) => (actions ?? []).some((action: any) => action.id === "setActiveExample");
          const switches = ((app?.windowKinds ?? []) as any[]).some((kind) => declares(kind.actions)) || declares(app?.actions);
          return { source: "descriptor", appId: app?.id ?? null, ids, switches };
        })()
      : { source: "disk", appId: row.app ?? null, ids: diskExampleIds(repoRoot, pluginRoot), switches: declaresSetActiveExampleInRust(repoRoot, pluginRoot) };
    resolved.set(index, answer);
    return answer;
  };

  //#region 🧪️PlayPaneDefaultsTests
  describe("play pane example defaults", () => {
    it("names only examples the pane's own app publishes", () => {
      const wrong: string[] = [];
      PLAY_RUNTIME_PANES.forEach((pane: any, index: number) => {
        if (pane.example === undefined) return;
        const published = publishedExamples(index);
        if (!published.ids.includes(pane.example)) wrong.push(`${pane.variant}: "${pane.example}" is not published by ${published.appId ?? "its app"} (${published.source}: ${published.ids.join(", ") || "none"})`);
      });
      expect(wrong).toEqual([]);
    });

    it("curates an example for every pane whose app publishes one", () => {
      const empty: string[] = [];
      PLAY_RUNTIME_PANES.forEach((pane: any, index: number) => {
        const published = publishedExamples(index);
        if (published.source !== "descriptor" || published.ids.length === 0 || pane.example !== undefined) return;
        empty.push(`${pane.variant}: would boot "${published.ids[0]}" by accident`);
      });
      expect(empty).toEqual([]);
    });

    it("checks against a manifest for every plugin that commits one, and never lets a picker-exempt pane claim an example", () => {
      const descriptorless: string[] = [];
      PLAY_RUNTIME_PANES.forEach((_: unknown, index: number) => {
        if (publishedExamples(index).source === "disk") descriptorless.push(PLAY_RUNTIME_TARGETS[index].pluginId);
      });
      expect(descriptorless.filter((pluginId) => !PLUGINS_WITHOUT_A_COMMITTED_DESCRIPTOR.includes(pluginId))).toEqual([]);
      const exempt = PLAY_RUNTIME_PANES.filter((_: unknown, index: number) => EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS.includes(PLAY_RUNTIME_TARGETS[index].pluginId));
      expect(exempt.filter((pane: any) => pane.example !== undefined).map((pane: any) => pane.variant)).toEqual([]);
    });

    it("names every pane whose curated example cannot reach its app, with the plugin that owes the fix", () => {
      const inert: string[] = [];
      PLAY_RUNTIME_PANES.forEach((pane: any, index: number) => {
        const published = publishedExamples(index);
        if (published.ids.length === 0 || published.switches) return;
        inert.push(pane.variant);
      });
      expect(inert.filter((variant) => PANES_WHOSE_APP_CANNOT_SWITCH_EXAMPLES[variant] === undefined)).toEqual([]);
    });

    it("carries the curated example into the brand as a boot default, never as a lock", () => {
      for (const pane of PLAY_PANES) {
        expect(pane.brand.defaults?.exampleId).toBe(pane.example);
        expect(pane.brand.locks?.exampleId).toBeUndefined();
      }
      expect(PLAY_PANES.filter((pane: any) => pane.brand.defaults !== undefined).length).toBe(PLAY_RUNTIME_PANES.filter((pane: any) => pane.example !== undefined).length);
    });
  });
  //#endregion 🧪️PlayPaneDefaultsTests
}
