import Ajv from "ajv";
import { afterAll, expect, test } from "bun:test";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import ts from "typescript";
import { createTaxonomyPathMatcher, semanticOwnedInputFileSnapshot } from "../../../🔍️discovery/🟦️.ts";
import { assertLexicalInputOutsideOpaque, noFollowDirectoryChain, verifyNoFollowDirectoryChain } from "../../📁️input/🟦️.ts";
import { loadTaxonomy, type LoadedTaxonomy } from "../../🔣️taxonomy/🟦️.ts";

type Row = Readonly<{ id: string; action: "none" | "new-root" | "delete-leaf" | "link-leaf" | "link-parent" | "link-root" | "link-ancestor" | "missing-prefix" | "linked-prefix"; accepted: boolean; physical: "file" | "absent" | "symlink" }>;
const root = resolve(import.meta.dir, "../.."), library = dirname(root);
type WitnessRow = Readonly<{ id: string; action: "none" | "replace" | "link" | "delete"; accepted: boolean; physical: "directory" | "absent" | "symlink" }>;
const fixture = JSON.parse(readFileSync(join(root, "🧫️fixtures/🔣️taxonomy-input/🔣️.json"), "utf8")) as Readonly<{ schemaVersion: 1; cases: readonly Row[]; witnessCases: readonly WitnessRow[] }>;

const bytes = readFileSync(join(library, "🔣️taxonomy.json")), expectedHash = createHash("sha256").update(bytes).digest("hex");
type PhysicalObservation = Readonly<{ kind: "file"; hash: string }> | Readonly<{ kind: "directory" | "other"; hash: null }> | Readonly<{ kind: "absent" | "symlink" }>;
const oracle = 'const fs=require("node:fs"),crypto=require("node:crypto"),readline=require("node:readline");readline.createInterface({input:process.stdin}).on("line",line=>{const {id,path}=JSON.parse(line);let result;try{const s=fs.lstatSync(path);result=s.isSymbolicLink()?{kind:"symlink"}:{kind:s.isFile()?"file":s.isDirectory()?"directory":"other",hash:s.isFile()?crypto.createHash("sha256").update(fs.readFileSync(path)).digest("hex"):null};}catch(e){if(e.code!=="ENOENT"){process.stdout.write(JSON.stringify({id,error:e.message})+"\\n");return;}result={kind:"absent"};}process.stdout.write(JSON.stringify({id,result})+"\\n");});';
const requests = new Map<number, { resolve: (value: PhysicalObservation) => void; reject: (error: Error) => void }>();
let oracleProcess: ReturnType<typeof spawn> | undefined, oracleCompletion: Promise<number | null> | undefined, oracleId = 0;
let oracleFailure: Error | undefined, oracleClosed = false;
/** 🔬️ Shares one independent native observer while preserving each fixture's exact observation. */
function observe(path: string): Promise<PhysicalObservation> {
  if (oracleFailure || oracleClosed) return Promise.reject(oracleFailure ?? new Error("Physical oracle is closed"));
  if (!oracleProcess) {
    oracleProcess = spawn("node", ["-e", oracle], { stdio: ["pipe", "pipe", "inherit"] });
    const refuse = (error: Error): void => { oracleFailure = error; for (const request of requests.values()) request.reject(error); requests.clear(); };
    oracleCompletion = new Promise((resolve) => { oracleProcess!.once("close", (code) => { oracleClosed = true; if (requests.size) refuse(new Error("Physical oracle closed with pending observations: " + code)); resolve(code); }); oracleProcess!.once("error", (error) => { refuse(error); resolve(null); }); });
    oracleProcess.stdin!.on("error", refuse);
    oracleProcess.stdout!.on("error", refuse);
    let pending = "";
    oracleProcess.stdout!.setEncoding("utf8");
    oracleProcess.stdout!.on("data", (chunk: string) => {
      pending += chunk;
      let end: number;
      while ((end = pending.indexOf("\n")) >= 0) {
        let response: { id: number; result: PhysicalObservation; error?: string };
        try { response = JSON.parse(pending.slice(0, end)); } catch (error) { refuse(error as Error); return; }
        pending = pending.slice(end + 1);
        const request = requests.get(response.id);
        if (!request) { refuse(new Error("Physical oracle returned an unknown observation")); return; }
        requests.delete(response.id);
        if (response.error) request.reject(new Error(response.error)); else request.resolve(response.result);
      }
    });
  }
  return new Promise((resolve, reject) => { const id = ++oracleId; requests.set(id, { resolve, reject }); oracleProcess!.stdin!.write(JSON.stringify({ id, path }) + "\n"); });
}
afterAll(async () => {
  if (!oracleProcess) return;
  oracleProcess.stdin?.end();
  const cancel = setTimeout(() => oracleProcess!.kill(), 1_000);
  try { expect(await oracleCompletion).toBe(0); } finally { clearTimeout(cancel); }
});
const link = (target: string, path: string, directory: boolean): void => symlinkSync(target, path, directory ? process.platform === "win32" ? "junction" : "dir" : "file");

