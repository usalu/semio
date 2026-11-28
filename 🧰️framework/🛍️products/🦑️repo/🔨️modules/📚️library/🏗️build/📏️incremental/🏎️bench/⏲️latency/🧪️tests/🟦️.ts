/** ⏲️ Contract: the budgets of the developer commands are a closed table that validates against its schema; a profile reduces to time per phase group and round trip counts exactly as an independent Python model says; judging applies the declared tolerance, breach ratio and speed factor; the runner measures every selected command on the fixed targets with the profile switched on and reports medians against the budget.
 * @see ../🧬️schema/🔣️.json */
import{expect,test}from"bun:test";
import Ajv from"ajv";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{createRepoProfileV1}from"../../../../⏱️profile/🟨️.mjs";
import{LATENCY_GROUPS_V1,LATENCY_POLICY_V1,latencyArgumentsV1,latencyExceededV1,latencyBudgetIssuesV1,latencyBudgetsV1,latencyGroupOfV1,latencyJudgeV1,latencyMedianV1,latencyObserveV1,latencyParseProfileV1,latencyRenderV1,latencyResultV1,latencySchemaV1,type LatencyCommandV1,type LatencyObservationV1,type LatencyProfileDocumentV1}from"../🟦️.ts";
import{latencyArgumentsV1 as parseArguments,latencyOptionNamesV1,runLatencyCommandV1}from"../⌨️command/🟦️.ts";
import{latencyCallV1,latencySelectV1,runLatencyV1,type LatencyPortsV1,type LatencyRunV1}from"../🏃️runner/🟦️.ts";
import corpus from"./🔣️.json";

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1();
const model=()=>{
 expect(python,"python3 is required for the independent model").toBeTruthy();
 return JSON.parse(Bun.spawnSync([python!,"-I",join(here,"🐍️.py"),join(here,"..","🧬️schema","🔣️.json"),join(here,"..","🎯️budgets","🔣️.json"),join(here,"🔣️.json")]).stdout.toString()) as {observe:LatencyObservationV1[];judge:{status:string;rows:unknown[]}[]};
};
const command=(id:string)=>latencyBudgetsV1.commands.find(row=>row.id===id)!;

test("the budget table validates against its schema and is closed; the five developer commands are budgeted",()=>{
 const ajv=new Ajv({strict:false}),validate=ajv.compile({$schema:latencySchemaV1.$schema,definitions:latencySchemaV1.definitions,$ref:"#/definitions/Budgets"});
 expect(validate(latencyBudgetsV1),JSON.stringify(validate.errors)).toBe(true);
 expect(latencyBudgetIssuesV1()).toEqual([]);
 expect(latencyBudgetsV1.commands.map(row=>row.id)).toEqual(["inputs","run","test-file","typecheck","fleet-status"]);
 for(const row of latencyBudgetsV1.commands){expect(Object.keys(row.phases).length,row.id).toBeGreaterThan(0);expect(Object.keys(row.trips).sort(),row.id).toEqual(["cacheLookup","graphService","spawn"]);expect(row.title.en.length>0&&row.title.de.length>0).toBe(true);}
 expect(latencyBudgetIssuesV1({...latencyBudgetsV1,commands:[{...command("inputs"),args:["inputs","{nowhere}"],phases:{graph:999999}},command("inputs")]}).length).toBe(3);
 expect(latencyArgumentsV1(command("test-file"))).toEqual(["test-file",latencyBudgetsV1.targets.file]);
 expect(latencyArgumentsV1(command("run"))).toEqual(["run",latencyBudgetsV1.targets.project,"--exclude-dependencies","--dry-run"]);
});

test("profiles reduce to groups and round trips exactly as the independent model says",()=>{
 const expected=model();
 corpus.observe.forEach((item,at)=>expect(latencyObserveV1(item.documents as LatencyProfileDocumentV1[],item.wall),item.name).toEqual(expected.observe[at]!));
 expect(latencyGroupOfV1("spawn.git")).toBe("children");expect(latencyGroupOfV1("rt.graph-service")).toBeNull();expect(latencyGroupOfV1("mystery")).toBe("other");
 expect(LATENCY_GROUPS_V1.map(row=>row.id)).toEqual([...latencySchemaV1.definitions.Group.enum] as never);
});

test("judging equals the independent model on every case, with tolerance, breach ratio and speed factor",()=>{
 const expected=model();
 corpus.judge.forEach((item,at)=>expect(latencyJudgeV1(item.observation as LatencyObservationV1,command(item.command),item.factor),item.name).toEqual(expected.judge[at]! as never));
 const edge=(wall:number)=>latencyJudgeV1({...(corpus.judge[0]!.observation as LatencyObservationV1),wallMs:wall},command("inputs"),1).rows[0]!.status;
 expect([edge(60250),edge(60251),edge(120250),edge(120251)]).toEqual(["ok","warn","warn","breach"]);
 expect(latencyMedianV1([3,null,1,2,10])).toBe(2.5);expect(latencyMedianV1([])).toBeNull();
});

