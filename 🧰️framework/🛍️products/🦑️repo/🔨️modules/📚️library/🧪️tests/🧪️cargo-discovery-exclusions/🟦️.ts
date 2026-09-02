import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { transformSync } from "esbuild";
import glob from "fast-glob";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";
import { taxonomyRelativePathIsExcluded } from "../../🔍️discovery/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8")) as { schemaVersion: number; opaquePaths: string[]; virtualRoots: string[]; traversal: { enumeration: string; metadata: string; symlinks: string }; symlinks: string[]; manifests: { path: string; package: string; admitted: boolean }[]; execution: { target: string; command: string; launchName: string; launchCommand: string; launchGroup: string; launchOrder: number } };
const taxonomy = JSON.parse(readFileSync(join(repoRoot, library, "🔣️taxonomy.json"), "utf8"));
const source = ts.createSourceFile("🟦️.ts", readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
const names = ["generateCargoVariants", "getCargoWorkspaceIndex"];
const declarations = names.map((name) => {
  const rows = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
  if (rows.length !== 1) throw new Error(`Expected one actual ${name} implementation`);
  return rows[0]!.getText(source);
}).join("\n");
const prefix = source.statements.filter(ts.isVariableStatement).flatMap((node) => [...node.declarationList.declarations]).find((node) => node.name.getText(source) === "CARGO_PREFIX_WORDS")!;
const code = `const ${prefix.getText(source)};\n${declarations}`;

/** 🧫️ Models filesystem entries in memory and rejects any attempt to touch an opaque path. */
function virtualFilesystem(root: string, namesOnly = false): { api: any; reads: string[]; fileReads: string[]; content: Map<string, string> } {
  const content = new Map(vector.manifests.map((row) => [row.path, `[package]\nname = "${row.package}"\n`]));
  const directories = new Set([""]);
  for (const path of [...content.keys(), ...vector.symlinks]) for (let parent = dirname(path).replaceAll("\\", "/"); parent !== "."; parent = dirname(parent).replaceAll("\\", "/")) directories.add(parent);
  const reads: string[] = [], fileReads: string[] = [];
  const locate = (path: string): string => {
    const local = relative(root, path).replaceAll("\\", "/");
    if (local.startsWith("..") || vector.opaquePaths.some((opaque) => local === opaque || local.startsWith(`${opaque}/`))) throw new Error(`Opaque virtual access: ${local}`);
    reads.push(local);
    return local;
  };
  const kind = (path: string) => ({ isDirectory: () => directories.has(path), isFile: () => content.has(path), isSymbolicLink: () => vector.symlinks.includes(path), isBlockDevice: () => false, isCharacterDevice: () => false, isFIFO: () => false, isSocket: () => false });
  const inspect = (path: string) => { const local = locate(path); if (!content.has(local) && !directories.has(local) && !vector.symlinks.includes(local)) throw new Error(`Unknown virtual node: ${local}`); return kind(local); };
  const api = {
    readdirSync(path: string, options?: { withFileTypes?: boolean }) {
      if (namesOnly && options?.withFileTypes) throw new Error("Metadata enumeration before lexical admission");
      const local = locate(path);
      if (!directories.has(local)) throw new Error(`Unknown virtual directory: ${local}`);
      const children = [...new Set([...directories, ...content.keys(), ...vector.symlinks])].filter((child) => child !== local && dirname(child).replaceAll("\\", "/").replace(/^\.$/u, "") === local).sort();
      return options?.withFileTypes ? children.map((child) => ({ name: child.split("/").at(-1)!, ...kind(child) })) : children.map((child) => child.split("/").at(-1)!);
    },
    readFileSync(path: string) { const local = locate(path); fileReads.push(local); if (!content.has(local)) throw new Error(`Unknown virtual file: ${local}`); return content.get(local)!; },
    statSync: inspect,
    lstatSync: inspect,
  };
  return { api, reads, fileReads, content };
}

test("Cargo discovery shares the exact schema-owned lexical exclusions", () => {
  expect(vector.schemaVersion).toBe(1);
  expect(vector.traversal).toEqual({ enumeration: "names-only", metadata: "after-lexical-admission", symlinks: "do-not-follow" });
  expect(Object.values(taxonomy.pathExclusions).map((row: any) => row.path.replace(/\/+$/u, "")).sort()).toEqual([...vector.opaquePaths].sort());
  for (const row of vector.manifests) for (const path of [row.path, row.path.replaceAll("/", "\\")]) expect(taxonomyRelativePathIsExcluded(path, taxonomy)).toBe(!row.admitted);
});

for (const name of vector.virtualRoots) test(`Cargo discovery never touches excluded virtual trees: ${name}`, () => {
  const root = join(repoRoot, name), expected = vector.manifests.filter((row) => row.admitted).map((row) => row.package).sort();
  const oracleFs = virtualFilesystem(root);
  const files = glob.sync("**/Cargo.toml", { cwd: root, onlyFiles: true, dot: true, followSymbolicLinks: false, ignore: vector.opaquePaths.flatMap((path) => [path, `${path}/**`]), fs: oracleFs.api });
  expect(files.sort()).toEqual(vector.manifests.filter((row) => row.admitted).map((row) => row.path).sort());
  for (const compiled of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), transformSync(code, { loader: "ts", target: "es2022" }).code]) {
    const fs = virtualFilesystem(root, true), cache = { current: null };
    let taxonomyReads = 0;
    const implementation = new Function("cachedCrateIndex", "getWorkspaceRoot", "readdirSync", "readFileSync", "lstatSync", "join", "dirname", "relative", "loadTaxonomy", "taxonomyRelativePathIsExcluded", `${compiled}\nreturn getCargoWorkspaceIndex;`)(cache, () => root, fs.api.readdirSync, fs.api.readFileSync, fs.api.lstatSync, join, dirname, relative, () => { taxonomyReads++; return taxonomy; }, taxonomyRelativePathIsExcluded);
    const result = implementation(root);
    expect([...result.exactPkgNames].sort()).toEqual(expected);
    expect([...result.exactPkgNames].sort()).toEqual(files.map((path) => /name = "([^"]+)"/u.exec(oracleFs.content.get(path)!)![1]).sort());
    expect(taxonomyReads).toBe(1);
    expect(implementation(root)).toBe(result);
    expect(taxonomyReads).toBe(1);
    expect(fs.reads.every((path) => !vector.opaquePaths.some((opaque) => path === opaque || path.startsWith(`${opaque}/`)))).toBe(true);
    expect(fs.fileReads.some((path) => vector.symlinks.includes(path))).toBe(false);
  }
  console.log("[DEBUG] Cargo virtual exclusion proof", JSON.stringify({ root: name, packages: expected, independentGlobFiles: files.length }));
});

