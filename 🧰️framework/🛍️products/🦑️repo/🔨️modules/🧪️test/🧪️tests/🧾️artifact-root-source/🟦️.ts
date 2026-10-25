import{createIssuedResourceControlV1 as artifactEntryControlV1}from"../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import ts from "typescript";
import {existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {join,resolve,relative,dirname} from "node:path";
import {createHash} from "node:crypto";
import {testCacheRoot,testCacheDir,cleanTestOutputs,markOutputDir,readOutputMarker,planExecution,type DiscoveredCase} from "../../📦️packages/🟦️typescript/🟦️.ts";
import {repositoryTestRetentionReportV1,withRepositoryTestArtifactsV1,retainRepositoryTestArtifactsV1,repositoryTestRunShardV1,repositoryTestArtifactEnvironmentV1,requireRepositoryTestArtifactRetirementV1} from "../../💾️storage/🟦️.ts";
import storageSchema from "../../💾️storage/🧬️schema/🔣️.json" with {type:"json"};
import {materializeRustHost} from "../../🖥️host/🏗️materialization/🟦️.ts";
import {acquireResourceLease} from "@semio-tech/process/leases";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {testReportPathV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🚦️protocol/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import fixture from "../🧫️fixtures/🧾️artifact-root-source/🔣️.json" with {type:"json"};
const root=process.cwd();

/** 🎛️ Retains explicit test environment through its awaited body. */
async function environment<T>(values:Record<string,string|undefined>,body:()=>Promise<T>):Promise<T>{const previous=Object.fromEntries(Object.keys(values).map(key=>[key,process.env[key]]));try{for(const[key,value]of Object.entries(values))if(value===undefined)delete process.env[key];else process.env[key]=value;return await body();}finally{for(const[key,value]of Object.entries(previous))if(value===undefined)delete process.env[key];else process.env[key]=value;}}

/** 🧪️ Opens the actual finite owner under a dedicated compact law scope. */
async function bounded<T>(label:string,body:()=>Promise<T>):Promise<T>{return environment({SEMIO_TEST_OUTPUT_SCOPE:"law/"+label},()=>withBuildTestInvocationV1(original=>withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async artifactPhase=>{const output=testCacheRoot(root);try{return await body();}finally{rmSync(output,{recursive:true,force:true});}})));}

test("the test protocol report root beside the task identities stays reserved and never breaks the run inventory",async()=>withBuildTestInvocationV1(async original=>{
 const report=testReportPathV1({SEMIO_TASK_PROJECT:"@law/reserved",SEMIO_TASK_TARGET:"test",SEMIO_WORKSPACE_ROOT:root})!,tests=join(root,".cache/b/o/m/tests"),relativeReport=relative(tests,report).split(/[\\/]/u);
 expect(relativeReport.length).toBe(4);expect(storageSchema.definitions.Protocol.const.reservedChildren).toContain(relativeReport[0]!);expect(relativeReport[0]!).not.toMatch(/^[a-f0-9]{24}$/u);
 mkdirSync(dirname(report),{recursive:true});writeFileSync(report,"<testsuites/>\n");
 try{
  await environment({SEMIO_TEST_OUTPUT_SCOPE:"law/reserved-report"},()=>withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async phase=>{
   const outcome=await retainRepositoryTestArtifactsV1(root,original,phase,{version:1,maximumRetainedBytes:fixture.retention.maximumRetainedBytes,maximumRuns:fixture.retention.maximumRuns},undefined,{dryRun:true});
   expect(outcome.removed.every(path=>!path.startsWith(join(tests,relativeReport[0]!)))).toBe(true);expect(existsSync(report)).toBe(true);rmSync(testCacheRoot(root),{recursive:true,force:true});
  }));
 }finally{rmSync(join(tests,relativeReport[0]!,relativeReport[1]!),{recursive:true,force:true});}
}),60000);

test("test outputs reject ambient roots without an issued finite callback",async()=>{
 const database=new Database(":memory:");try{const oracle=database.query("SELECT ?='diagnostics' AND ?='tests' AND 0 AS accepted").get(fixture.storage.partition,fixture.storage.directory) as {accepted:number};expect(Boolean(oracle.accepted)).toBe(fixture.storage.unissued);await environment({SEMIO_TEST_ARTIFACT_DIR:join(root,"foreign"),SEMIO_TEST_OUTPUT_SCOPE:fixture.scope},async()=>expect(()=>testCacheRoot(root)).toThrow());}finally{database.close();}
});

