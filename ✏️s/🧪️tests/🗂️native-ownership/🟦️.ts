import { test, expect } from "bun:test";
import assert from "node:assert/strict";
import { readFile, readdir, access } from "node:fs/promises";
import { createRequire } from "node:module";
import { dirname, join, resolve, relative, sep, posix } from "node:path";
import corpus from "./🧫️fixtures/🔣️.json";

const s = resolve(import.meta.dir, "../.."), root = dirname(s), require = createRequire(join(root, "package.json"));
const parse = require("@iarna/toml").parse;
const normalized = (value: string) => posix.normalize(value);
const portable = (value: string) => !value.includes("\\") && !/^[a-z]:/i.test(value) && !posix.isAbsolute(value);
const owns = (members: readonly string[], children: readonly string[]) => new Set(members.map(normalized)).size === members.length && members.every(member => {
  const path = normalized(member);
  return portable(member) && path !== ".." && !path.startsWith("../") && !children.some(child => path === normalized(child) || path.startsWith(normalized(child) + "/"));
});
const manifest = async (path: string) => {
  const source = await readFile(path, "utf8"), value = Bun.TOML.parse(source) as any;
  assert.deepEqual(value, parse(source));
  return value;
};

type Table = Record<string, unknown>;
type Definition = { path: string; value: Table };
type Boundary = { owner: string; children: readonly string[] };
const table = (value: unknown): Table => value !== null && typeof value === "object" && !Array.isArray(value) ? value as Table : {};
const strings = (value: unknown): string[] => {
  if (value === undefined) return [];
  assert.ok(Array.isArray(value) && value.every(item => typeof item === "string"));
  return value;
};
const coordinate = (directory: string, path: string) => posix.relative("/workspace", posix.resolve("/workspace", directory, path));
const within = (directory: string, ancestor: string) => directory === ancestor || directory.startsWith(ancestor + "/");
const sections = (value: Table) => [value, ...Object.values(table(value.target)).map(table)].flatMap(scope => ["dependencies", "dev-dependencies", "build-dependencies"].map(name => table(scope[name])));
const audit = async (rows: readonly Definition[], boundaries: readonly Boundary[], available: (path: string) => Promise<boolean>) => {
  const definitions = new Map(rows.map(row => [posix.dirname(row.path), row])), owners = new Map([...definitions].filter(([, row]) => row.value.workspace !== undefined)), issues: string[] = [];
  const reject = (condition: boolean, path: string, reason: string) => { if (!condition) issues.push(path + ": " + reason); };
  const nearest = (directory: string) => {
    for (let path = directory; path !== "."; path = posix.dirname(path)) if (owners.has(path)) return path;
    return owners.has(".") ? "." : undefined;
  };
  const memberships = new Map<string, Set<string>>();
  for (const [owner, row] of owners) {
    const workspace = table(row.value.workspace), declared = strings(workspace.members), children = boundaries.find(boundary => boundary.owner === owner)?.children ?? [], selected = new Set<string>(), names = new Set<string>();
    reject(owns(declared, children), row.path, "noncanonical or descendant member");
    for (const member of [...declared, ...(row.value.package !== undefined && !declared.includes(".") ? ["."] : [])]) {
      const directory = coordinate(owner, member), definition = definitions.get(directory), pkg = table(definition?.value.package);
      reject(within(directory, owner), row.path, "member escapes owner");
      reject(definition?.value.package !== undefined, row.path, "member is not a discovered package: " + member);
      reject(nearest(directory) === owner, row.path, "member crosses a private workspace: " + member);
      reject(typeof pkg.name === "string" && !names.has(pkg.name), row.path, "duplicate or absent package name");
      if (typeof pkg.name === "string") names.add(pkg.name);
      selected.add(directory);
    }
    memberships.set(owner, selected);
  }
  const checkPath = async (directory: string, path: string, owner: string | undefined, label: string) => {
    reject(portable(path), label, "nonportable dependency path");
    const target = coordinate(directory, path);
    reject(await available(posix.join(target, "Cargo.toml")), label, "missing local dependency manifest");
    const boundary = boundaries.find(row => row.owner === owner);
    if (boundary) reject(!boundary.children.some(child => within(target, coordinate(boundary.owner, child))), label, "dependency enters removable child");
  };
  for (const [directory, row] of definitions) {
    const owner = nearest(directory), pkg = table(row.value.package), workspace = table(owners.get(owner ?? "")?.value.workspace), providers = table(workspace.dependencies);
    if (row.value.package !== undefined) {
      reject(owner !== undefined && memberships.get(owner)?.has(directory) === true, row.path, "undeclared package");
      if (typeof pkg.workspace === "string") reject(portable(pkg.workspace) && coordinate(directory, pkg.workspace) === owner, row.path, "foreign explicit workspace");
    }
    if (row.value.workspace !== undefined) for (const [name, provider] of Object.entries(table(table(row.value.workspace).dependencies))) {
      const dependency = table(provider);
      if (typeof dependency.path === "string") await checkPath(directory, dependency.path, directory, row.path + " provider " + name);
    }
    for (const group of sections(row.value)) for (const [name, value] of Object.entries(group)) {
      const dependency = table(value), inherited = dependency.workspace === true, provider = inherited ? providers[name] : value;
      reject(!inherited || provider !== undefined, row.path, "missing inherited provider: " + name);
      const resolved = table(provider);
      if (typeof resolved.path === "string") await checkPath(inherited ? owner! : directory, resolved.path, owner, row.path + " dependency " + name);
    }
  }
  return issues;
};