test("registry catalog filesystem excludes opaque names before metadata enumeration", () => {
  const discovery = ts.createSourceFile("discovery.ts", readFileSync(join(repoRoot, library, "🔍️discovery/🟦️.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const nodes = discovery.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "registryCatalogInputView");
  expect(nodes).toHaveLength(1);
  const code = nodes[0]!.getText(discovery).replace(/^export /u, "");
  for (const rootName of vector.virtualRoots) for (const compiled of [new Bun.Transpiler({ loader: "ts" }).transformSync(code), transformSync(code, { loader: "ts", target: "es2022" }).code]) {
    const root = join(repoRoot, rootName), fs = virtualFilesystem(root, true);
    const factory = new Function("lstatSync", "readdirSync", "readFileSync", "resolve", "relative", "dirname", "join", "pathIsExcluded", "registryCatalogGitlinkBoundaries", compiled + "\nreturn registryCatalogInputView;")(fs.api.lstatSync, fs.api.readdirSync, fs.api.readFileSync, resolve, relative, dirname, join, (workspace: string, path: string) => { const local = relative(workspace, path).replaceAll("\\", "/"); return local !== "" && taxonomyRelativePathIsExcluded(local, taxonomy); }, () => new Set<string>());
    const view = factory(root, taxonomy);
    const files: string[] = [], links: string[] = [];
    const walk = (path: string): void => {
      for (const entry of view.entries(path)) {
        const child = path ? path + "/" + entry.name : entry.name;
        if (entry.nodeKind === "directory") walk(child);
        else if (entry.nodeKind === "symlink") { links.push(child); expect(() => view.readText(child)).toThrow("symlink"); }
        else { files.push(child); expect(view.readText(child)).toBe(fs.content.get(child)); }
      }
    };
    walk("");
    expect(files.sort()).toEqual(vector.manifests.filter((row) => row.admitted).map((row) => row.path).sort());
    expect(links.sort()).toEqual([...vector.symlinks].sort());
    for (const path of vector.opaquePaths) expect(() => view.kind(path)).toThrow("nonopaque");
    expect(fs.fileReads.some((path) => vector.symlinks.includes(path))).toBe(false);
    console.log("[DEBUG] registry catalog virtual exclusion proof", JSON.stringify({ root: rootName, files: files.length, symlinks: links.length }));
  }
});

test("registers the Cargo exclusion gate through Nx and both launch catalogs", () => {
  const expected = vector.execution;
  const project = JSON.parse(readFileSync(join(repoRoot, library, "📦️packages/🟦️typescript/📋️project.json"), "utf8"));
  expect(project.targets[expected.target]?.options.command).toBe(expected.command);
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launches = parseJsonc(readFileSync(join(repoRoot, path), "utf8")).configurations.filter((entry: { name: string }) => entry.name === expected.launchName);
    expect(launches).toHaveLength(1);
    expect(launches[0].command).toBe(expected.launchCommand);
    expect(launches[0].presentation).toEqual({ group: expected.launchGroup, order: expected.launchOrder });
  }
});
