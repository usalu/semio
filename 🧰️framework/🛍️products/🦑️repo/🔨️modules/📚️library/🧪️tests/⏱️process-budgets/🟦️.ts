import { expect, test } from "bun:test";
import { readFileSync, mkdtempSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { buildBudgetMs, cmdBudgetMs, daemonBudgetMs, defaultBudgetMs, orchestratorBudgetMs, workspaceScriptExists } from "../../🏃️process/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️process-budgets/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/⏱️process-budgets/🔣️.json", import.meta.url), "utf8"));
const libraryPath = fileURLToPath(new URL("../../📦️packages/🟦️typescript/🟦️.ts", import.meta.url));
/** 🧵️ The Execa 1.x completion oracle, typed at its CommonJS boundary. */
type ExecaOracle = (file: string, args: readonly string[], options: { readonly env?: NodeJS.ProcessEnv; readonly timeout?: number; readonly reject?: boolean }) => Promise<{ readonly code: number | null; readonly stdout: string; readonly stderr: string; readonly timedOut: boolean }>;
const execa: ExecaOracle = createRequire(import.meta.url)("execa");
const budgetKeys = ["SEMIO_BUILD_BUDGET_MS", "SEMIO_CMD_BUDGET_MS", "SEMIO_ORCHESTRATOR_BUDGET_MS", "SEMIO_DAEMON_BUDGET_MS"];

test("owned snapshot Cargo feature admission remains separate from the ordinary artifact suite",async()=>{
 const corpus=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️snapshot-test-selection/🔣️.json",import.meta.url),"utf8"));
 const artifactRoot=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifactRoot)throw Error("SEMIO_TEST_ARTIFACT_DIR must name the ticket generated directory");
 const root=fileURLToPath(new URL("../../../../../../../",import.meta.url)),runner=fileURLToPath(new URL("../../⚡️caching/📦️artifacts/🦀️rust/🟦️.ts",import.meta.url));
 for(const row of corpus.cases){const output=mkdtempSync(join(artifactRoot,"snapshot-features-"));
  const code=`const{mock}=await import("bun:test");const library=await import(${JSON.stringify(libraryPath)});mock.module(${JSON.stringify(libraryPath)},()=>({...library,runCargoTestBudgeted:async(packages,cwd,args)=>{console.log(JSON.stringify({packages,cwd,args}));console.log("owned-assertions-complete")}}));process.argv=[process.execPath,"owned-route",...${JSON.stringify(row.command)}];const{runArtifactRustPackageMain}=await import(${JSON.stringify(runner)});await runArtifactRustPackageMain(${JSON.stringify(join(root,corpus.packageRoot))},${JSON.stringify(corpus.packageName)},{testFeatures:${JSON.stringify(corpus.ordinaryFeatures)},snapshotSqliteTestFeatures:${row.snapshotFeatures===undefined?"undefined":JSON.stringify(row.snapshotFeatures)},snapshotSqliteTests:["owned-test-source"]});`;
  const result=await execa(process.execPath,["-e",code],{env:{...cleanEnv(),SEMIO_TEST_ARTIFACT_DIR:output},timeout:10000,reject:false});if(result.code!==0)throw Error(`Owned feature route failed: ${result.stdout}\n${result.stderr}`);expect(result.code).toBe(0);expect(result.stdout).toContain("owned-assertions-complete");
  const call=JSON.parse(result.stdout.split(/\r?\n/u).find(line=>line.startsWith("{"))!);expect(call.packages).toEqual([corpus.packageName]);expect(call.cwd).toBe(root.replace(/[/\\]$/u,""));expect(call.args.flatMap((value:string,index:number)=>value==="--features"?[call.args[index+1]]:[])).toEqual(row.expectedFeatures);expect(call.args).toContain("--lib");

 }
},30000);

/** 🧼️ Keeps the budget contract independent of the developer's launch environment. */
function cleanEnv(): NodeJS.ProcessEnv {
  return { ...process.env, ...Object.fromEntries(budgetKeys.map(key => [key, undefined])), SEMIO_COVERAGE: "0" };
}

