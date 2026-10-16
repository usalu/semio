/** 🔧️ TypeScript parity of the opening and merge-policy config mutations and the Nx inputs that track their sources. */
import type { BackboneWorkerTestDependencies } from "../../../🔨️modules/🏪️store/👷️worker/🟦️.ts";
import { workerLawFakesV1 } from "../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";
import type { TestSource } from "../../../🧪️tests/🧰️worker-law-fakes/🟦️.ts";

type Vitest = NonNullable<ImportMeta["vitest"]>;

export async function registerConfigMutationParityTests(vitest: Vitest, dependencies: BackboneWorkerTestDependencies, source: TestSource): Promise<void> {
  const { beforeEach, describe, expect, it } = vitest;
  const { resetWorkerLaw } = workerLawFakesV1(vitest, dependencies, source);
  //#region 🔖️ConfigMutationTests
  describe("config mutation TypeScript parity", () => {
    beforeEach(resetWorkerLaw);
    it("opening mutations replace one coordinate, preserve siblings, and invert exactly", async () => {
      const { applyOpeningConfigMutation, clearDefaultApp, inverseOpeningConfigMutation, setDefaultApp } = await import("@semio-tech/framework/config/mutations");
      const dialect = { artifactKind: "s.demo.demo", standard: "1", subset: "*" };
      const viewer = { pluginId: "demo", appId: "viewer" };
      const editor = { pluginId: "demo", appId: "editor" };
      const replacement = { pluginId: "draft", appId: "drafting" };
      const base = {
        defaults: [
          { dialect, role: "viewer" as const, app: viewer },
          { dialect, role: "editor" as const, app: editor },
        ],
      };
      const set = setDefaultApp(dialect, "editor", replacement);
      const afterSet = applyOpeningConfigMutation(base, set);
      expect(afterSet).toEqual({
        defaults: [
          { dialect, role: "viewer", app: viewer },
          { dialect, role: "editor", app: replacement },
        ],
      });
      expect(inverseOpeningConfigMutation(set, base)).toEqual([setDefaultApp(dialect, "editor", editor)]);
      const clear = clearDefaultApp(dialect, "editor");
      expect(applyOpeningConfigMutation(base, clear)).toEqual({ defaults: [{ dialect, role: "viewer", app: viewer }] });
      expect(inverseOpeningConfigMutation(clear, base)).toEqual([setDefaultApp(dialect, "editor", editor)]);
      expect(inverseOpeningConfigMutation(clearDefaultApp(dialect, "editor"), { defaults: [] })).toEqual([]);
    });

    it("change-merge-policy applies and inverts the prior whole-record setting", async () => {
      const { applyMergePolicyConfigMutation, changeMergePolicy, inverseMergePolicyConfigMutation } = await import("@semio-tech/framework/config/mutations");
      const mutation = changeMergePolicy("Vigilant");
      expect(applyMergePolicyConfigMutation({ policy: "Normal" }, mutation)).toEqual({ policy: "Vigilant" });
      expect(inverseMergePolicyConfigMutation(mutation, { policy: "Normal" })).toEqual([changeMergePolicy("Normal")]);
    });

    it("Nx project inputs track every external OS config and plugin-host source compiled by the targets", async () => {
      const { readFile } = await import("node:fs/promises");
      const tsProject = JSON.parse(await readFile(new URL("./📦️packages/🟦️typescript/📋️project.json", source.url), "utf8")) as { namedInputs: { default: string[] } };
      const hostProject = JSON.parse(await readFile(new URL("./🖥️host/📦️packages/🦀️rust/📋️project.json", source.url), "utf8")) as { namedInputs: { default: string[] } };
      expect(tsProject.namedInputs.default).toContain("{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*");
      expect(hostProject.namedInputs.default).toEqual(
        expect.arrayContaining(["{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/**/*.rs", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*.rs", "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🎚️config/**/*.json"]),
      );
    });
  });
  //#endregion 🔖️ConfigMutationTests
}
