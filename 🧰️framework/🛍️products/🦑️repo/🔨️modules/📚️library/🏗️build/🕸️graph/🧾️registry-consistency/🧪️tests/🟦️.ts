import { expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import Ajv from "ajv";
import consistencySchema from "../../../🧬️schema/🧾️owner-consistency.json" with { type: "json" };
import registrySchema from "../../../🧬️schema/📋️owners.json" with { type: "json" };
import { BUILD_OWNER_REGISTRY_FILE, cargoPackageName, explainOwnerlessRoot, ownerlessCrateManifests, checkBuildOwnerRegistry, findOwnerFindings, memoryOwnerTree, moveOwnerScopes, reconcileBuildOwnerRegistry, syncOwnerRegistry, type OwnerFinding, type OwnerRow, type OwnerTreePolicy } from "../🟦️.ts";
import { readBuildOwnerRegistryV1 } from "../../🚪️admission/🟦️.ts";
import { createOwnerRegistryTextV1, updateBuildOwnerRegistryV1, withOwnerRegistryLockV1 } from "../🔏️writer/🟦️.ts";
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

test("an on-disk peer write that arrives after reconciliation read its baseline is kept and the reconciliation runs again on it", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-peer-"));
  const file = join(root, BUILD_OWNER_REGISTRY_FILE);
  const peer = JSON.stringify({ version: 1, owners: [{ name: "external", root: "external", manifest: "external/Cargo.toml", packages: [] }], catalogProviders: [] });
  let seen = false;
  const editingPolicy: OwnerTreePolicy = { ...policy, get generated() {
    if (!seen) {
      seen = true;
      writeFileSync(file, peer);
    }
    return policy.generated;
  }};
  try {
    mkdirSync(join(root, "needed"));
    writeFileSync(join(root, "needed", "Cargo.toml"), '[package]\nname = "needed"\n');
    writeFileSync(file, JSON.stringify({ version: 1, owners: [], catalogProviders: [] }));
    expect(reconcileBuildOwnerRegistry(root, { scope: "needed", write: true }, editingPolicy).registry.owners.map(owner => owner.name)).toEqual(["external", "needed"]);
    expect(seen).toBe(true);
    expect(JSON.parse(readFileSync(file, "utf8")).owners.map((owner: { name: string }) => owner.name)).toEqual(["external", "needed"]);
    expect(readdirSync(root).sort()).toEqual(["needed", BUILD_OWNER_REGISTRY_FILE].sort());
    expect(reconcileBuildOwnerRegistry(root, { scope: "needed", write: true }, policy).changed).toBe(false);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("additions only adds the missing rows and leaves every other row alone", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-additions-"));
  try {
    mkdirSync(join(root, "new"));
    writeFileSync(join(root, "new", "Cargo.toml"), '[package]\nname = "crate-new"\n');
    writeFileSync(join(root, BUILD_OWNER_REGISTRY_FILE), JSON.stringify({ version: 1, owners: [{ name: "gone", root: "gone", manifest: "gone/Cargo.toml", packages: [] }], catalogProviders: [] }));
    expect(ownerlessCrateManifests(root, undefined, policy)).toEqual(["new/Cargo.toml"]);
    const result = reconcileBuildOwnerRegistry(root, { write: true, additionsOnly: true }, policy);
    expect(result.added).toEqual(["crate-new"]);
    expect(result.removed).toEqual([]);
    expect(JSON.parse(readFileSync(join(root, BUILD_OWNER_REGISTRY_FILE), "utf8")).owners.map((owner: { name: string }) => owner.name)).toEqual(["crate-new", "gone"]);
    expect(ownerlessCrateManifests(root, undefined, policy)).toEqual([]);
    expect(explainOwnerlessRoot(root, "new", policy)).toContain("the row of crate-new exists");
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("the registry lock refuses a second writer instead of hanging, releases after a failure and takes over the lock of an ended or stale holder", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-lock-"));
  const file = join(root, BUILD_OWNER_REGISTRY_FILE);
  const lock = join(tmpdir(), `semio-build-owners-${createHash("sha256").update(file).digest("hex").slice(0, 16)}.lock`);
  try {
    expect(() => withOwnerRegistryLockV1(file, () => withOwnerRegistryLockV1(file, () => 0, { waitMilliseconds: 80 }), { waitMilliseconds: 80 })).toThrow("is locked by another writer");
    expect(withOwnerRegistryLockV1(file, () => "free")).toBe("free");
    expect(() => withOwnerRegistryLockV1(file, () => { throw Error("boom"); })).toThrow("boom");
    expect(withOwnerRegistryLockV1(file, () => "free again")).toBe("free again");
    const ended = Bun.spawnSync([process.execPath, "-e", ""]).pid;
    mkdirSync(lock);
    writeFileSync(join(lock, "owner"), `${ended} ${Date.now()}\n`);
    expect(withOwnerRegistryLockV1(file, () => "ended holder replaced", { waitMilliseconds: 2000 })).toBe("ended holder replaced");
    mkdirSync(lock);
    writeFileSync(join(lock, "owner"), "unreadable\n");
    expect(withOwnerRegistryLockV1(file, () => "stale holder replaced", { staleMilliseconds: 0, waitMilliseconds: 2000 })).toBe("stale holder replaced");
    mkdirSync(lock);
    writeFileSync(join(lock, "owner"), `${process.pid} 0\n`);
    expect(() => withOwnerRegistryLockV1(file, () => 0, { staleMilliseconds: 0, waitMilliseconds: 80 })).toThrow("is locked by another writer");
    rmSync(lock, { recursive: true, force: true });
    expect(existsSync(lock)).toBe(false);
  } finally { rmSync(lock, { recursive: true, force: true }); rmSync(root, { recursive: true, force: true }); }
});

for (const row of fixture.duplicates) {
  test(`owner duplicates: ${row.name}`, () => {
    const root = mkdtempSync(join(tmpdir(), "owners-duplicates-"));
    const file = join(root, BUILD_OWNER_REGISTRY_FILE);
    const raw = JSON.stringify({ version: 1, owners: row.owners, catalogProviders: [] }, null, 2);
    try {
      writeFileSync(file, raw);
      if ("refusal" in row) {
        expect(() => readBuildOwnerRegistryV1(JSON.parse(raw))).toThrow(row.refusal);
        expect(() => reconcileBuildOwnerRegistry(root, { write: true }, policy)).toThrow(row.refusal);
        expect(readFileSync(file, "utf8")).toBe(raw);
      } else {
        expect(readBuildOwnerRegistryV1(JSON.parse(raw)).owners.map((owner) => owner.name)).toEqual(row.names);
        const healed = reconcileBuildOwnerRegistry(root, { write: true, additionsOnly: true }, policy);
        expect(healed.changed).toBe(row.healed);
        expect(JSON.parse(readFileSync(file, "utf8")).owners.map((owner: OwnerRow) => owner.name)).toEqual(row.names);
        expect(reconcileBuildOwnerRegistry(root, { write: true, additionsOnly: true }, policy).changed).toBe(false);
      }
      expect(readdirSync(root)).toEqual([BUILD_OWNER_REGISTRY_FILE]);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

const ownerRow = (name: string, packages: string[] = []) => ({ name, root: name.slice(3), manifest: `${name.slice(3)}/📋️project.json`, packages });
const registryText = (rows: readonly ReturnType<typeof ownerRow>[]): string => `${JSON.stringify({ version: 1, owners: rows, catalogProviders: [] }, null, 2)}\n`;
const ownerNamesOf = (file: string): string[] => JSON.parse(readFileSync(file, "utf8")).owners.map((owner: { name: string }) => owner.name).sort();
const stagedFilesOf = (directory: string): string[] => readdirSync(directory).filter((name) => name.endsWith(".tmp"));
const addOwnerRow = (name: string) => (source: string): { value: boolean; text?: string } => {
  const registry = JSON.parse(source);
  if (registry.owners.some((owner: { name: string }) => owner.name === name)) return { value: false };
  registry.owners.push(ownerRow(name));
  return { value: true, text: `${JSON.stringify(registry, null, 2)}\n` };
};
const writerModule = join(import.meta.dir, "..", "🔏️writer", "🟦️.ts");
const adderScript = (file: string, name: string, hooks = "{}"): string => `import { updateBuildOwnerRegistryV1 } from ${JSON.stringify(writerModule)};
updateBuildOwnerRegistryV1(${JSON.stringify(file)}, (source) => { const registry = JSON.parse(source); Atomics.wait(new Int32Array(new SharedArrayBuffer(4)), 0, 0, 120); if (registry.owners.some((owner) => owner.name === ${JSON.stringify(name)})) return { value: false }; registry.owners.push({ name: ${JSON.stringify(name)}, root: ${JSON.stringify(name.slice(3))}, manifest: ${JSON.stringify(`${name.slice(3)}/📋️project.json`)}, packages: [] }); return { value: true, text: JSON.stringify(registry, null, 2) + "\\n" }; }, ${hooks});`;
const withRegistryFile = (rows: readonly ReturnType<typeof ownerRow>[], run: (file: string, root: string) => void | Promise<void>): Promise<void> | void => {
  const root = mkdtempSync(join(tmpdir(), "owners-writer-"));
  const file = join(root, BUILD_OWNER_REGISTRY_FILE);
  writeFileSync(file, registryText(rows));
  const done = (): void => { rmSync(join(tmpdir(), `semio-build-owners-${createHash("sha256").update(file).digest("hex").slice(0, 16)}.lock`), { recursive: true, force: true }); rmSync(root, { recursive: true, force: true }); };
  try { const result = run(file, root); if (result instanceof Promise) return result.finally(done); done(); } catch (error) { done(); throw error; }
};

test("the shared registry writer refuses an invalid registry and leaves the old file byte for byte", () => {
  withRegistryFile([ownerRow("@t/a")], (file, root) => {
    const before = readFileSync(file, "utf8");
    const duplicate = registryText([ownerRow("@t/a"), ownerRow("@t/a", ["other"])]);
    expect(() => updateBuildOwnerRegistryV1(file, () => ({ value: 0, text: duplicate }))).toThrow("Canonical build owner identity collision");
    expect(() => updateBuildOwnerRegistryV1(file, () => ({ value: 0, text: "{}\n" }))).toThrow("Canonical build owner registry required");
    expect(() => updateBuildOwnerRegistryV1(file, () => ({ value: 0, text: "not json" }))).toThrow();
    expect(readFileSync(file, "utf8")).toBe(before);
    expect(stagedFilesOf(root)).toEqual([]);
    expect(updateBuildOwnerRegistryV1(file, addOwnerRow("@t/b"))).toBe(true);
    expect(ownerNamesOf(file)).toEqual(["@t/a", "@t/b"]);
  });
});

test("two concurrent adders in separate processes both land exactly once, a shared row lands once, and no row is lost", async () => {
  await withRegistryFile([ownerRow("@t/a")], async (file, root) => {
    const names = ["@t/b", "@t/c", "@t/d", "@t/same", "@t/same", "@t/e"];
    const children = names.map((name) => Bun.spawn([process.execPath, "-e", adderScript(file, name)], { stdout: "pipe", stderr: "pipe" }));
    const codes = await Promise.all(children.map((child) => child.exited));
    expect(codes).toEqual(names.map(() => 0));
    expect(ownerNamesOf(file)).toEqual(["@t/a", "@t/b", "@t/c", "@t/d", "@t/e", "@t/same"]);
    expect(readBuildOwnerRegistryV1(JSON.parse(readFileSync(file, "utf8"))).owners).toHaveLength(6);
    expect(stagedFilesOf(root)).toEqual([]);
  });
});

test("a hand edit between the read and the rename is detected, the writer computes again on the new file and the peer's row survives", () => {
  withRegistryFile([ownerRow("@t/a")], (file) => {
    let computed = 0;
    const result = updateBuildOwnerRegistryV1(file, (source) => {
      computed++;
      if (computed === 1) writeFileSync(file, registryText([ownerRow("@t/a"), ownerRow("@t/peer")]));
      return addOwnerRow("@t/b")(source);
    });
    expect(result).toBe(true);
    expect(computed).toBe(2);
    expect(ownerNamesOf(file)).toEqual(["@t/a", "@t/b", "@t/peer"]);
  });
});

test("a failure or a crash between the temporary write and the rename leaves the old file, and the next writer lands its row", async () => {
  await withRegistryFile([ownerRow("@t/a")], async (file, root) => {
    const before = readFileSync(file, "utf8");
    expect(() => updateBuildOwnerRegistryV1(file, addOwnerRow("@t/b"), { rename: () => { throw Error("disk gone"); } })).toThrow("disk gone");
    expect(readFileSync(file, "utf8")).toBe(before);
    expect(stagedFilesOf(root)).toEqual([]);
    const crashed = Bun.spawn([process.execPath, "-e", adderScript(file, "@t/b", "{ rename: () => process.exit(7) }")], { stdout: "pipe", stderr: "pipe" });
    expect(await crashed.exited).toBe(7);
    expect(readFileSync(file, "utf8")).toBe(before);
    expect(stagedFilesOf(root)).toHaveLength(1);
    expect(readBuildOwnerRegistryV1(JSON.parse(readFileSync(file, "utf8"))).owners).toHaveLength(1);
    expect(updateBuildOwnerRegistryV1(file, addOwnerRow("@t/c"))).toBe(true);
    expect(ownerNamesOf(file)).toEqual(["@t/a", "@t/c"]);
  });
});

test("the registry is created once and whole: a second creator or an invalid registry creates nothing", () => {
  const root = mkdtempSync(join(tmpdir(), "owners-create-"));
  const file = join(root, BUILD_OWNER_REGISTRY_FILE);
  try {
    expect(() => createOwnerRegistryTextV1(file, registryText([ownerRow("@t/a"), ownerRow("@t/a", ["x"])]))).toThrow("Canonical build owner identity collision");
    expect(existsSync(file)).toBe(false);
    expect(createOwnerRegistryTextV1(file, registryText([ownerRow("@t/a")]))).toBe(true);
    expect(createOwnerRegistryTextV1(file, registryText([ownerRow("@t/b")]))).toBe(false);
    expect(ownerNamesOf(file)).toEqual(["@t/a"]);
    expect(stagedFilesOf(root)).toEqual([]);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("only the shared writer writes the registry: no other source of the library publishes it", () => {
  const library = join(import.meta.dir, "..", "..", "..", "..");
  for (const path of ["🏗️build/🕸️graph/🧾️registry-consistency/🟦️.ts", "🧪️testing/🧫️private-repository/🧰️adoption/🟦️.ts"]) {
    const source = readFileSync(join(library, path), "utf8");
    expect(source, path).not.toMatch(/renameSync\(|writeFileSync\([^)]*build-owners|copyFileSync\([^)]*build-owners/);
  }
});
