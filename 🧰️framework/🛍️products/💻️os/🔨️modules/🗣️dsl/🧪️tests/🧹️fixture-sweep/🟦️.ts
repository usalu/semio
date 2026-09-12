import { fileURLToPath as testFileUrlToPath } from "node:url";
const testSourceUrl = new URL("../../🧹️fixture-sweep/📜️script.ts", import.meta.url);
/** 🧭️ Independent preservation oracle for the fleet-only test-package extraction. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";

const read = (path: string): string => readFileSync(path, "utf8");
const sha = (text: string): string => createHash("sha256").update(text).digest("hex");
const marker = "//#region 🔖️ExampleAssetDiscovery";
const kernelPath = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const sourcePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep";

interface FixtureSweepReceipt {
  readonly artifactDir: string;
  readonly assertions: number;
  readonly sha256: string;
}

interface FixtureSweepEvidence {
  readonly summary: string;
  readonly coverage: string;
}

interface FixtureSweepLawGroup {
  readonly package: string;
  readonly target: { readonly kind: "test"; readonly name: string };
  readonly laws: readonly string[];
}

export function fixtureSweepLawGroup(): FixtureSweepLawGroup {
  const fixture = JSON.parse(read(testFileUrlToPath(new URL("../../../🧹️fixture-sweep/🧫️fixtures/🔣️.json", import.meta.url))));
  return { package: fixture.package, target: { kind: "test", name: fixture.target }, laws: fixture.laws };
}

function assertFixtureSweepOutput(output: string, assertions: number): FixtureSweepEvidence {
  const sweep = [...output.matchAll(/\[dsl-fixture-sweep\] (\d+) example dir\(s\), (\d+) \.semio fixture file\(s\) found, (\d+) law-check\(s\) run across (\d+) registered app kind\(s\), (\d+) unmapped fixture\(s\)/gu)];
  const coverage = [...output.matchAll(/example asset coverage: (\d+) slug\(s\) on new 🖼️assets layout, (\d+) soft-skipped mid-migration/gu)];
  assert.equal(assertions, 2, "the native runner must execute both fixture-sweep laws");
  assert.equal(sweep.length, 1, "one actual fleet sweep summary is required");
  assert.equal(coverage.length, 1, "one actual asset coverage summary is required");
  assert(Number(sweep[0]![1]) > 0 && Number(sweep[0]![2]) > 0 && Number(sweep[0]![3]) > 0, "an empty or all-unmapped fleet cannot pass");
  assert.equal(Number(sweep[0]![4]), 54, "the native sweep must cover every registered app kind");
  assert(Number(coverage[0]![1]) > 0, "all-soft-skipped asset coverage cannot pass");
  return { summary: sweep[0]![0], coverage: coverage[0]![0] };
}

/** 🧭️ Verifies native law receipts without placing domain parsing inside the package command leaf. */
export function assertFixtureSweepLawCoverage(receipts: readonly FixtureSweepReceipt[]): void {
  assert.equal(receipts.length, 1, "one exact fixture-sweep receipt is required");
  const receipt = receipts[0]!;
  const output = [0, 1].map(index => [".stdout", ".stderr"].map(suffix => readFileSync(join(receipt.artifactDir, `law-${index}${suffix}`), "utf8")).join("\n")).join("\n");
  const evidence = assertFixtureSweepOutput(output, receipt.assertions);
  console.log(`[DEBUG] ${evidence.summary}; ${evidence.coverage}; exact assertions=2; executable=${receipt.sha256}; evidence=${receipt.artifactDir}`);
}

