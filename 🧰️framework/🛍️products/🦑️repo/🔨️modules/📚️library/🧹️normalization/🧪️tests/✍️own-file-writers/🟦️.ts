import { afterAll, expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, relative, resolve } from "node:path";
import { readBuildOwnerRegistryV1 } from "../../../🏗️build/🕸️graph/🚪️admission/🟦️.ts";
import { applyTaxonomyPlan } from "../../🚀️apply/🟦️.ts";
import { inventoryTaxonomy } from "../../🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../../🧠️planning/🟦️.ts";
import { ownFileWriterFor, readOwnFileWritersV1 } from "../../✍️own-file-writers/🟦️.ts";

const library = resolve(import.meta.dir, "../../.."), SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json", REGISTRY = "📋️build-owners.json";
const sandbox = mkdtempSync(join(tmpdir(), "semio-own-file-writers-"));
afterAll(() => rmSync(sandbox, { recursive: true, force: true }));

const row = (name: string, root: string) => ({ name, root, manifest: `${root}/📋️project.json`, packages: [] as string[] });
const registryText = (...owners: ReturnType<typeof row>[]) => `${JSON.stringify({ version: 1, owners, catalogProviders: [] }, null, 2)}\n`;

function treeOf(root: string): Record<string, string> {
  const rows: Record<string, string> = {};
  const visit = (path: string): void => {
    const stat = lstatSync(join(root, path));
    if (stat.isDirectory()) for (const name of readdirSync(join(root, path)).sort()) visit(path ? `${path}/${name}` : name);
    else rows[path] = `${stat.mode & 0o7777}:${readFileSync(join(root, path)).toString("base64")}`;
  };
  visit("");
  return rows;
}

function fixture(name: string, registry: (scope: string) => string) {
  const root = join(sandbox, name), ticketDir = join(root, "🧪️tests"), workspace = join(ticketDir, "🧪️fixture"), scope = relative(root, workspace).replaceAll("\\", "/");
  mkdirSync(workspace, { recursive: true });
  mkdirSync(dirname(join(root, SCHEMA_REL)), { recursive: true });
  const taxonomy = JSON.parse(readFileSync(join(library, "../../../../..", SCHEMA_REL), "utf8"));
  delete taxonomy.generatorContracts["plugin-registry"].inputDiscovery;
  writeFileSync(join(root, SCHEMA_REL), `${JSON.stringify(taxonomy, null, 2)}\n`);
  const files: Record<string, string> = {
    "NoEmoji/📋️project.json": "{}\n",
    "🧪️consumer/🟦️component.ts": "export const consumer = 1;\n",
    "🧪️user/🟦️.ts": 'export { consumer } from "../🧪️consumer/🟦️component.ts";\n',
    [REGISTRY]: registry(scope),
  };
  for (const [path, bytes] of Object.entries(files)) { mkdirSync(dirname(join(workspace, path)), { recursive: true }); writeFileSync(join(workspace, path), bytes); }
  chmodSync(join(workspace, REGISTRY), 0o640);
  const git = (...args: string[]) => execFileSync("git", args, { cwd: root, encoding: "utf8" }).trim();
  git("init", "--quiet", "--object-format=sha1"); git("config", "user.name", "Own File Writers"); git("config", "user.email", "own-file-writers@invalid.example"); git("config", "commit.gpgsign", "false");
  git("add", "--all", "--", ".", `:(exclude,literal)${SCHEMA_REL}`); git("commit", "--quiet", "-m", "fixture");
  const baselineCommit = git("rev-parse", "HEAD");
  const plan = planTaxonomy(inventoryTaxonomy({ repoRoot: root, scope, ticketDir, workers: 1 }), { baselineCommit, excludedTreeDigests: [] });
  return { root, ticketDir, workspace, scope, baselineCommit, plan };
}

test("the declared mapping names the registry writer and nothing else", () => {
  expect(ownFileWriterFor(`some/dir/${REGISTRY}`)?.publish).toBeFunction();
  expect(ownFileWriterFor("some/dir/📋️project.json")).toBeUndefined();
  expect(() => readOwnFileWritersV1({ version: 1, writers: [{ id: "nobody", fileName: "x.json", authority: "x" }] })).toThrow(/no implementation/u);
  const twice = { id: "build-owner-registry", fileName: REGISTRY, authority: "x" };
  expect(() => readOwnFileWritersV1({ version: 1, writers: [twice, twice] })).toThrow(/declared twice/u);
  expect(() => readOwnFileWritersV1({ version: 2, writers: [] })).toThrow(/declaration required/u);
});

