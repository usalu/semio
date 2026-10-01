/** 🧭️ Concrete fleet ownership, preservation and exact native report laws. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";

const sweepRoot = fileURLToPath(new URL("../..", import.meta.url));
const sourcePath = "✏️s/🧑‍💻dev/🧹️fixture-sweep";
const kernelPath = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const read = (path: string): string => readFileSync(path, "utf8");
const json = (path: string): any => JSON.parse(read(join(sweepRoot, path)));
const sha = (text: string): string => createHash("sha256").update(text).digest("hex");
const portable = (path: string): string => path.replaceAll("\\", "/");

interface FixtureSweepReceipt {
  readonly artifactDir: string;
  readonly assertions: number;
  readonly sha256: string;
}

interface FixtureSweepLawGroup {
  readonly package: string;
  readonly target: { readonly kind: "test"; readonly name: string };
  readonly laws: readonly string[];
}

interface FixtureSweepEvidence {
  readonly directories: number;
  readonly fixtures: number;
  readonly checks: number;
  readonly registered: number;
  readonly unmapped: number;
  readonly covered: number;
  readonly skipped: number;
}

export function fixtureSweepLawGroup(): FixtureSweepLawGroup {
  const fixture = json("🧫️fixtures/🔣️.json");
  return { package: fixture.package, target: { kind: "test", name: fixture.target }, laws: fixture.laws };
}

function assertFixtureSweepOutput(output: string, assertions: number): FixtureSweepEvidence {
  const sweep = [...output.matchAll(/\[dsl-fixture-sweep\] (\d+) example dir\(s\), (\d+) \.semio fixture file\(s\) found, (\d+) law-check\(s\) run across (\d+) registered app kind\(s\), (\d+) unmapped fixture\(s\)/gu)];
  const coverage = [...output.matchAll(/\[dsl-fixture-sweep\] example asset coverage: (\d+) slug\(s\) on new 🖼️assets layout, (\d+) soft-skipped mid-migration/gu)];
  assert.equal(assertions, fixtureSweepLawGroup().laws.length, "every declared fleet law must execute");
  assert.equal(sweep.length, 1, "one actual fleet sweep summary is required");
  assert.equal(coverage.length, 1, "one actual example coverage summary is required");
  const values = sweep[0]!.slice(1).map(Number);
  assert(values.slice(0, 4).every(value => value > 0), "directories, fixtures, checks and registry cannot be empty");
  return { directories: values[0]!, fixtures: values[1]!, checks: values[2]!, registered: values[3]!, unmapped: values[4]!, covered: Number(coverage[0]![1]), skipped: Number(coverage[0]![2]) };
}

/** 🧾️ Receipts must retain both actual native fleet law summaries. */
export function assertFixtureSweepLawCoverage(receipts: readonly FixtureSweepReceipt[]): void {
  assert.equal(receipts.length, 1, "one exact fleet receipt is required");
  const receipt = receipts[0]!;
  const output = fixtureSweepLawGroup().laws.map((_law, index) => [".stdout", ".stderr"].map(suffix => read(join(receipt.artifactDir, `law-${index}${suffix}`))).join("\n")).join("\n");
  const evidence = assertFixtureSweepOutput(output, receipt.assertions);
  console.log(`[dsl-fixture-sweep] exact native fleet receipt: ${JSON.stringify(evidence)}; assertions=${receipt.assertions}; executable=${receipt.sha256}; evidence=${receipt.artifactDir}`);
}

function repoRoot(): string {
  let root = sweepRoot;
  while (!existsSync(join(root, "nx.json"))) {
    const parent = dirname(root);
    assert.notEqual(parent, root, "repository root is required");
    root = parent;
  }
  return root;
}

function exampleInventory(root: string): { directories: string[]; files: string[] } {
  const directories: string[] = [];
  const pending = [root];
  while (pending.length) {
    const current = pending.pop()!;
    for (const entry of readdirSync(current, { withFileTypes: true })) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || ["node_modules", "target", "🦑️repo"].includes(entry.name)) continue;
      const child = join(current, entry.name);
      if (entry.name === "📚️examples") directories.push(child);
      pending.push(child);
    }
  }
  const files: string[] = [];
  const independent: string[] = [];
  const collect = (path: string): void => {
    for (const entry of readdirSync(path, { withFileTypes: true })) {
      const child = join(path, entry.name);
      if (entry.isDirectory()) collect(child);
      else if (entry.isFile() && entry.name.endsWith(".semio")) independent.push(portable(relative(root, child)));
    }
  };
  for (const directory of directories) {
    for (const slug of readdirSync(directory, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
      const path = join(directory, slug.name);
      const assets = join(path, "🖼️assets");
      const search = existsSync(assets) ? assets : path;
      collect(search);
      for (const file of new Bun.Glob("**/*.semio").scanSync({ cwd: search, onlyFiles: true, dot: true, followSymlinks: false })) files.push(portable(relative(root, join(search, file))));
    }
  }
  assert.deepEqual(files.sort(), independent.sort(), "Bun glob and independent recursive discovery agree exactly");
  return { directories: directories.map(path => portable(relative(root, path))).sort(), files };
}