function repoRoot(): string {
  let root = testFileUrlToPath(new URL(".", testSourceUrl));
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
  const fixture = JSON.parse(read(join(testFileUrlToPath(new URL(".", testSourceUrl)), "🧫️fixtures/🔣️.json")));
  const document = JSON.parse(read(join(testFileUrlToPath(new URL(".", testSourceUrl)), "🧬️schema/🔣️.json")));
  const sweepAjv = new Ajv({ strict: true, allErrors: true });
  sweepAjv.addSchema(document);
  const validate = sweepAjv.getSchema(`${document.$id}#/$defs/DslFixtureSweepExtractionV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const oldKernel = read(join(root, kernelPath, "Cargo.toml"));
  const retained = read(join(testFileUrlToPath(new URL(".", testSourceUrl)), "🦀️.rs"));
  assert(!oldKernel.includes("dsl-fixture-sweep-full"), "fleet feature must leave the kernel");
  const packageDir = join(testFileUrlToPath(new URL(".", testSourceUrl)), "📦️packages/🦀️rust");
  const manifest = Bun.TOML.parse(read(join(packageDir, "Cargo.toml"))) as any;
  const kernel = Bun.TOML.parse(oldKernel) as any;
  const source = read(join(testFileUrlToPath(new URL(".", testSourceUrl)), "🧪️tests/🧹️fixture-sweep/🦀️.rs"));
  assert(source.startsWith("//! 🧭️ Full-fleet example laws; public kernel APIs and production providers only.\n\n#[cfg(test)]\nmod tests"), "the extracted module has no additional feature or ignore guard");
  const body = source.slice(source.indexOf("mod tests")).replaceAll("semio_framework_os_kernel::", "crate::").trimEnd() + "\n\n";
  const retainedBody = retained.slice(retained.indexOf(marker));
  const observed = {
    module: sha(body), retained: sha(retainedBody),
    registry: body.split("//#region 🔖️Registry")[1]!.split("//#endregion")[0]!.split("\n").filter(line => /^\s*\("/u.test(line)).map(line => line.trim()),
    laws: [...body.matchAll(/async fn (repo_wide_\w+)/gu)].map(match => `tests::${match[1]}`),
    dependencies: Object.entries(manifest.dependencies).filter(([alias]) => alias !== "semio-framework-os-kernel").map(([alias, dependency]: [string, any]) => ({ alias, package: dependency.package ?? alias, path: relative(root, resolve(packageDir, dependency.path)), features: dependency.features ?? [], defaultFeatures: dependency["default-features"] ?? true, optional: dependency.optional ?? false })),
    kernelDependencies: Object.keys(kernel["dev-dependencies"]),
    kernelFeature: Object.hasOwn(kernel.features, "dsl-fixture-sweep-full"),
    kernelMount: read(join(root, kernelPath, "🦀️.rs")).includes('#[path = "../../🔨️modules/🗣️dsl/🧹️fixture-sweep/🦀️.rs"]'),
    ignored: /#\[ignore/u.test(body),
  };
  const expected = {
    ...observed, module: fixture.moduleSha256, retained: fixture.retainedSha256, registry: fixture.registry, laws: fixture.laws,
    dependencies: fixture.dependencies.map((dependency: object) => ({ ...dependency, optional: false })),
    kernelDependencies: ["semio-framework-async-macros"], kernelFeature: false, kernelMount: true, ignored: false,
  };
  const exact = new Ajv({ strict: true }).compile({ const: expected });
  assert.deepEqual(observed, expected, "entire fleet registry, test/discovery bytes, dependencies and kernel-only tests are preserved");
  assert(exact(observed));
  for (const [name, text, digest] of [["fleet", body, fixture.moduleSha256], ["kernel", retainedBody, fixture.retainedSha256]]) {
    const independent = Buffer.from(await crypto.subtle.digest("SHA-256", new TextEncoder().encode(text))).toString("hex");
    assert.equal(independent, digest, name);
  }
  const mutations: Record<string, (value: typeof observed) => void> = {
    "missing-dependency": value => { value.dependencies.pop(); },
    "extra-dependency": value => { value.dependencies.push({ ...value.dependencies[0]!, alias: "extra" }); },
    "changed-package": value => { value.dependencies[0]!.package = "wrong"; },
    "optional-dependency": value => { value.dependencies[0]!.optional = true; },
    "changed-features": value => { value.dependencies[0]!.features = ["wrong"]; },
    "missing-registry": value => { value.registry.pop(); },
    "duplicate-registry": value => { value.registry.push(value.registry[0]!); },
    "changed-registry": value => { value.registry[0] = "wrong"; },
    "reordered-registry": value => { value.registry.reverse(); },
    "missing-law": value => { value.laws.pop(); },
    "ignored-law": value => { value.ignored = true; },
    "changed-module": value => { value.module = "0".repeat(64); },
    "changed-kernel-tests": value => { value.retained = "0".repeat(64); },
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
  assert.equal(resolve(packageDir, manifest.test[0].path), join(testFileUrlToPath(new URL(".", testSourceUrl)), "🧪️tests/🧹️fixture-sweep/🦀️.rs"));
  assert.equal(resolve(packageDir, manifest.dependencies["semio-framework-os-kernel"].path), join(root, kernelPath));
  assert(!manifest.dependencies["semio-framework-os-kernel"].optional);
  assert.equal(manifest.features, undefined);
  assert.equal(manifest["dev-dependencies"], undefined);
  for (const dependency of fixture.dependencies) assert(existsSync(join(root, dependency.path, "Cargo.toml")), `dependency owner exists: ${dependency.alias}`);
  const runner = read(join(packageDir, "📜️script.ts"));
  assert(runner.includes('RUST_TEST_NOCAPTURE: "1"') && runner.includes("runExactCargoLaws"), "real counts and exact native terminals remain observable");
  const project = JSON.parse(read(join(packageDir, "📋️project.json")));
  assert.equal(project.targets["test-quick"].options.command, "bun ./📜️script.ts source-check");
  assert.equal(project.targets["test-exhaustive"].options.command, "bun ./📜️script.ts test exhaustive");
  const domain = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
  assert(JSON.parse(read(join(root, domain, "📚️library/🔣️taxonomy.json"))).testPhases.includes("dsl"));
  const router = read(join(root, domain, "🧪️test/📜️script.ts"));
  assert(router.includes('.register("dsl", DslScript)') && router.includes('"@semio-tech/dsl-fixture-sweep-rs:test"'), "the root DSL phase routes to the dedicated fleet leaf");
  const workspace = Bun.TOML.parse(read(join(root, "Cargo.toml"))) as any;
  assert.equal(workspace.workspace.members.filter((path: string) => path === `${sourcePath}/📦️packages/🦀️rust`).length, 1);
  const inventory = exampleInventory(root);
  assert(inventory.directories.length > 0 && inventory.files.length > 0, "fixture discovery cannot be empty");
  console.log(`[DEBUG] DSL extraction oracle: 54 registry rows, 28 moved fleet edges, 1 retained/shared async macro, 2 preserved laws, ${fixture.cases.length} hostile cases; ${inventory.directories.length} example directories, ${inventory.files.length} asset-first .semio files, discovery SHA-256 ${sha(JSON.stringify(inventory))}; native law-check/unmapped counts pending execution`);
}
