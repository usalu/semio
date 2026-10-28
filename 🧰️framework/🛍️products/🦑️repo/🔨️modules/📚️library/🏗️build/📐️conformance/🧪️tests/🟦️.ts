/** 📐️ Backend conformance: the scenario schema and data are valid and consistent, the reference function equals an independent Python implementation, the judge rejects every way a backend can break a scenario, the in-house engine satisfies all scenarios, and a real Nx and a real Bazel are judged by the same scenarios when their binaries are named (`SEMIO_TEST_NX_BIN`, `SEMIO_TEST_BAZEL_BINARY`). */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {testPythonV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import scenarios from "../📋️scenarios/🔣️.json" with {type:"json"};
import {buildBackendIdsV1,buildBackendRowV1,buildBackendDriverUrlV1} from "../../🔌️backends/🟦️.ts";
import catalog from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {conformanceArgumentsV1,conformanceOptionNamesV1,conformanceRequestV1,renderConformanceListV1} from "../⌨️command/🟦️.ts";
import {conformanceClosureV1,conformanceConcurrencyV1,conformanceEvaluateRunV1,conformanceIssuesV1,conformanceMarkersV1,conformanceReferenceOutputsV1,conformanceReportV1,conformanceSpecV1,conformanceTaskIdsV1,conformanceVerdictV1,renderConformanceReportV1,type ConformanceMarkerV1,type ConformanceSetV1} from "../🟦️.ts";
import {runConformanceV1,type ConformanceDriverV1,type ConformanceHostV1,type ConformanceSessionV1} from "../🏃️runner/🟦️.ts";
import {stubOutputsV1} from "../🎭️stub/🟦️.ts";
import {semioActionKeyV1,semioCanonicalV1,semioOutputsDigestV1} from "../../🔧️tools/🐙️semio/🟦️.ts";
import {driver as semioDriver} from "../../🔧️tools/🐙️semio/📐️conformance/🟦️.ts";
import {startRemoteCacheServerV1} from "../../🌐️remote-cache/🖥️server/🟦️.ts";

const set=scenarios as unknown as ConformanceSetV1,oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),stubFile=join(import.meta.dir,"../🎭️stub/🟦️.ts");
const ajv=new Ajv({strict:true,allowUnionTypes:true}),define=(name:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${name}`});
const python=(input:unknown):any=>{const run=spawnSync(testPythonV1()??"python3",["-I",oracle],{input:JSON.stringify(input),encoding:"utf8",maxBuffer:64*1024*1024});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
const spec=conformanceSpecV1(set.workspace,join(testScratchRootV1("backend-conformance-spec"),"markers.log")),sources=Object.keys(set.workspace.files),files=set.workspace.files as Record<string,string>;
const scratchRoot=testScratchRootV1("backend-conformance");
const marker=(kind:ConformanceMarkerV1["kind"],task:string,at:number):ConformanceMarkerV1=>({kind,task,at});

test("the schema is valid and validates the scenario set and every shape the way the repository validator does",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 expect(define("ScenarioSet")(scenarios)).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.ScenarioSet,scenarios,schema)).toEqual([]);
 for(const[name,value,ok]of [
  ["Run",{exitCode:0,milliseconds:3},true],["Run",{exitCode:"0",milliseconds:3},false],["Run",{exitCode:0,milliseconds:3,restored:["a:b"]},true],["Run",{exitCode:0,milliseconds:3,restored:["ab"]},false],
  ["Event",{kind:"start",task:"a:b"},true],["Event",{kind:"stop",task:"a:b"},false],["Marker",{kind:"end",task:"a:b",at:1},true],
  ["Verdict","regression",true],["Verdict","fine",false],["TaskId","core:build",true],["TaskId","Core:build",false],
  ["RunExpect",{exit:"zero",executed:["a:b"]},true],["RunExpect",{exit:"maybe"},false],["RunExpect",{surprise:1},false],
  ["OutputCheck",{task:"a:b",path:"a/out/x.txt",state:"expected"},true],["OutputCheck",{task:"a:b",path:"../x",state:"expected"},false],
  ["Report",{version:1,rows:[],passed:0,failed:0,unobserved:0},true],["Report",{version:1,rows:[],passed:0,failed:0},false]
 ] as const){
  expect(define(name)(value),name+JSON.stringify(value)).toBe(ok);
  expect(validateJsonSchemaSubset(schema.definitions[name as keyof typeof schema.definitions],value,schema).length===0,name+JSON.stringify(value)).toBe(ok);
 }
});

test("the scenarios are consistent, bilingual and cover every capability, and each way of breaking them is named",()=>{
 expect(conformanceIssuesV1(scenarios)).toEqual([]);
 expect(set.scenarios.length).toBeGreaterThanOrEqual(20);
 for(const scenario of set.scenarios){expect(scenario.title.de,scenario.id).not.toBe(scenario.title.en);expect(buildBackendRowV1("semio").capabilities[scenario.capability],scenario.id).toBeDefined();}
 const mutate=(change:(copy:any)=>void)=>{const copy=structuredClone(scenarios) as any;change(copy);return conformanceIssuesV1(copy).join("\n");};
 expect(mutate(copy=>{copy.scenarios.push({...copy.scenarios[0]});})).toContain("duplicate scenario");
 expect(mutate(copy=>{copy.scenarios[1].steps[0].targets=["nope:build"];})).toContain("unknown task nope:build");
 expect(mutate(copy=>{copy.scenarios[0].title.de=copy.scenarios[0].title.en;})).toContain("German title equals English");
 expect(mutate(copy=>{copy.scenarios=copy.scenarios.filter((row:any)=>row.capability!=="progress");})).toContain("capability progress has no scenario");
 expect(mutate(copy=>{copy.workspace.projects[0].targets.build.outputs=["elsewhere/out.txt"];})).toContain("leaves its project");
 expect(mutate(copy=>{copy.workspace.projects[1].targets.build.outputs=copy.workspace.projects[0].targets.build.outputs.map((p:string)=>p.replace("core","util"));})).not.toContain("declared by");
 expect(mutate(copy=>{copy.workspace.projects[0].targets.build.dependsOn=["ghost:build"];})).toContain("unknown task dependency");
 expect(mutate(copy=>{copy.scenarios[0].steps=[];})).toContain("schema");
 expect(set.scenarios.every(row=>row.evidence===undefined||row.evidence.every(item=>["incremental","parity","concurrency","bazel-contract"].includes(item.suite)))).toBe(true);
 expect(conformanceTaskIdsV1(set.workspace)).toContain("app:test");
 expect(conformanceClosureV1(set.workspace,["app:test"])).toEqual(["app:build","app:test","core:build","util:build"]);
});

test("the reference outputs equal the independent Python implementation, before and after edits",()=>{
 const edited={...files,"core/src/a.txt":files["core/src/a.txt"]+"more\n","edge/src/a.txt":files["edge/src/a.txt"]+"# comment\n"};
 const read=(table:Record<string,string>)=>(path:string)=>table[path]!;
 for(const table of [files,edited]){
  const typescript=conformanceReferenceOutputsV1(spec,sources,read(table)),other=python({reference:{spec,files:table}}).reference;
  expect(typescript).toEqual(other);
  expect(Object.keys(typescript).length).toBe(Object.values(spec.tasks).reduce((sum,task)=>sum+task.outputs.length,0));
 }
 const before=conformanceReferenceOutputsV1(spec,sources,read(files)),after=conformanceReferenceOutputsV1(spec,sources,read(edited));
 expect(after["core/out/core.txt"]).not.toBe(before["core/out/core.txt"]);
 expect(after["app/out/app.txt"]).not.toBe(before["app/out/app.txt"]);
 expect(after["util/out/util.txt"]).toBe(before["util/out/util.txt"]);
 expect(after["edge/out/edge.txt"]).toBe(before["edge/out/edge.txt"]);
 expect(after["user/out/user.txt"]).toBe(before["user/out/user.txt"]);
});

test("the stub process writes exactly the reference outputs and leaves ordered markers; failing and sleeping modes behave",()=>{
 const directory=join(scratchRoot,"stub"),markers=join(directory,"markers.log");
 mkdirSync(directory,{recursive:true});writeFileSync(markers,"");
 for(const[path,body]of Object.entries(files)){mkdirSync(dirname(join(directory,path)),{recursive:true});writeFileSync(join(directory,path),body);}
 const local=conformanceSpecV1(set.workspace,markers);writeFileSync(join(directory,"spec.json"),JSON.stringify(local));
 const run=(id:string)=>spawnSync(process.execPath,[stubFile,join(directory,"spec.json"),id],{cwd:directory,encoding:"utf8"});
 for(const id of ["core:build","util:build","app:build","app:test"])expect(run(id).status,id).toBe(0);
 const expected=conformanceReferenceOutputsV1(local,sources,path=>files[path]!);
 for(const task of ["core:build","util:build","app:build","app:test"])for(const path of local.tasks[task]!.outputs)expect(readFileSync(join(directory,path),"utf8"),path).toBe(expected[path]!);
 expect(run("broken:build").status).toBe(1);
 expect(run("nope:build").status).toBe(2);
 const marks=conformanceMarkersV1(readFileSync(markers,"utf8"));
 expect(marks.filter(item=>item.kind==="start").map(item=>item.task)).toEqual(["core:build","util:build","app:build","app:test","broken:build"]);
 expect(marks.some(item=>item.kind==="fail"&&item.task==="broken:build")).toBe(true);
 expect(conformanceMarkersV1("start\ta:b\t5\ngarbage\nend\ta:b\tnotanumber\nend\ta:b\t9\n").length).toBe(2);
 const timed=spawnSync(process.execPath,[stubFile,join(directory,"spec.json"),"p1:build"],{cwd:directory,encoding:"utf8"});expect(timed.status).toBe(0);
 const rooted=join(directory,"products");mkdirSync(rooted);
 expect(spawnSync(process.execPath,[stubFile,join(directory,"spec.json"),"app:lint",rooted,join(rooted,"stamp")],{cwd:directory,encoding:"utf8"}).status).toBe(0);
 expect(existsSync(join(rooted,"stamp"))).toBe(true);
 expect(stubOutputsV1(local.tasks["core:build"]!,["core/src/a.txt"],()=>"x")["core/out/core.txt"]).toMatch(/^[0-9a-f]{64}\n$/u);
});

test("canonical serialisation, output digests, action keys and marker concurrency agree with the oracle",()=>{
 const canonical=[{b:[1,"x",null,{z:true,a:false}],a:"é\u0001"},[],{},"s",7,{k:undefined,j:1}];
 const digests=[[{path:"a",kind:"file",mode:0o644,sha256:"0".repeat(64)},{path:"b",kind:"file",mode:0o755,sha256:"1".repeat(64)}],[{path:"d",kind:"directory",mode:0o700},{path:"l",kind:"symlink",mode:0o777,link:"d"}],[]];
 const keys=[{task:"a:b",execution:{kind:"command",program:"bun",args:["x"]},args:["--f"],outputs:["a/out"],files:[["b","2"],["a","1"]],environment:[["E","v"]],dependencies:[["c:d","3"],["a:b","4"]]},{task:"z:z",execution:{kind:"noop"},args:[],outputs:[],files:[],environment:[],dependencies:[]}];
 const markers=[[marker("start","a:b",1),marker("start","c:d",2),marker("end","a:b",3),marker("fail","c:d",4)],[marker("start","a:b",1),marker("end","a:b",2)],[]];
 const answer=python({canonical,digests,keys,concurrency:markers});
 expect(answer.canonical).toEqual(canonical.map(semioCanonicalV1));
 expect(answer.digests).toEqual(digests.map(row=>semioOutputsDigestV1(row as never)));
 expect(answer.keys).toEqual(keys.map(row=>semioActionKeyV1(row as never)));
 expect(answer.concurrency).toEqual(markers.map(conformanceConcurrencyV1));
 expect(answer.concurrency).toEqual([2,1,0]);
 expect(semioActionKeyV1({...keys[0]!,files:[["a","1"],["b","9"]]} as never)).not.toBe(answer.keys[0]);
 const mode=digests[0]!.map(row=>({...row,mode:row.path==="a"?0o600:row.mode}));
 expect(semioOutputsDigestV1(mode as never)).toBe(answer.digests[0]);
});

test("the evidence a scenario cites exists in the incremental, parity, concurrency and Bazel contract suites it reuses",()=>{
 const read=(path:string)=>JSON.parse(readFileSync(join(import.meta.dir,"../../"+path),"utf8"));
 const known:Record<string,Set<string>>={
  incremental:new Set(read("📏️incremental/📋️scenarios/🔣️.json").scenarios.map((row:any)=>row.id)),
  concurrency:new Set(read("📏️incremental/🤝️concurrent/🎬️workloads/🔣️.json").workloads.map((row:any)=>row.id)),
  parity:new Set(read("🪞️parity/🔣️samples.json").samples.map((row:any)=>row.id)),
  "bazel-contract":new Set(read("🔧️tools/🏗️bazel/🧪️tests/🧬️schema/📋️dispatch.json").definitions.Phase.enum)
 };
 let cited=0;
 for(const scenario of set.scenarios)for(const item of scenario.evidence??[]){cited++;expect(known[item.suite]?.has(item.id),`${scenario.id} cites ${item.suite}/${item.id}`).toBe(true);}
 expect(cited).toBeGreaterThanOrEqual(12);
 expect(new Set(set.scenarios.flatMap(row=>(row.evidence??[]).map(item=>item.suite)))).toEqual(new Set(["incremental","parity","concurrency","bazel-contract"]));
});

const edges={"app:build":["core:build","util:build"],"app:test":["app:build"]};
const observation=(overrides:Record<string,unknown>={})=>({run:{exitCode:0,milliseconds:1},markers:[marker("start","core:build",1),marker("end","core:build",2),marker("start","app:build",3),marker("end","app:build",4)],before:{"core/out/core.txt":"aa"},after:{"core/out/core.txt":"bb","app/out/app.txt":"cc"},expected:{"core/out/core.txt":"bb","app/out/app.txt":"cc"},...overrides});
test("the judge accepts what meets an expectation and names each way it is missed, identically in TypeScript and in the oracle",()=>{
 const cases:{name:string;expect:any;observation:any;failures:string[]}[]=[
  {name:"all met",expect:{exit:"zero",executed:["core:build","app:build"],ordered:true,outputs:[{task:"core:build",path:"core/out/core.txt",state:"changed"},{task:"app:build",path:"app/out/app.txt",state:"expected"}],maxConcurrent:1},observation:observation(),failures:[]},
  {name:"wrong exit",expect:{exit:"zero"},observation:observation({run:{exitCode:1,milliseconds:1}}),failures:["exit"]},
  {name:"expected failure missing",expect:{exit:"nonzero"},observation:observation(),failures:["exit"]},
  {name:"extra task",expect:{executed:["core:build"]},observation:observation(),failures:["executed"]},
  {name:"missing task",expect:{executed:["core:build","app:build","util:build"]},observation:observation(),failures:["executed"]},
  {name:"nothing executed expected",expect:{executed:[]},observation:observation(),failures:["executed"]},
  {name:"each",expect:{executedEach:["util:build"]},observation:observation(),failures:["executedEach:util:build"]},
  {name:"started once",expect:{startsOnce:["core:build"]},observation:observation({markers:[marker("start","core:build",1),marker("start","core:build",1),marker("end","core:build",2)]}),failures:["startsOnce:core:build"]},
  {name:"output absent but present",expect:{outputs:[{task:"core:build",path:"core/out/core.txt",state:"absent"}]},observation:observation(),failures:["absent:core/out/core.txt"]},
  {name:"output wrong content",expect:{outputs:[{task:"core:build",path:"core/out/core.txt",state:"expected"}]},observation:observation({after:{"core/out/core.txt":"zz"}}),failures:["expected:core/out/core.txt"]},
  {name:"output missing",expect:{outputs:[{task:"core:build",path:"core/out/core.txt",state:"expected"}]},observation:observation({after:{}}),failures:["expected:core/out/core.txt"]},
  {name:"unchanged violated",expect:{outputs:[{task:"core:build",path:"core/out/core.txt",state:"unchanged"}]},observation:observation(),failures:["unchanged:core/out/core.txt"]},
  {name:"changed violated",expect:{outputs:[{task:"core:build",path:"core/out/core.txt",state:"changed"}]},observation:observation({after:{"core/out/core.txt":"aa"}}),failures:["changed:core/out/core.txt"]},
  {name:"order violated",expect:{ordered:true},observation:observation({markers:[marker("start","core:build",1),marker("start","app:build",2),marker("end","core:build",3),marker("end","app:build",4)]}),failures:["ordered:app:build"]},
  {name:"too concurrent",expect:{maxConcurrent:1},observation:observation({markers:[marker("start","core:build",1),marker("start","app:build",2),marker("end","core:build",3),marker("end","app:build",4)]}),failures:["maxConcurrent"]},
  {name:"not concurrent enough",expect:{minConcurrent:2},observation:observation(),failures:["minConcurrent"]}
 ];
 const answers=python({evaluate:cases.map(row=>({expect:row.expect,observation:{exitCode:row.observation.run.exitCode,markers:row.observation.markers,before:row.observation.before,after:row.observation.after,expected:row.observation.expected},edges}))}).evaluate as string[][];
 for(const[index,row]of cases.entries()){
  const result=conformanceEvaluateRunV1(row.expect,row.observation,edges);
  const names=result.failures.map(text=>text);
  expect(names.length>0,row.name).toBe(row.failures.length>0);
  expect(answers[index]!.length>0,row.name).toBe(row.failures.length>0);
  expect(answers[index],row.name).toEqual([...row.failures].sort());
 }
 expect(conformanceEvaluateRunV1({restored:["core:build"]},observation(),edges)).toEqual({failures:[],unobserved:["restored"]});
 expect(conformanceEvaluateRunV1({restored:["core:build"]},observation({run:{exitCode:0,milliseconds:1,restored:["core:build"]}}),edges).failures).toEqual([]);
 expect(conformanceEvaluateRunV1({restored:["core:build"]},observation({run:{exitCode:0,milliseconds:1,restored:[]}}),edges).failures.length).toBe(1);
 expect(conformanceEvaluateRunV1({events:"covers-executed"},observation(),edges).unobserved).toEqual(["events"]);
 expect(conformanceEvaluateRunV1({events:"covers-executed"},observation({run:{exitCode:0,milliseconds:1,events:[{kind:"start",task:"core:build"}]}}),edges).failures.length).toBe(3);
});

test("verdicts and reports combine a backend's claim with what was measured",()=>{
 expect(conformanceVerdictV1("native","pass")).toBe("pass");
 expect(conformanceVerdictV1("adapter","fail")).toBe("regression");
 expect(conformanceVerdictV1("missing","fail")).toBe("gap");
 expect(conformanceVerdictV1("missing","pass")).toBe("stale-claim");
 expect(conformanceVerdictV1("native","unobserved")).toBe("unobserved");
 expect(conformanceVerdictV1("native","error")).toBe("error");
 const rows=(["pass","gap","regression","stale-claim","unobserved","error"] as const).map((verdict,at)=>({scenario:"s"+at,backend:"semio",capability:"run",claim:"native" as const,measured:"pass" as const,verdict,failures:verdict==="regression"?["x did not hold"]:[],milliseconds:1}));
 const report=conformanceReportV1(rows);
 expect(report).toMatchObject({passed:2,failed:3,unobserved:1});
 expect(define("Report")(report)).toBe(true);
 const english=renderConformanceReportV1(report,set,"en"),german=renderConformanceReportV1(report,set,"de");
 expect(english).toContain("REGRESSION");expect(german).toContain("RÜCKSCHRITT");expect(english).toContain("2 satisfied, 3 failed, 1 unobserved");expect(german).toContain("2 erfüllt, 3 fehlgeschlagen, 1 nicht beobachtet");
 expect(english).not.toBe(german);
 expect(renderConformanceListV1(set,"en")).toContain("plan-closure");expect(renderConformanceListV1(set,"de")).toContain(set.scenarios[0]!.title.de);
});

test("the command arguments are closed, bilingual and match the operations catalogue",()=>{
 expect(conformanceRequestV1(["conformance","run","--json"])).toEqual(conformanceArgumentsV1(["run","--json"]));
 expect(()=>conformanceRequestV1(["run","--json"])).toThrow("Erwartet");
 expect(conformanceArgumentsV1(["list"])).toMatchObject({operation:"list",backends:"all",scenarios:"all",json:false});
 expect(conformanceArgumentsV1(["run","--implementations=semio","--scenario=run-cold,cache-restore","--json","--keep","--lang=de"])).toMatchObject({operation:"run",backends:["semio"],scenarios:["run-cold","cache-restore"],json:true,keep:true,language:"de"});
 for(const bad of [[],["go"],["run","--nope"],["run","--json","--json"],["run","--implementations"],["run","--implementations=nobody"],["run","--scenario=nope"],["run","--lang=fr"],["run","--json=1"]])expect(()=>conformanceArgumentsV1(bad),JSON.stringify(bad)).toThrow();
 try{conformanceArgumentsV1(["run","--implementations=nobody"]);}catch(error){expect(String(error)).toContain("Unbekannt");}
 const operation=(catalog as any).operations.find((row:any)=>row.name==="conformance");
 if(operation){
  const names=conformanceOptionNamesV1(),declared=new Set(operation.flags as string[]);
  for(const flag of [...names.valued,...names.plain])if(flag!=="--lang")expect(declared.has(flag),flag).toBe(true);
  for(const flag of declared)expect([...names.valued,...names.plain],flag).toContain(flag);
 }
});

const host=(directory:string,extra:Partial<ConformanceHostV1>={}):ConformanceHostV1=>({scratch:directory,runtime:process.execPath,stubSource:readFileSync(stubFile,"utf8"),signal:new AbortController().signal,
 loadDriver:async id=>(await import(buildBackendDriverUrlV1(id).href)).driver as ConformanceDriverV1,
 startRemote:async()=>{const server=await startRemoteCacheServerV1({directory:join(directory,"remote-"+Math.random().toString(36).slice(2)),host:"127.0.0.1",port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:1,defaultInstance:"main",tokens:{}});return {url:server.url+"/main",stop:()=>server.stop()};},
 now:Date.now,progress:async()=>{},...extra});

test("the in-house engine satisfies every scenario and its report validates",async()=>{
 const report=await runConformanceV1(set,["semio"],"all",host(join(scratchRoot,"semio")));
 expect(report.rows.map(row=>[row.scenario,row.verdict,row.failures])).toEqual(set.scenarios.map(row=>[row.id,"pass",[]]));
 expect(report).toMatchObject({passed:set.scenarios.length,failed:0,unobserved:0});
 expect(define("Report")(report)).toBe(true);
 expect(validateJsonSchemaSubset(schema.definitions.Report,report,schema)).toEqual([]);
},300000);

const broken=(name:string,wrap:(session:ConformanceSessionV1,input:Parameters<ConformanceDriverV1["prepare"]>[0])=>ConformanceSessionV1):ConformanceDriverV1=>({id:"semio",async prepare(input){return wrap(await semioDriver.prepare(input),input);}});
const judged=async(name:string,driver:ConformanceDriverV1,only:string[])=>{
 const report=await runConformanceV1(set,["semio"],only,host(join(scratchRoot,"broken-"+name),{loadDriver:async()=>driver}));
 return Object.fromEntries(report.rows.map(row=>[row.scenario,row.measured]));
};
test("the judge fails a backend that forgets its cache, hides failures, ignores cancellation, serialises, or invents a record",async()=>{
 const control=["run-cold","plan-closure"];
 const amnesia=await judged("amnesia",broken("amnesia",(session)=>({...session,run:async(targets,options,signal)=>{await session.clearLocalCache();return session.run(targets,options,signal);}})),[...control,"cache-restore","invalidation","early-cutoff","test-results","restore-deleted-output"]);
 expect(amnesia).toMatchObject({"run-cold":"pass","plan-closure":"pass","cache-restore":"fail","invalidation":"fail","early-cutoff":"fail","test-results":"fail","restore-deleted-output":"fail"});
 const liar=await judged("liar",broken("liar",(session)=>({...session,run:async(targets,options,signal)=>({...(await session.run(targets,options,signal)),exitCode:0})})),[...control,"failure-isolation","failure-not-cached"]);
 expect(liar).toMatchObject({"run-cold":"pass","failure-isolation":"fail","failure-not-cached":"fail"});
 const deaf=await judged("deaf",broken("deaf",(session)=>({...session,run:(targets,options)=>session.run(targets,options,new AbortController().signal)})),[...control,"cancellation"]);
 expect(deaf).toMatchObject({"run-cold":"pass","cancellation":"fail"});
 const serial=await judged("serial",broken("serial",(session)=>({...session,run:(targets,options,signal)=>session.run(targets,{...options,parallel:1},signal)})),[...control,"parallelism-bounded","serial-bound"]);
 expect(serial).toMatchObject({"parallelism-bounded":"fail","serial-bound":"pass"});
 const inventor=await judged("inventor",broken("inventor",(session)=>({...session,run:async(targets,options,signal)=>({...(await session.run(targets,options,signal)),restored:["core:build"]})})),[...control,"run-record"]);
 expect(inventor).toMatchObject({"run-cold":"pass","run-record":"fail"});
 const blind=await judged("blind",broken("blind",(session)=>({...session,observes:new Set(),run:async(targets,options,signal)=>{const {events:_,...rest}=await session.run(targets,options,signal);return rest;}})),["progress"]);
 expect(blind).toMatchObject({"progress":"unobserved"});
 const thrower=await judged("thrower",{id:"semio",async prepare(){throw Error("cannot start");}},["run-cold"]);
 expect(thrower).toMatchObject({"run-cold":"error"});
},600000);

const gated=(name:string,variable:string,backend:string)=>test.skipIf(!process.env[variable])(`a real ${name} is judged by the same scenarios and every verdict agrees with the registry`,async()=>{
 const report=await runConformanceV1(set,[backend],"all",host(join(scratchRoot,backend)));
 console.log(renderConformanceReportV1(report,set,"en"));
 expect(report.rows.filter(row=>["regression","stale-claim","error"].includes(row.verdict)).map(row=>[row.scenario,row.verdict,row.failures])).toEqual([]);
},1800000);
gated("Nx","SEMIO_TEST_NX_BIN","nx");
gated("Bazel","SEMIO_TEST_BAZEL_BINARY","bazel");

test("every registered backend with a driver module is reachable through the registry",()=>{
 for(const id of buildBackendIdsV1()){
  const row=buildBackendRowV1(id);
  expect(existsSync(fileURLToPath(new URL(row.driver,new URL("../../🔌️backends/🟦️.ts",import.meta.url)))),id).toBe(true);
 }
 rmSync(scratchRoot,{recursive:true,force:true});
});