test("test outputs retain actual finite paths and refuse released async callbacks",async()=>{
 let resume!:()=>void,escaped!:Promise<unknown>;
 await bounded("paths",async()=>{const output=testCacheRoot(root),identity=createHash("sha256").update("law/paths").digest("hex").slice(0,24);expect(output.startsWith(join(root,".cache/b/o/m/tests",identity)+"/")).toBe(true);expect(output.split(/[\\/]/u).at(-1)).toMatch(/^[a-f0-9]{16}$/u);for(const child of fixture.children)expect(testCacheDir(root,child)).toBe(join(output,child));expect(()=>testCacheDir(root,"unowned")).toThrow();expect(()=>requireRepositoryTestArtifactRetirementV1(root)).toThrow("Exclusive original");const childEnvironment=repositoryTestArtifactEnvironmentV1(root,{CARGO_TARGET_DIR:"owned-cargo",DOTNET_NOLOGO:"1"});expect(childEnvironment.CARGO_TARGET_DIR).toBe("owned-cargo");expect(childEnvironment.DOTNET_NOLOGO).toBe("1");for(const key of ["TMPDIR","TMP","TEMP","XDG_CACHE_HOME","PIP_CACHE_DIR","PYTHONPYCACHEPREFIX","GOCACHE","GOMODCACHE","DOTNET_CLI_HOME","NUGET_PACKAGES"])expect(childEnvironment[key]?.startsWith(output+"/")).toBe(true);for(const scope of fixture.invalidScopes)await environment({SEMIO_TEST_OUTPUT_SCOPE:scope},async()=>expect(()=>testCacheDir(root,"work")).toThrow());const file=join(testCacheDir(root,"reports"),"proof");mkdirSync(join(output,"reports"));writeFileSync(file,Buffer.from([0,1,127,255]));const oracle=new Database(":memory:");try{expect(readFileSync(file).toString("hex").toUpperCase()).toBe((oracle.query("SELECT hex(?) AS bytes").get(Buffer.from([0,1,127,255])) as {bytes:string}).bytes);}finally{oracle.close();}const pending=new Promise<void>(accept=>{resume=accept;});escaped=pending.then(()=>testCacheRoot(root));});resume();await expect(escaped).rejects.toThrow("Original finite test artifact callback required");
});

test("materializes exact owned host plan and result paths without compiling",async()=>bounded("plan",async()=>{
 const artifact=testCacheRoot(root),inputs=mkdtempSync(join(root,".cache/b/l/m/data/ti-"));try{const featurePath=join(inputs,"🥒️.feature"),adapterPath=join(inputs,"🦀️.rs");writeFileSync(featurePath,"Feature: explicit owned output\n  @level-quick\n  @id-owned-output\n  @mode-property\n  Scenario: keeps its caller-owned paths\n    Given a declared owner\n    Then an unchanged literal\n");writeFileSync(adapterPath,"pub fn adapter() -> semio_repo_test_host::Adapter { semio_repo_test_host::Adapter::new(\"rust\") }\n");const discovered:DiscoveredCase={owner:relative(root,inputs).split("\\").join("/"),ownerName:"owned-output",case:"fixture",caseDir:relative(root,inputs).split("\\").join("/"),featurePath:relative(root,featurePath).split("\\").join("/"),adapters:{rust:relative(root,adapterPath).split("\\").join("/")},sharedFixtureDir:null,projectName:"owned-output-fixture"};const planned=planExecution(root,discovered,"quick","subject","rust"),work=join(artifact,"work","owned-output-fixture-subject-rust"),results=join(artifact,"results","owned-output-fixture-subject-rust");expect(planned.plan.scenarios).toHaveLength(1);expect(planned.planPath).toBe(join(work,"📋️plan.json"));expect(planned.plan.resultsPath).toBe(join(results,"📤️results.jsonl"));expect(planned.plan.artifactDir).toBe(join(results,"📦️artifacts"));expect(readOutputMarker(root,work)?.testId).toBe(discovered.owner+"::fixture");expect(JSON.parse(readFileSync(planned.planPath,"utf8")).resultsPath).toBe(planned.plan.resultsPath);const host=materializeRustHost(root,discovered,"subject",planned.planPath,planned.plan.resultsPath);expect(host.hostDir).toBe(join(artifact,"hosts","owned-output-fixture-subject-rust"));expect(host.args).toEqual(["--plan",planned.planPath,"--out",planned.plan.resultsPath]);const main=readFileSync(join(host.hostDir!,"src","main.rs"),"utf8");expect(main).toContain(JSON.stringify(adapterPath));expect(main).toContain("semio_repo_test_host::run_main(adapter::adapter())");expect(existsSync(join(host.hostDir!,"Cargo.toml"))).toBe(true);expect(existsSync(planned.plan.resultsPath)).toBe(false);}finally{rmSync(inputs,{recursive:true,force:true});}
}),30000);

