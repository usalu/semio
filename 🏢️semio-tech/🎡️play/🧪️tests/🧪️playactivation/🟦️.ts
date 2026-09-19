import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runtimeComponentClosure } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, repoRoot: string): Promise<void> {
  const { playActivationLanes, playRuntimeComponentIds, mergePlayActivationReceipts } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayActivationTests
  const sha = (seed: string): string => seed.repeat(64).slice(0, 64);
  const lane = (name: string, plugins: readonly (readonly [string, string, number])[]) => ({
    lane: name,
    receipt: { schema: "semio.dev.activation/v1", variant: name, profile: "dev", plugins: plugins.map(([pluginId, seed, rebuiltAt]) => ({ pluginId, artifactSha256: sha(seed), rebuiltAt })) },
  });

  describe("playActivationLanes", () => {
    it("covers exactly the play union with pane lanes only", () => {
      const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
      const covered = new Set<string>();
      for (const variant of playActivationLanes()) for (const id of runtimeComponentClosure(components, [PLAYGROUND_BUILD_TARGETS.find(row => row.variant === variant)!.pluginId])) covered.add(id);
      expect([...covered].sort()).toEqual([...playRuntimeComponentIds()].sort());
    });

    it("never picks a lane that would need stdio", () => {
      expect(playActivationLanes()).not.toContain("s");
    });

    it("is exactly what the Nx graph activates and prepares", () => {
      const project = JSON.parse(readFileSync(join(repoRoot, "🏢️semio-tech/🎡️play/📋️project.json"), "utf8"));
      const lanes = [...playActivationLanes()].sort();
      const from = (target: string, pattern: RegExp) => project.targets[target].dependsOn.flatMap((entry: string) => pattern.exec(entry)?.[1] ?? []).sort();
      expect(from("activate-dev", /^@semio-tech\/framework-os-dev:activate-(.+)-react-dev$/)).toEqual(lanes);
      expect(from("prepare-dev", /^@semio-tech\/framework-os-dev:prepare-(.+)-react-dev$/)).toEqual(lanes);
      expect(from("prepare-release", /^@semio-tech\/framework-os-dev:prepare-(.+)-react-release$/)).toEqual(lanes);
    });
  });

  describe("mergePlayActivationReceipts", () => {
    it("unions every lane into one receipt named after the primary lane", () => {
      const merged = mergePlayActivationReceipts([lane("demonstrator", [["cad", "b", 7], ["demonstrator", "c", 7]]), lane("energy", [["energy", "d", 9]])], ["cad", "demonstrator", "energy"], "demonstrator");
      expect(merged.variant).toBe("demonstrator");
      expect(merged.plugins.map((row: any) => row.pluginId)).toEqual(["cad", "demonstrator", "energy"]);
    });

    it("keeps the earliest rebuiltAt when lanes agree", () => {
      const merged = mergePlayActivationReceipts([lane("draw", [["flow", "b", 12]]), lane("flow", [["flow", "b", 3]])], ["flow"], "draw");
      expect(merged.plugins[0].rebuiltAt).toBe(3);
    });

    it("refuses lanes that disagree about one artifact", () => {
      expect(() => mergePlayActivationReceipts([lane("draw", [["flow", "b", 7]]), lane("flow", [["flow", "f", 7]])], ["flow"], "draw")).toThrow(/Stale play activation lane: flow/);
    });

    it("refuses a union that misses or exceeds the expected components", () => {
      expect(() => mergePlayActivationReceipts([lane("cad", [["cad", "c", 7], ["vcs", "a", 7]])], ["cad", "fem"], "cad")).toThrow(/missing: fem; extra: vcs/);
    });

    it("refuses a lane directory holding another variant's receipt", () => {
      expect(() => mergePlayActivationReceipts([{ ...lane("energy", [["energy", "d", 7]]), lane: "fem3d" }], ["energy"], "fem3d")).toThrow(/lane fem3d carries a energy receipt/);
    });

    it("refuses a release receipt", () => {
      const release = lane("energy", [["energy", "d", 7]]);
      expect(() => mergePlayActivationReceipts([{ ...release, receipt: { ...release.receipt, profile: "release" } }], ["energy"], "energy")).toThrow(/not a dev activation: release/);
    });
  });
  //#endregion 🧪️PlayActivationTests
}
