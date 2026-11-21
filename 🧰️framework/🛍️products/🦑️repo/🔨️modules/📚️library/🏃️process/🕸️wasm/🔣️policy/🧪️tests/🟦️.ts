import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";
import { dirname, join, posix, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "smol-toml";
import ts from "typescript";
import fixture from "./🔣️.json" with { type: "json" };
import { cargoWorkspacePreparationInvocationV1, parseCargoPreparation } from "../../../../🗂️workspaces/🦀️cargo/🟦️.ts";
import { wasmCompilerSelectionArgumentsV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { readGraphOutputCatalog } from "../../../../../../../../🔨️modules/🕸️graph/🛂️manifest/📇️catalog/🟦️.ts";
import { renderGraphArtifacts } from "../../../../../../../../🔨️modules/🕸️graph/🛂️manifest/📽️projection/🟦️.ts";
import { readRustSourceClosureV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts";

const repository = process.cwd();
const read = (path: string) => readFileSync(path, "utf8");

test("each Puzzle Rust graph source declares its artifact's existing publisher to Cargo", async () => {
  for (const group of fixture.groups) {
    const artifact = join(repository, fixture.artifactRoot, group.artifact);
    const producer = join(artifact, "📦️packages/🦀️rust");
    const project = await Bun.file(join(producer, "📋️project.json")).json();
    expect(project.name).toBe(group.project);
    expect(project.targets[fixture.command[0]!].options.command).toBe("repo ./📜️script.ts graph-generate");
    const catalog = readGraphOutputCatalog(join(artifact, "🛂️manifest/📇️outputs.json"));
    const graphSources = new Set([catalog.shared.rustRegistry, ...catalog.manifests.map(row => row.rust)].map(path => join(artifact, "🤖️generated", path)));
    for (const consumer of group.consumers) {
      const directory = join(artifact, consumer.directory);
      const manifest = parse(read(join(directory, "Cargo.toml"))) as { package: { name: string; metadata: { semio: { preparation?: { script: string; command: string[] } } } }; lib: { path: string } };
      expect(manifest.package.name).toBe(consumer.package);
      const independent = manifest.package.metadata.semio.preparation;
      expect(independent?.command, consumer.package).toEqual(fixture.command);
      const recipe = parseCargoPreparation(independent);
      expect(recipe).toEqual(independent!);
      expect(resolve(directory, recipe.script)).toBe(join(producer, "📜️script.ts"));
      const library = join(directory, manifest.lib.path);
      const mounts = [...read(library).matchAll(/#\[path = "([^"]*🤖️generated\/[^"]+)"\]/gu)];
      expect(mounts.length, consumer.package).toBeGreaterThan(0);
      for (const mount of mounts) expect(graphSources.has(resolve(dirname(library), mount[1]!)), mount[1]).toBe(true);
    }
  }
});

test("the wasm owner awaits Cargo preparation for the exact component before entering its compiler lane", () => {
  const manifest = join(repository, fixture.component);
  const independent = parse(read(manifest)) as { package: { name: string; workspace: string } };
  expect(independent.package.name).toBe(fixture.package);
  expect(join(dirname(manifest), independent.package.workspace, "Cargo.toml")).toBe(join(repository, fixture.workspace));
  const source = ts.createSourceFile("policy.ts", read(fileURLToPath(new URL("../🟦️.ts", import.meta.url))), ts.ScriptTarget.Latest, true);
  const owner = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "buildRepositoryWasmWebV1")!;
  const calls = new Map<string, ts.CallExpression>();
  function visit(node: ts.Node): void {
    if (ts.isCallExpression(node) && ts.isIdentifier(node.expression)) calls.set(node.expression.text, node);
    ts.forEachChild(node, visit);
  }
  visit(owner);
  const database = new Database(":memory:");
  try {
    const positions = fixture.sequence.map(name => calls.get(name)?.getStart(source) ?? -1);
    expect(database.query("select ?1 >= 0 and ?1 < ?2 and ?2 < ?3 and ?3 < ?4 as ordered").get(...positions)).toEqual({ ordered: 1 });
    const preparation = calls.get("prepareCargoWorkspaceInvocation")!;
    expect(ts.isAwaitExpression(preparation.parent)).toBe(true);
    expect(preparation.arguments.slice(1).map(arg => arg.getText(source))).toEqual(["workspace", "selection.cargo", "options.componentDir", "store.environment", "control"]);
  } finally {
    database.close();
  }
  const paths = fixture.preparationStorage, storage = { version: 1 as const, directory: join(repository, paths.directory), leaseDirectory: join(repository, paths.leaseDirectory), program: join(repository, paths.program) };
  for (const mode of fixture.modes) {
    const selection = wasmCompilerSelectionArgumentsV1({ mode, manifest, configuration: join(repository, ".cargo/config.toml"), selectors: [], threads: false, features: [], noDefaultFeatures: false });
    const actual = cargoWorkspacePreparationInvocationV1(storage, repository, selection.cargo, dirname(manifest), { SEMIO_BUILD_MODE: mode });
    expect(actual?.args.slice(1)).toEqual(["synchronize", "--manifest", fixture.workspace, "--package", independent.package.name]);
    expect(actual?.environment.SEMIO_BUILD_MODE).toBe(mode);
  }
});

test("a cold Puzzle registry is refused until its graph publisher supplies the Rust modules", async () => {
  const cold = fixture.cold, artifact = join(repository, fixture.artifactRoot, cold.artifact);
  const consumer = join(artifact, cold.consumer);
  const catalog = readGraphOutputCatalog(join(artifact, "🛂️manifest/📇️outputs.json"));
  const authored = (path: string) => path === consumer ? readFileSync(path) : null;
  await expect(readRustSourceClosureV1([consumer], repository, cold.cfg, async path => authored(path))).rejects.toMatchObject({
    key: cold.error, values: { file: posix.join(fixture.artifactRoot, cold.artifact, cold.consumer), module: cold.module }
  });
  const generatedRoot = join(artifact, "🤖️generated");
  const rendered = renderGraphArtifacts(repository, generatedRoot, catalog, false);
  const published = new Map(rendered.artifacts.map(row => [row.path, Buffer.from(row.content)]));
  const admitted = await readRustSourceClosureV1([consumer], repository, cold.cfg, async path => authored(path) ?? published.get(path) ?? null);
  expect(admitted.map(input => input.path).sort()).toEqual([
    posix.join(fixture.artifactRoot, cold.artifact, cold.consumer),
    ...cold.outputs.map(path => posix.join(fixture.artifactRoot, cold.artifact, "🤖️generated", path))
  ].sort());
});
