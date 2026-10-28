import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {fleetDurationV1,fleetTextV1,formatFleetReportV1,summarizeFleetV1,validateFleetFactsV1,validateFleetReportV1,type FleetFactsV1,type FleetLanguageV1,type FleetReportV1 as FleetReport} from "../🟦️.ts";
import {collectFleetFactsV1,readFleetBazelV1,readFleetGenerationV1,readFleetSealedV1,type FleetReadersV1} from "../📥️readers/🟦️.ts";
import {fleetArgumentsV1,runFleetCliV1} from "../⌨️command/🟦️.ts";
import {requireTestToolV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";

const oracle=join(import.meta.dir,"🐍️.py"),ajv=new Ajv({strict:false,allowUnionTypes:true}),definitions=schema.definitions,messages=definitions.Messages.const as Record<string,Record<FleetLanguageV1,string>>;
const validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions,$ref:`#/definitions/${name}`});
type Case={id:string;language:FleetLanguageV1;facts:FleetFactsV1;report:any};
const cases=corpus.cases as unknown as Case[];
const python=(seed:number,count:number):Case[]=>{
 const result=spawnSync(requireTestToolV1("python"),[oracle,"generate",String(seed),String(count)],{encoding:"utf8",maxBuffer:1<<28});
 expect(result.status,result.stderr).toBe(0);
 return result.stdout.split("\n").filter(Boolean).map(line=>JSON.parse(line) as Case);
};
const report=(id:string)=>cases.find(row=>row.id===id)!.report;
const plain=<T>(value:T):T=>JSON.parse(JSON.stringify(value)) as T;

test("the policy, every fact and every report of the corpus satisfy the closed schema, checked by the in-repo validator and by Ajv alike",()=>{
 expect(validator("PolicySchema")(definitions.Policy.const)).toBe(true);
 const facts=validator("Facts"),reports=validator("Report"),corpusCheck=validator("Corpus");
 expect(corpusCheck(corpus)).toBe(true);
 for(const row of cases){
  expect(facts(row.facts),row.id+" facts "+JSON.stringify(facts.errors?.[0])).toBe(true);
  expect(validateFleetFactsV1(row.facts),row.id).toEqual([]);
  expect(reports(row.report),row.id+" report "+JSON.stringify(reports.errors?.[0])).toBe(true);
  expect(validateFleetReportV1(row.report),row.id).toEqual([]);
 }
});

test("both validators reject the same mutations of a valid report and a valid facts document",()=>{
 const base=report("join-grant-and-children"),facts=cases.find(row=>row.id==="join-grant-and-children")!.facts,reports=validator("Report"),factsCheck=validator("Facts");
 const reportMutations:Record<string,(value:any)=>void>={
  "unknown top-level property":value=>{value.extra=1;},
  "missing summary":value=>{delete value.summary;},
  "negative age":value=>{value.runs[0].ageSeconds=-1;},
  "unknown run state":value=>{value.runs[0].state="sleeping";},
  "string pid":value=>{value.runs[0].pid="1";},
  "unknown pressure":value=>{value.memory.pressure="loud";},
  "finding without text":value=>{value.findings=[{id:"x",severity:"warn",subject:"s",values:{}}];},
  "unknown severity":value=>{value.findings=[{id:"x",severity:"fatal",subject:"s",values:{},text:"t"}];},
  "wrong version":value=>{value.version=2;},
 };
 for(const [name,mutate] of Object.entries(reportMutations)){const copy=plain(base);mutate(copy);expect(reports(copy),name).toBe(false);expect(validateFleetReportV1(copy).length>0,name).toBe(true);}
 const factMutations:Record<string,(value:any)=>void>={
  "unknown property":value=>{value.extra=1;},
  "holder without mode":value=>{delete value.leases.holders[0].mode;},
  "negative megabytes":value=>{value.capacity.grants[0].reservedMegabytes=-1;},
  "unknown source id":value=>{value.sources[0].id="network";},
  "unknown pressure":value=>{value.host.pressure="loud";},
 };
 for(const [name,mutate] of Object.entries(factMutations)){const copy=plain(facts);mutate(copy);expect(factsCheck(copy),name).toBe(false);expect(validateFleetFactsV1(copy).length>0,name).toBe(true);}
});

