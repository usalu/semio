import { expect, test } from "bun:test";
import { dirname, join, relative, resolve } from "node:path";
import { copyFileSync, lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";
import { execFileSync } from "node:child_process";
import Ajv from "ajv";
import fg from "fast-glob";
import plugin, { cacheInternals } from "../../../🟨️.mjs";

type Corpus = { version: 1; files: Record<string, string>; cases: { id: string; files: string[]; names: string[] }[]; authorities: string[]; snapshot: { entry: string; inputs: string[]; nextSource: string; nextFile: string; nextInputs: string[] } };
const fixture = JSON.parse(readFileSync(join(import.meta.dir, "..", "🧫️fixtures/🔣️.json"), "utf8")) as Corpus;
const schema = JSON.parse(readFileSync(join(import.meta.dir, "..", "🧬️schema/🔣️.json"), "utf8"));
const toml = createRequire(import.meta.url)("@iarna/toml");
const library = resolve(import.meta.dir, "../../.."), repository = resolve(library, "../../../../..");

/** 🧬️ Executes the actual authored plugin within its complete owned command-source closure. */
async function copiedPlugin(root: string): Promise<typeof plugin> {
  const revision = JSON.parse(readFileSync(join(library, "⚡️caching/🧫️fixtures/🔁️graph-revision/🔣️.json"), "utf8"));
  const files = [...revision.files.map((path: string) => relative(repository, resolve(library, path))), "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs", ...cacheInternals.relativeScriptInputs(revision.commandSources.map((path: string) => join(repository, path)), repository).map((path: string) => path.slice("{workspaceRoot}/".length))];
  for (const path of new Set<string>(files)) { const destination = join(root, path); mkdirSync(dirname(destination), { recursive: true }); copyFileSync(join(repository, path), destination); }
  return (await import(pathToFileURL(join(root, relative(repository, library), "🟨️.mjs")).href)).default;
}

/** 📥️ Materializes each producer law in its caller's ticket without running native commands. */
function workspace(): string {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "nx-inference-"));
  for (const [path, source] of Object.entries(fixture.files)) { mkdirSync(dirname(join(root, path)), { recursive: true }); writeFileSync(join(root, path), source); }
  return root;
}

/** 🔎️ The independent inventory never follows links and intersects precisely the supplied list. */
function oracle(root: string, supplied: readonly string[]): string[] {
  const inventory = new Set(fg.sync(plugin.createNodesV2[0], { cwd: root, dot: true, followSymbolicLinks: false, ignore: ["**/target/**"] }));
  const paths = [...new Set(supplied.filter(path => !path.includes("\0") && !path.split(/[\\/]/u).some(part => !part || part === "." || part === "..")).map(path => path.replaceAll("\\", "/")))].filter(path => inventory.has(path));
  const explicit = new Map(paths.filter(path => path.endsWith("📋️project.json")).map(path => [dirname(path), JSON.parse(readFileSync(join(root, path), "utf8")).name as string]));
  const names = [...explicit.values()];
  for (const path of paths.filter(path => path.endsWith("Cargo.toml"))) if (!explicit.has(dirname(path))) names.push(toml.parse(readFileSync(join(root, path), "utf8")).package.name);
  return names.sort();
}

test("closed inference corpus preserves unique semantic cases", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  expect(new Set(fixture.cases.map(row => row.id)).size).toBe(fixture.cases.length);
});

