import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { transformSync } from "esbuild";
import { parse as parseJsonc } from "jsonc-parser";
import ts from "typescript";

const repoRoot = resolve(import.meta.dir, "../../../../../../../");
const library = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library";
const vector = JSON.parse(readFileSync(join(import.meta.dir, "../🧾️registry-catalog-gitlink-boundary/🔣️.json"), "utf8")) as {
  schemaVersion: number;
  virtualRoot: string;
  boundary: string;
  ancestorFile: string;
  siblingFile: string;
  nestedFiles: string[];
  execution: { target: string; command: string; launchName: string; launchCommand: string; launchGroup: string; launchOrder: number };
};
const discoverySource = readFileSync(join(repoRoot, library, "🔍️discovery/🟦️.ts"), "utf8");
const discovery = ts.createSourceFile("discovery.ts", discoverySource, ts.ScriptTarget.Latest, true);
const declaration = (name: string): ts.FunctionDeclaration => {
  const rows = discovery.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === name);
  if (rows.length !== 1) throw new Error(`Expected exactly one ${name} implementation`);
  return rows[0]!;
};
const compilers = (code: string): string[] => [new Bun.Transpiler({ loader: "ts" }).transformSync(code), transformSync(code, { loader: "ts", target: "es2022" }).code];

/** 🧫️ In-memory tree with plain content/directory maps, tracking every read so nothing beneath a declared boundary can slip through unnoticed. */
function virtualFilesystem(root: string, files: Record<string, string>) {
  const content = new Map(Object.entries(files));
  const directories = new Set([""]);
  for (const path of content.keys()) for (let parent = dirname(path).replaceAll("\\", "/"); parent !== "."; parent = dirname(parent).replaceAll("\\", "/")) directories.add(parent);
  const reads: string[] = [], fileReads: string[] = [];
  const locate = (path: string): string => { const local = relative(root, path).replaceAll("\\", "/"); reads.push(local); return local; };
  const kind = (path: string) => ({ isDirectory: () => directories.has(path), isFile: () => content.has(path), isSymbolicLink: () => false, isBlockDevice: () => false, isCharacterDevice: () => false, isFIFO: () => false, isSocket: () => false });
  const inspect = (path: string) => { const local = locate(path); if (!content.has(local) && !directories.has(local)) throw new Error(`Unknown virtual node: ${local}`); return kind(local); };
  const api = {
    readdirSync(path: string) {
      const local = locate(path);
      if (!directories.has(local)) throw new Error(`Unknown virtual directory: ${local}`);
      return [...new Set([...directories, ...content.keys()])].filter((child) => child !== local && dirname(child).replaceAll("\\", "/").replace(/^\.$/u, "") === local).map((child) => child.split("/").at(-1)!).sort();
    },
    readFileSync(path: string) { const local = locate(path); fileReads.push(local); if (!content.has(local)) throw new Error(`Unknown virtual file: ${local}`); return content.get(local)!; },
    statSync: inspect,
    lstatSync: inspect,
  };
  return { api, reads, fileReads };
}

test("registry catalog gitlink boundaries parses only stage-zero 160000 index rows, NFC-normalized and cached", () => {
  const code = declaration("registryCatalogGitlinkBoundaries").getText(discovery);
  const decomposedPath = "\u267b\ufe0fmit-bestand/re\u0301sistance";
  const rows = [
    "100644 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 0\t🔣️top-level.json",
    `160000 92036c7ca0149b43ddea28db8c8e516f983fe718 0\t${vector.boundary}`,
    "160000 92036c7ca0149b43ddea28db8c8e516f983fe718 1\t♻️mit-bestand/conflicted-stage",
    "100644 bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb 0\t♻️mit-bestand/🔣️owner.json",
    `160000 cccccccccccccccccccccccccccccccccccccccc 0\t${decomposedPath}`,
  ].join("\0") + "\0";
  for (const compiled of compilers(code)) {
    const calls: { command: string; args: string[]; cwd?: string }[] = [];
    const execFileSyncStub = (command: string, args: string[], options: { cwd?: string }) => { calls.push({ command, args, cwd: options.cwd }); return rows; };
    const cache = new Map<string, ReadonlySet<string>>();
    const implementation = new Function("execFileSync", "gitlinkBoundaryCache", `${compiled}\nreturn registryCatalogGitlinkBoundaries;`)(execFileSyncStub, cache);
    const boundaries = implementation(repoRoot);
    expect([...boundaries].sort()).toEqual([vector.boundary, decomposedPath.normalize("NFC")].sort());
    expect(decomposedPath.normalize("NFC")).not.toBe(decomposedPath);
    expect(boundaries.has(decomposedPath)).toBe(false);
    expect(calls).toEqual([{ command: "git", args: ["ls-files", "--stage", "-z"], cwd: repoRoot }]);
    expect(implementation(repoRoot)).toBe(boundaries);
    expect(calls).toHaveLength(1);
  }
});

test("registry catalog input view treats a gitlink boundary as a terminal leaf", () => {
  const code = declaration("registryCatalogInputView").getText(discovery).replace(/^export /u, "");
  const root = join(repoRoot, vector.virtualRoot);
  const files: Record<string, string> = { [vector.ancestorFile]: "{}", [vector.siblingFile]: "{}" };
  for (const path of vector.nestedFiles) files[path] = "{}";
  for (const compiled of compilers(code)) {
    const fs = virtualFilesystem(root, files);
    const factory = new Function("lstatSync", "readdirSync", "readFileSync", "resolve", "relative", "dirname", "join", "pathIsExcluded", "registryCatalogGitlinkBoundaries", `${compiled}\nreturn registryCatalogInputView;`)(fs.api.lstatSync, fs.api.readdirSync, fs.api.readFileSync, resolve, relative, dirname, join, () => false, () => new Set([vector.boundary]));
    const view = factory(root, {});
    expect(view.kind(vector.boundary)).toBe("directory");
    expect(view.entries(vector.boundary)).toEqual([]);
    const visited: string[] = [];
    const walk = (path: string): void => {
      for (const entry of view.entries(path)) {
        const child = path ? `${path}/${entry.name}` : entry.name;
        visited.push(child);
        if (entry.nodeKind === "directory") walk(child);
      }
    };
    walk("");
    expect(visited).toContain(vector.boundary);
    expect(visited).toContain(vector.ancestorFile);
    expect(visited).toContain(vector.siblingFile);
    for (const nested of vector.nestedFiles) {
      expect(visited).not.toContain(nested);
      expect(fs.reads).not.toContain(nested);
      expect(fs.fileReads).not.toContain(nested);
    }
    expect(fs.reads.some((path) => path.startsWith(`${vector.boundary}/`))).toBe(false);
  }
});

test("registers the registry catalog gitlink boundary gate through Nx and both launch catalogs", () => {
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