test("a real profile carries the round trip markers and reads back to counts",()=>{
 const lines:string[]=[];let time=0;
 const profile=createRepoProfileV1({mode:"json",pid:7,now:()=>time,write:text=>{lines.push(text);}});
 profile.phase("door.script",()=>{time+=5;profile.mark("rt.graph-service");profile.event("spawn.git",3,"x");profile.mark("rt.graph-service");time+=10;});
 time+=20;profile.flush();
 const documents=latencyParseProfileV1("noise\n"+lines.join("")+"{broken\n");
 expect(documents.length).toBe(1);
 const seen=latencyObserveV1(documents,time);
 expect(seen.trips).toEqual({graphService:2,cacheLookup:null,spawn:1});
 expect(seen.phases.door).toBe(12);expect(seen.phases.children).toBe(3);expect(seen.phases.uncovered).toBe(20);
});

const ports=(runs:readonly LatencyRunV1[],factor=1):{ports:LatencyPortsV1;calls:string[][];stages:string[]}=>{
 const calls:string[][]=[],stages:string[]=[];let at=0;
 return {calls,stages,ports:{spawn:async call=>{calls.push([call.command,...call.args,call.env[LATENCY_POLICY_V1.profileVariable]??""]);return runs[Math.min(at++,runs.length-1)]!;},progress:stage=>{stages.push(stage);},check:()=>{},factor,clock:()=>"2026-11-27T12:00:00.000Z",host:()=>({cpus:8,memoryMb:16384,loadPerCore:0.5}),signal:new AbortController().signal}};
};
const profileText=(rows:{phase:string;self:number}[])=>JSON.stringify({pid:1,version:1,unit:"ms",total:1000,covered:0,uncovered:0,rows})+"\n";

test("the runner measures each selected command on the fixed targets with the profile on and judges the median",async()=>{
 const slow:LatencyRunV1={exit:0,wallMs:130000,stderr:profileText([{phase:"graph.load",self:100000},{phase:"rt.graph-service",self:0}])},fast:LatencyRunV1={exit:0,wallMs:20000,stderr:profileText([{phase:"door.script",self:6000},{phase:"graph.load",self:5000},{phase:"tool.nx",self:4000},{phase:"spawn.git",self:2000},{phase:"misc.x",self:2000}])};
 const run=ports([slow,fast,fast]),report=await runLatencyV1({root:"/repo",executable:"bun",script:"door.ts",commands:["inputs","typecheck"],runs:3,environment:{SEMIO_X:"1",PATH:"/bin"}},run.ports);
 expect(run.calls.length).toBe(6);
 expect(run.calls[0]).toEqual(["bun",join("/repo","door.ts"),"repo","inputs",latencyBudgetsV1.targets.project,"json"]);
 expect(run.stages).toEqual(["inputs:1/3","inputs:2/3","inputs:3/3","typecheck:1/3","typecheck:2/3","typecheck:3/3"]);
 expect(report.results.map(row=>row.command)).toEqual(["inputs","typecheck"]);
 const first=report.results[0]!;
 expect(first.status).toBe("ran");expect(first.verdict!.status).toBe("ok");
 expect(first.verdict!.rows.find(row=>row.metric==="wall")!.value).toBe(20000);
 expect(report.host.speedFactor).toBe(1);
 const slowReport=await runLatencyV1({root:"/repo",executable:"bun",script:"door.ts",commands:["inputs"],runs:1,environment:{}},ports([slow]).ports);
 expect(slowReport.results[0]!.verdict!.status).toBe("breach");
 expect(latencyExceededV1(slowReport,true)).toContain("inputs wall");expect(latencyExceededV1(slowReport).length).toBeGreaterThan(0);
 expect(latencyRenderV1(slowReport,"en")).toContain("BREACH");expect(latencyRenderV1(slowReport,"de")).toContain("VERLETZT");
 expect(latencyRenderV1(slowReport,"en")).toContain("unmeasured");
});