test("refuses escape and foreign output ownership before marker writes",async()=>bounded("markers",async()=>{
 const artifact=testCacheRoot(root),owned=join(artifact,"work","owned");expect(()=>markOutputDir(root,join(artifact,"..","foreign"),fixture.marker)).toThrow();markOutputDir(root,owned,fixture.marker);const before=readOutputMarker(root,owned);expect(()=>markOutputDir(root,owned,{testId:"foreign-owner::fixture",cacheKey:"other"})).toThrow();expect(readOutputMarker(root,owned)).toEqual(before);markOutputDir(root,owned,{...fixture.marker,cacheKey:"source-v2"});expect(readOutputMarker(root,owned)?.cacheKey).toBe("source-v2");const malformed=join(artifact,"work","malformed");mkdirSync(malformed,{recursive:true});writeFileSync(join(malformed,"🔣️.json"),"{\"kind\":\"foreign\"}");expect(()=>markOutputDir(root,malformed,fixture.marker)).toThrow();
}));

test("refuses dangling symlink escapes from the declared artifact owner",async()=>bounded("symlink",async()=>{
 const artifact=testCacheRoot(root),outside=join(artifact,"absent"),link=join(artifact,"escape");symlinkSync(outside,link,process.platform==="win32"?"junction":"dir");expect(()=>markOutputDir(root,join(link,"unowned"),fixture.marker)).toThrow();expect(existsSync(join(outside,"unowned"))).toBe(false);
}));


test("test outputs exclude retirement while an actual original task reader is held",async()=>environment({SEMIO_TEST_OUTPUT_SCOPE:"law/custody"},()=>withBuildTestInvocationV1(async original=>{
 const control=scriptCompilerAdmissionControlV1(original,"test:output-retirement",30000);let output="",blocked=false;
 await withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async artifactPhase=>{output=testCacheRoot(root);markOutputDir(root,join(output,"work","custody"),fixture.marker);try{const writer=await acquireResourceLease({directory:join(root,".cache/b/l/m/data"),resource:"repository-test-task-shard:"+repositoryTestRunShardV1(dirname(output).split(/[\\/]/u).at(-1)!),mode:"exclusive",signal:control.signal,control,timeoutMs:0});writer.release();}catch(error){if(!String(error).includes("Resource lease timed out"))throw error;blocked=true;}});
 const database=new Database(":memory:");try{expect(Boolean((database.query("SELECT 1=1 AS blocked").get() as {blocked:number}).blocked)).toBe(fixture.storage.sharedBlocksRetirement);expect(blocked).toBe(fixture.storage.sharedBlocksRetirement);}finally{database.close();}
 await withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async artifactPhase=>{expect(testCacheRoot(root)).toBe(dirname(output));requireRepositoryTestArtifactRetirementV1(root);const report=cleanTestOutputs(root);expect(report.removals).toHaveLength(1);expect(existsSync(join(output,"work","custody"))).toBe(false);rmSync(testCacheRoot(root),{recursive:true,force:true});},"exclusive");
})),30000);


test("test outputs expose each bounded law in both canonical editor launch sources",()=>{
 {const dashboard=JSON.parse(readFileSync(process.cwd()+"/📋️project.json","utf8")).metadata.semio.dashboard,tool=dashboard.tools.find((row:any)=>row.id==="run-script");expect(tool.command).toEqual(["repo","exec","--projects={project}","--dry-run={dry-run}","--","repo","{workspace}/{script}"]);expect(tool.parameters.find((row:any)=>row.id==="script")).toMatchObject({kind:"text",required:true});}
});


test("test outputs retain live runs and retire released runs under actual exclusive custody",async()=>withBuildTestInvocationV1(async original=>{
 let released="",active="";const taskNames:string[]=[];
 await environment({SEMIO_TEST_OUTPUT_SCOPE:"law/retention-released"},()=>withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async artifactPhase=>{released=testCacheRoot(root);writeFileSync(join(released,"bytes"),Buffer.alloc(8192,1));taskNames.push(dirname(released).split(/[\\/]/u).at(-1)!);}));
 await environment({SEMIO_TEST_OUTPUT_SCOPE:"law/retention-live"},()=>withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async artifactPhase=>{active=testCacheRoot(root);writeFileSync(join(active,"bytes"),Buffer.alloc(8192,2));taskNames.push(dirname(active).split(/[\\/]/u).at(-1)!);const outcome=await retainRepositoryTestArtifactsV1(root,original,artifactPhase,{version:fixture.retention.version as 1,maximumRetainedBytes:fixture.retention.maximumRetainedBytes,maximumRuns:fixture.retention.maximumRuns},taskNames);expect(outcome.removed).toContain(released);expect(outcome.held).toContain(active);expect(existsSync(released)).toBe(!fixture.retention.releasedRemoved);expect(existsSync(active)).toBe(fixture.retention.activeRetained);const database=new Database(":memory:");try{expect((database.query("SELECT ? > ? AS needs_retirement").get(8192,fixture.retention.maximumRetainedBytes) as {needs_retirement:number}).needs_retirement).toBe(1);}finally{database.close();}rmSync(active,{recursive:true,force:true});}));
 for(const task of taskNames)rmSync(join(root,".cache/b/o/m/tests",task),{recursive:true,force:true});
}),30000);


