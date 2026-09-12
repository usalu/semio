import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { tmpdir } from "node:os";
import Ajv from "ajv";
import { canonicalGoPlan } from "../../📦️packages/🟦️typescript/🟦️.ts";

type Vector = Readonly<{
  contract: string;
  layout: Readonly<{ testsDirectory: string; implementationFilename: string; opaqueDirectoryNames: readonly string[]; sourceDirectoryNames: readonly string[] }>;
  module: string;
  dependency: Readonly<{ directory: string; module: string; domain: string; package: string; production: string }>;
  nativePackages: readonly Readonly<{ owner: string; package: string; production: string }>[];
  cases: readonly Readonly<{ owner: string; domain: string; case: string; package: string; production: string; productionImports?: readonly string[]; test: string }>[];
  opaqueCases: readonly Readonly<{ owner: string; domain: string; case: string; package: string; production: string; productionImports?: readonly string[]; test: string }>[];
  expectedPackages: readonly string[];
  expectedSources: readonly string[];
  expectedTests: readonly string[];
}>;

const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🐹️canonical-go-discovery/🔣️.json"), "utf8")) as Vector;
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🐹️canonical-go-discovery/🔣️.json"), "utf8"));

/** 🧱️Materializes one language-neutral discovery vector as an isolated Go module. */
function materialize(root: string): void {
  writeFileSync(join(root, "go.mod"), `module ${vector.module}\n\ngo 1.24\n\nrequire ${vector.dependency.module} v0.0.0\n\nreplace ${vector.dependency.module} => ./${vector.dependency.directory}\n`);
  const dependencyRoot = join(root, vector.dependency.directory);
  const dependencyDomain = join(dependencyRoot, vector.dependency.domain);
  mkdirSync(dependencyDomain, { recursive: true });
  writeFileSync(join(dependencyRoot, "go.mod"), `module ${vector.dependency.module}\n\ngo 1.24\n`);
  writeFileSync(join(dependencyDomain, "🐹️.go"), `package ${vector.dependency.package}\n${vector.dependency.production}\n`);
  for (const row of vector.nativePackages) {
    const owner = join(root, row.owner);
    mkdirSync(owner, { recursive: true });
    writeFileSync(join(owner, "🐹️.go"), `package ${row.package}\n${row.production}\n`);
  }
  for (const row of [...vector.cases, ...vector.opaqueCases]) {
    const owner = row.owner === "." ? root : join(root, row.owner);
    const sourceDomain = join(owner, row.domain);
    const testCase = join(owner, "🧪️tests", row.case);
    mkdirSync(sourceDomain, { recursive: true });
    mkdirSync(testCase, { recursive: true });
    const imports = row.productionImports?.length ? `import (${row.productionImports.map((name) => `\"${name}\"`).join("\n")})\n` : "";
    writeFileSync(join(sourceDomain, "🐹️.go"), `package ${row.package}\n${imports}${row.production}\n`);
    writeFileSync(join(testCase, "🐹️.go"), `package ${row.package}\nimport \"testing\"\n${row.test}\n`);
  }
}

test("canonical Go plans preserve private-package tests through the Go toolchain oracle", () => {
  expect(vector.contract).toBe("canonical-go-input-projection-v2");
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  const root = mkdtempSync(join(tmpdir(), "semio-canonical-go-plan-"));
  try {
    materialize(root);
    const plan = canonicalGoPlan(root, vector.layout);
    expect(plan.packages).toEqual(vector.expectedPackages);
    expect(Object.values(plan.testReplacements).map((path) => relative(realpathSync(root), path).replaceAll("\\", "/")).sort()).toEqual(vector.cases.map((row) => `${row.owner === "." ? "" : `${row.owner}/`}🧪️tests/${row.case}/🐹️.go`).sort());
    expect(Object.values(plan.sourceReplacements).map((path) => relative(realpathSync(root), path).replaceAll("\\", "/")).sort()).toEqual([...vector.expectedSources].sort());
    expect(Object.values(plan.replacements).map((path) => relative(realpathSync(root), path).replaceAll("\\", "/")).some((path) => path.startsWith("🧫️fixtures/"))).toBe(false);
    for (const source of vector.expectedSources) expect(Object.values(plan.replacements).map((path) => relative(realpathSync(root), path).replaceAll("\\", "/"))).toContain(source);
    const overlay = join(root, "overlay.json");
    writeFileSync(overlay, `${JSON.stringify({ Replace: plan.replacements }, null, 2)}\n`);
    const oracle = spawnSync("go", ["test", `-overlay=${overlay}`, "-v", ...plan.packages], { cwd: root, encoding: "utf8", env: { ...process.env, GOWORK: "off" } });
    expect(oracle.status, oracle.stderr).toBe(0);
    for (const name of vector.expectedTests) expect(oracle.stdout).toContain(`=== RUN   ${name}`);
    expect(oracle.stdout).not.toContain("TestFixtureMustStayOpaque");
    console.log("[DEBUG] Canonical Go discovery oracle", JSON.stringify({ packages: plan.packages, tests: vector.expectedTests }));
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}, 30_000);