test("actual inference honors only fresh no-follow supplied candidates", async () => {
  const root = workspace();
  try {
    const actual = await copiedPlugin(root);
    symlinkSync(join(root, "a"), join(root, "linked"), process.platform === "win32" ? "junction" : "dir");
    mkdirSync(join(root, "leaf-link"));
    symlinkSync(process.platform === "win32" ? join(root, "a") : join(root, "a/Cargo.toml"), join(root, "leaf-link/Cargo.toml"), process.platform === "win32" ? "junction" : "file");
    expect(lstatSync(join(root, "linked")).isSymbolicLink()).toBe(true);
    const nodeOracle = String.raw`const fs=require("node:fs"),p=require("node:path"),fg=require("fast-glob"),toml=require("@iarna/toml");const root=process.argv[1],cases=JSON.parse(process.argv[2]);const inventory=new Set(fg.sync("**/{📋️project.json,Cargo.toml,bun.lock,*.patch}",{cwd:root,dot:true,followSymbolicLinks:false,ignore:["**/target/**"]}));const rows=cases.map(row=>{const paths=[...new Set(row.files.filter(path=>!path.includes("\0")&&!path.split(/[\\/]/).some(part=>!part||part==="."||part==="..")).map(path=>path.replaceAll("\\","/")))].filter(path=>inventory.has(path));const explicit=new Map(paths.filter(path=>path.endsWith("📋️project.json")).map(path=>[p.dirname(path),JSON.parse(fs.readFileSync(p.join(root,path),"utf8")).name]));const names=[...explicit.values()];for(const path of paths.filter(path=>path.endsWith("Cargo.toml")))if(!explicit.has(p.dirname(path)))names.push(toml.parse(fs.readFileSync(p.join(root,path),"utf8")).package.name);return{id:row.id,names:names.sort()};});process.stdout.write(JSON.stringify(rows));`;
    const nodeRows = JSON.parse(execFileSync("node", ["-e", nodeOracle, root, JSON.stringify(fixture.cases)], { cwd: repository, encoding: "utf8", timeout: 3_000 }));
    expect(nodeRows).toEqual(fixture.cases.map(row => ({ id: row.id, names: row.names })));
    for (const row of fixture.cases) {
      const files = row.files.map(path => path === "$absolute" ? join(root, "a/Cargo.toml") : path);
      expect(oracle(root, files), row.id).toEqual(row.names);
      const result = await actual.createNodesV2[1](files, {}, { workspaceRoot: root });
      expect(result.flatMap(([, row]) => Object.keys(row.projects)).sort(), row.id).toEqual(row.names);
    }
    rmSync(join(root, "a/Cargo.toml"));
    expect((await actual.createNodesV2[1](["a/Cargo.toml"], {}, { workspaceRoot: root })).length).toBe(0);
    writeFileSync(join(root, "a/Cargo.toml"), fixture.files["a/Cargo.toml"]!);
    expect((await actual.createNodesV2[1](["a/Cargo.toml"], {}, { workspaceRoot: root })).flatMap(([, row]) => Object.keys(row.projects))).toEqual(oracle(root, ["a/Cargo.toml"]));
    const rootLink = root + "-link";
    symlinkSync(root, rootLink, process.platform === "win32" ? "junction" : "dir");
    try { await expect(actual.createNodesV2[1]([], {}, { workspaceRoot: rootLink })).rejects.toThrow("real workspace ancestry"); }
    finally { rmSync(rootLink); }
    for (const authority of fixture.authorities) {
      const path = join(root, relative(repository, library), authority), source = readFileSync(path), target = join(root, "authority-copy");
      writeFileSync(target, source); rmSync(path);
      symlinkSync(process.platform === "win32" ? root : target, path, process.platform === "win32" ? "junction" : "file");
      try {
        expect(lstatSync(path).isSymbolicLink()).toBe(true);
        await expect(actual.createNodesV2[1]([], {}, { workspaceRoot: root })).rejects.toThrow("real authority file");
      } finally { rmSync(path); writeFileSync(path, source); rmSync(target); }
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}, 30_000);

test("Rust source closure reuse stays inside one fresh inference snapshot", () => {
  const root = workspace(), cache = cacheInternals.createRustSourceCache();
  class Observations extends Map { lookups = 0; override has(key: unknown): boolean { this.lookups++; return super.has(key); } }
  const first = new Observations(), entry = join(root, fixture.snapshot.entry);
  const inputs = (snapshot: Observations): string[] => cacheInternals.rustSourceFiles([entry], root, cache, snapshot).map((path: string) => relative(root, path).replaceAll("\\", "/"));
  try {
    expect(inputs(first)).toEqual(fixture.snapshot.inputs);
    expect(first.lookups).toBeGreaterThan(0);
    first.lookups = 0;
    expect(inputs(first)).toEqual(fixture.snapshot.inputs);
    expect(first.lookups).toBe(0);
    writeFileSync(entry, fixture.snapshot.nextSource);
    writeFileSync(join(root, fixture.snapshot.nextFile), "pub const VALUE: u8 = 4;");
    const second = new Observations();
    expect(inputs(second)).toEqual(fixture.snapshot.nextInputs);
    expect(second.lookups).toBeGreaterThan(0);
    expect(fg.sync("a/*.rs", { cwd: root, followSymbolicLinks: false }).sort()).toEqual([...new Set([...fixture.snapshot.inputs, fixture.snapshot.nextFile])].sort());
  } finally { rmSync(root, { recursive: true, force: true }); }
});