test("taxonomy physical input actions satisfy an independent closed schema", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["cases"]).toEqual([{"id":"ordinary-warm","action":"none","accepted":true,"physical":"file"},{"id":"same-bytes-new-root","action":"new-root","accepted":true,"physical":"file"},{"id":"warm-deleted-leaf","action":"delete-leaf","accepted":false,"physical":"absent"},{"id":"warm-linked-leaf","action":"link-leaf","accepted":false,"physical":"symlink"},{"id":"warm-linked-inner-parent","action":"link-parent","accepted":false,"physical":"symlink"},{"id":"warm-linked-root","action":"link-root","accepted":false,"physical":"symlink"},{"id":"warm-linked-root-ancestor","action":"link-ancestor","accepted":false,"physical":"symlink"},{"id":"missing-prefix-before-parent","action":"missing-prefix","accepted":false,"physical":"absent"},{"id":"linked-prefix-before-parent","action":"linked-prefix","accepted":false,"physical":"symlink"}]);expect(fixture["witnessCases"]).toEqual([{"id":"unchanged-after-capture","action":"none","accepted":true,"physical":"directory"},{"id":"replaced-after-capture","action":"replace","accepted":false,"physical":"directory"},{"id":"linked-after-capture","action":"link","accepted":false,"physical":"symlink"},{"id":"absent-after-capture","action":"delete","accepted":false,"physical":"absent"}]);
  
  
  
});

for (const row of fixture.cases) test(row.id, async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Taxonomy input laws require an owned ticket artifact directory");
  mkdirSync(output, { recursive: true });
  const base = mkdtempSync(join(output, "taxonomy-input-")), ancestor = join(base, "ancestor");
  let repoRoot = join(ancestor, "repo"), taxonomyPath = "input/taxonomy.json", probe = join(repoRoot, taxonomyPath);
  mkdirSync(dirname(probe), { recursive: true });
  writeFileSync(probe, bytes);
  try {
    expect(await observe(probe)).toEqual({ kind: "file", hash: expectedHash });
    const first = loadTaxonomy({ repoRoot, taxonomyPath });
    expect(first.input.contentHash).toBe(expectedHash);
    if (row.action === "new-root") { repoRoot = join(base, "second"); probe = join(repoRoot, taxonomyPath); mkdirSync(dirname(probe), { recursive: true }); writeFileSync(probe, bytes); }
    if (row.action === "delete-leaf") rmSync(probe);
    if (row.action === "link-leaf") { const target = join(base, "leaf.json"); renameSync(probe, target); link(target, probe, false); }
    if (row.action === "link-parent") { const parent = dirname(probe), target = join(base, "input"); renameSync(parent, target); link(target, parent, true); probe = parent; }
    if (row.action === "link-root") { const target = join(base, "root"); renameSync(repoRoot, target); link(target, repoRoot, true); probe = repoRoot; }
    if (row.action === "link-ancestor") { const target = join(base, "parent"); renameSync(ancestor, target); link(target, ancestor, true); probe = ancestor; }
    if (row.action === "missing-prefix") { taxonomyPath = "missing/../input/taxonomy.json"; probe = join(repoRoot, "missing"); }
    if (row.action === "linked-prefix") { const target = join(base, "prefix"); mkdirSync(target); probe = join(repoRoot, "linked"); link(target, probe, true); taxonomyPath = "linked/../input/taxonomy.json"; }
    const physical = await observe(probe);
    expect(physical.kind, row.id).toBe(row.physical);
    if (row.accepted) {
      const next = loadTaxonomy({ repoRoot, taxonomyPath });
      expect(next.input.contentHash).toBe(expectedHash);
      expect(next.input).not.toBe(first.input);
      expect(next.schema).not.toBe(first.schema);
      expect(next.schema).toEqual(first.schema);
      expect(next.pathMatcher).not.toBe(first.pathMatcher);
      if (row.action === "new-root") expect(next.path).not.toBe(first.path);
      expect(await observe(join(repoRoot, taxonomyPath))).toEqual({ kind: "file", hash: expectedHash });
    } else expect(() => loadTaxonomy({ repoRoot, taxonomyPath }), row.id).toThrow();
    console.log("[DEBUG] taxonomy input oracle; id=" + row.id + "; physical=" + physical.kind + "; accepted=" + row.accepted);
  } finally { rmSync(base, { recursive: true, force: true }); }
}, 5_000);

