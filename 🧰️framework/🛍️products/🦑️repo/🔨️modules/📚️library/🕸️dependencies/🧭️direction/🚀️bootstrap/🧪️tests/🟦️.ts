import { expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import glob from "fast-glob";
import { loadDependencyDirectionPolicy } from "../🟦️.ts";

const owner = resolve(import.meta.dir, ".."), library = resolve(owner, "../../.."), repo = resolve(library, "../../../../..");
const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as { cases: { id: string; mutation: string; accept: boolean }[]; freshness: { id: string; initialLocator: string; nextLocator: string; initialPlugins: string[]; nextPlugins: string[]; otherRootPlugins: string[]; owner: string; nextRole: string; nextExports: string[]; nextBannedStem: string } };
const output = process.env.SEMIO_TEST_ARTIFACT_DIR!;
if (!output) throw Error("Caller-owned SEMIO_TEST_ARTIFACT_DIR is required");
mkdirSync(output, { recursive: true });
const taxonomy = readFileSync(join(library, "🔣️taxonomy.json"), "utf8");
const packages = ["🧰️framework/example", "✏️s/🔌️plugins/test"];
const write = (root: string, path: string, text: string): void => { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), text); };
const document = (root: string, path: string, value: unknown): void => write(root, path, JSON.stringify(value));
function workspace(): string {
  const root = realpathSync(mkdtempSync(join(output, "policy-authority-")));
  document(root, "nx.json", {});
  document(root, "📋️project.json", { metadata: { semio: { taxonomy: "policy/taxonomy.json" } } });
  write(root, "policy/taxonomy.json", taxonomy);
  document(root, "package.json", { workspaces: packages, semio: { workspace: { schemaVersion: 1, members: packages, owners: [] } } });
  for (const [index, path] of packages.entries()) document(root, path + "/package.json", { name: "@test/owner-" + index, exports: { ".": "./index.ts" } });
  return root;
}
function link(root: string, path: string, directory: boolean): void {
  const target = join(root, "owned-real-" + path.replaceAll("/", "-"));
  renameSync(join(root, path), target);
  symlinkSync(target, join(root, path), directory ? process.platform === "win32" ? "junction" : "dir" : "file");
}
const node = (source: string, args: string[]): unknown => JSON.parse(execFileSync("node", ["--input-type=commonjs", "-e", source, ...args], { encoding: "utf8" }));
const oracle = String.raw`const fs=require("node:fs"),p=require("node:path");const input=process.argv[1];let ok=true;try{if(input.split(/[\\/]/).some(x=>x==="."||x===".."))throw Error();const root=p.resolve(input);for(let x=root;;x=p.dirname(x)){if(fs.lstatSync(x).isSymbolicLink())throw Error();if(x===p.dirname(x))break;}const project=JSON.parse(fs.readFileSync(p.join(root,"📋️project.json"),"utf8"));const paths=["nx.json","📋️project.json",project.metadata.semio.taxonomy,"package.json",...JSON.parse(fs.readFileSync(p.join(root,"package.json"),"utf8")).workspaces.map(x=>x+"/package.json"),"✏️s/🔌️plugins"];for(const rel of paths){let x=root;for(const part of rel.split("/")){x=p.join(x,part);try{if(fs.lstatSync(x).isSymbolicLink())throw Error("link");}catch(e){if(e.code==="ENOENT"&&(rel.endsWith("/package.json")||rel==="✏️s/🔌️plugins"))break;throw e;}}if(rel.endsWith("package.json")&&fs.existsSync(x))JSON.parse(fs.readFileSync(x,"utf8"));}}catch(e){ok=false;}process.stdout.write(JSON.stringify(ok));`;