test("process budgets follow neutral defaults and opt-in overrides", () => {
  expect(new Ajv({ strict: true }).validate(schema, fixture)).toBe(true);
  const previous = budgetKeys.map(key => process.env[key]);
  const readers = { build: buildBudgetMs, command: cmdBudgetMs, orchestrator: orchestratorBudgetMs, daemon: daemonBudgetMs };
  try {
    for (const key of budgetKeys) delete process.env[key];
    expect(Object.fromEntries(Object.entries(readers).map(([kind, read]) => [kind, read()]))).toEqual(fixture.defaults);
    expect(defaultBudgetMs("cargo")).toBe(fixture.defaults.build);
    expect(defaultBudgetMs("bun")).toBe(fixture.defaults.command);
    for (const value of [125, 0]) {
      for (const key of budgetKeys) process.env[key] = String(value);
      expect(Object.values(readers).map(read => read())).toEqual([value, value, value, value]);
    }
  } finally {
    budgetKeys.forEach((key, index) => {
      if (previous[index] === undefined) delete process.env[key];
      else process.env[key] = previous[index];
    });
  }
});

for (const row of fixture.processes) {
  test(`process budgets match Execa completion and timeout: ${row.name}`, async () => {
    const child = `await new Promise(resolve => setTimeout(resolve, ${row.durationMs})); console.log("build-complete");`;
    const code = `const { runProbe } = await import(${JSON.stringify(libraryPath)}); try { const result = runProbe(process.execPath, ["-e", ${JSON.stringify(child)}], { budgetMs: ${row.budgetMs} }); console.log(JSON.stringify({ status: result.status, output: result.stdout.trim(), timeout: false })); } catch (error) { console.log(JSON.stringify({ status: null, output: "", timeout: error.code === "ETIMEDOUT" })); }`;
    const [reference, actual] = await Promise.all([
      execa(process.execPath, ["-e", child], { env: cleanEnv(), timeout: row.budgetMs, reject: false }),
      execa(process.execPath, ["-e", code], { env: cleanEnv(), timeout: 10000, reject: false }),
    ]);
    expect(actual.code).toBe(0);
    const result = JSON.parse(actual.stdout);
    expect(result.timeout).toBe(row.timeout);
    expect(result.output).toBe(row.output);
    expect(Boolean(reference.timedOut)).toBe(result.timeout);
    expect(reference.stdout).toBe(result.output);
    if (!row.timeout) expect(result.status).toBe(reference.code);
  }, 12000);
}

for (const runner of ["runCmd", "runCmdStatus", "runTestBudgeted"]) {
  test(`process budgets let ${runner} complete with the unlimited build preset`, async () => {
    const code = `const { ${runner}, buildBudgetMs } = await import(${JSON.stringify(libraryPath)}); await ${runner}(process.execPath, ["-e", "await Bun.sleep(120); console.log('build-complete')"], { budgetMs: buildBudgetMs() });`;
    const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_CMD_BUDGET_MS: "1", SEMIO_TEST_BUDGET_MS: "1" }, timeout: 5000, reject: false });
    expect(result.code, result.stderr).toBe(0);
    expect(result.stdout).toBe("build-complete");
    expect(result.stderr).not.toContain("[budget]");
  });
}

test("process budgets preserve explicit async test deadlines", async () => {
  const code = `const { runTestBudgeted } = await import(${JSON.stringify(libraryPath)}); await runTestBudgeted(process.execPath, ["-e", "await Bun.sleep(2000)"], { budgetMs: 100 });`;
  const result = await execa(process.execPath, ["-e", code], { env: cleanEnv(), timeout: 5000, reject: false });
  expect(result.code).not.toBe(0);
  expect(result.stderr).toContain("[budget]");
});

test("process budgets let captured nextest compilation finish before budgeted assertions", async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw new Error("SEMIO_TEST_ARTIFACT_DIR must name the active ticket generated directory");
  const directory = mkdtempSync(join(artifactRoot, "unlimited-nextest-"));
  const code = `
    const { mock } = await import("bun:test");
    const preparePath=${JSON.stringify(fileURLToPath(new URL("../../🗂️workspaces/🦀️cargo/📜️script.ts",import.meta.url)))};const originalBunSpawnSync=Bun.spawnSync.bind(Bun);Bun.spawnSync=(args,options)=>args[1]===preparePath?{exitCode:0,stdout:new Uint8Array(),stderr:new Uint8Array()}:originalBunSpawnSync(args,options);
    const native = await import("node:child_process");
    const spawn = native.spawn, spawnSync = native.spawnSync;
    mock.module("node:child_process", () => ({
      ...native,
      spawnSync: (cmd, args, opts) => cmd === "cargo" ? { status: 0 } : spawnSync(cmd, args, opts),
      spawn: (cmd, args, opts) => cmd === "cargo" ? spawn(process.execPath, ["-e", args.includes("list") ? "await Bun.sleep(120); console.log('{}')" : "console.log('assertions-complete')"], opts) : spawn(cmd, args, opts),
    }));
    const { runCargoTestBudgeted } = await import(${JSON.stringify(libraryPath)});
    await runCargoTestBudgeted([], process.cwd());
  `;
  const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_BUILD_BUDGET_MS: "0", SEMIO_TEST_BUDGET_MS: "1000", SEMIO_TEST_ARTIFACT_DIR: directory }, timeout: 5000, reject: false });
  expect(result.code, result.stderr).toBe(0);
  expect(result.stdout).toBe("assertions-complete");
});

