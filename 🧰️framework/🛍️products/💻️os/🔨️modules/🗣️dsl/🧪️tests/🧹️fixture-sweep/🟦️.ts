import { fileURLToPath as testFileUrlToPath } from "node:url";
const sweepRoot = testFileUrlToPath(new URL("../../🧹️fixture-sweep/", import.meta.url));
/** 🧭️ Independent preservation oracle for the kernel-only M5 conformance test package. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";

const read = (path: string): string => readFileSync(path, "utf8");
const sha = (text: string): string => createHash("sha256").update(text).digest("hex");
const kernelPath = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const sourcePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep";

interface FixtureSweepReceipt {
  readonly artifactDir: string;
  readonly assertions: number;
  readonly sha256: string;
}

interface FixtureSweepEvidence {
  readonly grammar: string;
  readonly protocol: string;
  readonly coverage: string;
}

interface FixtureSweepLawGroup {
  readonly package: string;
  readonly target: { readonly kind: "test"; readonly name: string };
  readonly laws: readonly string[];
}

export function fixtureSweepLawGroup(): FixtureSweepLawGroup {
  const fixture = JSON.parse(read(join(sweepRoot, "🧫️fixtures/🔣️.json")));
  return { package: fixture.package, target: { kind: "test", name: fixture.target }, laws: fixture.laws };
}

function assertFixtureSweepOutput(output: string, assertions: number): FixtureSweepEvidence {
  const grammar = [...output.matchAll(/\[dsl-fixture-sweep\] m5 grammar auto-discovery: (\d+) facet\(s\) found, (\d+) checked, (\d+) soft-skipped, (\d+) stdio-exempt soft failure\(s\), (\d+) hard failure\(s\)/gu)];
  const protocol = [...output.matchAll(/\[dsl-fixture-sweep\] m5 protocol auto-discovery: (\d+) facet\(s\) found, (\d+) checked, (\d+) soft-skipped, (\d+) stdio-exempt-or-known-gap soft failure\(s\), (\d+) hard failure\(s\)/gu)];
  const coverage = [...output.matchAll(/\[dsl-fixture-sweep\] m5 production coverage auto-discovery: (\d+) facet\(s\) found, (\d+) checked, (\d+) stdio-exempt soft failure\(s\), (\d+) hard failure\(s\)/gu)];
  assert.equal(assertions, fixtureSweepLawGroup().laws.length, "the native runner must execute every declared M5 law");
  assert.equal(grammar.length, 1, "one actual grammar auto-discovery summary is required");
  assert.equal(protocol.length, 1, "one actual protocol auto-discovery summary is required");
  assert.equal(coverage.length, 1, "one actual production coverage summary is required");
  assert(Number(grammar[0]![1]) > 0 && Number(grammar[0]![2]) > 0, "an empty or all-soft-skipped grammar sweep cannot pass");
  assert(Number(protocol[0]![1]) > 0 && Number(protocol[0]![2]) > 0, "an empty or all-soft-skipped protocol sweep cannot pass");
  assert(Number(coverage[0]![1]) > 0 && Number(coverage[0]![2]) > 0, "an empty or all-soft-skipped production coverage sweep cannot pass");
  assert.equal(Number(grammar[0]![5]) + Number(protocol[0]![5]) + Number(coverage[0]![4]), 0, "a hard failure cannot be reported by a passing law");
  return { grammar: grammar[0]![0], protocol: protocol[0]![0], coverage: coverage[0]![0] };
}

/** 🧭️ Verifies native law receipts without placing domain parsing inside the package command leaf. */
export function assertFixtureSweepLawCoverage(receipts: readonly FixtureSweepReceipt[]): void {
  assert.equal(receipts.length, 1, "one exact fixture-sweep receipt is required");
  const receipt = receipts[0]!;
  const laws = fixtureSweepLawGroup().laws;
  const output = laws.map((_law, index) => [".stdout", ".stderr"].map(suffix => readFileSync(join(receipt.artifactDir, `law-${index}${suffix}`), "utf8")).join("\n")).join("\n");
  const evidence = assertFixtureSweepOutput(output, receipt.assertions);
  console.log(`[DEBUG] ${evidence.grammar}; ${evidence.protocol}; ${evidence.coverage}; exact assertions=${laws.length}; executable=${receipt.sha256}; evidence=${receipt.artifactDir}`);
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
  const independentlyWalked: string[] = [];
  const collect = (path: string): void => {
    for (const entry of readdirSync(path, { withFileTypes: true })) {
      const child = join(path, entry.name);
      if (entry.isDirectory()) collect(child);
      else if (entry.isFile() && entry.name.endsWith(".semio")) independentlyWalked.push(relative(root, child));
    }
  };
  for (const directory of directories) {
    for (const slug of readdirSync(directory, { withFileTypes: true }).filter(entry => entry.isDirectory())) {
      const path = join(directory, slug.name);
      const assets = join(path, "🖼️assets");
      const search = existsSync(assets) ? assets : path;
      collect(search);
      for (const file of new Bun.Glob("**/*.semio").scanSync({ cwd: search, onlyFiles: true, dot: true })) files.push(relative(root, join(search, file)));
    }
  }
  assert.deepEqual(files.sort(), independentlyWalked.sort(), "Bun glob and independent recursive fixture discovery agree exactly");
  return { directories: directories.map(path => relative(root, path)).sort(), files };
}

