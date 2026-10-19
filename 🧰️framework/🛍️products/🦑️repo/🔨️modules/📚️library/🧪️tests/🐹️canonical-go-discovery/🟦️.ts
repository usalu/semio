import { expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { join, relative } from "node:path";
import { tmpdir } from "node:os";
import Ajv from "ajv";
import { canonicalGoPlan } from "../../🧪️testing/🐹️go-plan/🟦️.ts";

type Vector = Readonly<{
  contract: string;
  layout: Readonly<{ testsDirectory: string; implementationFilename: string; packagesDirectory: string; languageDirectory: string; opaqueDirectoryNames: readonly string[]; sourceDirectoryNames: readonly string[] }>;
  module: string;
  dependency: Readonly<{ directory: string; module: string; domain: string; package: string; production: string }>;
  nativePackages: readonly Readonly<{ owner: string; package: string; production: string }>[];
  cases: readonly Readonly<{ owner: string; domain: string; case: string; package: string; production: string; productionImports?: readonly string[]; test: string }>[];
  oraclePrograms: readonly Readonly<{ directory: string; production: string }>[];
  hostAdapterCases: readonly Readonly<{ owner: string; case: string; test: string }>[];
  opaqueCases: readonly Readonly<{ owner: string; domain: string; case: string; package: string; production: string; productionImports?: readonly string[]; test: string }>[];
  expectedPackages: readonly string[];
  expectedSources: readonly string[];
  expectedTests: readonly string[];
  packageModule: Readonly<{
    owner: string;
    module: string;
    package: string;
    production: string;
    members: readonly Readonly<{ domain: string; production: string }>[];
    cases: readonly Readonly<{ case: string; package: string; mapped: boolean; imports?: readonly string[]; test: string }>[];
    nestedModule: Readonly<{
      domain: string;
      module: string;
      package: string;
      production: string;
      members: readonly Readonly<{ domain: string; production: string }>[];
      expectedPackages: readonly string[];
      expectedSources: readonly string[];
    }>;
    expectedPackages: readonly string[];
    expectedSources: readonly string[];
    expectedTests: readonly string[];
    expectedGoTests: readonly string[];
  }>;
}>;

const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🐹️canonical-go-discovery/🔣️.json"), "utf8")) as Vector;


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
  for (const row of vector.oraclePrograms) {
    mkdirSync(join(root, row.directory), { recursive: true });
    writeFileSync(join(root, row.directory, "🐹️.go"), `package main\n${row.production}\n`);
  }
  for (const row of vector.hostAdapterCases) {
    const testCase = join(root, row.owner, "🧪️tests", row.case);
    mkdirSync(testCase, { recursive: true });
    writeFileSync(join(testCase, "🐹️.go"), `package adapter\n${row.test}\n`);
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
  expect(vector.contract).toBe("canonical-go-input-projection-v3");
  
  expect(vector["contract"]).toEqual("canonical-go-input-projection-v3");
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
    expect(Object.values(plan.replacements).some((path) => vector.hostAdapterCases.some((row) => path.includes(row.case)))).toBe(false);
    expect(oracle.stdout).not.toContain("TestHostAdapterMustStayOutside");
    for (const row of vector.oraclePrograms) {
      expect(plan.packages.some((name) => name.includes("oracles"))).toBe(false);
      expect(Object.values(plan.replacements).some((path) => path.includes(row.directory))).toBe(false);
      const program = spawnSync("go", ["run", join(root, row.directory, "🐹️.go")], { cwd: root, encoding: "utf8", env: { ...process.env, GOWORK: "off" } });
      expect(program.status, program.stderr).toBe(0);
      expect(program.stderr).toContain("oracle");
    }
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}, 30_000);