for (const nextest of [true, false]) {
  test(`process budgets separate coverage compilation from assertions: nextest=${nextest}`, async () => {
    const code = `
      const { mock } = await import("bun:test");
      const preparePath=${JSON.stringify(fileURLToPath(new URL("../../🗂️workspaces/🦀️cargo/📜️script.ts",import.meta.url)))};const originalBunSpawnSync=Bun.spawnSync.bind(Bun);Bun.spawnSync=(args,options)=>args[1]===preparePath?{exitCode:0,stdout:new Uint8Array(),stderr:new Uint8Array()}:originalBunSpawnSync(args,options);
    const native = await import("node:child_process");
      const spawn = native.spawn, spawnSync = native.spawnSync;
      mock.module("node:child_process", () => ({
        ...native,
        spawnSync: (cmd, args, opts) => cmd === "cargo" ? { status: ${nextest ? 0 : 1} } : spawnSync(cmd, args, opts),
        spawn: (cmd, args, opts) => {
          if (cmd !== "cargo") return spawn(cmd, args, opts);
          const building = args.includes("--no-run") || args.includes("--list");
          const reporting = args.includes("report");
          if (!building && !reporting && !args.includes("--no-clean")) throw new Error("Coverage must reuse its compiled artifacts");
          return spawn(process.execPath, ["-e", building ? "await Bun.sleep(300); console.log('coverage-built')" : reporting ? "console.log('coverage-reported')" : "console.log('assertions-complete')"], opts);
        },
      }));
      const { runCargoTestBudgeted } = await import(${JSON.stringify(libraryPath)});
      await runCargoTestBudgeted([], process.cwd());
    `;
    const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_COVERAGE: "1", SEMIO_TEST_BUDGET_MS: "200" }, timeout: 5000, reject: false });
    expect(result.code, result.stderr).toBe(0);
    expect(result.stdout).toBe("coverage-built\nassertions-complete\ncoverage-reported");
  });
}

test("process budgets enforce an explicitly selected build deadline", async () => {
  const code = `const { runCmd, buildBudgetMs } = await import(${JSON.stringify(libraryPath)}); runCmd(process.execPath, ["-e", "await Bun.sleep(2000)"], { budgetMs: buildBudgetMs() });`;
  const result = await execa(process.execPath, ["-e", code], { env: { ...cleanEnv(), SEMIO_BUILD_BUDGET_MS: "100" }, timeout: 5000, reject: false });
  expect(result.code).not.toBe(0);
  expect(result.stderr).toContain("exceeded 100ms");
});

test("process runner reaches workspace scripts before same-named bins", async () => {
  expect(workspaceScriptExists("nx")).toBe(true);
  expect(workspaceScriptExists("no-such-workspace-script")).toBe(false);
  const code = `const { runCmd } = await import(${JSON.stringify(libraryPath)}); runCmd("bun", ["nx", "--version"]);`;
  const result = await execa(process.execPath, ["-e", code], { env: cleanEnv(), timeout: 120_000, reject: false });
  expect(result.code).toBe(0);
  expect(`${result.stdout}\n${result.stderr}`).toContain("bootstrap");
});

