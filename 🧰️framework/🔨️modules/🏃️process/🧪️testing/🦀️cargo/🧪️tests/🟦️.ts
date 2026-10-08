import { test, expect } from "bun:test";
import { readFileSync, readdirSync, mkdirSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { resolve, join } from "node:path";
import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { runInNewContext } from "node:vm";

test("Cargo retains failed build stdout without admitting invalid metadata", async () => {
  const owner = resolve(import.meta.dir, ".."), fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚨️failed-build/🔣️.json"), "utf8"));
  const api = await import("../🟦️.ts"), base = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8")).policies[0];
  const artifact = process.env.SEMIO_TEST_ARTIFACT_DIR!; expect(artifact).toBeTruthy(); mkdirSync(artifact, { recursive: true });
  const temporary = mkdtempSync(join(artifact, "cargo-failed-output-"));
  const program = "const fs=require('node:fs'),bytes=Buffer.from(process.env.CARGO_FAILED_STDOUT,'utf8'),split=Number(process.env.CARGO_FAILED_SPLIT);if(process.env.CARGO_FAILED_CALLS)fs.appendFileSync(process.env.CARGO_FAILED_CALLS,JSON.stringify(process.argv.slice(1))+'\\n');process.stdout.write(bytes.subarray(0,split));setImmediate(()=>{process.stdout.write(bytes.subarray(split));process.exitCode=Number(process.env.CARGO_FAILED_EXIT);});";
  try {
    for (const [index, row] of fixture.cases.entries()) {
      const directory = join(temporary, String(index)); mkdirSync(directory);
      const environment = { ...process.env, CARGO_FAILED_STDOUT: row.stdout, CARGO_FAILED_SPLIT: String(row.splitBytes), CARGO_FAILED_EXIT: String(row.exitCode) };
      const oracle = spawnSync("node", ["-e", program], { env: environment, encoding: "utf8" });
      expect(oracle.status).toBe(row.exitCode); expect(oracle.stdout).toBe(row.stdout);
      const calls = join(directory, "calls.jsonl"), policy = { ...base, artifactDirectory: directory, buildDirectory:join(directory,"build"),leaseDirectory:join(directory,"leases"),retainArtifacts: true, buildBudgetMs: 1_000, coverageEnabled: false, coveragePath: null };
      await expect(api.runCargoTestsV1({ manifestPath: base.manifestPath, packages: ["owner-package"], cwd: directory, environment: { ...environment, CARGO_FAILED_CALLS: calls } }, policy, { command: "node", args: ["-e", program, "--"] })).rejects.toThrow();
      const retained = join(directory, readdirSync(directory).find(name => name.startsWith("semio-nextest-"))!);
      expect(readFileSync(join(retained, "build-failure.stdout.txt"), "utf8")).toBe(oracle.stdout);
      expect(() => readFileSync(join(retained, "binaries-metadata.json"))).toThrow();
      expect(readFileSync(calls, "utf8").trim().split("\n")).toHaveLength(1);
      console.log(`[DEBUG] Cargo failed build ${row.name}: status=${row.exitCode} exact-retained-stdout=true assert-not-run=true`);
    }
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}, 10_000);

test("Kernel package executes neutral target selections through its exact bounded Cargo driver",async()=>{
 const repository=resolve(import.meta.dir,"../../../../../.."),kernel=join(repository,"🧰️framework/🛍️products/💻️os"),fixture=JSON.parse(readFileSync(join(kernel,"🧫️fixtures/🏃️kernel-test-runner/🔣️.json"),"utf8")),require=createRequire(import.meta.url),ts=require("typescript"),api=await import("../🟦️.ts"),levels=await import("../../🎚️budget/🟦️.ts");
 const source=readFileSync(join(kernel,"📦️packages/🦀️rust/📜️script.ts"),"utf8"),parsed=ts.createSourceFile("kernel-script.ts",source,ts.ScriptTarget.Latest,true),declaration=parsed.statements.find((row:any)=>ts.isClassDeclaration(row)&&row.name?.text==="TestScript");expect(declaration).toBeTruthy();
 const body=ts.transpileModule(declaration.getText(parsed),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText;
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(output).toBeTruthy();mkdirSync(output,{recursive:true});const temporary=mkdtempSync(join(output,"kernel-driver-selection-")),oldLevel=process.env.SEMIO_TEST_LEVEL,oldCoverage=process.env.SEMIO_COVERAGE;
 try{
  mkdirSync(join(temporary,"src"));mkdirSync(join(temporary,"tests"));const manifest=join(temporary,"Cargo.toml"),config=join(temporary,"nextest.toml"),target=join(temporary,"target");writeFileSync(manifest,fixture.manifest);writeFileSync(join(temporary,"src/lib.rs"),fixture.source);writeFileSync(join(temporary,"tests/selection.rs"),fixture.integration);writeFileSync(config,fixture.config);
  const base=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧫️fixtures/🔣️.json"),"utf8")).policies[0];
  for(const row of fixture.cases){
   process.env.SEMIO_TEST_LEVEL="fundamental";process.env.SEMIO_COVERAGE="0";
   const environment={...process.env,CARGO_TARGET_DIR:target,CARGO_BUILD_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:target,SEMIO_CARGO_TEST_POLICY:JSON.stringify({...base,manifestPath:manifest,targetDirectory:target,configPath:config,artifactDirectory:temporary,buildDirectory:join(temporary,"build"),leaseDirectory:join(temporary,"leases"),retainArtifacts:false,coverageEnabled:false})};let calls=0;
   const Runner=runInNewContext(body+";TestScript",{BundleScript:class{},resolve,process:{env:environment},resolveTestLevel:(args:string[])=>{const resolved=levels.resolveTestLevel(args);environment.SEMIO_TEST_LEVEL=resolved.level;return resolved;},readCargoTestPolicyV1:api.readCargoTestPolicyV1,runCargo:()=>{throw Error("Kernel runner bypasses exact bounded Cargo policy");},runCargoTestsV1:async(request:Parameters<typeof api.runCargoTestsV1>[0],policy:Parameters<typeof api.runCargoTestsV1>[1])=>{calls++;expect(request.manifestPath).toBe(manifest);expect(request.packages).toEqual([fixture.packageName]);expect([...request.extraArgs!]).toEqual(row.extraArgs);expect(policy.level).toBe(row.level);await api.runCargoTestsV1({...request,environment},policy);}});
   const runner=new Runner();runner.root=temporary;await runner.run(row.arguments);expect(calls).toBe(1);console.log(`[DEBUG] kernel package neutral selection=${row.id} policy=${row.level} actual-native=true`);
  }
 }finally{if(oldLevel===undefined)delete process.env.SEMIO_TEST_LEVEL;else process.env.SEMIO_TEST_LEVEL=oldLevel;if(oldCoverage===undefined)delete process.env.SEMIO_COVERAGE;else process.env.SEMIO_COVERAGE=oldCoverage;rmSync(temporary,{recursive:true,force:true});}
},30000);

test("Cargo keeps capture and output controls on Nextest execution with independent CLI authorities",async()=>{
  const owner=resolve(import.meta.dir,".."),fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🎬️execution/🔣️.json"),"utf8")),base=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8")).policies[0],api=await import("../🟦️.ts");
  const help=spawnSync("cargo",["nextest","run","--help"],{encoding:"utf8"});expect(help.status).toBe(0);for(const option of ["--no-capture","--success-output","--failure-output"])expect(help.stdout).toContain(option);
  const program="const{parseArgs}=require('node:util');const args=JSON.parse(process.argv[1]);const options={'no-capture':{type:'boolean'},nocapture:{type:'boolean'},'success-output':{type:'string'},'failure-output':{type:'string'}};process.stdout.write(JSON.stringify(parseArgs({args,options,strict:false,tokens:true}).tokens.filter(t=>t.kind==='option'&&t.name in options).map(t=>t.rawName+(t.value===undefined?'':'='+t.value))));";
  for(const item of fixture.cases){
    const split=api.partitionNextestExecutionFilters(item.arguments);expect(split).toEqual({buildArgs:item.buildArgs,executionArgs:item.executionArgs,libtestArgs:item.libtestArgs});
    const oracle=spawnSync("node",["-e",program,JSON.stringify(item.arguments.slice(0,item.arguments.includes("--")?item.arguments.indexOf("--"):item.arguments.length))],{encoding:"utf8"});expect(oracle.status).toBe(0);
    const projected=spawnSync("node",["-e",program,JSON.stringify(split.executionArgs)],{encoding:"utf8"});expect(projected.status).toBe(0);expect(JSON.parse(projected.stdout)).toEqual(JSON.parse(oracle.stdout));
    const plan=api.cargoTestPlanV1({manifestPath:base.manifestPath,packages:["owner-package"],cwd:"/owner",extraArgs:item.arguments},base,"/owner/results/binaries.json");for(const option of ["--no-capture","--nocapture","--success-output","--failure-output"])expect(plan[0]!.args.some(arg=>arg.split("=",1)[0]===option)).toBe(false);expect(plan[0]!.budgetMs).toBe(base.buildBudgetMs);expect(plan[1]!.budgetMs).toBe(base.assertionBudgets[base.level]);
  }
  for(const args of fixture.invalid)expect(()=>api.partitionNextestExecutionFilters(args)).toThrow();
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(output).toBeTruthy();mkdirSync(output,{recursive:true});const temporary=mkdtempSync(join(output,"cargo-execution-selection-"));
  try{
    mkdirSync(join(temporary,"src"));const manifest=join(temporary,"Cargo.toml"),config=join(temporary,"nextest.toml"),target=join(temporary,"target");writeFileSync(manifest,fixture.native.manifest);writeFileSync(join(temporary,"src/lib.rs"),fixture.native.source);writeFileSync(config,fixture.native.config);
    const environment={...process.env,CARGO_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:target},policy={...base,level:"quick",manifestPath:manifest,targetDirectory:target,configPath:config,artifactDirectory:temporary,buildDirectory:join(temporary,"build"),leaseDirectory:join(temporary,"leases"),retainArtifacts:false};
    for(const item of fixture.cases)await api.runCargoTestsV1({manifestPath:manifest,packages:[fixture.native.packageName],cwd:temporary,extraArgs:item.arguments,environment},policy);
  }finally{rmSync(temporary,{recursive:true,force:true});}
},15000);

test("Cargo refuses an empty selected snapshot suite with the independent Nextest status",async()=>{
  const owner=resolve(import.meta.dir,".."),fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🕳️selection/🔣️.json"),"utf8")),base=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8")).policies[0],api=await import("../🟦️.ts");
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(output).toBeTruthy();mkdirSync(output,{recursive:true});const temporary=mkdtempSync(join(output,"cargo-empty-selection-"));
  try{
    mkdirSync(join(temporary,"src"));const manifest=join(temporary,"Cargo.toml"),config=join(temporary,"nextest.toml"),target=join(temporary,"target");writeFileSync(manifest,fixture.manifest);writeFileSync(join(temporary,"src/lib.rs"),fixture.source);writeFileSync(config,fixture.config);
    const environment={...process.env,CARGO_TARGET_DIR:target,CARGO_BUILD_BUILD_DIR:target},policy={...base,level:"quick",manifestPath:manifest,targetDirectory:target,configPath:config,artifactDirectory:temporary,buildDirectory:join(temporary,"build"),leaseDirectory:join(temporary,"leases"),retainArtifacts:false},request={manifestPath:manifest,packages:[fixture.packageName],cwd:temporary,extraArgs:["--lib","-E",fixture.filter],environment};
    const oracle=spawnSync("cargo",["nextest","run","--config-file",config,"--profile","quick","--manifest-path",manifest,"--lib","--no-tests",fixture.action,"-E",fixture.filter],{cwd:temporary,env:environment,encoding:"utf8"});expect(oracle.status).toBe(fixture.expectedStatus);expect(oracle.stderr).toContain("no tests");
    await expect(api.runCargoTestsV1(request,policy)).rejects.toThrow();
  }finally{rmSync(temporary,{recursive:true,force:true});}
});