/** 📦️Materializes the package-root module of the language-neutral vector: sources below the module, cases at the owner. */
function materializePackageModule(root: string): string {
  const row = vector.packageModule;
  const owner = join(root, row.owner);
  const module = join(owner, "📦️packages", "🐹️go");
  mkdirSync(module, { recursive: true });
  writeFileSync(join(module, "go.mod"), `module ${row.module}\n\ngo 1.24\n`);
  writeFileSync(join(module, "🐹️.go"), `package ${row.package}\n${row.production}\n`);
  for (const member of row.members) {
    mkdirSync(join(module, member.domain), { recursive: true });
    writeFileSync(join(module, member.domain, "🐹️.go"), `package ${row.package}\n${member.production}\n`);
  }
  const nested = join(module, row.nestedModule.domain);
  mkdirSync(nested, { recursive: true });
  writeFileSync(join(nested, "go.mod"), `module ${row.nestedModule.module}\n\ngo 1.24\n`);
  writeFileSync(join(nested, "🐹️.go"), `package ${row.nestedModule.package}\n${row.nestedModule.production}\n`);
  for (const member of row.nestedModule.members) {
    mkdirSync(join(nested, member.domain), { recursive: true });
    writeFileSync(join(nested, member.domain, "🐹️.go"), `package ${row.nestedModule.package}\n${member.production}\n`);
  }
  for (const testCase of row.cases) {
    const directory = join(owner, "🧪️tests", testCase.case);
    mkdirSync(directory, { recursive: true });
    const imports = ["testing", ...(testCase.imports ?? [])].map((name) => `\"${name}\"`).join("\n");
    writeFileSync(join(directory, "🐹️.go"), `package ${testCase.package}\nimport (${imports})\n${testCase.test}\n`);
  }
  return module;
}

test("package-root Go modules compile registered member sources and owner cases of their own package", () => {
  const row = vector.packageModule;
  const root = mkdtempSync(join(tmpdir(), "semio-canonical-go-package-module-"));
  try {
    const module = materializePackageModule(root);
    const real = realpathSync(module);
    const plan = canonicalGoPlan(module, vector.layout);
    expect(plan.packages).toEqual(row.expectedPackages);
    expect(Object.values(plan.sourceReplacements).map((path) => relative(real, path).replaceAll("\\", "/")).sort()).toEqual([...row.expectedSources].sort());
    expect(Object.values(plan.testReplacements).map((path) => relative(real, path).replaceAll("\\", "/")).sort()).toEqual([...row.expectedTests].sort());
    const parent = canonicalGoPlan(join(root, row.owner, "📦️packages"), vector.layout);
    expect(Object.keys(parent.replacements)).toEqual([]);
    const overlay = join(root, "overlay.json");
    writeFileSync(overlay, `${JSON.stringify({ Replace: plan.replacements }, null, 2)}\n`);
    const oracle = spawnSync("go", ["test", `-overlay=${overlay}`, "-v", ...plan.packages], { cwd: module, encoding: "utf8", env: { ...process.env, GOWORK: "off" } });
    expect(oracle.status, oracle.stderr + oracle.stdout).toBe(0);
    for (const name of row.expectedGoTests) expect(oracle.stdout).toContain(`=== RUN   ${name}`);
    expect(oracle.stdout).not.toContain("TestPackageModuleAdapterMustStayOutside");
    const nested = join(module, row.nestedModule.domain);
    const nestedPlan = canonicalGoPlan(nested, vector.layout);
    expect(nestedPlan.packages).toEqual(row.nestedModule.expectedPackages);
    expect(Object.values(nestedPlan.sourceReplacements).map((path) => relative(realpathSync(nested), path).replaceAll("\\", "/")).sort()).toEqual([...row.nestedModule.expectedSources].sort());
    expect(Object.values(plan.replacements).some((path) => path.includes(row.nestedModule.domain))).toBe(false);
    const nestedOverlay = join(root, "nested-overlay.json");
    writeFileSync(nestedOverlay, `${JSON.stringify({ Replace: nestedPlan.replacements }, null, 2)}\n`);
    const nestedVet = spawnSync("go", ["vet", `-overlay=${nestedOverlay}`, ...nestedPlan.packages], { cwd: nested, encoding: "utf8", env: { ...process.env, GOWORK: "off" } });
    expect(nestedVet.status, nestedVet.stderr).toBe(0);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}, 30_000);

test("the library module compiles its root package and leaves its standalone oracle programs to go run", () => {
  const library = join(import.meta.dir, "..", "..");
  const plan = canonicalGoPlan(library);
  expect(plan.packages).toEqual(["."]);
  expect(Object.values(plan.replacements).some((path) => path.includes("🔮️oracles"))).toBe(false);
  const temporary = mkdtempSync(join(tmpdir(), "semio-canonical-go-library-"));
  try {
    const overlay = join(temporary, "overlay.json");
    writeFileSync(overlay, `${JSON.stringify({ Replace: plan.replacements }, null, 2)}\n`);
    const vet = spawnSync("go", ["vet", `-overlay=${overlay}`, ...plan.packages], { cwd: library, encoding: "utf8", env: { ...process.env, GOWORK: "off" } });
    expect(vet.status, vet.stderr).toBe(0);
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
}, 60_000);
