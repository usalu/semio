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
 * default document: the framework's own `NAVBAR_EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS`. `stdio` used to be
 * exempt there because only one of its nine shipped editor apps published an example; all nine publish
 * one now (ticket 26/09/19 `📓️stdio-examples.md`), so it is exempt nowhere. */
const EXAMPLE_PICKER_EXEMPT_PLUGIN_IDS: readonly string[] = ["demonstrator"];

/** @emoji 🕳️ The only plugins that commit no descriptor at all, so no manifest states which examples
 * their apps publish and a pane of theirs can only be checked against the example DIRECTORIES on disk —
 * plugin-wide, never per dialect. Named so that ANY OTHER plugin losing its descriptor fails this gate
 * instead of silently dropping out of it; which of these two actually has a pane is the pane catalog's
 * business, so the assertion is a subset, not an equality. */
const PLUGINS_WITHOUT_A_COMMITTED_DESCRIPTOR: readonly string[] = ["playbook"];

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
};

/** @emoji 🏷️ The navbar example picker renders `label.native.en` — authored prose, NEVER the example id
 * spelled out: wfc3d's curated `tower-stack` renders "Tower With A Cantilever" and gis2d's `demo` renders
 * "Reuse Map". A boot check that word-matches the kebab-case id against the picker's trigger text therefore
 * reports "wrong default example" for a pane booting exactly the curated one — the two false positives of
 * ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP `📓️audit-visual.md` §3, retracted with browser
 * evidence in `📓️default-example.md`. A boot check compares LABEL to LABEL, and this gate is what states
 * that every curated example HAS one, so the mapping is never guessed from the id again. */
const CURATED_EXAMPLE_LABEL_SOURCE = "label.native.en";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, repoRoot: string): Promise<void> {
  const { PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_PANES } = dependencies;
  const { describe, expect, it } = vitest;
  const { dialectCoordinate } = await import("../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts");
  const readDescriptor = pluginDescriptorReader(repoRoot);
  type Published = { readonly source: "descriptor" | "disk"; readonly appId: string | null; readonly ids: readonly string[]; readonly labels: Readonly<Record<string, string>>; readonly switches: boolean };
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
   *   renderer's React surface and this gate is a static read of committed JSON.
   * — `labels` is the text each row RENDERS in that picker: play is terminology-`native` and locale-`en`
   *   (`PLAY_LOCALE`/`PLAY_TERMINOLOGY`), so the navbar shows `label.native.en`, which is authored prose
   *   and NOT the example id spelled out. */
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
          const offered = wanted === null ? [] : (manifest.examples ?? []).filter((example: any) => example.dialect && dialectCoordinate(example.dialect) === wanted);
          const ids = [...new Set(offered.map((example: any) => example.id as string))] as string[];
          const labels = Object.fromEntries(ids.map((id) => [id, offered.find((example: any) => example.id === id)?.label?.native?.en ?? ""]));
          const declares = (actions: readonly any[] | undefined) => (actions ?? []).some((action: any) => action.id === "setActiveExample");
          const switches = ((app?.windowKinds ?? []) as any[]).some((kind) => declares(kind.actions)) || declares(app?.actions);
          return { source: "descriptor", appId: app?.id ?? null, ids, labels, switches };
        })()
      : { source: "disk", appId: row.app ?? null, ids: diskExampleIds(repoRoot, pluginRoot), labels: {}, switches: declaresSetActiveExampleInRust(repoRoot, pluginRoot) };
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

    it(`renders every curated example under the ${CURATED_EXAMPLE_LABEL_SOURCE} a boot check reads instead of the id`, () => {
      const unlabelled: string[] = [];
      PLAY_RUNTIME_PANES.forEach((pane: any, index: number) => {
        if (pane.example === undefined) return;
        const published = publishedExamples(index);
        if (published.source !== "descriptor" || !published.ids.includes(pane.example)) return;
        if (!published.labels[pane.example]) unlabelled.push(`${pane.variant}: "${pane.example}" publishes no ${CURATED_EXAMPLE_LABEL_SOURCE}, so its navbar picker would boot on the placeholder`);
      });
      expect(unlabelled).toEqual([]);
    });

    /** @emoji 🏷️ The acceptance suite asserts the pane's navbar trigger renders exactly the catalog's
     * `exampleLabel` (`🧪️tests/🎭️acceptance/🟦️.ts`), which only means something while that text is the
     * descriptor's own `label.native.en`. Pinned here rather than resolved in the browser test because a
     * Playwright worker reading 31 plugin descriptors — puzzle's alone is 4.8 MB — to learn one string per
     * pane would spend more time parsing JSON than booting shells. A pane whose descriptor is stale or
     * missing (the nine `stdio` panes while their descriptor predates `📓️stdio-examples.md`) is checked by
     * the presence law below and joins this one the moment its descriptor is regenerated. */
    it(`pins each curated example's rendered ${CURATED_EXAMPLE_LABEL_SOURCE} into the catalog as exampleLabel`, () => {
      const wrong: string[] = [];
      PLAY_RUNTIME_PANES.forEach((pane: any, index: number) => {
        if (pane.example === undefined) return;
        const published = publishedExamples(index);
        const label = published.labels[pane.example];
        if (published.source !== "descriptor" || !label) return;
        if (pane.exampleLabel !== label) wrong.push(`${pane.variant}: exampleLabel ${JSON.stringify(pane.exampleLabel)} ≠ ${JSON.stringify(label)}`);
      });
      expect(wrong).toEqual([]);
    });

    it("gives every curated example a rendered label the acceptance run can read, and gives no other pane one", () => {
      expect(PLAY_RUNTIME_PANES.filter((pane: any) => (pane.example === undefined) !== (pane.exampleLabel === undefined)).map((pane: any) => pane.variant)).toEqual([]);
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
