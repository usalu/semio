import { describe, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { tmpdir } from "node:os";
import { basename, dirname, relative, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { semanticDirectoryKindId } from "../../🔍️discovery/🟦️.ts";

type Fixture = Readonly<{
  version: number;
  owners: readonly Readonly<{ owner: string; exports: readonly string[]; consumers: readonly string[] }>[];
  directoryContexts: readonly Readonly<{ name: string; parentKind: string; ancestorKinds?: readonly string[]; kind: string }>[];
  wgpuOutputRoots: readonly Readonly<{ path: string; inclusion: "tracked" | "ignored" }>[];
  wgpuGeneratorImplementationInputs: readonly string[];
  goOracle: Readonly<{ module: string; requirement: string; localReplacement: string }>;
}>;

const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const fixture = JSON.parse(readFileSync(resolve(libraryRoot, "🧫️fixtures/🧱️root-artifact-dependency-source/🔣️.json"), "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(resolve(libraryRoot, "🧬️schema/🧱️root-artifact-dependency-source/🔣️.json"), "utf8"));

function exportedNames(path: string): ReadonlySet<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
  const names = new Set<string>();
  for (const statement of source.statements) {
    if (!statement.modifiers?.some(({ kind }) => kind === ts.SyntaxKind.ExportKeyword)) continue;
    if (ts.isVariableStatement(statement)) {
      for (const declaration of statement.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
    } else if ("name" in statement && statement.name && ts.isIdentifier(statement.name)) names.add(statement.name.text);
  }
  return names;
}

function relativeSpecifier(consumer: string, owner: string): string {
  const value = relative(dirname(consumer), owner).replaceAll("\\", "/");
  return value.startsWith(".") ? value : `./${value}`;
}

describe("root artifact and dependency source ownership", () => {
  test("validates the portable contract and contextual semantic owners", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(validate({ ...fixture, extra: true })).toBe(false);
    const taxonomy = JSON.parse(readFileSync(resolve(libraryRoot, "🔣️taxonomy.json"), "utf8"));
    for (const row of fixture.directoryContexts) {
      expect(semanticDirectoryKindId(row.name, taxonomy, { parentKindId: row.parentKind, ancestorKindIds: row.ancestorKinds }), `${row.parentKind}/${row.name}`).toBe(row.kind);
    }
    expect(taxonomy.generatorContracts["wgpu-frame-worker"].outputRoots).toEqual(fixture.wgpuOutputRoots);
    const inputs = taxonomy.generatorContracts["wgpu-frame-worker"].inputPatterns;
    for (const path of fixture.wgpuGeneratorImplementationInputs) expect(inputs, path).toContain(path);
    expect(inputs).not.toContain("📜️script.ts");
  });

  test("binds every implementation API to an anonymous semantic owner and direct consumer", () => {
    for (const row of fixture.owners) {
      const absolute = resolve(repoRoot, row.owner);
      expect(basename(absolute), row.owner).toBe("🟦️.ts");
      expect(existsSync(absolute), row.owner).toBe(true);
      const exports = exportedNames(absolute);
      for (const name of row.exports) expect(exports.has(name), `${row.owner}:${name}`).toBe(true);
      for (const consumer of row.consumers) {
        const source = readFileSync(resolve(repoRoot, consumer), "utf8");
        expect(source, `${consumer} -> ${row.owner}`).toContain(relativeSpecifier(consumer, row.owner));
      }
    }
    const root = readFileSync(resolve(repoRoot, "📜️script.ts"), "utf8");
    for (const name of ["renderWgpuBrowserBundles", "assertWgpuPackageToolchain", "renderWgpuPackageArtifacts", "runWgpuPackageGenerator", "function dependencyJsTokens", "function dependencyFreezeCheck"]) {
      expect(root, name).not.toContain(name);
    }
  });

  test("matches the dependency Go parser to the native Go manifest oracle", async () => {
    const inventory = await import(resolve(repoRoot, fixture.owners.find(({ owner }) => owner.includes("/📇️inventory/"))!.owner));
    const sandbox = mkdtempSync(resolve(tmpdir(), "semio-dependency-go-oracle-"));
    try {
      const moduleText = `module ${fixture.goOracle.module}\n\ngo 1.25\n\nrequire (\n ${fixture.goOracle.requirement} v1.2.3 // indirect\n ${fixture.goOracle.localReplacement} v0.0.0\n)\nreplace ${fixture.goOracle.localReplacement} => ../replaced\n`;
      const manifest = resolve(sandbox, "go.mod");
      writeFileSync(manifest, moduleText);
      const oracle = Bun.spawnSync(["go", "mod", "edit", "-json", manifest], { cwd: sandbox });
      expect(oracle.exitCode, oracle.stderr.toString()).toBe(0);
      const native = JSON.parse(oracle.stdout.toString()) as { Module: { Path: string }; Require: readonly { Path: string; Indirect?: boolean }[]; Replace: readonly { Old: { Path: string } }[] };
      const parsed = inventory.dependencyParseGoModule(moduleText);
      expect(parsed.module).toBe(native.Module.Path);
      expect(parsed.requirements.map(({ name }: { name: string }) => name)).toEqual(native.Require.map(({ Path }) => Path));
      expect(parsed.localReplaces).toEqual(native.Replace.map(({ Old }) => Old.Path));
    } finally {
      rmSync(sandbox, { recursive: true, force: true });
    }
  });

  test("dispatches projected WGPU inputs and cancels through the same package command", () => {
    const cwd = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript");
    const projection = JSON.stringify({ contractId: "wgpu-frame-worker", schemaVersion: 1, moves: [], edits: [], removals: [] }) + "\n";
    const env = { ...process.env, SEMIO_GENERATOR_PREVIEW: "1", SEMIO_GENERATOR_PREVIEW_PROTOCOL: "package-projected-inputs-v1", SEMIO_GENERATOR_PREVIEW_CANCEL_FILE: "" };
    const success = spawnSync("bun", ["./📜️script.ts", "preview-generated"], { cwd, env, input: projection, encoding: "utf8", maxBuffer: 8 * 1024 * 1024, timeout: 15_000 });
    expect(success.status, success.stderr).toBe(0);
    expect(success.stderr).toBe("");
    const manifest = JSON.parse(success.stdout);
    expect(manifest.contractId).toBe("wgpu-frame-worker");
    expect(manifest.nodes.map(({ path }: { path: string }) => path)).toEqual(fixture.wgpuOutputRoots.map(({ path }) => path));
    const cancelRoot = resolve(repoRoot, ".🧬semio/🦑️repo/⚡️cache");
    mkdirSync(cancelRoot, { recursive: true });
    const cancelDirectory = mkdtempSync(resolve(cancelRoot, "root-artifact-dependency-preview-"));
    const cancelFile = resolve(cancelDirectory, "🛑️cancel");
    writeFileSync(cancelFile, "cancel\n");
    const started = Date.now();
    try {
      const cancelled = spawnSync("bun", ["./📜️script.ts", "preview-generated"], { cwd, env: { ...env, SEMIO_GENERATOR_PREVIEW_CANCEL_FILE: cancelFile }, input: projection, encoding: "utf8", maxBuffer: 8 * 1024 * 1024, timeout: 15_000 });
      expect(cancelled.status).not.toBe(0);
      expect(cancelled.stderr).toMatch(/cancelled/u);
      expect(cancelled.stdout).toBe("");
      expect(Date.now() - started).toBeLessThan(5_000);
    } finally {
      rmSync(cancelDirectory, { recursive: true, force: true });
    }
  }, 15_000);

  test("registers one Bun/Nx/editor route", () => {
    const command = "bun nx run @semio-tech/repo-lib:test-root-artifact-dependency-source";
    const project = JSON.parse(readFileSync(resolve(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json"), "utf8"));
    expect(project.targets["test-root-artifact-dependency-source"].options.command).toBe("bun ./📜️script.ts test root-artifact-dependency-source");
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const launch = Bun.JSONC.parse(readFileSync(resolve(repoRoot, path), "utf8")) as { configurations: readonly { name?: string; command?: string }[] };
      expect(launch.configurations.filter(({ name, command: value }) => name === "🧹clean🧩️taxonomy🧪️root-artifact-dependency-source" && value === command)).toHaveLength(1);
    }
  });
});
