import { readdirSync, readFileSync, existsSync, statSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** @emoji 📂️ Repository-relative root every plugin directory lives directly below. */
const PLUGINS_ROOT = "✏️s/🔌️plugins";

/** @emoji 🗄️ What stdio's SHIPPED component assembles: every artifact crate whose apps
 * `component-app-assembly` turns on. Its `full-app-catalog` sibling closes the same `StdioApps` enum
 * over all 88 subsets and costs ≈600 000 wasm functions, which `wasm-component-ld` refuses over
 * wasmparser's 1 000 000-function ceiling — so the shipped fleet must stay this bounded set, and
 * `default`/`plugin-root` must never reach `full-app-catalog`. */
const STDIO_COMPONENT_APP_CRATES: readonly string[] = ["csv", "html", "json", "md", "tsv", "txt", "xml"];

/** @emoji 📂️ Every plugin directory name, in on-disk order. */
function pluginDirectories(repoRoot: string): readonly string[] {
  return readdirSync(join(repoRoot, PLUGINS_ROOT)).filter(name => statSync(join(repoRoot, PLUGINS_ROOT, name)).isDirectory());
}

/** @emoji 📂️ The plugin directory a registry row's crate lives under. */
function pluginDirectoryOfCratePath(cratePath: string): string | undefined {
  const parts = cratePath.split("/");
  return parts[0] === "✏️s" && parts[1] === "🔌️plugins" ? parts[2] : undefined;
}

/** @emoji 🗂️ The apps every plugin descriptor declares, BOTH roles, keyed by the plugin DIRECTORY that
 * owns it — `undefined` for a directory that commits no descriptor, which the caller must then account
 * for explicitly. Reading by directory (never by whichever descriptors happen to exist) is what keeps a
 * plugin from disappearing out of these gates the moment it stops shipping one. */
function descriptorAppsByDirectory(repoRoot: string): ReadonlyMap<string, { readonly pluginId: string; readonly apps: readonly any[] } | undefined> {
  const byDirectory = new Map<string, { readonly pluginId: string; readonly apps: readonly any[] } | undefined>();
  for (const directory of pluginDirectories(repoRoot)) {
    const file = join(repoRoot, PLUGINS_ROOT, directory, "🔣️.json");
    if (!existsSync(file)) { byDirectory.set(directory, undefined); continue; }
    const manifest = JSON.parse(readFileSync(file, "utf8")).manifest;
    byDirectory.set(directory, { pluginId: manifest.pluginId, apps: manifest.apps ?? [] });
  }
  return byDirectory;
}

/** @emoji 🎛️ Every prop play's grid hands one pane's `FrameworkOsShell`. Pinned as a SET because the
 * navbar's editor⇄viewer role group is suppressed from the mount side, never from the catalog: an app
 * pin (`appRole`, a role-bearing `appId` lock), a chrome suppression or a filter that hides the pane's
 * own plugin would each make `…#viewer` unreachable while every other law here stays green. A new prop
 * therefore has to be added here deliberately, with the viewer law below re-run. */
const PLAY_PANE_SHELL_PROPS: readonly string[] = ["appId", "brand", "defaults", "locks", "pluginFilter", "plugins", "shellId", "storageNamespace", "suppressAutoIntroduction", "surfaceSessionFactories"];

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, repoRoot: string): Promise<void> {
  const { PLAYGROUND_BUILD_TARGETS, PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_HOST_VARIANT, playExpectedVariants, playRuntimeComponentIds, isIconName } = dependencies;
  const { describe, expect, it } = vitest;
  const { PLUGIN_BUILD_TARGETS, EXTENSION_TARGETS, PLUGIN_HOST_CONFIGS } = await import("../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts");
  const { dialectCoordinate } = await import("../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts");
  const appsByDirectory = descriptorAppsByDirectory(repoRoot);
  const editorAppIds = (directory: string | undefined): readonly string[] => (appsByDirectory.get(directory!)?.apps ?? []).filter((app: any) => app.role === "editor").map((app: any) => app.id as string);
  const paneDirectory = (pluginId: string): string => pluginDirectoryOfCratePath(PLUGIN_BUILD_TARGETS.find(entry => entry.pluginId === pluginId)!.cratePath)!;

  /** @emoji 🏠️ The ONE app that hosts other apps rather than being one — resolved the way the shell
   * itself resolves it (`resolveRequiredHostApps`: an editor app whose artifact-kind tail is the host
   * crate's declared `hostAppId`), so play never guesses a literal. Its siblings Home and Space are
   * ordinary artifact apps and each carry their own pane. */
  const hostShellAppIds = (): readonly string[] => {
    const ids: string[] = [];
    for (const host of PLUGIN_HOST_CONFIGS) {
      const row = PLUGIN_BUILD_TARGETS.find(entry => entry.pluginId === host.pluginId)!;
      const directory = pluginDirectoryOfCratePath(row.cratePath)!;
      const descriptor = JSON.parse(readFileSync(join(repoRoot, PLUGINS_ROOT, directory, "🔣️.json"), "utf8")).manifest;
      const matches = (descriptor.apps ?? []).filter((app: any) => app.role === "editor" && app.dialect?.artifactKind.split(".").at(-1) === host.hostAppId);
      if (matches.length !== 1) throw new Error(`host app "${host.hostAppId}" of ${host.pluginId} resolved to ${matches.length} editor apps`);
      ids.push(matches[0].id);
    }
    return ids;
  };

  //#region 🧪️PlayPaneCoverageTests
  describe("play pane coverage", () => {
    it("lists every playground app exactly once", () => {
      const panes = PLAY_RUNTIME_PANES.map((pane: any) => pane.variant);
      expect(new Set(panes).size).toBe(panes.length);
      expect([...panes].sort()).toEqual([...playExpectedVariants(PLAYGROUND_BUILD_TARGETS)].sort());
    });

    /** @emoji 🧩️ Play shows ALL plugins: a plugin directory with no pane is a coverage regression, never
     * a configuration choice, so this law carries no exemption list to hide one behind. */
    it("shows every plugin directory in at least one pane", () => {
      const byPluginId = new Map(PLUGIN_BUILD_TARGETS.map(row => [row.pluginId, row]));
      const shown = new Set(PLAY_RUNTIME_TARGETS.map((row: any) => pluginDirectoryOfCratePath(byPluginId.get(row.pluginId)!.cratePath)));
      expect(pluginDirectories(repoRoot).filter(directory => !shown.has(directory)).sort()).toEqual([]);
    });

    it("reaches every editor app a plugin descriptor declares, except the host shell's own", () => {
      const exempt = new Set(hostShellAppIds());
      const reachable = new Set<string>();
      for (const row of PLAY_RUNTIME_TARGETS) {
        if (row.app) reachable.add(row.app);
        else for (const id of editorAppIds(paneDirectory(row.pluginId))) reachable.add(id);
      }
      const unreachable = [...appsByDirectory.values()].flatMap(entry => (entry?.apps ?? []).filter((app: any) => app.role === "editor").map((app: any) => app.id as string)).filter(id => !exempt.has(id) && !reachable.has(id));
      expect(unreachable).toEqual([]);
    });

    /** @emoji 👁️ …and every VIEWER app, without a second pane for it, through the navbar's editor⇄viewer
     * role group. Restated against the ONE coordinate function both sides share (`dialectCoordinate`),
     * exactly as the sibling example gate restates `appSwitchesExamples` — importing
     * `🏛️ShellHost/🔀️surface-switch/🟦️.ts` pulls the `@semio-tech/framework` barrel through Vite and
     * blows this suite's 15 s budget, and this gate is a static read of committed JSON:
     * — the app a pane opens is `resolveBootPrimaryAppV1(apps, undefined, row.app, undefined)`, which
     *   with no `VITE_SEMIO_APP_ID` pin and no `appRole` (play states neither) is the registry `app`
     *   column, else `apps[0]`;
     * — the role group renders exactly when `surfaceRoleAppsV1` finds an editor AND a viewer at that
     *   app's dialect coordinate (`🏛️ShellHost/🟦️.tsx` `sessionRoleApps`).
     * A pane whose boot editor declares no viewer sibling is a dead end; a viewer no pane's dialect
     * reaches is an app play never shows. */
    it("reaches every viewer app through its pane's editor⇄viewer switch", () => {
      const viewerOf = (apps: readonly any[], app: any): any => {
        if (app?.dialect === undefined) return undefined;
        const coordinate = dialectCoordinate(app.dialect);
        return apps.find((candidate: any) => candidate.role === "viewer" && candidate.dialect !== undefined && dialectCoordinate(candidate.dialect) === coordinate);
      };
      const deadEnds: string[] = [], reachable = new Set<string>();
      for (const row of PLAY_RUNTIME_TARGETS) {
        const apps = appsByDirectory.get(paneDirectory(row.pluginId))?.apps;
        if (apps === undefined) continue;
        const app = apps.find((candidate: any) => candidate.id === row.app) ?? apps[0];
        const viewer = viewerOf(apps, app);
        if (viewer === undefined) deadEnds.push(`${row.variant}: ${app?.id ?? "no app"} has no viewer sibling`);
        else reachable.add(viewer.id);
      }
      expect(deadEnds).toEqual([]);
      const declared = PLAY_RUNTIME_TARGETS.flatMap((row: any) => (appsByDirectory.get(paneDirectory(row.pluginId))?.apps ?? []).filter((app: any) => app.role === "viewer").map((app: any) => app.id as string));
      expect([...new Set(declared)].filter(id => !reachable.has(id)).sort()).toEqual([]);
    });

    /** @emoji 🎛️ …and play's own mount never suppresses that switch. See {@link PLAY_PANE_SHELL_PROPS}. */
    it("mounts every pane with exactly the shell props the grid states", () => {
      const source = readFileSync(join(repoRoot, "🏢️semio-tech/🎡️play/🟦️.tsx"), "utf8");
      const mounts = [...source.matchAll(/<FrameworkOsShell\b([^>]*?)\/>/gs)];
      expect(mounts.length).toBe(1);
      expect([...new Set([...mounts[0]![1]!.matchAll(/(\w+)=/g)].map(match => match[1]!))].sort()).toEqual([...PLAY_PANE_SHELL_PROPS].sort());
    });

    /** @emoji 🛂️ The law above reads apps from committed descriptors, so a plugin that commits none
     * would pass it by being invisible to it. Every plugin directory therefore MUST commit its
     * `🔣️.json` (`bun nx run <plugin>:describe` emits it beside `🛂️.descriptor.semio`): a missing
     * descriptor is a failure here, not an exemption. */
    it("reads a committed descriptor for every plugin directory", () => {
      expect([...appsByDirectory].filter(([, entry]) => entry === undefined).map(([directory]) => directory).sort()).toEqual([]);
    });

    /** @emoji 🧩️ Play covers ALL components: every plugin AND every extension the registry knows is in
     * the union some pane's closure activates — an extension whose plugin declares no `consumes` for its
     * `contributes` capability lands in no closure at all and fails right here. */
    it("activates every registry component the panes need", () => {
      const union = new Set(playRuntimeComponentIds());
      expect([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS].map(row => row.pluginId).filter(id => !union.has(id)).sort()).toEqual([]);
    });

    it("keeps stdio's component bounded to the app fleet it can link", () => {
      expect(PLAYGROUND_BUILD_TARGETS.find((row: any) => row.variant === "stdio").app).toBe("s.stdio.md@commonmark/*#editor");
      const cargo = readFileSync(join(repoRoot, PLUGINS_ROOT, "🗄️stdio/📦️packages/🦀️rust/Cargo.toml"), "utf8");
      const features = Object.fromEntries([...cargo.matchAll(/^([a-z0-9-]+) = \[(.*)\]$/gm)].map(row => [row[1]!, [...row[2]!.matchAll(/"([^"]+)"/g)].map(item => item[1]!)]));
      const closure = new Set<string>(), pending = ["default"];
      while (pending.length) {
        const next = pending.pop()!;
        if (closure.has(next)) continue;
        closure.add(next);
        for (const activation of features[next] ?? []) if (features[activation]) pending.push(activation);
      }
      expect([...closure].filter(name => name === "full-app-catalog")).toEqual([]);
      expect([...closure].flatMap(name => features[name]!).flatMap(item => [...item.matchAll(/^semio-s-artifact-stdio-([a-z0-9]+)\/component-app-assembly$/g)].map(match => match[1]!)).sort()).toEqual([...STDIO_COMPONENT_APP_CRATES].sort());
    });

    it("keeps the host shell variant out of the grid", () => {
      expect(PLAY_RUNTIME_PANES.map((pane: any) => pane.variant)).not.toContain(PLAY_HOST_VARIANT);
    });

    it("satisfies its own schema under an independent validator", () => {
      const runtime = join(repoRoot, "🏢️semio-tech/🎡️play/🔨️modules/🧩️runtime");
      const Ajv = createRequire(import.meta.url)("ajv");
      const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(join(runtime, "🧬️schema/🔣️.json"), "utf8")));
      const valid = validate(JSON.parse(readFileSync(join(runtime, "🔣️.json"), "utf8")));
      expect(validate.errors ?? []).toEqual([]);
      expect(valid).toBe(true);
    });

    it("uses only registered icons", () => {
      expect(PLAY_RUNTIME_PANES.filter((pane: any) => !isIconName(pane.icon)).map((pane: any) => pane.icon)).toEqual([]);
    });
  });
  //#endregion 🧪️PlayPaneCoverageTests
}