test("Cargo execution preserves exact owner policy and separates compilation from assertions", async () => {
  const owner=resolve(import.meta.dir,".."), require=createRequire(import.meta.url), fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8")), schema=JSON.parse(readFileSync(resolve(owner,"🧬️schema/🔣️.json"),"utf8"));
  const validate=new (require("ajv").default)({strict:true}).compile(schema), api=await import("../🟦️.ts");
  expect(api.cargoLintPlanV1({manifestPath:"/owner/Cargo.toml",packages:["owner-package"],cwd:"/owner",extraArgs:fixture.lint.arguments},fixture.policies[0]).args).toEqual(fixture.lint.command);
  expect(api.partitionNextestExecutionFilters(fixture.arguments)).toEqual(fixture.partition);
  for(const policy of fixture.policies){
    expect(validate(policy)).toBe(true); expect(api.readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify(policy)})).toEqual(policy);
    const plan=api.cargoTestPlanV1({manifestPath:"/owner/Cargo.toml",packages:["owner-package"],cwd:"/owner",extraArgs:fixture.arguments},policy,"/owner/results/binaries.json");
    expect(plan.every((row:{budgetMs:number})=>Number.isSafeInteger(row.budgetMs))).toBe(true);
    expect(plan.filter((row:{phase:string})=>row.phase==="assert").map((row:{budgetMs:number})=>row.budgetMs)).toEqual([policy.assertionBudgets[policy.level]]);
    expect(plan.filter((row:{phase:string})=>row.phase!=="assert").every((row:{budgetMs:number})=>row.budgetMs===policy.buildBudgetMs)).toBe(true);
    expect(plan[0].args).toContain("/owner/Cargo.toml");
    if(policy.nextest && !policy.coveragePath){expect(plan[0].args).not.toContain("-E");expect(plan[1].args).toContain("-E");expect(plan[1].args).not.toContain("--features");expect(plan[1].args).toContain("/owner/results/binaries.json");}
    expect(plan.at(-1)?.phase).toBe(policy.coveragePath?"report":"assert");
    expect(validate({...policy,assertionBudgets:{...policy.assertionBudgets,[policy.level]:-1}})).toBe(false);
    expect(()=>api.readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify({...policy,assertionBudgets:{...policy.assertionBudgets,[policy.level]:-1}})})).toThrow();
  }
  expect(api.readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify(fixture.policies[0]),SEMIO_TEST_LEVEL:"quick"}).assertionBudgets.quick).toBe(120000);
  const selected=api.readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify({...fixture.policies[0],coveragePath:"/owner/report.lcov"}),SEMIO_TEST_LEVEL:"exhaustive",SEMIO_COVERAGE:"1"});expect(selected.level).toBe("exhaustive");expect(api.cargoTestPlanV1({manifestPath:"/owner/Cargo.toml",packages:[],cwd:"/owner"},selected,"/owner/metadata.json").at(-1)?.phase).toBe("report");
  const output=process.env.SEMIO_TEST_ARTIFACT_DIR!;expect(output).toBeTruthy();mkdirSync(output,{recursive:true});const temporary=mkdtempSync(join(output,"cargo-port-"));
  try {
    for(const base of fixture.policies){
      const log=join(temporary,`${base.level}.jsonl`),policy={...base,manifestPath:join(temporary,"Cargo.toml"),configPath:join(temporary,"nextest.toml"),artifactDirectory:temporary,buildDirectory:join(temporary,"build"),leaseDirectory:join(temporary,"leases"),retainArtifacts:false,coveragePath:base.coveragePath?join(temporary,"coverage.lcov"):null};
      const request={manifestPath:policy.manifestPath,packages:["owner-package"],cwd:temporary,extraArgs:fixture.arguments,environment:{...process.env,CARGO_PORT_LOG:log}};
      const program="const fs=require('node:fs'),args=process.argv.slice(1);fs.appendFileSync(process.env.CARGO_PORT_LOG,JSON.stringify(args)+'\\n');if(args[0]==='nextest'&&args[1]==='list')process.stdout.write(JSON.stringify({rust_build_meta:{},rust_binaries:{}}));";
      await api.runCargoTestsV1(request,policy,{command:"node",args:["-e",program,"--"]});
      const calls=readFileSync(log,"utf8").trim().split("\n").map(line=>JSON.parse(line));
      expect(calls.length).toBe(base.coveragePath?3:2);
      const lintLog=join(temporary,`lint-${base.level}.jsonl`);await api.runCargoLintV1({...request,extraArgs:fixture.lint.arguments,environment:{...request.environment,CARGO_PORT_LOG:lintLog}},policy,{command:"node",args:["-e",program,"--"]});expect(JSON.parse(readFileSync(lintLog,"utf8").trim())).toEqual(["clippy","--manifest-path",policy.manifestPath,"-p","owner-package","--all-targets","--features","feature-a","--","-D","warnings"]);
      const oracle=spawnSync("node",["-e","process.stdout.write(JSON.stringify(JSON.parse(process.argv[1]).map(row=>row[0])))",JSON.stringify(calls)],{encoding:"utf8"});expect(oracle.status).toBe(0);expect(JSON.parse(oracle.stdout)).toEqual(calls.map(row=>row[0]));
      if(base.nextest&&!base.coveragePath){const metadata=calls[1][calls[1].indexOf("--binaries-metadata")+1];expect(()=>readFileSync(metadata)).toThrow();}
    }
    let capture="";const execution=await import("../../🎛️execution/🟦️.ts");await execution.runBudgetedTestCommand("node",["-e","process.stdout.write('é'.repeat(3))"],{cwd:temporary,budgetMs:1000,throwOnFailure:true,captureStdout:{limitBytes:6,onChunk:bytes=>{capture+=Buffer.from(bytes).toString('utf8');}}});expect(capture).toBe("ééé");
    await expect(execution.runBudgetedTestCommand("node",["-e","process.stdout.write('12345')"],{cwd:temporary,budgetMs:1000,throwOnFailure:true,captureStdout:{limitBytes:4,onChunk:()=>{}}})).rejects.toThrow("output-limit");
  }finally{rmSync(temporary,{recursive:true,force:true});}
  expect(()=>api.readCargoTestPolicyV1({})).toThrow("Explicit Cargo test policy required");
  expect(()=>api.readCargoTestPolicyV1({SEMIO_CARGO_TEST_POLICY:JSON.stringify({...fixture.policies[0],unknown:true})})).toThrow();
  expect(()=>api.partitionNextestExecutionFilters(["-E"])).toThrow();
  expect(()=>api.cargoTestPlanV1({manifestPath:"",packages:[],cwd:"/owner"},fixture.policies[0],"/owner/results/binaries.json")).toThrow();
},10000);

