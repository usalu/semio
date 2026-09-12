import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { cargoProfileDir, getWorkspaceRoot, selectComponentWasmProfile } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { cargoTargetDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { publicationWasmPath } from "../../🛂️descriptor-verification/🟦️.ts";
import { pluginWasmArtifactPath } from "../../../🖨️describe/🏗️component-build/🟦️.ts";

type GeneratorContract = { readonly previewTarget?: string };
type LaunchEntry = {
  readonly name?: string;
  readonly type?: string;
  readonly request?: string;
  readonly command?: string;
  readonly cwd?: string;
  readonly presentation?: { readonly group?: string; readonly order?: number };
};

describe("plugin registry generated preview launchers", () => {
  it("exposes every owned generator preview exactly once in contract order", () => {
    const repoRoot = getWorkspaceRoot();
    const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as {
      readonly generatorContracts: Readonly<Record<string, GeneratorContract>>;
    };
    const launch = Bun.JSONC.parse(readFileSync(join(repoRoot, ".vscode/launch.json"), "utf8")) as { readonly configurations: readonly LaunchEntry[] };
    const expected = Object.entries(taxonomy.generatorContracts).filter((entry): entry is [string, GeneratorContract & { readonly previewTarget: string }] => typeof entry[1].previewTarget === "string");

    const previewOrder = [
      ["actor-typegen", 206.01], ["assets-build", 206.02], ["async-typegen", 206.03],
      ["dev-distribution-bundle", 206.035],
      ["framework-manifest", 206.04], ["graph-catalog", 206.05], ["jco-package-adapter", 206.055],
      ["playground-session", 206.057],
      ["plugin-registry", 206.06], ["print-latex-tokens", 206.07], ["report-actor-network", 206.075], ["scale-fixture", 206.08],
      ["schema-entity-catalog", 206.09], ["shell-typegen", 206.1], ["styling-tokens", 206.11],
      ["ticket-important-fem-handoff", 206.115], ["ui-axes", 206.12], ["ui-contract", 206.13],
      ["wgpu-frame-worker", 206.14],
    ] as const;
    expect(expected.map(([contractId]) => contractId)).toEqual(previewOrder.map(([contractId]) => contractId));
    expected.forEach(([contractId, contract], index) => {
      const name = `📦️preview🤖️${contractId}`;
      const matches = launch.configurations.filter((entry) => entry.name === name);
      expect(matches, name).toEqual([
        {
          name,
          type: "node-terminal",
          request: "launch",
          command: `bun nx run ${contract.previewTarget}`,
          cwd: "${workspaceFolder}",
          presentation: { group: "4_build", order: previewOrder[index]![1] },
        },
      ]);
    });
  });

  it("registers the strict catalog completion target with an explicit fresh build root", () => {
    const repoRoot = getWorkspaceRoot();
    const launch = Bun.JSONC.parse(readFileSync(join(repoRoot, ".vscode/launch.json"), "utf8")) as { readonly configurations: readonly LaunchEntry[] };
    expect(launch.configurations.filter(({ name }) => name === "📦️catalog-complete🤖️plugin-registry")).toEqual([{
      name: "📦️catalog-complete🤖️plugin-registry",
      type: "node-terminal",
      request: "launch",
      command: "bun nx run @semio-tech/plugin-registry:catalog-complete -- --build-root \"${input:catalogFreshBuildRoot}\"",
      cwd: "${workspaceFolder}",
      presentation: { group: "4_build", order: 206.066 },
    }]);
  });
});

describe("WASI codegen profile policy", () => {
  it("independently validates all neutral profile routes and native Cargo policy", async () => {
    const { default: Ajv } = await import("ajv");
    const { default: toml } = await import("@iarna/toml");
    const root = getWorkspaceRoot();
    const fixtureRoot = join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "../../../🧬️schema/🔣️.json"), "utf8"));
    expect(new Ajv({ strict: true, allErrors: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/WasiProfilePolicyV1`)!(fixture)).toBe(true);
    for (const vector of fixture.cases) {
      const independent = vector.override === null ? vector.mode === "dev" ? "wasm-dev" : "wasm-release" : fixture.runtimeDirectories.includes(vector.override) ? vector.override : null;
      expect(independent).toBe(vector.expectedProfile);
      if (independent === null) expect(() => selectComponentWasmProfile(vector.mode, vector.override)).toThrow();
      else {
        const selected = selectComponentWasmProfile(vector.mode, vector.override ?? undefined);
        expect(selected).toBe(independent);
        expect(cargoProfileDir(selected)).toBe(vector.expectedDirectory);
      }
    }
    const manifest = toml.parse(readFileSync(join(root, "Cargo.toml"), "utf8")) as any;
    expect(manifest.profile.dev["codegen-units"]).toBeUndefined();
    for (const override of Object.values(manifest.profile.dev.package ?? {})) expect((override as any)["codegen-units"]).toBeUndefined();
    const { package: developmentPackageOverrides, ...developmentProfile } = manifest.profile["wasm-dev"];
    expect(developmentProfile).toEqual({ inherits: "dev", "codegen-units": 1 });
    expect(developmentPackageOverrides).toEqual(fixture.developmentPackageOverrides);
    const freshTarget = join(root, "target", "registry-profile-fixture");
    for (const vector of fixture.artifactPaths) {
      expect(pluginWasmArtifactPath(root, vector.packageName, vector.profile ?? undefined, freshTarget)).toBe(join(freshTarget, ...vector.expectedRelativePath.split("/")));
    }
    console.log("[DEBUG] registry-wasm-profile routes=" + fixture.cases.length + " package-overrides=" + Object.keys(developmentPackageOverrides).length + " artifact-paths=" + fixture.artifactPaths.length);
    expect(manifest.profile["wasm-release"]).toMatchObject({ inherits: "release", "opt-level": "s", lto: "thin", "codegen-units": 1, strip: "symbols", incremental: false, "trim-paths": "object" });
  });

  it("keeps generated native, root preflight, and MCP runtime profiles identical without debug", () => {
    const root = getWorkspaceRoot();
    for (const path of [
      "📜️script.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧭️session/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts/🦀️.rs",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
    ]) {
      const source = readFileSync(join(root, path), "utf8");
      const declaration = source.split("\n").find((line) => line.includes("const PLUGIN_WASM_PROFILE_DIRS"));
      expect(declaration, path).toContain('["wasm-dev", "wasm-release"]');
      expect(declaration, path).not.toContain('"debug"');
    }
    const describe = ["🏗️component-build", "🏭️fresh-component"].map((owner) => readFileSync(join(root, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/${owner}/🟦️.ts`), "utf8")).join("\n");
    expect(describe.match(/"--target", "wasm32-wasip2", "--profile", "wasm-dev"/g)).toHaveLength(2);
    const scale = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust/📜️script.ts"), "utf8");
    expect(scale).toContain('"--target", "wasm32-wasip2", "--profile", "wasm-dev"');
  });

  it("never substitutes a first-found development artifact for publication identity", () => {
    const root = getWorkspaceRoot();
    const wasmTarget = join(cargoTargetDirectory(root), "wasm32-wasip2");
    const release = join(wasmTarget, "wasm-release", "fixture.wasm");
    const available = new Map([
      [join(wasmTarget, "wasm-dev", "fixture.wasm"), "dev-hash"],
      [join(wasmTarget, "debug", "fixture.wasm"), "stale-hash"],
      [release, "release-hash"],
    ]);
    expect(publicationWasmPath(root, "fixture.wasm")).toBe(release);
    expect(available.get(publicationWasmPath(root, "fixture.wasm"))).toBe("release-hash");
    available.delete(release);
    expect(available.get(publicationWasmPath(root, "fixture.wasm"))).toBeUndefined();
  });
});
