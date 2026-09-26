import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { cargoProfileDir, getWorkspaceRoot, selectComponentWasmProfile } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { cargoTargetDirectory } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { publicationWasmPath } from "../../🛂️descriptor-verification/🟦️.ts";
import { pluginWasmArtifactPath } from "../../../🖨️describe/🏗️component-build/🟦️.ts";

type GeneratorContract = { readonly previewTarget?: string };

describe("plugin registry generator preview targets", () => {
  it("names every owned generator preview target in taxonomy order", () => {
    const repoRoot = getWorkspaceRoot();
    const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as {
      readonly generatorContracts: Readonly<Record<string, GeneratorContract>>;
    };
    const expected = Object.entries(taxonomy.generatorContracts).filter((entry): entry is [string, GeneratorContract & { readonly previewTarget: string }] => typeof entry[1].previewTarget === "string");
    const previewOrder = [
      ["actor-typegen", 206.01], ["assets-build", 206.02], ["async-typegen", 206.03],
      ["dev-distribution-bundle", 206.035], ["flow-browser-package", 206.037],
      ["framework-manifest", 206.04], ["graph-catalog", 206.05], ["jco-package-adapter", 206.055],
      ["mutation-source-authority", 206.056], ["playground-session", 206.057],
      ["plugin-registry", 206.06], ["print-latex-tokens", 206.07], ["report-actor-network", 206.075], ["scale-fixture", 206.08],
      ["schema-entity-catalog", 206.09], ["shell-typegen", 206.1], ["styling-tokens", 206.11],
      ["ui-axes", 206.12], ["ui-contract", 206.13],
      ["wgpu-frame-worker", 206.14],
    ] as const;
    expect(expected.map(([contractId]) => contractId)).toEqual(previewOrder.map(([contractId]) => contractId));
    expected.forEach(([contractId, contract]) => {
      expect(contract.previewTarget, contractId).toMatch(/:/);
    });
  });

  it("registers catalog-complete on the plugin-registry project", () => {
    const repoRoot = getWorkspaceRoot();
    const project = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📋️project.json"), "utf8")) as { targets: Record<string, unknown> };
    expect(project.targets["catalog-complete"]).toBeDefined();
  });

  it("uses taxonomy folder emojis in every generated dev launcher name", async () => {
    const repoRoot = getWorkspaceRoot();
    const { generatePlaygroundRegistry } = await import("../../🎮️playground/🔎️discovery/🟦️.ts");
    const { generateLaunchJson } = await import("../../🚀️launch/🟦️.ts");
    const { playgroundLaunchNamePrefix } = await import("../../🚀️launch/🏷️name-prefix/🟦️.ts");
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    const launch = Bun.JSONC.parse(generateLaunchJson(repoRoot, playgrounds, [])) as {
      readonly configurations: readonly { name?: string }[];
    };
    for (const playground of playgrounds) {
      const prefix = playgroundLaunchNamePrefix(playground, repoRoot, playgrounds);
      expect(prefix, playground.variant).not.toBe(`🧩️${playground.variant}`);
      const reactName = `🛠️dev${prefix}⚛️react`;
      expect(launch.configurations.some((entry) => entry.name === reactName), playground.variant).toBe(true);
    }
  });

  it("registers one react and one wgpu dev launcher for every playground variant", async () => {
    const repoRoot = getWorkspaceRoot();
    const { generatePlaygroundRegistry } = await import("../../🎮️playground/🔎️discovery/🟦️.ts");
    const { generateLaunchJson, playgroundDevCommand } = await import("../../🚀️launch/🟦️.ts");
    const playgrounds = generatePlaygroundRegistry(repoRoot);
    expect(playgrounds.length).toBeGreaterThan(0);
    const launch = Bun.JSONC.parse(generateLaunchJson(repoRoot, playgrounds, [])) as {
      readonly configurations: readonly { name?: string; command?: string; env?: Readonly<Record<string, string>> }[];
    };
    const mismatches: string[] = [];
    for (const renderer of ["react", "wgpu"] as const) {
      const marker = renderer === "react" ? "⚛️react" : "🧊️wgpu🌐️wasm";
      const pool = launch.configurations.filter((entry) => String(entry.name ?? "").endsWith(marker) && !/👤️\d+/u.test(String(entry.name ?? "")));
      expect(pool.length, renderer).toBeGreaterThan(0);
      for (const playground of playgrounds) {
        const port = String(playground.ports[renderer]);
        const matches = pool.filter(
          (entry) =>
            entry.command === playgroundDevCommand(playground.variant) &&
            entry.env?.SEMIO_PLUGIN === playground.variant &&
            entry.env.SEMIO_RENDERER === renderer &&
            entry.env.S_OS_PORT === port,
        );
        if (matches.length !== 1) mismatches.push(`${playground.variant}:${renderer} has ${matches.length} launcher(s) on port ${port}`);
      }
    }
    expect(mismatches.join("\n")).toBe("");
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
    expect(developmentProfile).toEqual({ inherits: "dev", "codegen-units": 1, incremental: false });
    expect(developmentPackageOverrides).toEqual(fixture.developmentPackageOverrides);
    const freshTarget = join(root, "target", "registry-profile-fixture");
    for (const vector of fixture.artifactPaths) {
      expect(pluginWasmArtifactPath(root, vector.packageName, vector.profile ?? undefined, freshTarget)).toBe(join(freshTarget, ...vector.expectedRelativePath.split("/")));
    }
    expect(manifest.profile["wasm-release"]).toMatchObject({ inherits: "release", "opt-level": "s", lto: "thin", "codegen-units": 1, strip: "symbols", incremental: false, "trim-paths": "object" });
  });

  it("resolves every runtime component from its crate's deliverable, which describe reads without building", () => {
    const root = getWorkspaceRoot();
    for (const path of [
      "📜️script.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📽️projection/🟦️.ts",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🗿️artifacts/🦀️.rs",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs",
      "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🟦️.ts",
    ]) {
      const source = readFileSync(join(root, path), "utf8");
      const declaration = source.split("\n").find((line) => line.includes("const PLUGIN_COMPONENT_PROFILE_DIRS"));
      expect(declaration, path).toContain('"dist/component-dev", "dist/component-release"');
      expect(source, path).not.toContain("PLUGIN_WASM_TARGET");
    }
    const describe = readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏗️component-build/🟦️.ts"), "utf8");
    expect(describe).not.toContain("pluginComponentRustcArgs");
    const inferred = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs"), "utf8");
    expect(inferred).toContain('dependsOn: ["component-dev"]');
    const component = readFileSync(join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts"), "utf8");
    expect(component).toContain('"--lib", "--crate-type", "cdylib", "--target", "wasm32-wasip2", "--profile", profile');
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

type RenderedLaunch = {
  readonly configurations: readonly { name?: string; command?: string }[];
  readonly compounds?: readonly { name: string; configurations: readonly string[] }[];
};

/** @emoji ♻️ Renders `.vscode/launch.json` once for the whole block — a playground discovery walk over
 * the workspace costs seconds, and every assertion below reads the same output. */
let renderedLaunch: Promise<RenderedLaunch> | undefined;
const launchOutput = (): Promise<RenderedLaunch> => (renderedLaunch ??= (async () => {
  const repoRoot = getWorkspaceRoot();
  const { generatePlaygroundRegistry } = await import("../../🎮️playground/🔎️discovery/🟦️.ts");
  const { generateLaunchJson } = await import("../../🚀️launch/🟦️.ts");
  return Bun.JSONC.parse(generateLaunchJson(repoRoot, generatePlaygroundRegistry(repoRoot), [])) as RenderedLaunch;
})());

describe("launch configuration identity", () => {
  it("gives every configuration a unique name and keeps the user slot out of the collapse", async () => {
    const { devLaunchNameSuffix } = await import("../../🚀️launch/🏷️name-prefix/🟦️.ts");
    expect(devLaunchNameSuffix("🛠️dev🖥️s👤️2⚛️react")).toBe("👤️2⚛️react");
    expect(devLaunchNameSuffix("🛠️dev🖥️s⚛️react")).toBe("⚛️react");
    const launch = await launchOutput();
    const counts = new Map<string, number>();
    for (const entry of launch.configurations) counts.set(String(entry.name), (counts.get(String(entry.name)) ?? 0) + 1);
    expect([...counts].filter(([, count]) => count > 1).map(([name]) => name)).toEqual([]);
  });

  it("resolves every compound member to a real configuration", async () => {
    const launch = await launchOutput();
    const names = new Set(launch.configurations.map((entry) => String(entry.name)));
    const broken = (launch.compounds ?? []).flatMap((compound) => compound.configurations.filter((member) => !names.has(member)).map((member) => `${compound.name} -> ${member}`));
    expect(broken).toEqual([]);
    expect((launch.compounds ?? []).length).toBeGreaterThan(0);
  });

  it("registers a launch row for every canonical root lifecycle command", async () => {
    const launch = await launchOutput();
    const commands = new Set(launch.configurations.map((entry) => String(entry.command ?? "")));
    for (const command of ["setup", "start", "dev", "generate", "lint", "format", "test", "build", "publish"]) {
      expect([...commands].some((row) => row === `bun nx run workspace:${command}` || row.startsWith(`bun nx run workspace:${command} `)), command).toBe(true);
    }
  });
});
