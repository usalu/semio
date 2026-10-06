/** 🖌️ Real OS task admission agrees with the independent system filesystem oracle. */
import { expect, test } from "bun:test";

import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import fixture from "./🧫️fixtures/🔣️.json";

const owner = join(import.meta.dir,"../..");
let repoRoot = owner;
while (!existsSync(join(repoRoot,"nx.json"))) repoRoot = dirname(repoRoot);

for (const row of fixture.cases) test(row.name, async () => {
  const sourcePath = join(owner,"🟦️.ts"), testsPath = join(owner,"🧪️tests/🟦️.ts");
  const missing = join(owner,`missing-${crypto.randomUUID()}`);
  const selectedSource = row.source?sourcePath:missing, selectedTests = row.tests?testsPath:missing;
  const oracle = existsSync(selectedSource) && existsSync(selectedTests);
  expect(oracle).toBe(row.admit);
  const execution = join(repoRoot,"🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts");
  const entrypoint = join(repoRoot,"🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts");
  const source = `import {mock} from "bun:test";const calls=[];const execution=await import(${JSON.stringify(execution)}),fs=await import("node:fs/promises"),access=fs.access;mock.module("node:fs/promises",()=>({...fs,access:async(path)=>access(path===${JSON.stringify(sourcePath)}?${JSON.stringify(selectedSource)}:path===${JSON.stringify(testsPath)}?${JSON.stringify(selectedTests)}:path)}));mock.module(${JSON.stringify(execution)},()=>({...execution,runOwnedCommand:async(executable,argv)=>calls.push(argv)}));mock.module(${JSON.stringify(entrypoint)},()=>({runScriptMain:async(router)=>router.run(["test"])}));await import(${JSON.stringify(join(owner,"📜️script.ts"))});console.log(JSON.stringify(calls));`;
  const child = Bun.spawn([process.execPath,"-e",source],{cwd:repoRoot,stdout:"pipe",stderr:"pipe",env:process.env});
  const [stdout,stderr,status] = await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
  expect(status===0,stderr).toBe(row.admit);
  if (row.admit) {
    const calls = JSON.parse(stdout) as string[][];
    expect(calls.length).toBe(2);
    expect(calls[0]!.at(-1)).toBe(sourcePath);
    expect(calls[1]).toEqual(["test",testsPath,join(owner,"🧪️tests/🧭️ownership/🟦️.ts")]);
  } else expect(stdout).toBe("");
  console.log(`[DEBUG] Paint2dHost task ${row.name}: ${row.admit?"admitted":"refused"}; filesystem oracle=${oracle}`);
});