test("a taxonomy move that renames an owner root updates the registry through the writer", () => {
  const f = fixture("rename", (scope) => registryText(row("subject", `${scope}/NoEmoji`)));
  const registryEdits = f.plan.edits.filter((edit) => edit.path.endsWith(REGISTRY));
  expect(registryEdits.map((edit) => edit.newValue).sort()).toEqual([`${f.scope}/🧪️NoEmoji`, `${f.scope}/🧪️NoEmoji/📋️project.json`].sort());
  expect(applyTaxonomyPlan(f.plan, { repoRoot: f.root, ticketDir: f.ticketDir, expectedBaselineCommit: f.baselineCommit }).state).toBe("committed");
  const path = join(f.workspace, REGISTRY), after = readFileSync(path, "utf8");
  expect(readBuildOwnerRegistryV1(JSON.parse(after)).owners).toEqual([row("subject", `${f.scope}/🧪️NoEmoji`)]);
  expect(statSync(path).mode & 0o7777).toBe(0o640);
  expect(readdirSync(f.workspace).filter((name) => name.includes(".tmp"))).toEqual([]);
  expect(existsSync(join(f.workspace, "🧪️NoEmoji/📋️project.json"))).toBe(true);
});

test("an edit that would produce a duplicate root is refused and both files stay unchanged", () => {
  const f = fixture("duplicate", (scope) => registryText(row("subject", `${scope}/NoEmoji`), row("taken", `${scope}/🧪️NoEmoji`)));
  expect(f.plan.edits.some((edit) => edit.path.endsWith(REGISTRY))).toBe(true);
  const before = treeOf(f.workspace), registryBefore = readFileSync(join(f.workspace, REGISTRY), "utf8");
  let outcome = "committed";
  try { outcome = applyTaxonomyPlan(f.plan, { repoRoot: f.root, ticketDir: f.ticketDir, expectedBaselineCommit: f.baselineCommit }).state; } catch (error) { outcome = String(error); }
  expect(outcome).toBe("rolled-back");
  expect(readFileSync(join(f.workspace, REGISTRY), "utf8")).toBe(registryBefore);
  expect(treeOf(f.workspace)).toEqual(before);
  expect(existsSync(join(f.workspace, "NoEmoji/📋️project.json"))).toBe(true);
  expect(existsSync(join(f.workspace, "🧪️NoEmoji"))).toBe(false);
});

test("the registry writer refuses a registry that is no longer the planned preimage", () => {
  const dir = mkdtempSync(join(sandbox, "stale-")), file = join(dir, REGISTRY), text = registryText(row("a", "a"));
  writeFileSync(file, text);
  expect(() => ownFileWriterFor(file)!.check(registryText(row("a", "a"), row("b", "a")))).toThrow(/identity collision/u);
  expect(() => ownFileWriterFor(file)!.check("{")).toThrow();
  ownFileWriterFor(file)!.check(registryText(row("a", "a")));
  const writer = ownFileWriterFor(file)!.publish, hash = (value: string) => new Bun.CryptoHasher("sha256").update(value).digest("hex");
  expect(() => writer(file, { contentHash: hash("other"), mode: 0o644 }, Buffer.from(registryText(row("b", "b"))))).toThrow(/preimage changed/u);
  expect(readFileSync(file, "utf8")).toBe(text);
  expect(() => writer(file, { contentHash: hash(text), mode: 0o644 }, Buffer.from(registryText(row("a", "a"), row("a", "b"))))).toThrow();
  expect(readFileSync(file, "utf8")).toBe(text);
  writer(file, { contentHash: hash(text), mode: 0o600 }, Buffer.from(registryText(row("b", "b"))));
  expect(readFileSync(file, "utf8")).toBe(registryText(row("b", "b")));
  expect(statSync(file).mode & 0o7777).toBe(0o600);
});
