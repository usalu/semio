import { readdirSync, readFileSync, existsSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

/** @emoji 🗂️ Editor apps every plugin descriptor declares, keyed by the owning plugin id. */
function descriptorEditorApps(repoRoot: string): ReadonlyMap<string, readonly string[]> {
  const pluginsRoot = join(repoRoot, "✏️s/🔌️plugins"), apps = new Map<string, readonly string[]>();
  for (const directory of readdirSync(pluginsRoot)) {
    const file = join(pluginsRoot, directory, "🔣️.json");
    if (!existsSync(file)) continue;
    const manifest = JSON.parse(readFileSync(file, "utf8")).manifest;
    apps.set(manifest.pluginId, (manifest.apps ?? []).filter((app: any) => app.role === "editor").map((app: any) => app.id));
  }
  return apps;
}

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, repoRoot: string): Promise<void> {
  const { PLAYGROUND_BUILD_TARGETS, PLAY_RUNTIME_PANES, PLAY_RUNTIME_TARGETS, PLAY_HOST_VARIANT, playExpectedVariants, playRuntimeComponentIds, isIconName } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayPaneCoverageTests
  describe("play pane coverage", () => {
    it("lists every playground app exactly once", () => {
      const panes = PLAY_RUNTIME_PANES.map((pane: any) => pane.variant);
      expect(new Set(panes).size).toBe(panes.length);
      expect([...panes].sort()).toEqual([...playExpectedVariants(PLAYGROUND_BUILD_TARGETS)].sort());
    });

    it("reaches every editor app a plugin descriptor declares, except the host shell's own", () => {
      const host = PLAYGROUND_BUILD_TARGETS.find((row: any) => row.variant === PLAY_HOST_VARIANT).pluginId;
      const reachable = new Set<string>();
      const apps = descriptorEditorApps(repoRoot);
      for (const row of PLAY_RUNTIME_TARGETS) {
        if (row.app) reachable.add(row.app);
        else for (const id of apps.get(row.pluginId) ?? []) reachable.add(id);
      }
      const unreachable = [...apps].filter(([pluginId]) => pluginId !== host).flatMap(([, ids]) => ids).filter(id => !reachable.has(id));
      expect(unreachable).toEqual([]);
    });

    it("never needs the unlinkable stdio component", () => {
      expect(playRuntimeComponentIds()).not.toContain("stdio");
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
