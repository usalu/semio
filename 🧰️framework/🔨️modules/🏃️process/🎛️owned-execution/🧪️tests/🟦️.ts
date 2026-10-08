import { expect, test } from "bun:test";
import { existsSync, mkdtempSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { resolve } from "node:path";

const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));

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
    try { await api.runOwnedCommand(executable,["-e",source],root,"owned-vector",fixture.reportExecutionBudgetMs,{signal:controller.signal,onLine:(line:string)=>{lines.push(line);if(row.mode === "abort")controller.abort();}}); }
    catch (failure) { error = String(failure); }
    return {lines,error:row.expectedError === null ? error : error?.includes(row.expectedError) ? row.expectedError : error,created:existsSync(marker)};
  };
  const api = await import(owner);
  for (const row of fixture.cases) {
    const expected = {lines:row.expectedLines,error:row.expectedError,created:false};
    expect(await run(api,process.execPath,row)).toEqual(expected);
    const runner = `const fs=require('node:fs');const root=process.argv[2];const row=JSON.parse(fs.readFileSync(0,'utf8'));const existsSync=fs.existsSync;const resolve=require('node:path').resolve;const fixture=${JSON.stringify({reportExecutionBudgetMs:fixture.reportExecutionBudgetMs})};const run=${run.toString()};import(require('node:url').pathToFileURL(process.argv[1]).href).then(api=>run(api,process.execPath,row)).then(value=>console.log('RESULT '+JSON.stringify(value)));`;
    const independent = spawnSync("node",["-e",runner,module,root],{input:JSON.stringify(row),encoding:"utf8",timeout:fixture.reportExecutionBudgetMs+5_000});
    writeFileSync(resolve(root, row.mode + ".log"), independent.stdout + independent.stderr);
    expect(independent.status).toBe(0);
    const result = independent.stdout.match(/RESULT (\{[^\n]+\})/u);
    expect(JSON.parse(result![1])).toEqual(expected);
  }
}, 60_000);


test("progress remains observable until explicitly stopped", async () => {
  const { startNativeProgress } = await import("../🟦️.ts");
  const lines: string[] = [], stop = startNativeProgress(fixture.progress.label, fixture.progress.intervalMs, line => lines.push(line));
  await Bun.sleep(fixture.progress.intervalMs * 4); stop();
  expect(lines.some(line => new RegExp(fixture.progress.pattern).test(line))).toBe(true);
  const count=lines.length;await Bun.sleep(fixture.progress.intervalMs*4);expect(lines.length).toBe(count);
});


test("owned timeout and stop drain descendant-created groups and inherited pipes", async () => {
  const { spawn, spawnSync } = await import("node:child_process"), { default: treeKill } = await import("tree-kill");
  const corpus = JSON.parse(readFileSync(resolve(import.meta.dir, "../🧫️fixtures/🌳️ownership/🔣️.json"), "utf8"));
  const { runOwnedCommand } = await import("../🟦️.ts");
  const root = mkdtempSync(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!, "owned-groups-"));
  const alive = (pid: number): boolean => {
    try { process.kill(pid, 0); } catch { return false; }
    if (process.platform === "win32") return true;
    const state = spawnSync("ps", ["-o", "stat=", "-p", String(pid)], { encoding: "utf8" });
    return state.status === 0 && !state.stdout.trim().startsWith("Z");
  };
  const source = "const {spawn}=require('node:child_process');const child=spawn(process.execPath,['-e','setInterval(()=>{},10000)'],{detached:process.platform!=='win32',stdio:'inherit'});console.log('READY '+JSON.stringify({root:process.pid,descendant:child.pid}));setInterval(()=>{},10000)";
  const unrelated = spawn("node", ["-e", "setInterval(()=>{},10000)"], { stdio: "ignore" });
  const unrelatedExited = new Promise<void>(accept => unrelated.once("exit", () => accept()));
  try {
    for (const row of corpus.cases) {
      for (const implementation of ["tree-kill", "owned"] as const) {
        let pids: number[] = [], settled = false, failure = "", oracle: ReturnType<typeof spawn> | undefined, work: Promise<void> | undefined;
        const controller = new AbortController();
        const ready = (line: string): void => {
          if (!line.startsWith("READY ")) return;
          const value = JSON.parse(line.slice(6)); pids = [value.root, value.descendant];
          if (process.platform !== "win32") {
            const groups = spawnSync("ps", ["-o", "pgid=", "-p", pids.join(",")], { encoding: "utf8" }).stdout.trim().split(/\s+/).map(Number);
            expect(new Set(groups).size).toBe(2);
          }
          if (implementation === "tree-kill") setTimeout(() => treeKill(value.root, "SIGKILL"), row.mode === "timeout" ? row.timeoutMs : 0);
          else if (row.mode === "abort") controller.abort();
          else if (row.mode === "stop") process.emit("SIGTERM");
        };
        try {
          if (implementation === "owned") {
            work = runOwnedCommand("node", ["-e", source], root, "owned-groups", row.timeoutMs, { signal: controller.signal, onLine: ready }).catch(error => { failure = String(error); }).finally(() => { settled = true; });
          } else {
            oracle = spawn("node", ["-e", source], { detached: process.platform !== "win32", stdio: ["ignore", "pipe", "pipe"] });
            oracle.stdout!.on("data", bytes => String(bytes).trim().split("\n").forEach(ready));
            work = new Promise<void>(accept => oracle!.once("close", () => { settled = true; failure = row.expected.reason; accept(); }));
          }
          await Promise.race([work, Bun.sleep(row.settleMs)]);
          for (let attempt = 0; attempt < 20 && pids.some(alive); attempt++) await Bun.sleep(25);
          const result = { settled, survivors: pids.filter(alive).length, unrelatedAlive: alive(unrelated.pid!), reason: failure.includes(row.expected.reason) ? row.expected.reason : failure };
          console.log(`[DEBUG] owned groups implementation=${implementation} mode=${row.mode} result=${JSON.stringify(result)}`);
          expect(pids.length).toBe(2);
          expect(result).toEqual(row.expected);
        } finally {
          for (const pid of [...pids].reverse()) if (alive(pid)) process.kill(pid, "SIGKILL");
          if (oracle?.exitCode === null && oracle.signalCode === null) oracle.kill("SIGKILL");
          if (work) await Promise.race([work, Bun.sleep(1000)]);
        }
      }
    }
  } finally {
    unrelated.kill("SIGKILL"); await unrelatedExited; rmSync(root, { recursive: true, force: true });
  }
}, 60_000);
