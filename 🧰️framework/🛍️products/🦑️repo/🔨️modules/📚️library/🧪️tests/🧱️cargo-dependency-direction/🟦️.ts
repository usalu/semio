import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import Ajv from "ajv/dist/2020.js";
import TOML from "@iarna/toml";
import { cargoDirectionInventory, cargoDirectionMetadata, cargoDependencyDirectionReport, type CargoDirectionPackage, type CargoDirectionPolicy, type CargoDirectionViolation, type CargoDirectionProblem } from "../../🕸️dependencies/🧭️direction/🦀️cargo/🟦️.ts";

const library = resolve(import.meta.dir, "../.."), repo = resolve(library, "../../../../..");
const read = (path: string): any => JSON.parse(readFileSync(join(library, path), "utf8"));
const fixture = read("🧫️fixtures/🧱️cargo-dependency-direction/🔣️.json") as { schemaVersion: number; policy: CargoDirectionPolicy; cases: readonly { id: string; packages: readonly CargoDirectionPackage[]; violations: readonly CargoDirectionViolation[]; problems: readonly CargoDirectionProblem[] }[]; corruptions: readonly string[]; policyCorruptions: readonly string[] };
const schema = read("🧬️schema/🧱️cargo-dependency-direction/🔣️.json");
const write = (root: string, path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };

test("portable Cargo declarations define exact physical and semantic verdicts", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(fixture.cases.map((row) => row.id)).size).toBe(fixture.cases.length);
  for (const row of fixture.cases) {
    const report = cargoDependencyDirectionReport(row.packages, row.packages, fixture.policy);
    expect(report.violations, row.id).toEqual(row.violations);
    expect(report.problems, row.id).toEqual(row.problems);
  }
});

test("independent Cargo metadata and TOML parsing validate every neutral verdict", async () => {
  if (!process.env.SEMIO_TEST_ARTIFACT_DIR) throw Error("Cargo direction laws require caller-owned SEMIO_TEST_ARTIFACT_DIR");
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR);
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "cargo-direction-")));
  try {
    for (const [index, row] of fixture.cases.entries()) {
      const members = (row.id === "implicit-local-member" ? row.packages.slice(0, 1) : row.packages).map((pkg) => pkg.owner);
      const cwd = join(root, String(index)), workspace: any = { workspace: { resolver: "2", members, metadata: { semio: { repository: { "schema-version": 1, "member-manifests": members.map(owner => owner + "/Cargo.toml") } } } } };
      for (const pkg of row.packages) {
        const manifest: any = { package: { name: pkg.name, version: "0.1.0", edition: "2021", metadata: { unrelated: { role: "framework" }, semio: pkg.role === null ? {} : { role: pkg.role } } }, lib: { path: "lib.rs" } };
        for (const dependency of pkg.dependencies) {
          const table = dependency.kind === "dev" ? "dev-dependencies" : dependency.kind === "build" ? "build-dependencies" : "dependencies";
          const parent = dependency.platform ? ((manifest.target ??= {})[dependency.platform] ??= {}) : manifest;
          const value: any = { package: dependency.name, optional: dependency.optional };
          if (dependency.owner) value.path = relative(pkg.owner, dependency.owner).replaceAll("\\", "/");
          else value.version = "0.1.0";
          if (row.id === "upward-development-renamed") {
            (workspace.workspace.dependencies ??= {})[dependency.alias] = { ...value, path: dependency.owner };
            (parent[table] ??= {})[dependency.alias] = { workspace: true };
          } else (parent[table] ??= {})[dependency.alias] = value;
        }
        const text = TOML.stringify(manifest);
        expect((TOML.parse(text) as any).package.metadata.semio.role ?? null, row.id).toBe(pkg.role);
        write(cwd, `${pkg.owner}/Cargo.toml`, text);
        write(cwd, `${pkg.owner}/lib.rs`, "pub fn neutral() {}\n");
      }
      write(cwd, "Cargo.toml", TOML.stringify(workspace));
      const inventory = cargoDirectionInventory(cwd);
      expect([...inventory].sort((a, b) => a.owner.localeCompare(b.owner)), row.id).toEqual([...row.packages].sort((a: CargoDirectionPackage, b: CargoDirectionPackage) => a.owner.localeCompare(b.owner)));
      const process = Bun.spawn(["cargo", "metadata", "--format-version", "1", "--no-deps", "--offline"], { cwd, stdout: "pipe", stderr: "pipe" });
      const [stdout, stderr, status] = await Promise.all([new Response(process.stdout).text(), new Response(process.stderr).text(), process.exited]);
      expect(status, `${row.id}: ${stderr}`).toBe(0);
      const graph = cargoDirectionMetadata(JSON.parse(stdout), cwd);
      const missingMember = JSON.parse(stdout);
      missingMember.workspace_members.pop();
      expect(() => cargoDirectionMetadata(missingMember, cwd), row.id).toThrow();
      const report = cargoDependencyDirectionReport(graph, inventory, fixture.policy);
      expect(report.violations, row.id).toEqual(row.violations);
      expect(report.problems, row.id).toEqual(row.problems);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 45_000);

test("Cargo graph rejects omissions, altered declarations and unclassified owners", () => {
  const inventory = fixture.cases.find((row) => row.id === "upward-build-optional-platform")!.packages;
  for (const id of fixture.corruptions) {
    const graph: any = structuredClone(inventory);
    if (id === "missing-package" || id === "missing-member" || id === "missing-local-target") graph.pop();
    if (id === "missing-dependency") graph[0].dependencies = [];
    if (id === "wrong-kind") graph[0].dependencies[0].kind = "normal";
    if (id === "wrong-platform") graph[0].dependencies[0].platform = null;
    if (id === "wrong-optional") graph[0].dependencies[0].optional = false;
    if (id === "wrong-role") graph[0].role = "framework";
    if (id === "duplicate-package") graph.push(graph[0]);
    if (id === "unclassified-owner") graph[0].owner = "unknown/source";
    expect(() => cargoDependencyDirectionReport(graph, inventory, fixture.policy), id).toThrow();
  }
});

test("authored Cargo role vocabulary satisfies the same portable policy contract", () => {
  const taxonomy = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema.$defs.Policy);
  const policy = { areaLayers: taxonomy.areaLayers, ...taxonomy.cargoDependencyDirections };
  expect(validate(policy), JSON.stringify(validate.errors)).toBe(true);
  expect(policy.roles).toEqual(fixture.policy.roles);
  expect(taxonomy.roles).toEqual(policy.roles);
  expect(Object.keys(policy.rules)).toEqual(Object.keys(fixture.policy.rules));
  for (const [name, rule] of Object.entries(policy.rules) as [string, CargoDirectionPolicy["rules"][string]][]) {
    const witness = fixture.policy.rules[name]!;
    expect(rule.fromRoles, name).toEqual(witness.fromRoles);
    expect(rule.toRoles, name).toEqual(witness.toRoles);
    expect(rule.toOwnerSegments, name).toEqual(witness.toOwnerSegments);
    expect(rule.fromOwnerPaths?.length ?? 0, name).toBe(witness.fromOwnerPaths?.length ?? 0);
  }
});

