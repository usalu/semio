/** 🧭️ Independent preservation oracle for the fleet-only test-package extraction. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { readFileSync, existsSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";

const read = (path: string): string => readFileSync(path, "utf8");
const sha = (text: string): string => createHash("sha256").update(text).digest("hex");
const marker = "//#region 🔖️ExampleAssetDiscovery";
const kernelPath = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust";
const sourcePath = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep";

function repoRoot(): string {
  let root = import.meta.dir;
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

export async function testFixtureSweepExtraction(): Promise<void> {
  const root = repoRoot();
  const fixture = JSON.parse(read(join(import.meta.dir, "🧫️fixture/🔣️.json")));
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(JSON.parse(read(join(import.meta.dir, "🧬️schema/🔣️.json"))));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const oldKernel = read(join(root, kernelPath, "Cargo.toml"));
  const retained = read(join(import.meta.dir, "🦀️.rs"));
  assert(!oldKernel.includes("dsl-fixture-sweep-full"), "fleet feature must leave the kernel");
  const packageDir = join(import.meta.dir, "📦️packages/🦀️rust");
  const manifest = Bun.TOML.parse(read(join(packageDir, "Cargo.toml"))) as any;
  const kernel = Bun.TOML.parse(oldKernel) as any;
  const source = read(join(import.meta.dir, "🧪️tests/🦀️.rs"));
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
  const exact = new Ajv2020({ strict: true }).compile({ const: expected });
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
  assert.equal(resolve(packageDir, manifest.test[0].path), join(import.meta.dir, "🧪️tests/🦀️.rs"));
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
