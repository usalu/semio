import assert from "node:assert/strict";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import plugin from "../../../🟨️.mjs";
import type { InventoryProject } from "../../📇️inventory/🟦️.ts";
import { CACHE_POLICY, slash, sourceFiles } from "../../🔍️discovery/📂️source/🟦️.ts";

export type ArtifactPackageRecord = {
  root: string;
  source: string;
  rust: { cargoName: string; nxName: string; manifest: string };
  typescript?: { name: string; nxName: string; manifest: string; entry: { types: string; import: string } };
};

/** 🧭️ Discovers artifact owners from taxonomy roots, independently of package naming. */
export function artifactPackageInventory(root: string): { schemaVersion: 1; packages: ArtifactPackageRecord[] } {
  const owners: string[] = [];
  const walk = (directory: string): void => {
    for (const entry of readdirSync(join(root, directory), { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.isSymbolicLink() || entry.name === "node_modules" || entry.name === ".git" || entry.name === ".🧬semio" || CACHE_POLICY.generatedDirectories.includes(entry.name)) continue;
      const path = slash(join(directory, entry.name));
      if (entry.name !== "🗿️artifacts") {
        walk(path);
        continue;
      }
      for (const artifact of readdirSync(join(root, path), { withFileTypes: true })) {
        const owner = slash(join(path, artifact.name));
        if (artifact.isDirectory() && existsSync(join(root, owner, "🦀️.rs"))) owners.push(owner);
      }
    }
  };
  for (const base of ["✏️s", "🧰️framework"]) if (existsSync(join(root, base))) walk(base);
  const projectFiles = sourceFiles(root).filter((path) => path.endsWith("📋️project.json") || path.endsWith("Cargo.toml"));
  const discovered = (plugin.createNodesV2 as any)[1](projectFiles, {}, { workspaceRoot: root }).flatMap(([, result]: any) => Object.values(result.projects)) as InventoryProject[];
  const projects = new Map(discovered.map((project) => [slash(project.root), project]));
  const packages = owners
    .map((owner): ArtifactPackageRecord => {
      const source = `${owner}/🦀️.rs`;
      const rustRoot = `${owner}/📦️packages/🦀️rust`;
      const rustManifest = `${rustRoot}/Cargo.toml`;
      assert.ok(existsSync(join(root, rustManifest)), `Artifact source has no Rust package declaration: ${source}`);
      const cargo = createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(join(root, rustManifest), "utf8"));
      assert.equal(cargo.lib?.path, "../../🦀️.rs", `${rustManifest} must compile its taxonomy source directly`);
      const rustProject = projects.get(rustRoot);
      assert.ok(rustProject, `Nx omitted ${rustRoot}`);
      assert.ok(rustProject.tags?.includes("role:artifact"), `${rustProject.name} must declare role:artifact`);
      const nativeSources = rustProject.namedInputs?.nativeSources ?? [];
      const ownsSource = nativeSources.some((input: unknown) => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !input.startsWith("!") && new (globalThis as any).Bun.Glob(input.slice("{workspaceRoot}/".length)).match(source));
      assert.ok(ownsSource, `${rustProject.name} does not hash its taxonomy source`);
      assert.deepEqual(rustProject.targets?.build?.outputs, ["{projectRoot}/dist/build"], `${rustProject.name} has an invalid build output contract`);
      const record: ArtifactPackageRecord = { root: owner, source, rust: { cargoName: cargo.package.name, nxName: rustProject.name, manifest: rustManifest } };
      const typescriptSource = `${owner}/🟦️.ts`;
      if (existsSync(join(root, typescriptSource))) {
        const typescriptRoot = `${owner}/📦️packages/🟦️typescript`;
        const manifest = `${typescriptRoot}/package.json`;
        assert.ok(existsSync(join(root, manifest)), `TypeScript artifact source has no package declaration: ${typescriptSource}`);
        const declaration = JSON.parse(readFileSync(join(root, manifest), "utf8"));
        const project = projects.get(typescriptRoot);
        assert.ok(project, `Nx omitted ${typescriptRoot}`);
        assert.equal(project.name, declaration.name, `${manifest} and Nx names differ`);
        assert.ok(
          project.namedInputs?.default?.some((input: unknown) => typeof input === "string" && input.startsWith(`{workspaceRoot}/${owner}/`)),
          `${project.name} does not hash its taxonomy owner`,
        );
        assert.deepEqual(project.targets?.build?.outputs, ["{projectRoot}/dist"], `${project.name} has an invalid build output contract`);
        record.typescript = { name: declaration.name, nxName: project.name, manifest, entry: declaration.exports?.["."] };
      }
      return record;
    })
    .sort((left, right) => left.root.localeCompare(right.root));
  assert.ok(packages.length > 0, "Artifact taxonomy has no package roots");
  assert.equal(new Set(packages.map((entry) => entry.root)).size, packages.length, "Artifact roots must be unique");
  return { schemaVersion: 1, packages };
}