test("portable bootstrap examples have distinct identities", () => {
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(fixture.cases.length);
});
for (const row of fixture.cases) test(row.id, () => {
  const root = workspace(); let selected = root;
  try {
    if (row.mutation === "root-ancestor-link") { symlinkSync(dirname(root), root + "-ancestor", process.platform === "win32" ? "junction" : "dir"); selected = root + "-ancestor/" + basename(root); }
    if (row.mutation === "project-link") link(root, "📋️project.json", false);
    if (row.mutation === "nx-link") link(root, "nx.json", false);
    if (row.mutation === "taxonomy-link") link(root, "policy/taxonomy.json", false);
    if (row.mutation === "taxonomy-parent-link") link(root, "policy", true);
    if (row.mutation === "package-link") link(root, packages[0]! + "/package.json", false);
    if (row.mutation === "package-parent-link") link(root, "🧰️framework", true);
    if (row.mutation === "plugin-root-link") link(root, "✏️s/🔌️plugins", true);
    if (row.mutation === "package-absent") rmSync(join(root, packages[0]!), { recursive: true });
    if (row.mutation === "plugin-absent") rmSync(join(root, "✏️s"), { recursive: true });
    if (row.mutation === "package-invalid") write(root, packages[0]! + "/package.json", "{");
    if (row.mutation === "root-link") { selected = root + "-link"; symlinkSync(root, selected, process.platform === "win32" ? "junction" : "dir"); }
    if (row.mutation === "raw-missing-prefix") selected = root + "/missing/../";
    if (row.mutation === "raw-linked-prefix") { symlinkSync(root, root + "/linked", process.platform === "win32" ? "junction" : "dir"); selected = root + "/linked/../"; }
    expect(node(oracle, [selected]), row.id + " Node no-follow oracle").toBe(row.accept);
    if (!row.accept) { expect(() => loadDependencyDirectionPolicy(selected), row.id).toThrow(); return; }
    const snapshot = loadDependencyDirectionPolicy(selected);
    const expected = glob.sync(packages.map(path => path + "/package.json"), { cwd: root, followSymbolicLinks: false, onlyFiles: true }).map(path => ({ owner: dirname(path).replaceAll("\\", "/"), name: JSON.parse(readFileSync(join(root, path), "utf8")).name })).sort((a,b) => a.owner.localeCompare(b.owner));
    expect(snapshot.workspacePackages.map(({ owner, name }) => ({ owner, name }))).toEqual(expected);
    expect(snapshot.plugins).toEqual(row.mutation === "plugin-absent" ? [] : ["test"]);
    expect(Object.isFrozen(snapshot)).toBe(true);
    expect(snapshot.sources.some(source => source.path === "policy/taxonomy.json" && source.kind === "file")).toBe(true);
  } finally { if (selected.endsWith("-link")) rmSync(selected); if (row.mutation === "root-ancestor-link") rmSync(root + "-ancestor"); rmSync(root, { recursive: true, force: true }); }
});