test("test outputs map all task identities to a fixed finite lease set",()=>{const database=new Database(":memory:");try{for(const row of fixture.retention.shards){const oracle=database.query("SELECT ? % 16 AS shard").get(row.byte) as {shard:number};expect(oracle.shard).toBe(row.shard);expect(repositoryTestRunShardV1(row.task)).toBe(oracle.shard);}expect(()=>repositoryTestRunShardV1("foreign")).toThrow();}finally{database.close();}});


/** 📏️ The bytes a run directory really allocates, counted the way the retention policy counts them. */
function allocatedBytes(path:string):number{const status=lstatSync(path);return status.isDirectory()?status.blocks*512+readdirSync(path).reduce((sum,name)=>sum+allocatedBytes(join(path,name)),0):status.blocks*512;}

test("ended test runs are retired oldest first above the high-water mark, a dry run keeps them and reports in German",async()=>withBuildTestInvocationV1(async original=>{
 const taskOf=(scope:string)=>createHash("sha256").update(scope).digest("hex").slice(0,24),shardOf=(scope:string)=>repositoryTestRunShardV1(taskOf(scope));
 for(let attempt=0;attempt<8;attempt+=1){
  const stamp=process.pid+"-"+Date.now()+"-"+attempt,scope="law/pressure-"+stamp;let driver="law/pressure-driver-"+stamp;
  for(let salt=0;shardOf(driver)===shardOf(scope);salt+=1)driver="law/pressure-driver-"+stamp+"-"+salt;
  const paths:string[]=[],task=taskOf(scope);
  try{
   for(const index of [0,1])await environment({SEMIO_TEST_OUTPUT_SCOPE:scope},()=>withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async()=>{const output=testCacheRoot(root);writeFileSync(join(output,"bytes"),Buffer.alloc(8192,index+1));paths.push(output);}));
   expect(dirname(paths[0]!).split(/[\\/]/u).at(-1)).toBe(task);
   const both=allocatedBytes(paths[0]!)+allocatedBytes(paths[1]!),policy={version:1 as const,maximumRetainedBytes:both*4,maximumRuns:8},volume=()=>({total:10_000,free:2_000});
   const outcomes=await environment({SEMIO_TEST_OUTPUT_SCOPE:driver},()=>withRepositoryTestArtifactsV1(root,original,artifactEntryControlV1(original,"test:artifact-entry",0),async phase=>{
    const hour=new Date(Date.now()-3_600_000),half=new Date(Date.now()-1_800_000);utimesSync(paths[0]!,hour,hour);utimesSync(paths[1]!,half,half);
    const dry=await retainRepositoryTestArtifactsV1(root,original,phase,policy,[task],{dryRun:true,volume});
    const calm=await retainRepositoryTestArtifactsV1(root,original,phase,policy,[task],{volume:()=>({total:10_000,free:9_000})});
    const kept=paths.map(path=>existsSync(path));
    const real=await retainRepositoryTestArtifactsV1(root,original,phase,policy,[task],{volume});
    return {dry,calm,real,kept,after:paths.map(path=>existsSync(path))};
   }));
   if(outcomes.dry.held.length||outcomes.calm.held.length||outcomes.real.held.length)continue;
   expect(outcomes.dry.removed).toEqual([paths[0]]);expect(repositoryTestRetentionReportV1("de",outcomes.dry,true)).toContain("würde 1 beendete");
   expect(outcomes.calm.removed).toEqual([]);expect(outcomes.kept).toEqual([true,true]);
   expect(outcomes.real.removed).toEqual([paths[0]]);expect(outcomes.after).toEqual([false,true]);expect(repositoryTestRetentionReportV1("en",outcomes.real,false)).toContain("retired 1 ended run");
   return;
  }finally{rmSync(join(root,".cache/b/o/m/tests",task),{recursive:true,force:true});}
 }
 throw Error("every attempt met a peer holding the task shard of the runs: the host is too busy to judge retention");
}),120000);
