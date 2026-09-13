import assert from "node:assert/strict";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { slash, sourceFiles } from "../../🔍️discovery/📂️source/🟦️.ts";
import { captureArtifactContract } from "../🏃️contract-capture/🟦️.ts";
import { artifactPackageInventory } from "../🧭️package-inventory/🟦️.ts";

/** 🧬️ Validates every taxonomy artifact's language-neutral package boundary. */
export class ArtifactPackageContractScript extends BundleScript {
  async run(): Promise<void> {
    const fixtureRoot = fileURLToPath(new URL("../../🧫️fixtures/artifact-packages/", import.meta.url));
    const schema = JSON.parse(readFileSync(join(fixtureRoot, "🛂️schema/🔣️.json"), "utf8"));
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8"));
    const { default: Ajv2020 } = await import("ajv/dist/2020.js");
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    for (const accepted of fixture.accepted) assert.ok(validate(accepted), JSON.stringify(validate.errors));
    for (const rejected of fixture.rejected) assert.equal(validate(rejected.value), false, `Negative fixture accepted: ${rejected.id}`);
    const contract = artifactPackageInventory(this.repoRoot);
    assert.ok(validate(contract), JSON.stringify(validate.errors));
    const cargoNames = new Set<string>(),
      nxNames = new Set<string>();
    for (const entry of contract.packages) {
      assert.ok(!cargoNames.has(entry.rust.cargoName), `Duplicate Cargo package ${entry.rust.cargoName}`);
      assert.ok(!nxNames.has(entry.rust.nxName), `Duplicate Nx project ${entry.rust.nxName}`);
      cargoNames.add(entry.rust.cargoName);
      nxNames.add(entry.rust.nxName);
      const packageRoot = join(this.repoRoot, dirname(entry.rust.manifest));
      const implementations = sourceFiles(packageRoot).filter((path) => /(?:^|\/)(?!📜️script\.ts$).+\.(?:rs|ts|tsx)$/.test(path) && !path.startsWith("dist/"));
      assert.deepEqual(implementations, [], `Rust package declarations contain implementation: ${implementations.join(", ")}`);
      const artifactMarker = "/🗿️artifacts/",
        marker = entry.root.indexOf(artifactMarker),
        parentSource = marker < 0 ? "" : `${entry.root.slice(0, marker)}/📦️packages/🦀️rust/🦀️.rs`;
      if (parentSource && existsSync(join(this.repoRoot, parentSource))) {
        const parent = readFileSync(join(this.repoRoot, parentSource), "utf8");
        assert.equal(/#\[path\s*=\s*"\.\.\/\.\.\/🗿️artifacts\//.test(parent), false, `${parentSource} recompiles taxonomy artifact implementation`);
        assert.equal(/^(?:pub\s+mod\s+(?:artifacts|editor|viewer)\b|pub\s+use\s+semio_s_artifact_)/m.test(parent), false, `${parentSource} publicly re-exports an artifact implementation`);
      }
      if (entry.typescript) {
        assert.ok(!nxNames.has(entry.typescript.nxName), `Duplicate Nx project ${entry.typescript.nxName}`);
        nxNames.add(entry.typescript.nxName);
        const declaration = JSON.parse(readFileSync(join(this.repoRoot, entry.typescript.manifest), "utf8"));
        assert.equal(declaration.type, "module");
        assert.equal(declaration.private, true);
        assert.equal(declaration.types, "./dist/🟦️.d.ts");
        assert.deepEqual(declaration.exports?.["."], entry.typescript.entry);
      }
    }
    const metadata = JSON.parse(await captureArtifactContract("cargo", ["metadata", "--locked", "--offline", "--format-version", "1"], this.repoRoot, 180_000));
    const cargoPackages = new Map<string, any>(metadata.packages.map((entry: any) => [entry.id, entry]));
    const byName = new Map<string, any>(metadata.packages.map((entry: any) => [entry.name, entry]));
    const nodes = new Map<string, any>((metadata.resolve?.nodes ?? []).map((entry: any) => [entry.id, entry]));
    for (const entry of contract.packages) {
      const cargo = byName.get(entry.rust.cargoName);
      assert.ok(cargo, `Cargo metadata omitted ${entry.rust.cargoName}`);
      assert.equal(slash(relative(this.repoRoot, cargo.manifest_path)), entry.rust.manifest);
      const visit = (id: string, requested: string[], defaults: boolean, route: string[], visited: Set<string>): void => {
        const dependency = cargoPackages.get(id);
        const enabled = new Set<string>(requested);
        if (defaults && dependency?.features?.default) enabled.add("default");
        const activated = new Set<string>();
        const forwarded = new Map<string, Set<string>>();
        const queue = [...enabled];
        while (queue.length) {
          const feature = queue.pop()!;
          for (const value of dependency?.features?.[feature] ?? []) {
            if (value.startsWith("dep:")) activated.add(value.slice(4));
            else if (value.includes("/")) {
              const [name, selected] = value.replace("?/", "/").split("/", 2);
              if (!forwarded.has(name)) forwarded.set(name, new Set());
              forwarded.get(name)!.add(selected);
            } else if (!enabled.has(value)) {
              enabled.add(value);
              queue.push(value);
            }
          }
        }
        const state = `${id}\0${[...enabled].sort().join(",")}\0${defaults}`;
        if (visited.has(state)) return;
        visited.add(state);
        const role = dependency?.metadata?.semio?.role;
        assert.ok(role !== "plugin" && role !== "extension", `${entry.rust.cargoName} reaches composition package ${[...route, dependency?.name].join(" -> ")}`);
        for (const edge of nodes.get(id)?.deps ?? []) {
          if (!edge.dep_kinds?.some((kind: any) => kind.kind === null)) continue;
          const target = cargoPackages.get(edge.pkg);
          const declarations = (dependency?.dependencies ?? []).filter((row: any) => row.kind === null && row.name === target?.name && (row.rename ?? row.name) === edge.name);
          const active = declarations.filter((row: any) => !row.optional || activated.has(edge.name) || enabled.has(edge.name));
          if (!active.length) continue;
          const features = new Set<string>(forwarded.get(edge.name) ?? []);
          for (const declaration of active) for (const feature of declaration.features ?? []) features.add(feature);
          visit(
            edge.pkg,
            [...features],
            active.some((row: any) => row.uses_default_features),
            [...route, dependency?.name],
            visited,
          );
        }
      };
      visit(cargo.id, [], true, [], new Set());
    }
    const legacy = sourceFiles(this.repoRoot).filter((path) => {
      if (!path.endsWith(".rs")) return false;
      try {
        return /semio_s_plugin_(?![a-z0-9_]*_test_oracle::)[a-z0-9_]+::artifacts::/.test(readFileSync(join(this.repoRoot, path), "utf8"));
      } catch (error) {
        if ((error as { code?: string }).code === "ENOENT") return false;
        throw error;
      }
    });
    assert.deepEqual(legacy, [], `Rust consumers retain composition artifact namespaces: ${legacy.join(", ")}`);
    console.log(
      `[artifact-package-contract] AJV=${fixture.accepted.length + 1}/${fixture.rejected.length} packages=${contract.packages.length} rust=${cargoNames.size} typescript=${contract.packages.filter((entry) => entry.typescript).length} dag=clean`,
    );
  }
}
