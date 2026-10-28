import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";

const library = join(import.meta.dir, "../../../.."), fixture = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🔁️verifier/🔣️.json"), "utf8"));
const inference = join(library, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"), observation = join(library, "🏗️build/👁️observation/🟨️.mjs"), declared = join(library, "🕸️dependencies/🟦️typescript/🟨️.mjs");

/** 🧒️ Runs one fresh process that prepares source analyses exactly as the graph child's fact check does, optionally analyses through a plain (not revision-pinned) load afterwards, and prints its verdict on the recorded rows. */
function freshVerdict(directory: string, rowsPath: string, plainPath?: string): { holds: boolean; plain?: string[] } {
  const script = join(directory, "verifier.mjs");
  writeFileSync(script, `const producer=await import(${JSON.stringify(inference)});await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
const {observationsHold}=await import(${JSON.stringify(observation)}),rows=JSON.parse((await import("node:fs")).readFileSync(${JSON.stringify(rowsPath)},"utf8"));
const plain=${JSON.stringify(plainPath ?? null)}===null?undefined:(await import(${JSON.stringify(declared)})).sourceRuntimeImportsV1(${JSON.stringify(plainPath ?? "")});
console.log(JSON.stringify({holds:await observationsHold(rows,64,new Set(),true),plain}));`);
  const child = Bun.spawnSync([process.execPath, script], { stdout: "pipe", stderr: "pipe" });
  if (child.exitCode !== 0) throw Error(`Fresh verifier failed: ${child.stderr.toString()}`);
  return JSON.parse(child.stdout.toString().trim().split("\n").at(-1)!);
}

/** 🔁️ A declared-import analysis fact recorded by inference holds in a fresh process that only prepared the graph's fact check: before, the parser registered on first use only, so every graph check found these facts stale and re-inferred their owners on every run. */
test("declared source-import facts hold in a fresh graph fact check and change only with their imports", async () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("Original caller artifacts required");
  mkdirSync(artifacts, { recursive: true });
  expect(readFileSync(join(library, "🏗️build/🕸️graph/🟦️.ts"), "utf8")).toContain(fixture.graphPreparation);
  const root = realpathSync(mkdtempSync(join(artifacts, "fresh-verifier-"))), typescript = createRequire(import.meta.url)("typescript");
  for (const [path, source] of Object.entries(fixture.files)) writeFileSync(join(root, path), source as string);
  const { captureObservedSourceInputsV1 } = await import(observation), { relativeSourceInputsAsyncV1 } = await import(declared);
  const captured = await captureObservedSourceInputsV1(() => relativeSourceInputsAsyncV1(fixture.contract, root), undefined, false);
  const rows = captured.observations.filter((row: { kind: string; fact: { provider?: string } }) => row.kind === "analysis" && row.fact.provider === fixture.provider), rowsPath = join(root, "rows.json");
  expect(rows.length).toBe(Object.keys(fixture.files).length);
  writeFileSync(rowsPath, JSON.stringify(rows));
  const imports = (source: string) => typescript.preProcessFile(source, true, true).importedFiles.map((entry: { fileName: string }) => entry.fileName);
  for (const row of fixture.cases) {
    const original = fixture.files[row.file], source = row.source ?? original;
    writeFileSync(join(root, row.file), source);
    expect(JSON.stringify(imports(source)) === JSON.stringify(imports(original)), row.id).toBe(row.holds);
    expect(freshVerdict(root, rowsPath).holds, row.id).toBe(row.holds);
    writeFileSync(join(root, row.file), original);
  }
  expect(freshVerdict(root, rowsPath, join(root, "entry.ts"))).toEqual({ holds: true, plain: imports(fixture.files["entry.ts"]) });
}, 60000);