test("the aggregation equals the golden corpus and the independent Python oracle on freshly seeded facts in both languages",()=>{
 for(const row of cases)expect(plain(summarizeFleetV1(row.facts,row.language)),row.id).toEqual(row.report);
 let compared=0;
 for(const seed of [101,102,103]){
  for(const row of python(seed,150)){expect(plain(summarizeFleetV1(row.facts,row.language)),row.id).toEqual(row.report);expect(validateFleetReportV1(row.report),row.id).toEqual([]);compared++;}
 }
 expect(compared).toBe(450);
});

test("every finding text of the schema is produced by some seeded case, and every text exists in both languages with the same placeholders",()=>{
 const produced=new Set<string>();
 for(const row of python(77,400))for(const finding of row.report.findings)produced.add(finding.id);
 const declared=Object.keys(messages).filter(key=>key.startsWith("finding.")).map(key=>key.slice(8));
 expect([...produced].sort()).toEqual(declared.sort());
 const holes=(text:string)=>[...text.matchAll(/\{([A-Za-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const [key,row] of Object.entries(messages)){expect(Object.keys(row).sort(),key).toEqual(["de","en"]);expect(holes(row.en),key).toEqual(holes(row.de));}
});

test("hand-checked numbers: runs join across the ledgers, waiting times add the holder's time left to the typical hold of everyone ahead, memory waiters are admitted as grants release",()=>{
 const joined=report("join-grant-and-children"),run=joined.runs[0];
 expect(joined.runs.length).toBe(1);
 expect([run.pid,run.pids,run.state,run.operation,run.phase,run.done,run.total,run.etaSeconds,run.priority,run.agent]).toEqual([100,[100,101,102,103],"running","run","inference",5,10,180,"agent","a1"]);
 expect(run.memory).toEqual({admittedMegabytes:3000,observedMegabytes:2000,deltaMegabytes:-1000,jobs:2});
 expect(run.implementation.freshness).toBe("current");
 expect(joined.memory.overReservedMegabytes).toBe(1000);
 const queue=report("lease-queue-estimate").queues.leases.find((row:any)=>row.resource==="R"),times=queue.waiters.map((row:any)=>row.estimatedWaitSeconds);
 expect(times).toEqual([600,720,null]);
 expect(report("lease-queue-estimate").queues.leases.find((row:any)=>row.resource==="S").waiters[0].estimatedWaitSeconds).toBe(0);
 expect(report("capacity-estimate").queues.capacity.waiters.map((row:any)=>row.estimatedWaitSeconds)).toEqual([50,500,null]);
 const graph=report("graph-custody").queues.graph;
 expect([graph.builder.pid,graph.builder.phase,graph.builder.done,graph.builder.total,graph.builder.etaSeconds,graph.computers,graph.slots,graph.waiting,graph.longestWaitSeconds,graph.estimatedWaitSeconds]).toEqual([21,"inference",55,117,720,2,1,2,420,720]);
 const bazel=report("bazel-slots").bazel;
 expect(bazel.servers.map((row:any)=>[row.slot,row.live,row.held,row.holderPid])).toEqual([["d0",false,false,null],["s1",true,false,null],["s2",true,true,40],["s10",false,false,null]]);
 expect([bazel.live,bazel.held,bazel.rssMegabytes]).toEqual([2,1,620]);
 const storage=report("storage-alerts");
 expect(storage.findings.map((row:any)=>row.id)).toEqual(["disk-low","disk-over-budget","image-full"]);
 expect(storage.storage.images.map((row:any)=>row.usedBasisPoints)).toEqual([null,9583,3750]);
 const sealed=report("sealed-blocked-and-drifted");
 expect([sealed.sealed.drifted,sealed.sealed.blocked,sealed.findings.map((row:any)=>row.id)]).toEqual([1,1,["sealed-blocked"]]);
 const generation=report("generation-coverage").generation;
 expect([generation.ageSeconds,generation.verified.basisPoints,generation.verified.ageSeconds]).toEqual([840,164,900]);
 const harsh=report("stuck-and-stale-and-pressure");
 expect(harsh.findings.map((row:any)=>row.id+":"+row.severity)).toEqual(["cycle:alert","holder-stuck:alert","memory-pressure:alert","capacity-wait:warn","holder-stale:warn","lease-wait:warn","under-reserved:warn"]);
 expect(harsh.findings.find((row:any)=>row.id==="holder-stuck").values.minutes).toBe(25);
 const edge=report("boundaries").findings.map((row:any)=>row.id+":"+row.subject);
 expect(edge).toEqual(["capacity-wait:pid 84","image-full:i1","lease-wait:pid 80","under-reserved:pid 83"]);
 expect(report("empty").runs).toEqual([]);expect(report("empty").findings.map((row:any)=>row.id)).toEqual(Array(6).fill("source-failed"));
});

test("an order-insensitive aggregation: shuffling every list of the facts leaves the report unchanged",()=>{
 for(const row of cases.slice(0,12)){
  const shuffled=plain(row.facts) as any,reverse=(list:unknown[]|undefined)=>list?.reverse();
  reverse(shuffled.leases?.holders);reverse(shuffled.leases?.waiters);reverse(shuffled.capacity?.grants);reverse(shuffled.capacity?.waiters);reverse(shuffled.bazel?.slots);reverse(shuffled.sealed?.entries);reverse(shuffled.storage?.images);
  expect(plain(summarizeFleetV1(shuffled,row.language)),row.id).toEqual(row.report);
 }
});

test("the text is compact, resolves every placeholder and has the same shape in English and German",()=>{
 for(const row of cases){
  const en=formatFleetReportV1("en",summarizeFleetV1(row.facts,"en")),de=formatFleetReportV1("de",summarizeFleetV1(row.facts,"de"));
  expect(en.length,row.id).toBe(de.length);
  expect(en.length,row.id).toBeLessThanOrEqual(120);
  for(const line of [...en,...de])expect(/\{[A-Za-z]+\}/u.test(line),row.id+" "+line).toBe(false);
 }
 const rich=summarizeFleetV1(cases.find(row=>row.id==="stuck-and-stale-and-pressure")!.facts,"en"),lines=formatFleetReportV1("en",rich);
 expect(lines[0]).toMatch(/^Fleet \d\d:\d\d:\d\d: 3 running, 0 blocked, 2 waiting; 3 alert\(s\), 4 warning\(s\)$/u);
 expect(lines).toContain("  [alert] Deadlock cycle among leases: 50/q → 60/q");
 expect(lines).toContain("  [warning] pid 70 uses 2500 MB but only 1000 MB are admitted");
 const german=formatFleetReportV1("de",summarizeFleetV1(cases.find(row=>row.id==="generation-coverage")!.facts,"de"));
 expect(german).toContain("Generation 566: delta gebaut vor 14 min (4 geänderte Eigentümer); geprüft 66 von 4019 Eigentümern (1 %) vor 15 min");
 expect(formatFleetReportV1("en",report("empty") as unknown as FleetReport,1).some(line=>line.startsWith("Runs: none"))).toBe(true);
 expect([fleetDurationV1("en",45),fleetDurationV1("en",600),fleetDurationV1("de",7200),fleetDurationV1("en",5400)]).toEqual(["45 s","10 min","2.0 h","90 min"]);
 expect(fleetTextV1("de","unit.min",{n:3})).toBe("3 min");
 expect(()=>fleetTextV1("en","nonsense")).toThrow();
});

test("a reader that fails or hangs leaves only its own section out; the rest of the report stays and the failure is a finding",async()=>{
 const good=cases.find(row=>row.id==="join-grant-and-children")!.facts;
 const readers:FleetReadersV1={
  leases:async()=>({leases:good.leases!}),
  capacity:async()=>{throw Error("capacity probe broke");},
  bazel:()=>new Promise(()=>{}),
  storage:async()=>({}),sealed:async()=>({}),generation:async()=>({}),
 };
 const facts=await collectFleetFactsV1({root:"/w",environment:{SEMIO_LANGUAGE:"en"},language:"en",now:()=>1_793_000_000_000},readers,50);
 expect(facts.sources.map(row=>[row.id,row.ok])).toEqual([["leases",true],["capacity",false],["bazel",false],["storage",true],["sealed",true],["generation",true]]);
 expect(facts.sources[1]!.error).toBe("capacity probe broke");
 expect(facts.sources[2]!.error).toContain("did not answer");
 const summary=summarizeFleetV1(facts,"en");
 expect(summary.runs.length).toBe(1);
 expect(summary.findings.map(row=>row.id)).toEqual(["source-failed","source-failed"]);
 expect(validateFleetReportV1(summary)).toEqual([]);
});

test("the command prints text or schema-valid JSON, refuses unknown options in both languages and exits 0",async()=>{
 const good=cases.find(row=>row.id==="graph-custody")!.facts;
 const readers:FleetReadersV1={leases:async()=>({leases:good.leases!}),capacity:async()=>({}),bazel:async()=>({}),storage:async()=>({}),sealed:async()=>({}),generation:async()=>({})};
 const run=async(args:string[],language="en")=>{const lines:string[]=[];const code=await runFleetCliV1("/w",args,{SEMIO_LANGUAGE:language},line=>lines.push(line),{readers,now:()=>1_793_000_000_000});return {code,lines};};
 const json=await run(["status","--json"]);
 expect(json.code).toBe(0);expect(json.lines.length).toBe(1);
 const parsed=JSON.parse(json.lines[0]!);expect(validateFleetReportV1(parsed)).toEqual([]);expect(parsed.queues.graph.builder.pid).toBe(21);
 const text=await run(["status","--limit=1"],"de");expect(text.lines[0]).toMatch(/^Flotte /u);expect(text.lines.some(line=>line.includes("Graph: Builder pid 21"))).toBe(true);
 for(const bad of [[],["status","--nonsense"],["status","--json","--json"],["status","--limit=0"],["status","--limit=x"],["status","--limit=999"],["other"]])await expect(run(bad as string[])).rejects.toThrow(/Usage|--limit|Verwendung/u);
 expect(fleetArgumentsV1(["status","--limit=5","--json"])).toEqual({json:true,limit:5});
 await expect(run(["status","--nonsense"])).rejects.toThrow("Verwendung: repo fleet status");
});

test("the plain-file readers parse the Bazel slot files, the generation pointer and manifest and the sealed store of a sandbox workspace",async()=>{
 const root=mkdtempSync(join(tmpdir(),"fleet-readers-")),context={root,environment:{SEMIO_LANGUAGE:"en"} as NodeJS.ProcessEnv,language:"en" as const,now:()=>1_793_000_000_000};
 try{
  const server=join(root,".cache","b","p","m","bazel","outputs","server");
  mkdirSync(join(server,"s0","server"),{recursive:true});mkdirSync(join(server,"s1"),{recursive:true});mkdirSync(join(server,"d0"),{recursive:true});mkdirSync(join(server,"junk"),{recursive:true});
  writeFileSync(join(server,"s0","server","server.pid.txt"),String(process.pid));
  writeFileSync(join(server,"s0.session.json"),JSON.stringify({version:1,slot:"s0",key:"0123456789abcdef",keys:["0123456789abcdef"],usedAt:1_792_999_000_000,session:"0123456789ab.d",agent:"codex"}));
  writeFileSync(join(server,"s1.session.json"),JSON.stringify({version:1,slot:"s0",key:"0123456789abcdef",keys:[],usedAt:1}));
  const bazel=(await readFleetBazelV1(context)).bazel!;
  expect(bazel.slotPrefix).toBe("bazel-slot-");
  expect(bazel.slots.map(row=>[row.slot,row.pid,row.key??null,row.agent??null])).toEqual([["d0",null,null,null],["s0",process.pid,"0123456789abcdef","codex"],["s1",null,null,null]]);
  expect(bazel.slots[1]!.treeMegabytes === null || bazel.slots[1]!.treeMegabytes! > 0).toBe(true);
  const directory=join(root,".cache","b","g","m","generations");
  expect(await readFleetGenerationV1(context)).toEqual({});
  mkdirSync(join(directory,"m"),{recursive:true});
  const hex=(character:string)=>character.repeat(64);
  writeFileSync(join(directory,"current.json"),JSON.stringify({version:1,generation:7,manifest:hex("a"),parent:null,verified:{atMs:1_792_999_100_000,count:3,owners:"Bw=="}}));
  writeFileSync(join(directory,"m",hex("a")+".json"),JSON.stringify({version:1,generation:7,parent:null,identity:hex("1"),implementation:hex("2"),env:hex("3"),membership:hex("4"),frontier:null,factIndex:hex("5"),global:hex("6"),owners:{x:{},y:{},z:{}},built:{kind:"delta",base:6,dirty:["x"]}}));
  const generation=(await readFleetGenerationV1(context)).generation!;
  expect([generation.number,generation.owners,generation.built,generation.implementation,generation.verified]).toEqual([7,3,{kind:"delta",dirty:1},hex("2"),{atMs:1_792_999_100_000,covered:3,count:3}]);
  writeFileSync(join(directory,"current.json"),JSON.stringify({version:1,generation:7,manifest:"nothex",parent:null}));
  await expect(readFleetGenerationV1(context)).rejects.toThrow(/fleet\.generation-pointer-invalid/u);
  const sealed=(await readFleetSealedV1(context)).sealed!;
  expect(sealed.entries).toEqual([]);expect(["sealed","live","promote"]).toContain(sealed.mode);
 }finally{rmSync(root,{recursive:true,force:true});}
});
