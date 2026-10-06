import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { resolve } from "node:path";
import Ajv from "ajv";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
test("validates neutral owned process vectors with Ajv",()=>expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true));

test("terminates a bounded owned descendant process", async () => {
  const owner = resolve(import.meta.dir, "../🟦️.ts");
  expect(existsSync(owner)).toBe(true);
  if (!existsSync(owner)) return;
  const artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR;
  expect(artifactParent).toBeTruthy();
  if (!artifactParent) return;
  const root = mkdtempSync(resolve(artifactParent, "owned-execution-"));
  const marker = resolve(root, "descendant.txt");
  const source = [
    "const {spawn}=require('node:child_process')",
    "const {writeFileSync}=require('node:fs')",
    "const child=spawn(process.execPath,['-e','setTimeout(() => {}, 10000)'],{stdio:'ignore'})",
    `writeFileSync(${JSON.stringify(marker)},String(child.pid))`,
    "setTimeout(() => {}, 10000)",
  ].join(";");
  const { runOwnedCommand } = await import(owner);
  try {
    await expect(runOwnedCommand(process.execPath, ["-e", source], root, "owned-descendant", 2_000, { stdout: "ignore" })).rejects.toThrow(/timeout 2000ms/);
    const descendant = Number(readFileSync(marker, "utf8"));
    const alive = (): boolean => {
      try {
        process.kill(descendant, 0);
        return true;
      } catch {
        return false;
      }
    };
    for (let attempt = 0; attempt < 40 && alive(); attempt++) await Bun.sleep(25);
    try {
      expect(alive()).toBe(false);
    } finally {
      if (alive()) process.kill(descendant, "SIGKILL");
    }
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
}, { timeout: 10_000 });

test("owned command cancellation and UTF-8 reports match independent Node execution", async () => {
  const owner = resolve(import.meta.dir, "../🟦️.ts"), artifactParent = process.env.SEMIO_TEST_ARTIFACT_DIR!;
  expect(artifactParent).toBeTruthy();
  const root = mkdtempSync(resolve(artifactParent, "owned-reports-")), module = resolve(root, "owner.mjs");
  const { createRequire } = await import("node:module"), { spawnSync } = await import("node:child_process");
  const bundled = await createRequire(import.meta.url)("esbuild").build({entryPoints:[owner],bundle:true,write:false,platform:"node",format:"esm",metafile:true,logLevel:"silent"});
  expect(Object.keys(bundled.metafile.inputs).every((path:string)=>path.startsWith("🧰️framework/🔨️modules/🏃️process/"))).toBe(true);
  writeFileSync(module, bundled.outputFiles[0].text);
  const run = async (api: any, executable: string, row: any): Promise<any> => {
    const marker = resolve(root, row.mode + ".txt"), controller = new AbortController(), lines: string[] = [];
    const source = row.mode === "lines" ? "const b=Buffer.from('first 🧪\\nlast');process.stdout.write(b.subarray(0,8));setTimeout(()=>process.stdout.write(b.subarray(8)),10)" : row.mode === "pre-abort" ? `require('node:fs').writeFileSync(${JSON.stringify(marker)},'spawned')` : row.mode === "abort" ? "console.log('ready');setTimeout(()=>{},10000)" : "process.exit(7)";
    if (row.mode === "pre-abort") controller.abort();
    let error: string | null = null;
    try { await api.runOwnedCommand(executable,["-e",source],root,"owned-vector",2_000,{signal:controller.signal,onLine:(line:string)=>{lines.push(line);if(row.mode === "abort")controller.abort();}}); }
    catch (failure) { error = String(failure); }
    return {lines,error:row.expectedError === null ? error : error?.includes(row.expectedError) ? row.expectedError : error,created:existsSync(marker)};
  };
  const api = await import(owner);
  for (const row of fixture.cases) {
    const expected = {lines:row.expectedLines,error:row.expectedError,created:false};
    expect(await run(api,process.execPath,row)).toEqual(expected);
    const runner = `const fs=require('node:fs');const root=process.argv[2];const row=JSON.parse(fs.readFileSync(0,'utf8'));const existsSync=fs.existsSync;const resolve=require('node:path').resolve;const run=${run.toString()};import(require('node:url').pathToFileURL(process.argv[1]).href).then(api=>run(api,process.execPath,row)).then(value=>console.log('RESULT '+JSON.stringify(value)));`;
    const independent = spawnSync("node",["-e",runner,module,root],{input:JSON.stringify(row),encoding:"utf8",timeout:5_000});
    writeFileSync(resolve(root, row.mode + ".log"), independent.stdout + independent.stderr);
    expect(independent.status).toBe(0);
    const result = independent.stdout.match(/RESULT (\{[^\n]+\})/u);
    expect(JSON.parse(result![1])).toEqual(expected);
  }
}, 20_000);


test("progress remains observable until explicitly stopped", async () => {
  const { startNativeProgress } = await import("../🟦️.ts");
  const lines: string[] = [], stop = startNativeProgress(fixture.progress.label, fixture.progress.intervalMs, line => lines.push(line));
  await Bun.sleep(fixture.progress.intervalMs * 4); stop();
  expect(lines.some(line => new RegExp(fixture.progress.pattern).test(line))).toBe(true);
  const count=lines.length;await Bun.sleep(fixture.progress.intervalMs*4);expect(lines.length).toBe(count);
});
