import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, lstatSync, utimesSync, symlinkSync, rmSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { spawnSync } from "node:child_process";

type Corpus = { schema: string; writes: readonly { name: string; initial: string; next: string; rewritten: boolean }[]; collections: readonly { name: string; files: Readonly<Record<string, string>>; ordered: readonly string[] }[]; refusals: readonly string[] };

/** 🧰️ Checks neutral file publication and traversal against portable vectors and independent tools. */
export async function testArtifactFiles(outputDirectory: string): Promise<void> {
  const owner = join(dirname(fileURLToPath(import.meta.url)), "..");
  const source = join(owner, "🟦️.ts"), corpus = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")) as Corpus;
  const require = createRequire(import.meta.url), ajv = new (require("ajv").default)();
  assert.equal(ajv.validate(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")), corpus), true, JSON.stringify(ajv.errors));
  const { writeGeneratedFileIfChanged, collectArtifactFiles } = await import(pathToFileURL(source).href);
  mkdirSync(outputDirectory, { recursive: true });
  const temporary = mkdtempSync(join(outputDirectory, "artifact-files-"));
  try {
    for (const row of corpus.writes) {
      const target = join(temporary, `${row.name}.txt`);
      writeFileSync(target, row.initial); utimesSync(target, 946684800, 946684800);
      const before = lstatSync(target).mtimeMs;
      assert.equal(writeGeneratedFileIfChanged(target, row.next), row.rewritten, row.name);
      assert.deepEqual(readFileSync(target), Buffer.from(row.next), row.name);
      assert.equal(lstatSync(target).mtimeMs === before, !row.rewritten, row.name);
    }
    for (const row of corpus.collections) {
      const root = join(temporary, row.name); mkdirSync(root);
      for (const [file, bytes] of Object.entries(row.files)) { const path = join(root, file); mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, bytes); }
      const actual = await collectArtifactFiles(root);
      const oracle = (await require("fast-glob")("**/*", { cwd: root, dot: true, onlyFiles: true, followSymbolicLinks: false })).sort();
      assert.deepEqual([...actual.keys()], row.ordered, row.name); assert.deepEqual([...actual.keys()], oracle, row.name);
      for (const [file, path] of actual) assert.deepEqual(readFileSync(path), Buffer.from(row.files[file]));
    }
    const root = join(temporary, "regular"); mkdirSync(root);
    const file = join(root, "file.txt"); writeFileSync(file, "bytes");
    const rootLink = join(temporary, "root-link"); symlinkSync(root, rootLink, process.platform === "win32" ? "junction" : "dir");
    const fileLink = join(root, "file-link"); symlinkSync(file, fileLink, "file");
    for (const refusal of corpus.refusals) {
      switch (refusal) {
        case "generated-directory": assert.throws(() => writeGeneratedFileIfChanged(root, "x"), /Invalid generated file/); break;
        case "generated-link": assert.throws(() => writeGeneratedFileIfChanged(fileLink, "x"), /Invalid generated file/); break;
        case "artifact-file-root": await assert.rejects(() => collectArtifactFiles(file), /Invalid artifact root/); break;
        case "artifact-link-root": await assert.rejects(() => collectArtifactFiles(rootLink), /Invalid artifact root/); break;
        case "artifact-link-child": await assert.rejects(() => collectArtifactFiles(root), /Unsupported artifact file/); break;
        case "cancelled-before-walk": { const controller = new AbortController(); controller.abort(); await assert.rejects(() => collectArtifactFiles(root, controller.signal), /abort/i); break; }
        default: assert.fail(`Unknown portable refusal: ${refusal}`);
      }
    }
    const bundled = join(temporary, "artifact-files.mjs");
    const result = await require("esbuild").build({ entryPoints: [source], outfile: bundled, bundle: true, platform: "node", format: "esm", metafile: true });
    assert.deepEqual(Object.keys(result.metafile.inputs).map((path: string) => path.replaceAll("\\", "/")), [source.substring(process.cwd().length + 1)]);
    const program = `const api = await import(${JSON.stringify(pathToFileURL(bundled).href)}); const rows = JSON.parse(${JSON.stringify(JSON.stringify(corpus.collections))}); const output=[]; for(const row of rows)output.push([...await api.collectArtifactFiles(${JSON.stringify(temporary)}+'/'+row.name)].map(([key])=>key)); console.log(JSON.stringify(output));`;
    for (const runtime of ["bun", "node"]) {
      const child = spawnSync(runtime, ["--input-type=module", "-e", program], { encoding: "utf8", timeout: 15000 });
      assert.equal(child.status, 0, `${runtime}: ${child.stderr}`); assert.deepEqual(JSON.parse(child.stdout), corpus.collections.map(row => row.ordered));
    }
    console.log(`[DEBUG] artifact-files writes=${corpus.writes.length} collections=${corpus.collections.length} refusals=${corpus.refusals.length} Ajv-fast-glob-Bun-Node=true`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}