/** 🧪️ AJV independently validates the language-neutral native report corpus. */
export function testFixtureSweepReportContract(): void {
  const fixture = json("🧫️fixtures/📊️report/🔣️.json");
  const schema = json("🧬️schema/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/SFixtureSweepReportV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const evidence = assertFixtureSweepOutput(fixture.valid.output, fixture.valid.assertions);
  const independent = ajv.compile({ ...schema.$defs.SFixtureSweepReportCaseWithExpected.properties.expected, const: fixture.valid.expected });
  assert(independent(evidence), JSON.stringify(independent.errors));
  assert.deepEqual(evidence, fixture.valid.expected);
  for (const hostile of fixture.hostile) assert.throws(() => assertFixtureSweepOutput(hostile.output, hostile.assertions), hostile.id);
}

/** 🏷️ Verifies the physical test owner and preserves separate kernel-only laws. */
export async function testFixtureSweepExtraction(): Promise<void> {
  const root = repoRoot();
  testFixtureSweepReportContract();
  const fixture = json("🧫️fixtures/🔣️.json");
  const document = json("🧬️schema/🔣️.json");
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/SFixtureSweepOwnershipV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const packageDir = join(sweepRoot, "📦️packages/🦀️rust");
  const manifest = Bun.TOML.parse(read(join(packageDir, "Cargo.toml"))) as any;
  const kernel = Bun.TOML.parse(read(join(root, kernelPath, "Cargo.toml"))) as any;
  const targetPath = resolve(packageDir, manifest.test[0].path);
  const target = read(targetPath);
  assert(target.startsWith("//! 🧭️ Fleet-wide example laws:"), "the mounted target retains its concrete fleet charter");
  const mounts = [...target.matchAll(/#\[path = "([^"]+)"\]\s*\n\s*mod (\w+);/gu)].map(match => ({ name: match[2]!, path: match[1]! }));
  const laws = [...target.matchAll(/#\[test\]\s*\n\s*fn (\w+)\(/gu)].map(match => `tests::${match[1]}`).sort();
  const dependencies = Object.entries(manifest.dependencies as Record<string, any>);
  const observed = {
    module: sha(target), targetPath: portable(relative(root, targetPath)), mounts, laws,
    workspaceDependencies: dependencies.filter(([, dependency]) => dependency.workspace === true).map(([alias]) => alias).sort(),
    pathDependencies: dependencies.filter(([, dependency]) => dependency.workspace !== true).map(([alias, dependency]) => ({ alias, package: dependency.package ?? alias, path: portable(relative(root, resolve(packageDir, dependency.path))), features: dependency.features ?? [], defaultFeatures: dependency["default-features"] ?? true, optional: dependency.optional ?? false })),
    kernelDependencies: Object.keys(kernel["dev-dependencies"]).sort(),
    kernelFeature: Object.hasOwn(kernel.features, "dsl-fixture-sweep-full"),
    kernelMount: read(join(root, kernelPath, "🦀️.rs")).includes('#[path = "../../🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🧹️fixture-sweep/🦀️.rs"]'),
    ignored: /#\[ignore/u.test(target),
  };
  const expected = { ...observed, module: fixture.moduleSha256, targetPath: fixture.targetPath, mounts: fixture.mounts, laws: fixture.laws, workspaceDependencies: fixture.workspaceDependencies, pathDependencies: fixture.pathDependencies, kernelDependencies: fixture.kernelDependencies, kernelFeature: false, kernelMount: true, ignored: false };
  const exact = new Ajv({ strict: true }).compile({ const: expected });
  assert.deepEqual(observed, expected, "fleet source bytes, laws, dependency edges and kernel-only mounts are preserved");
  assert(exact(observed));
  assert.equal(Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(target))).toString("hex"), fixture.moduleSha256, "independent module hash");
  const mutations: Record<string, (value: typeof observed) => void> = {
    "missing-workspace-dependency": value => { value.workspaceDependencies.pop(); },
    "duplicate-workspace-dependency": value => { value.workspaceDependencies.push(value.workspaceDependencies[0]!); },
    "changed-workspace-dependency": value => { value.workspaceDependencies[0] = "wrong"; },
    "reordered-workspace-dependency": value => { value.workspaceDependencies.reverse(); },
    "extra-path-dependency": value => { value.pathDependencies.push({ alias: "concrete", package: "concrete", path: "wrong", features: [], defaultFeatures: true, optional: false }); },
    "missing-law": value => { value.laws.pop(); },
    "ignored-law": value => { value.ignored = true; },
    "changed-module": value => { value.module = "0".repeat(64); },
    "moved-target": value => { value.targetPath = "🧰️framework/🔣️wrong.rs"; },
    "kernel-fleet-edge": value => { value.kernelDependencies.push("semio-s-fixture-sweep"); },
    "kernel-feature": value => { value.kernelFeature = true; },
    "missing-kernel-mount": value => { value.kernelMount = false; },
  };
  assert.deepEqual(Object.keys(mutations), fixture.cases);
  for (const name of fixture.cases) {
    const hostile = structuredClone(observed);
    mutations[name]!(hostile);
    assert(!exact(hostile), name);
    assert.notDeepEqual(hostile, expected, name);
  }
  assert.equal(manifest.package.name, fixture.package);
  assert.equal(manifest.test.length, 1);
  assert.equal(manifest.test[0].name, fixture.target);
  assert.equal(manifest.features, undefined);
  assert.equal(manifest["dev-dependencies"], undefined);
  const workspace = Bun.TOML.parse(read(join(root, "Cargo.toml"))) as any;
  assert.equal(workspace.workspace.members.filter((path: string) => path === `${sourcePath}/📦️packages/🦀️rust`).length, 1);
  const cargo = Bun.spawnSync(["cargo", "metadata", "--format-version", "1", "--no-deps", "--locked", "--offline"], { cwd: root, env: process.env });
  assert.equal(cargo.exitCode, 0, cargo.stderr.toString());
  const metadata = JSON.parse(cargo.stdout.toString());
  const actual = metadata.packages.find((item: any) => item.name === fixture.package);
  assert(actual, "Cargo independently discovers the concrete owner");
  assert.equal(actual.manifest_path, join(packageDir, "Cargo.toml"));
  assert.deepEqual(actual.targets.map((item: any) => ({ name: item.name, path: resolve(item.src_path), kind: item.kind })), [{ name: fixture.target, path: targetPath, kind: ["test"] }]);
  assert.deepEqual(actual.dependencies.map((item: any) => item.rename ?? item.name).sort(), fixture.workspaceDependencies);
  for (const dependency of actual.dependencies) {
    const alias = dependency.rename ?? dependency.name;
    const declaration = workspace.workspace.dependencies[alias];
    assert.equal(dependency.path, resolve(root, declaration.path), `Cargo and Bun resolve ${alias} identically`);
  }
  const renderer = metadata.packages.find((item: any) => item.name === fixture.rendererBoundary.package);
  assert(renderer, "Cargo independently discovers the renderer owner");
  assert.equal(renderer.manifest_path, join(root, fixture.rendererBoundary.owner, "Cargo.toml"));
  assert.equal(renderer.dependencies.filter((dependency: any) => dependency.path && ["✏️s", "🌎️hub"].some(area => relative(join(root, area), dependency.path).split(/[\\/]/u)[0] !== "..")).length, fixture.rendererBoundary.implementationEdges);
  assert.equal(renderer.dependencies.filter((dependency: any) => dependency.name === "semio-hub-puzzle").length, fixture.rendererBoundary.puzzleGuestEdges);
  const runner = read(join(packageDir, "📜️script.ts"));
  assert(runner.includes('RUST_TEST_NOCAPTURE: "1"') && runner.includes("runExactCargoLaws"), "exact native receipts remain observable");
  const project = JSON.parse(read(join(packageDir, "📋️project.json")));
  assert.equal(project.root, `${sourcePath}/📦️packages/🦀️rust`);
  assert.equal(project.sourceRoot, sourcePath);
  assert.equal(project.targets["test-quick"].options.command, "bun ./📜️script.ts source-check");
  assert.equal(project.targets["test-exhaustive"].options.command, "bun ./📜️script.ts test exhaustive");
  const domain = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
  assert(JSON.parse(read(join(root, domain, "📚️library/🔣️taxonomy.json"))).testPhases.includes("dsl"));
  const router = read(join(root, domain, "🧪️test/📜️script.ts"));
  assert(router.includes('.register("dsl", DslScript)') && router.includes(`"${project.name}:test"`), "root DSL orchestration routes to the concrete fleet owner");
  const inventory = exampleInventory(root);
  assert(inventory.directories.length > 0 && inventory.files.length > 0, "fixture discovery cannot be empty");
  console.log(`[dsl-fixture-sweep] ownership oracle: ${laws.length} fleet laws, ${dependencies.length} Cargo edges, ${fixture.cases.length} hostile ownership cases; ${inventory.directories.length} example directories, ${inventory.files.length} asset-first .semio files; discovery SHA-256 ${sha(JSON.stringify(inventory))}; native counts pending execution`);
}