test("owner classification rejects omitted, malformed and contradictory authority", () => {
  const packages = fixture.cases[0]!.packages;
  for (const id of fixture.policyCorruptions) {
    const policy: any = structuredClone(fixture.policy);
    if (id === "missing-owner-roles") delete policy.ownerRoles;
    if (id === "invalid-owner-pattern") policy.ownerRoles[0].path = "[";
    if (id === "unknown-owner-role") policy.ownerRoles[0].roles = ["unclassified"];
    if (id === "duplicate-owner-rule") policy.ownerRoles.push(policy.ownerRoles[0]);
    if (id === "empty-owner-pattern") policy.ownerRoles[0].path = "";
    if (id === "empty-source-selector") { policy.rules["cargo-framework-no-products"].fromRoles = []; delete policy.rules["cargo-framework-no-products"].fromOwnerPaths; }
    if (id === "empty-target-selector") { policy.rules["cargo-framework-no-products"].toRoles = []; delete policy.rules["cargo-framework-no-products"].toOwnerSegments; }
    if (id === "duplicate-source-role") policy.rules["cargo-framework-no-implementation-role"].fromRoles = ["framework", "framework"];
    if (id === "duplicate-target-role") policy.rules["cargo-framework-no-implementation-role"].toRoles = ["hub", "hub"];
    if (id === "unknown-rule-field") policy.rules["cargo-framework-no-products"].exceptions = ["framework/modules/general"];
    expect(() => cargoDependencyDirectionReport(packages, packages, policy), id).toThrow();
  }
});

test("production extension classification preserves the portable ownership verdict", () => {
  const taxonomy = JSON.parse(readFileSync(join(library, "🔣️taxonomy.json"), "utf8"));
  const policy = { areaLayers: taxonomy.areaLayers, ...taxonomy.cargoDependencyDirections };
  const witness = fixture.cases.find((row) => row.id === "physical-extension-cannot-declare-plugin")!;
  const owner = "✏️s/🔌️plugins/neutral/🧩️extensions/independent/📦️packages/🦀️rust";
  const packages = witness.packages.map((pkg) => ({ ...pkg, owner }));
  const report = cargoDependencyDirectionReport(packages, packages, policy);
  expect(report.violations).toEqual(witness.violations);
  expect(report.problems).toEqual(witness.problems.map((problem) => ({ ...problem, owner })));
});