test("fresh policy changes with root locator, plugin and package roles despite Node require caching", () => {
  const first = workspace(), second = workspace();
  try {
    const source = String.raw`const fs=require("node:fs"),p=require("node:path"),load=require(process.argv[1]).loadDependencyDirectionPolicy;const a=process.argv[2],b=process.argv[3],entry=process.argv[4],row=JSON.parse(process.argv[5]);const old=load(a),cached=require(entry),pkg=p.join(a,row.owner,"package.json");const doc=JSON.parse(fs.readFileSync(pkg,"utf8"));doc.semio={dependencyRole:row.nextRole};doc.exports=Object.fromEntries(row.nextExports.map(x=>[x,x==="."?"./index.ts":"./fresh.ts"]));fs.writeFileSync(pkg,JSON.stringify(doc));for(const plugin of row.nextPlugins.filter(x=>!row.initialPlugins.includes(x)))fs.mkdirSync(p.join(a,"✏️s/🔌️plugins",plugin));const taxonomy=JSON.parse(fs.readFileSync(p.join(a,row.initialLocator),"utf8"));taxonomy.bannedNameStems.push(row.nextBannedStem);fs.writeFileSync(p.join(a,row.nextLocator),JSON.stringify(taxonomy));fs.writeFileSync(p.join(a,"📋️project.json"),JSON.stringify({metadata:{semio:{taxonomy:row.nextLocator}}}));const fresh=load(a),other=load(b),owned=fresh.workspacePackages.find(x=>x.owner===row.owner);process.stdout.write(JSON.stringify({cached:cached===require(entry),oldPath:old.taxonomyPath,freshPath:fresh.taxonomyPath,oldPlugins:old.plugins,freshPlugins:fresh.plugins,otherPlugins:other.plugins,exports:owned.exports,role:owned.dependencyRole,oldRule:old.policy.forbidden.find(x=>x.name==="no-core-path"),freshRule:fresh.policy.forbidden.find(x=>x.name==="no-core-path")}));`;
    const result = node(source, [join(owner, "🟨️.cjs"), first, second, join(owner, "🟨️.cjs"), JSON.stringify(fixture.freshness)]) as any;
    expect(result.cached).toBe(true);
    expect(result.oldPath).toBe(fixture.freshness.initialLocator); expect(result.freshPath).toBe(fixture.freshness.nextLocator);
    expect(result.oldPlugins).toEqual(fixture.freshness.initialPlugins); expect(result.freshPlugins).toEqual(fixture.freshness.nextPlugins); expect(result.otherPlugins).toEqual(fixture.freshness.otherRootPlugins);
    expect(result.exports).toEqual(fixture.freshness.nextExports); expect(result.role).toBe(fixture.freshness.nextRole);
    expect(result.oldRule.to.path.includes(fixture.freshness.nextBannedStem)).toBe(false); expect(result.freshRule.to.path.includes(fixture.freshness.nextBannedStem)).toBe(true);
  } finally { rmSync(first, { recursive: true, force: true }); rmSync(second, { recursive: true, force: true }); }
});

test("shared full policy preserves declared severities and resolver authority", () => {
  const root = workspace();
  try {
  const snapshot = loadDependencyDirectionPolicy(root);
  expect(snapshot.policy.options.enhancedResolveOptions).toEqual({ exportsFields: ["exports"], conditionNames: ["semio-source","import","require","node","default"] });
  expect(snapshot.policy.forbidden.find(rule => rule.name === "no-impl-segment")?.severity).toBe("warn");
  expect(snapshot.policy.forbidden.some(rule=>rule.name==="no-generated-edits-upstream")).toBe(false);
  expect(snapshot.policy.forbidden.filter(rule=>rule.name.startsWith("no-cross-package-relative-")).length).toBe(snapshot.workspacePackages.length);
  expect(snapshot.policy.forbidden.find(rule=>rule.name==="plugins-framework-sdk-only")?.severity).toBe("error");
  expect(snapshot.policy.forbidden.filter(rule=>rule.name.startsWith("no-cross-package-relative-")).every(rule=>rule.severity==="error")).toBe(true);
  for (const name of ["framework-no-implementation","repo-no-implementation","s-modules-no-plugins",...Object.keys(snapshot.taxonomy.dependencyDirections.rules)]) expect(snapshot.policy.forbidden.find(rule => rule.name === name)?.severity).toBe("error");
  expect(snapshot.policy.forbidden.filter(rule => rule.name.startsWith("no-cross-technology-")).length).toBe(20);
  expect(snapshot.policy.options.tsPreCompilationDeps).toBe(true); expect(snapshot.policy.options.combinedDependencies).toBe(true);
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test("authority census supports progress and cancellation without returning partial policy", () => {
  const root = workspace();
  try {
    const updates: string[] = [], controller = new AbortController();
    loadDependencyDirectionPolicy(root, { onProgress: progress => updates.push(progress.phase) });
    expect(updates.at(-1)).toBe("complete");
    controller.abort();
    expect(() => loadDependencyDirectionPolicy(root, { signal: controller.signal })).toThrow("cancelled");
    expect(() => loadDependencyDirectionPolicy(root, { signal: new AbortController().signal, onProgress: () => { throw Error("caller cancellation"); } })).toThrow("caller cancellation");
  } finally { rmSync(root, { recursive: true, force: true }); }
});
