import { expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import Ajv from "ajv/dist/2020";
import consistencySchema from "../../../🧬️schema/🧾️owner-consistency.json" with { type: "json" };
import registrySchema from "../../../🧬️schema/📋️owners.json" with { type: "json" };
import { BUILD_OWNER_REGISTRY_FILE, cargoPackageName, checkBuildOwnerRegistry, findOwnerFindings, memoryOwnerTree, moveOwnerScopes, reconcileBuildOwnerRegistry, syncOwnerRegistry, type OwnerFinding, type OwnerRow, type OwnerTreePolicy } from "../🟦️.ts";
import { readBuildOwnerRegistryV1 } from "../../🚪️admission/🟦️.ts";
import fixture from "./🔣️.json" with { type: "json" };

const policy: OwnerTreePolicy = fixture.policy;
const subject = (finding: OwnerFinding): string => `${finding.kind}:${finding.owner ?? finding.path}`;
const text = (value: unknown): string => (typeof value === "string" ? value : JSON.stringify(value));
const filesOf = (files: Readonly<Record<string, unknown>>): Record<string, string> => Object.fromEntries(Object.entries(files).map(([path, value]) => [path, text(value)]));

test("the fixture cases satisfy their schema", () => {
  const oracle = new Ajv({ strict: false }).compile(consistencySchema);
  expect(oracle(fixture), JSON.stringify(oracle.errors)).toBe(true);
});

for (const row of fixture.moveScopes) {
  test(`move scopes: ${row.name}`, () => {
    expect(moveOwnerScopes(row.moves.map(([sourcePath, destinationPath]) => ({ sourcePath, destinationPath })), row.limit)).toEqual(row.scopes);
  });
}

for (const row of fixture.cases) {
  test(`registry consistency: ${row.name}`, () => {
    const scope = "scope" in row ? row.scope : undefined;
    const tree = memoryOwnerTree(filesOf(row.files), policy, "ignored" in row ? row.ignored : [], "tracked" in row ? row.tracked : []);
    const registry = readBuildOwnerRegistryV1(row.registry);
    expect(findOwnerFindings(registry, tree, policy, scope).map(subject).sort()).toEqual([...row.findings].sort());
    const result = syncOwnerRegistry(registry, tree, policy, scope);
    expect(result.registry.owners.map((owner) => `${owner.name}@${owner.root}`)).toEqual(row.sync.owners);
    expect(result.removed.map((entry) => entry.name)).toEqual(row.sync.removed);
    expect(result.added).toEqual(row.sync.added);
    expect(result.renamed.map((entry) => `${entry.from}>${entry.to}`)).toEqual(row.sync.renamed);
    expect(result.relocated.map((entry) => `${entry.name}:${entry.from}>${entry.to}`)).toEqual(row.sync.relocated);
    const expected = row.sync as { rows?: OwnerRow[]; removedNativeRoots?: string[][] };
    if (expected.rows) expect(result.registry.owners).toEqual(expected.rows);
    if (expected.removedNativeRoots) expect(result.removed.map((entry) => entry.nativeRoots)).toEqual(expected.removedNativeRoots);
    const oracle = new Ajv({ strict: false }).compile(registrySchema);
    expect(oracle(result.registry), JSON.stringify(oracle.errors)).toBe(true);
    const residual = (row.sync as { residual?: string[] }).residual ?? [];
    expect(findOwnerFindings(result.registry, tree, policy, scope).map(subject).sort()).toEqual([...residual].sort());
    expect(syncOwnerRegistry(result.registry, tree, policy, scope).registry).toEqual(result.registry);
  });
}

test("cargo package names match the TOML parser of the runtime on every fixture manifest", () => {
  const manifests = fixture.cases.flatMap((row) => Object.entries(filesOf(row.files)).filter(([path]) => path.endsWith("Cargo.toml")));
  expect(manifests.length).toBeGreaterThan(0);
  for (const [path, manifest] of manifests) expect(cargoPackageName(manifest), path).toBe((Bun.TOML.parse(manifest) as { package?: { name?: string } }).package?.name);
});

test("the file reconciler syncs a workspace on disk, writes once and then finds nothing", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-"));
  const write = (path: string, body: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), body); };
  try {
    write("a/📋️project.json", JSON.stringify({ name: "@t/a" }));
    write("m/Cargo.toml", '[package]\nname = "crate-m"\n');
    write("m/🧫️fixtures/f/📋️project.json", JSON.stringify({ name: "fixture" }));
    write(BUILD_OWNER_REGISTRY_FILE, JSON.stringify({ version: 1, owners: [{ name: "@t/old", root: "old", manifest: "old/📋️project.json", packages: [] }], catalogProviders: [] }));
    expect(checkBuildOwnerRegistry(root, undefined, policy).findings.map(subject).sort()).toEqual(["cargo-without-row:m/Cargo.toml", "missing-root:@t/old", "project-without-row:a/📋️project.json"]);
    const scoped = reconcileBuildOwnerRegistry(root, { scope: "m", write: true }, policy);
    expect(scoped.added).toEqual(["crate-m"]);
    expect(scoped.removed).toEqual([]);
    const full = reconcileBuildOwnerRegistry(root, { write: true }, policy);
    expect(full.added).toEqual(["@t/a"]);
    expect(full.removed.map((entry) => entry.name)).toEqual(["@t/old"]);
    expect(JSON.parse(readFileSync(join(root, BUILD_OWNER_REGISTRY_FILE), "utf8")).owners.map((owner: { name: string }) => owner.name)).toEqual(["@t/a", "crate-m"]);
    expect(reconcileBuildOwnerRegistry(root, { write: true }, policy).changed).toBe(false);
    expect(checkBuildOwnerRegistry(root, undefined, policy).findings).toEqual([]);
    write(BUILD_OWNER_REGISTRY_FILE, "{}");
    expect(checkBuildOwnerRegistry(root, undefined, policy).findings.map((finding) => finding.kind)).toEqual(["registry-invalid"]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("the file reconciler keeps rows that git tracks while the working tree lacks them and drops them once git forgets them", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-git-"));
  const git = (...args: string[]): void => { execFileSync("git", args, { cwd: root, stdio: "ignore" }); };
  const write = (path: string, body: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), body); };
  try {
    git("init", "-q");
    write("a/📋️project.json", JSON.stringify({ name: "@t/a" }));
    write("nat/Cargo.toml", '[package]\nname = "crate-nat"\n');
    write(BUILD_OWNER_REGISTRY_FILE, JSON.stringify({ version: 1, owners: [{ name: "@t/a", root: "a", manifest: "a/📋️project.json", packages: [], nativeRoots: ["nat"] }], catalogProviders: [] }));
    git("add", "a", "nat");
    rmSync(join(root, "a"), { recursive: true });
    const transient = reconcileBuildOwnerRegistry(root, { write: true }, policy);
    expect(transient.removed).toEqual([]);
    expect(transient.changed).toBe(false);
    expect(transient.findings.map(subject)).toEqual(["missing-root:@t/a"]);
    git("rm", "-r", "-q", "--cached", "a");
    const forgotten = reconcileBuildOwnerRegistry(root, { write: true }, policy);
    expect(forgotten.removed).toEqual([{ name: "@t/a", root: "a", nativeRoots: ["nat"] }]);
    expect(forgotten.added).toEqual(["crate-nat"]);
    expect(JSON.parse(readFileSync(join(root, BUILD_OWNER_REGISTRY_FILE), "utf8")).owners).toEqual([{ name: "crate-nat", root: "nat", manifest: "nat/Cargo.toml", packages: [] }]);
    expect(checkBuildOwnerRegistry(root, ["nat", "a"], policy).findings).toEqual([]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("a manifest symlink is reported as not an ordinary file on disk", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-link-"));
  try {
    mkdirSync(join(root, "a"), { recursive: true });
    writeFileSync(join(root, "real.json"), JSON.stringify({ name: "@t/a" }));
    symlinkSync(join(root, "real.json"), join(root, "a/📋️project.json"));
    writeFileSync(join(root, BUILD_OWNER_REGISTRY_FILE), JSON.stringify({ version: 1, owners: [{ name: "@t/a", root: "a", manifest: "a/📋️project.json", packages: [] }], catalogProviders: [] }));
    expect(checkBuildOwnerRegistry(root, undefined, policy).findings.map(subject)).toEqual(["manifest-not-file:@t/a"]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