test("Cargo reporter options remain execution-owned with independent Node argument authority",async()=>{
 const owner=resolve(import.meta.dir,".."), fixture=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/📣️reporting/🔣️.json"),"utf8")), base=JSON.parse(readFileSync(resolve(owner,"🧫️fixtures/🔣️.json"),"utf8")).policies[0], require=createRequire(import.meta.url), api=await import("../🟦️.ts");
 const program="const{parseArgs}=require('node:util');const args=JSON.parse(process.argv[1]);const p=parseArgs({args,options:{'status-level':{type:'string',multiple:true},'final-status-level':{type:'string',multiple:true}},strict:false,tokens:true});process.stdout.write(JSON.stringify(p.tokens.filter(t=>t.kind==='option'&&(t.name==='status-level'||t.name==='final-status-level')).map(t=>[t.name,t.value])));";
 for(const item of fixture.cases){
  const oracle=spawnSync("node",["-e",program,JSON.stringify(item.arguments.slice(0,item.arguments.includes("--")?item.arguments.indexOf("--"):item.arguments.length))],{encoding:"utf8"});expect(oracle.status).toBe(0);const reported=JSON.parse(oracle.stdout)as[string,string][],split=api.partitionNextestExecutionFilters(item.arguments);for(const[name,value]of reported){expect(split.executionArgs.some(arg=>arg===("--"+name)||arg===("--"+name+"="+value))).toBe(true);expect(split.buildArgs).not.toContain("--"+name);expect(split.buildArgs).not.toContain("--"+name+"="+value);expect(split.buildArgs).not.toContain(value);}
  const plan=api.cargoTestPlanV1({manifestPath:base.manifestPath,packages:["owner-package"],cwd:"/owner",extraArgs:item.arguments},base,"/owner/results/binaries.json");expect(plan[0]!.args.some(arg=>arg.startsWith("--status-level")||arg.startsWith("--final-status-level"))).toBe(false);expect(plan[0]!.budgetMs).toBe(base.buildBudgetMs);expect(plan[1]!.budgetMs).toBe(base.assertionBudgets[base.level]);
  const actual=spawnSync("node",["-e",program,JSON.stringify(plan[1]!.args.slice(0,plan[1]!.args.indexOf("--")))],{encoding:"utf8"});expect(actual.status).toBe(0);const values=JSON.parse(actual.stdout)as[string,string][];expect(values.filter(([name])=>name==="status-level")).toEqual([["status-level",item.status]]);expect(values.filter(([name])=>name==="final-status-level")).toEqual([["final-status-level",item.finalStatus]]);
 }
 for(const args of fixture.invalid)expect(()=>api.partitionNextestExecutionFilters(args)).toThrow();
});