test("Variable Neutral Owner Examples Match Independent Path Projection", () => {
  for (const row of corpus.cases) {
    expect(owns(row.members, row.children)).toBe(row.accepted);
    const independent = new Set(row.members.map(member => posix.resolve("/owner", member))).size === row.members.length && row.members.every(member => {
      const path = posix.resolve("/owner", member), base = "/owner";
      return !member.includes("\\") && !/^[a-z]:/i.test(member) && (path === base || path.startsWith(base + "/")) && row.children.every(child => {
        const boundary = posix.resolve(base, child);
        return path !== boundary && !path.startsWith(boundary + "/");
      });
    });
    expect(independent).toBe(row.accepted);
  }
});

test("Variable Complete Ownership Corpus Rejects Undeclared And Descendant Packages", async () => {
  for (const row of corpus.ownershipCases) {
    const paths = new Set(row.manifests.map(definition => definition.path)), available = async (path: string) => paths.has(path);
    const native = row.manifests.map(definition => ({ path: definition.path, value: Bun.TOML.parse(definition.source) as Table }));
    const independent = row.manifests.map(definition => ({ path: definition.path, value: parse(definition.source) as Table }));
    assert.deepEqual(native, independent);
    const observed = await audit(native, row.boundaries, available), oracle = await audit(independent, row.boundaries, available);
    assert.deepEqual(observed, oracle);
    expect(observed.length === 0).toBe(row.accepted);
  }
});

test("S Native Owner Has No Plugin Or Deployment Membership Or Providers", async () => {
  const value = await manifest(join(s, "Cargo.toml"));
  expect(owns(value.workspace.members, ["🔌️plugins", "🧑‍💻dev"])).toBe(true);
  for (const provider of Object.values(value.workspace.dependencies) as any[]) {
    if (!provider.path) continue;
    const path = relative(s, resolve(s, provider.path)).split(sep).join("/");
    expect(path === "🔌️plugins" || path.startsWith("🔌️plugins/") || path === "🧑‍💻dev" || path.startsWith("🧑‍💻dev/")).toBe(false);
  }
});

test("Every Discovered S Native Package Has Its Nearest Owner And Complete Dependency Admission", async () => {
  const definitions: Definition[] = [], omitted = new Set(["node_modules", "target", ".git", ".nx", "🗑️generated", ".venv", "__pycache__"]), pending = [s], paths: string[] = [];
  for (let cursor = 0; cursor < pending.length;) {
    const batch = pending.slice(cursor, cursor + 32); cursor += batch.length;
    const entries = await Promise.all(batch.map(directory => readdir(directory, { withFileTypes: true })));
    for (let index = 0; index < batch.length; index++) for (const entry of entries[index]!) {
      if (omitted.has(entry.name)) continue;
      const path = join(batch[index]!, entry.name);
      if (entry.isDirectory()) pending.push(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") paths.push(path);
      else if (entry.isSymbolicLink() && entry.name === "Cargo.toml") throw Error("Native manifest must have a defining physical body: " + path);
    }
  }
  for (let index = 0; index < paths.length; index += 32) {
    definitions.push(...await Promise.all(paths.slice(index, index + 32).map(async path => ({ path: relative(root, path).split(sep).join("/"), value: await manifest(path) }))));
  }
  const owners = definitions.filter(row => row.value.workspace !== undefined).map(row => posix.dirname(row.path)), packages = definitions.filter(row => row.value.package !== undefined), boundaries: Boundary[] = [{ owner: "✏️s", children: ["🔌️plugins", "🧑‍💻dev"] }, ...owners.filter(owner => owner.startsWith("✏️s/🔌️plugins/") && owner.split("/").length === 3).map(owner => ({ owner, children: ["🗿️artifacts", "🧩️extensions", "🔮️oracles", "👽️guest", "🏅️standards", "🏭️bridge"] }))];
  expect(owners.includes("✏️s")).toBe(true);
  const existence = new Map<string, Promise<boolean>>(), readExistence = async (path: string) => {
    try { await access(resolve(root, path)); return true; }
    catch (failure) { if ((failure as NodeJS.ErrnoException).code !== "ENOENT") throw failure; return false; }
  };
  const available = (path: string) => {
    let observation = existence.get(path);
    if (!observation) { observation = readExistence(path); existence.set(path, observation); }
    return observation;
  };
  assert.deepEqual(await audit(definitions, boundaries, available), []);
  const intrinsic = packages.filter(row => !row.path.split("/").some(part => ["🏅️standards", "👽️guest", "🔮️oracles", "🏭️bridge"].includes(part))).map(row => table(row.value.package).name);
  expect(new Set(intrinsic).size).toBe(intrinsic.length);
  console.log("[DEBUG] S discovered manifests=" + definitions.length + " native owners=" + owners.length + " packages=" + packages.length + " intrinsic=" + intrinsic.length);
});