const loaderProviders = new Map<string, string>();
for (const compiler of ["Bun", "TypeScript"] as const) for (const row of fixture.witnessCases) test("current loader retains absolute ancestor witnesses across the captured input read: " + compiler + ": " + row.id, async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("Taxonomy witness laws require an owned ticket artifact directory");
  let code = loaderProviders.get(compiler);
  if (!code) {
    const path = join(root, "🔣️taxonomy/🟦️.ts"), source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
    const declarations = source.statements.filter((node): node is ts.FunctionDeclaration => ts.isFunctionDeclaration(node) && node.name?.text === "loadTaxonomy");
    expect(declarations).toHaveLength(1);
    const declaration = declarations[0]!.getText(source).replace(/^export /u, "");
    code = compiler === "Bun" ? new Bun.Transpiler({ loader: "ts" }).transformSync(declaration) : ts.transpileModule(declaration, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
    loaderProviders.set(compiler, code);
  }
    const base = mkdtempSync(join(output, "taxonomy-witness-")), ancestor = join(base, "ancestor"), repoRoot = join(ancestor, "repo"), taxonomyPath = "input/taxonomy.json";
    mkdirSync(join(repoRoot, "input"), { recursive: true });
    writeFileSync(join(repoRoot, taxonomyPath), bytes);
    try {
      const first = loadTaxonomy({ repoRoot, taxonomyPath });
      const { path: _path, input: _input, pathMatcher: _matcher, ...facts } = first;
      let captures = 0;
      const capture = (root: string, coordinate: string) => {
        captures++;
        const input = semanticOwnedInputFileSnapshot(root, coordinate);
        if (row.action === "delete") rmSync(ancestor, { recursive: true });
        if (row.action === "link" || row.action === "replace") {
          const target = join(base, "prior"); renameSync(ancestor, target);
          if (row.action === "link") link(target, ancestor, true);
          else { mkdirSync(join(repoRoot, "input"), { recursive: true }); writeFileSync(join(repoRoot, taxonomyPath), bytes); }
        }
        return input;
      };
      const environment = { assertLexicalInputOutsideOpaque, noFollowDirectoryChain, verifyNoFollowDirectoryChain, semanticOwnedInputFileSnapshot: capture, relative, resolve, PARSED_TAXONOMIES: new Map([[expectedHash, facts]]), PARSED_TAXONOMY_CAPACITY: 8, parseTaxonomy: () => { throw new Error("Warm witness law must use the already validated current-byte facts"); }, createTaxonomyPathMatcher, TAXONOMY_RELATIVE_PATH: taxonomyPath };
      const load = new Function(...Object.keys(environment), code + "\nreturn loadTaxonomy;")(...Object.values(environment)) as typeof loadTaxonomy;
      let loaded: LoadedTaxonomy | undefined, error: unknown;
      try { loaded = load({ repoRoot, taxonomyPath }); } catch (failure) { error = failure; }
      expect(captures, row.id).toBe(1);
      expect(error === undefined, compiler + ": " + row.id).toBe(row.accepted);
      expect((await observe(ancestor)).kind, row.id).toBe(row.physical);
      if (row.accepted) expect(loaded?.input.contentHash).toBe(expectedHash);
      else expect(error).toBeInstanceOf(Error);
      console.log("[DEBUG] taxonomy witness oracle; provider=" + compiler + "; id=" + row.id + "; accepted=" + row.accepted);
    } finally { rmSync(base, { recursive: true, force: true }); }
}, 5_000);