export function testFixtureSweepReportContract(): void {
  const reportFixture = JSON.parse(read(testFileUrlToPath(new URL("🧫️fixtures/🔣️.json", import.meta.url))));
  const reportSchema = JSON.parse(read(testFileUrlToPath(new URL("🧬️schema/🔣️.json", import.meta.url))));
  const reportAjv = new Ajv2020({ strict: true, allErrors: true });
  assert(reportAjv.validate(reportSchema, reportFixture), JSON.stringify(reportAjv.errors));
  assert.deepEqual(assertFixtureSweepOutput(reportFixture.valid.output, reportFixture.valid.assertions), reportFixture.valid.expected);
  for (const hostile of reportFixture.hostile) assert.throws(() => assertFixtureSweepOutput(hostile.output, hostile.assertions), undefined, hostile.id);
}

export async function testFixtureSweepExtraction(): Promise<void> {
  const root = repoRoot();
  testFixtureSweepReportContract();
  const fixture = JSON.parse(read(join(sweepRoot, "🧫️fixtures/🔣️.json")));
  const document = JSON.parse(read(join(sweepRoot, "🧬️schema/🔣️.json")));
  const sweepAjv = new Ajv({ strict: true, allErrors: true });
  sweepAjv.addSchema(document);
  const validate = sweepAjv.getSchema(`${document.$id}#/$defs/DslFixtureSweepExtractionV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const oldKernel = read(join(root, kernelPath, "Cargo.toml"));
  assert(!oldKernel.includes("dsl-fixture-sweep-full"), "fleet feature must leave the kernel");
  const packageDir = join(sweepRoot, "📦️packages/🦀️rust");
  const manifest = Bun.TOML.parse(read(join(packageDir, "Cargo.toml"))) as any;
  const kernel = Bun.TOML.parse(oldKernel) as any;
  const targetPath = resolve(packageDir, manifest.test[0].path);
  const target = read(targetPath);
  assert(target.startsWith("//! 🧭️ Kernel-only M5 grammar, protocol and fixture-discovery conformance.\n"), "the mounted target keeps its kernel-only charter");
  const mounts = [...target.matchAll(/#\[path = "([^"]+)"\]\s*\n\s*mod (\w+);/gu)].map(match => ({ name: match[2]!, path: match[1]! }));
  const laws: string[] = [];
  for (const mount of mounts) {
    const body = read(resolve(targetPath, "..", mount.path));
    for (const match of body.matchAll(/#\[semio_framework_async_macros::async_test\]\s*\nasync fn (\w+)\(/gu)) laws.push(`${mount.name}::${match[1]}`);
  }
  const dependencies = Object.entries(manifest.dependencies as Record<string, any>);
  const observed = {
    module: sha(target), targetPath: relative(root, targetPath).replaceAll("\\", "/"), mounts, laws: laws.sort(),
    workspaceDependencies: dependencies.filter(([, dependency]) => dependency.workspace === true).map(([alias]) => alias).sort(),
    pathDependencies: dependencies.filter(([, dependency]) => dependency.workspace !== true).map(([alias, dependency]: [string, any]) => ({ alias, package: dependency.package ?? alias, path: relative(root, resolve(packageDir, dependency.path)).replaceAll("\\", "/"), features: dependency.features ?? [], defaultFeatures: dependency["default-features"] ?? true, optional: dependency.optional ?? false })),
    kernelDependencies: Object.keys(kernel["dev-dependencies"]).sort(),
    kernelFeature: Object.hasOwn(kernel.features, "dsl-fixture-sweep-full"),
    kernelMount: read(join(root, kernelPath, "🦀️.rs")).includes(`#[path = "../../🔨️modules/🗣️dsl/🧹️fixture-sweep/🧪️tests/🧹️fixture-sweep/🦀️.rs"]`),
    ignored: /#\[ignore/u.test(target),
  };
  const expected = {
    ...observed, module: fixture.moduleSha256, targetPath: fixture.targetPath, mounts: fixture.mounts, laws: fixture.laws,
    workspaceDependencies: fixture.workspaceDependencies,
    pathDependencies: fixture.pathDependencies.map((dependency: object) => ({ ...dependency, optional: false })),
    kernelDependencies: fixture.kernelDependencies, kernelFeature: false, kernelMount: true, ignored: false,
  };
  const exact = new Ajv({ strict: true }).compile({ const: expected });
  assert.deepEqual(observed, expected, "the mounted target, its module mounts, laws, dependency edges and kernel-only boundary are preserved");
  assert(exact(observed));
  const independent = Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(target))).toString("hex");
  assert.equal(independent, fixture.moduleSha256, "module");
  const mutations: Record<string, (value: typeof observed) => void> = {
    "missing-path-dependency": value => { value.pathDependencies.pop(); },
    "extra-path-dependency": value => { value.pathDependencies.push({ ...value.pathDependencies[0]!, alias: "extra" }); },
    "changed-package": value => { value.pathDependencies[0]!.package = "wrong"; },
    "optional-dependency": value => { value.pathDependencies[0]!.optional = true; },
    "changed-features": value => { value.pathDependencies[0]!.features = ["wrong"]; },
    "missing-workspace-dependency": value => { value.workspaceDependencies.pop(); },
    "duplicate-workspace-dependency": value => { value.workspaceDependencies.push(value.workspaceDependencies[0]!); },
    "changed-workspace-dependency": value => { value.workspaceDependencies[0] = "wrong"; },
    "reordered-workspace-dependency": value => { value.workspaceDependencies.reverse(); },
    "missing-mount": value => { value.mounts.pop(); },
    "changed-mount-path": value => { value.mounts[0]!.path = "../🔬️wrong/🦀️.rs"; },
    "missing-law": value => { value.laws.pop(); },
    "ignored-law": value => { value.ignored = true; },
    "changed-module": value => { value.module = "0".repeat(64); },
    "moved-target": value => { value.targetPath = "🧰️framework/🔣️wrong.rs"; },
    "kernel-fleet-edge": value => { value.kernelDependencies.push("stdio"); },
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
  assert.equal(targetPath, join(sweepRoot, "🧪️tests/🧹️fixture-sweep/🦀️.rs"));
  assert.equal(resolve(packageDir, "../../../../../📦️packages/🦀️rust"), join(root, kernelPath));
  assert.equal(manifest.dependencies["semio-framework-os-kernel"].workspace, true);
  assert.equal(manifest.features, undefined);
  assert.equal(manifest["dev-dependencies"], undefined);
  for (const dependency of fixture.pathDependencies) assert(existsSync(join(root, dependency.path, "Cargo.toml")), `dependency owner exists: ${dependency.alias}`);
  const workspace = Bun.TOML.parse(read(join(root, "Cargo.toml"))) as any;
  for (const alias of fixture.workspaceDependencies) assert(Object.hasOwn(workspace.workspace.dependencies, alias), `workspace dependency is declared: ${alias}`);
  const runner = read(join(packageDir, "📜️script.ts"));
  assert(runner.includes('RUST_TEST_NOCAPTURE: "1"') && runner.includes("runExactCargoLaws"), "real counts and exact native terminals remain observable");
  const project = JSON.parse(read(join(packageDir, "📋️project.json")));
  assert.equal(project.targets["test-quick"].options.command, "bun ./📜️script.ts source-check");
  assert.equal(project.targets["test-exhaustive"].options.command, "bun ./📜️script.ts test exhaustive");
  const domain = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
  assert(JSON.parse(read(join(root, domain, "📚️library/🔣️taxonomy.json"))).testPhases.includes("dsl"));
  const router = read(join(root, domain, "🧪️test/📜️script.ts"));
  assert(router.includes('.register("dsl", DslScript)') && router.includes('"@semio-tech/dsl-fixture-sweep-rs:test"'), "the root DSL phase routes to the dedicated fleet leaf");
  assert.equal(workspace.workspace.members.filter((path: string) => path === `${sourcePath}/📦️packages/🦀️rust`).length, 1);
  const inventory = exampleInventory(root);
  assert(inventory.directories.length > 0 && inventory.files.length > 0, "fixture discovery cannot be empty");
  console.log(`[DEBUG] DSL extraction oracle: ${observed.mounts.length} mounted M5 modules, ${observed.laws.length} preserved laws, ${observed.workspaceDependencies.length} workspace edges, ${observed.pathDependencies.length} path edges, ${fixture.cases.length} hostile cases; ${inventory.directories.length} example directories, ${inventory.files.length} asset-first .semio files, discovery SHA-256 ${sha(JSON.stringify(inventory))}; native law counts pending execution`);
}