test("a failing run is recorded with its reason and an unknown command is refused in both languages",async()=>{
 const bad:LatencyRunV1={exit:2,wallMs:10,stderr:""},run=ports([bad]),report=await runLatencyV1({root:"/r",executable:"bun",script:"d.ts",commands:["fleet-status"],runs:3,environment:{}},run.ports);
 expect(run.calls.length).toBe(1);
 expect(report.results[0]).toMatchObject({command:"fleet-status",status:"failed",reason:"exit 2",verdict:null});
 expect(latencyRenderV1(report,"de")).toContain("fehlgeschlagen: exit 2");
 expect(()=>latencySelectV1(latencyBudgetsV1,["nothing"])).toThrow(/Unknown command: nothing[\s\S]*Unbekannter Befehl: nothing/u);
 expect(latencyResultV1(command("inputs"),[],1).status).toBe("failed");
 expect(latencyCallV1({root:"/r",executable:"bun",script:"d.ts",environment:{SEMIO_REPO_PLAN_REUSE:"1",SEMIO_LANGUAGE:"de"}},["x"]).env).toMatchObject({SEMIO_REPO_PROFILE:"json",SEMIO_LANGUAGE:"de"});
 expect(latencyCallV1({root:"/r",executable:"bun",script:"d.ts",environment:{SEMIO_REPO_PLAN_REUSE:"1"}},["x"]).env.SEMIO_REPO_PLAN_REUSE).toBeUndefined();
 expect(latencyCallV1({root:"/r",executable:"bun",script:"d.ts",environment:{SEMIO_FLEET_GROUP:"g1",SEMIO_BUILD_BUDGET_MS:"5",SEMIO_SCRIPT_PARENT:"x"}},["x"]).env).toMatchObject({SEMIO_FLEET_GROUP:"g1",SEMIO_BUILD_BUDGET_MS:"5"});
 expect(latencyCallV1({root:"/r",executable:"bun",script:"d.ts",environment:{SEMIO_SCRIPT_PARENT:"x"}},["x"]).env.SEMIO_SCRIPT_PARENT).toBeUndefined();
});

test("the command line reads closed options, prints the table or JSON, writes the file and only fails strict runs on a breach",async()=>{
 expect(parseArguments(["--commands=inputs,run","--runs=2","--json","--strict","--lang=de"])).toMatchObject({commands:["inputs","run"],runs:2,json:true,strict:true,language:"de"});
 expect(parseArguments([])).toMatchObject({commands:"all",runs:3,json:false,strict:false});
 for(const bad of [["--runs=0"],["--runs=99"],["--lang=fr"],["--nothing"],["--json","--json"],["--commands"]])expect(()=>parseArguments(bad),bad.join(" ")).toThrow(/\n/u);
 const written:string[]=[],files:Record<string,string>={},slow:LatencyRunV1={exit:0,wallMs:500000,stderr:""};
 const host={write:(line:string)=>{written.push(line);},writeText:async(path:string,text:string)=>{files[path]=text;},environment:{SEMIO_LANGUAGE:"en"},root:"/r",executable:"bun",runs:ports([slow]).ports};
 const report=await runLatencyCommandV1(["--commands=inputs","--runs=1","--file=/out.json"],host);
 expect(report.results[0]!.verdict!.status).toBe("breach");
 expect(JSON.parse(files["/out.json"]!).results[0].command).toBe("inputs");
 expect(written[0]).toContain("### repo inputs");
 expect(written.at(-1)).toContain("Over budget");
 await expect(runLatencyCommandV1(["--commands=inputs","--runs=1","--strict"],{...host,runs:ports([slow]).ports})).rejects.toThrow(/Latency budget exceeded: inputs wall[\s\S]*Latenzbudget überschritten: inputs wall/u);
 const warned:LatencyRunV1={exit:0,wallMs:61000,stderr:profileText([{phase:"door.script",self:6000},{phase:"graph.load",self:25000},{phase:"tool.nx",self:12000},{phase:"spawn.git",self:8000},{phase:"misc.x",self:3000},{phase:"source.capture",self:4000}])};
 const calm=await runLatencyCommandV1(["--commands=inputs","--runs=1"],{...host,runs:ports([warned]).ports});
 expect(calm.results[0]!.verdict!.status).toBe("warn");
 await expect(runLatencyCommandV1(["--commands=inputs","--runs=1","--strict"],{...host,runs:ports([warned]).ports})).rejects.toThrow(/exceeded/u);
 await runLatencyCommandV1(["--commands=inputs","--runs=1","--json","--strict"],{...host,runs:ports([{exit:0,wallMs:100,stderr:""}]).ports});
 await runLatencyCommandV1(["--commands=inputs","--runs=1","--json"],host);
 expect(JSON.parse(written.at(-1)!).version).toBe(1);
});


test("the operations catalogue declares the latency subcommand with exactly the options its parser accepts",async()=>{
 const catalog=(await import("../../../../⚙️operations/🔣️catalog.json")).default as any,entry=catalog.operations.find((row:any)=>row.name==="bench"),names=latencyOptionNamesV1();
 const sub=entry.subcommands.find((row:any)=>row.name==="latency");
 expect(sub).toBeDefined();expect(sub.synopsis.startsWith("bench latency")).toBe(true);expect(sub.summary.en).not.toBe(sub.summary.de);
 expect([...sub.flags].sort()).toEqual([...names.valued,...names.plain,...names.shared.filter(name=>name!=="--lang")].sort());
 for(const name of [...names.valued,...names.plain]){const flag=catalog.flags.find((row:any)=>row.name===name);expect(flag,name).toBeDefined();expect(flag.operations).toContain("bench");expect(flag.value).toBe(names.valued.includes(name)?"required":"none");expect(flag.description.en).not.toBe(flag.description.de);expect(entry.flags).toContain(name);}
 expect(entry.synopsis).toContain("latency");
 expect(entry.examples.some((row:any)=>row.command.startsWith("repo bench latency"))).toBe(true);
});