test("process budgets bind nested native workspace profiles to the repository config",async()=>{
  const corpus=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️process-budgets/native-profile.json",import.meta.url),"utf8"));
  const schema=JSON.parse(readFileSync(new URL("../../🧬️schema/⏱️process-budgets/native-profile.json",import.meta.url),"utf8"));
  expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
  for(const coverage of corpus.coverage){
    const code=`const{mock}=await import("bun:test");const preparePath=${JSON.stringify(fileURLToPath(new URL("../../🗂️workspaces/🦀️cargo/📜️script.ts",import.meta.url)))};const originalBunSpawnSync=Bun.spawnSync.bind(Bun);Bun.spawnSync=(args,options)=>args[1]===preparePath?{exitCode:0,stdout:new Uint8Array(),stderr:new Uint8Array()}:originalBunSpawnSync(args,options);const native=await import("node:child_process");const spawn=native.spawn,spawnSync=native.spawnSync;mock.module("node:child_process",()=>({...native,spawnSync:(cmd,args,opts)=>cmd==="cargo"?{status:0}:spawnSync(cmd,args,opts),spawn:(cmd,args,opts)=>{if(cmd!=="cargo")return spawn(cmd,args,opts);console.error(JSON.stringify(args));return spawn(process.execPath,["-e",args.includes("list")?"console.log('{}')":"console.log('native-complete')"],opts)}}));const{runCargoTestBudgeted,getWorkspaceRoot}=await import(${JSON.stringify(libraryPath)});console.error(JSON.stringify({root:getWorkspaceRoot()}));await runCargoTestBudgeted([${JSON.stringify(corpus.nativePackage.name)}],process.cwd());`;
    const result=await execa(process.execPath,["-e",code],{env:{...cleanEnv(),SEMIO_COVERAGE:coverage?"1":"0",SEMIO_TEST_LEVEL:corpus.profile},timeout:12000,reject:false});
    if(result.code!==0)throw Error(`Native profile route failed: ${result.stdout}\n${result.stderr}`);expect(result.code).toBe(0);
    const records=result.stderr.replace(/\u001b\[[0-9;]*m/g,"").split(/\r?\n/u).map(line=>line.trim()).filter(line=>line.startsWith('["')||line.startsWith('{"root":')).map(line=>JSON.parse(line));
    const root=records.find(row=>!Array.isArray(row)).root;
    const calls=records.filter(row=>Array.isArray(row)&&row.includes("nextest"));
    expect(calls.length).toBe(2);
    for(const call of calls){expect(call[call.indexOf("--profile")+1]).toBe(corpus.profile);expect(call).toContain("--manifest-path");expect(call[call.indexOf("--manifest-path")+1]).toBe(join(root,corpus.nativePackage.manifest));expect(call).toContain("--config-file");expect(call[call.indexOf("--config-file")+1]).toBe(join(root,corpus.nativePackage.workspace,corpus.configPath));if(call.includes("--binaries-metadata"))for(const option of["-p","--package","--workspace","--lib","--features"])expect(call).not.toContain(option);}
  }
},30000);

test("process native profiles belong to each physical Cargo workspace",async()=>{
  const corpus=JSON.parse(readFileSync(new URL("../../🧫️fixtures/⏱️process-budgets/native-profile.json",import.meta.url),"utf8"));
  const {getWorkspaceRoot}=await import(libraryPath);const root=getWorkspaceRoot();
  const toml=createRequire(import.meta.url)("toml") as {parse:(source:string)=>any};
  const{discoverCargoWorkspaces,cargoWorkspaceMembers}=await import("../../🗂️workspaces/🦀️cargo/🟦️.ts");
  const scopes=discoverCargoWorkspaces(root);
  for(const row of corpus.workspaces){const source=readFileSync(join(root,row.directory,corpus.configPath),"utf8");const config=Bun.TOML.parse(source) as {profile:Record<string,{overrides?:readonly {filter:string}[];"slow-timeout":{period:string;"terminate-after":number}}>};expect(config).toEqual(toml.parse(source));expect(config.profile.quick.overrides?.length??0).toBe(row.overrideCount);for(const[level,period]of Object.entries(corpus.periods)){if(typeof period!=="string")throw Error("Portable native profile period must be text");expect(config.profile[level]["slow-timeout"]).toEqual({period,"terminate-after":1});}const members=cargoWorkspaceMembers(root,scopes.find(scope=>scope.directory===row.directory)!);for(const entry of config.profile.quick.overrides??[]){const name=/^package\(([a-z0-9-]+)\)$/.exec(entry.filter)?.[1];expect(name).toBeDefined();expect(members.some(member=>member.name===name)).toBe(true);}}
},30000);
