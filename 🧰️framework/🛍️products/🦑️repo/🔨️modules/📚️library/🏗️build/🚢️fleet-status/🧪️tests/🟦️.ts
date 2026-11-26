import {expect,spyOn,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {fleetDurationV1,fleetTextV1,formatFleetReportV1,summarizeFleetV1,validateFleetFactsV1,validateFleetReportV1,type FleetFactsV1,type FleetLanguageV1,type FleetReportV1 as FleetReport} from "../🟦️.ts";
import {collectFleetFactsV1,readFleetBazelV1,readFleetHostNamesV1,readFleetGenerationV1,readFleetSealedV1,readWorkerMappingV1,readWorkerTreesV1,treeDifferenceV1,workerMemoryOfV1,type FleetReadersV1} from "../📥️readers/🟦️.ts";
import {fleetArgumentsV1,runFleetCliV1} from "../⌨️command/🟦️.ts";
import {requireHostWardenV1} from "../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🛡️warden/🟦️.ts";
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
  storage:async()=>({}),sealed:async()=>({}),generation:async()=>({}),scheduler:async()=>({}),
 };
 const facts=await collectFleetFactsV1({root:"/w",environment:{SEMIO_LANGUAGE:"en"},language:"en",now:()=>1_793_000_000_000},readers,50);
 expect(facts.sources.map(row=>[row.id,row.ok])).toEqual([["leases",true],["capacity",false],["bazel",false],["storage",true],["sealed",true],["generation",true],["scheduler",true]]);
 expect(facts.sources[1]!.error).toBe("capacity probe broke");
 expect(facts.sources[2]!.error).toContain("did not answer");
 const summary=summarizeFleetV1(facts,"en");
 expect(summary.runs.length).toBe(1);
 expect(summary.findings.map(row=>row.id)).toEqual(["source-failed","source-failed"]);
 expect(validateFleetReportV1(summary)).toEqual([]);
});

test("the command prints text or schema-valid JSON, refuses unknown options in both languages and exits 0",async()=>{
 const good=cases.find(row=>row.id==="graph-custody")!.facts;
 const readers:FleetReadersV1={leases:async()=>({leases:good.leases!}),capacity:async()=>({}),bazel:async()=>({}),storage:async()=>({}),sealed:async()=>({}),generation:async()=>({}),scheduler:async()=>({})};
 const run=async(args:string[],language="en")=>{const lines:string[]=[];const code=await runFleetCliV1("/w",args,{SEMIO_LANGUAGE:language},line=>lines.push(line),{readers,now:()=>1_793_000_000_000});return {code,lines};};
 const json=await run(["status","--json"]);
 expect(json.code).toBe(0);expect(json.lines.length).toBe(1);
 const parsed=JSON.parse(json.lines[0]!);expect(validateFleetReportV1(parsed)).toEqual([]);expect(parsed.queues.graph.builder.pid).toBe(21);
 const text=await run(["status","--limit=1"],"de");expect(text.lines[0]).toMatch(/^Flotte /u);expect(text.lines.some(line=>line.includes("Graph: Builder pid 21"))).toBe(true);
 for(const bad of [[],["status","--nonsense"],["status","--json","--json"],["status","--limit=0"],["status","--limit=x"],["status","--limit=999"],["other"]])await expect(run(bad as string[])).rejects.toThrow(/Usage|--limit|Verwendung/u);
 expect(fleetArgumentsV1(["status","--limit=5","--json"])).toEqual({json:true,limit:5});
 expect(fleetArgumentsV1(["status","--lang=de","--limit=5","--json"])).toEqual({json:true,limit:5});expect(fleetArgumentsV1(["status","--lang","en","--json"])).toEqual({json:true,limit:20});
 await expect(run(["status","--language","en"])).rejects.toThrow(/Usage|Verwendung/u);
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

test("the storage reader answers from the recorded usage of the cache partitions: after one measurement it never walks the machine cache again",async()=>{
 const {readFleetStorageV1}=await import("../📥️readers/🟦️.ts"),context={root:process.cwd(),environment:process.env,language:"en" as const,now:Date.now};
 await readFleetStorageV1(context);
 const fs=await import("node:fs"),walked:string[]=[],original=fs.readdirSync,spy=spyOn(fs,"readdirSync").mockImplementation(((path:unknown,...rest:unknown[])=>{walked.push(String(path));return (original as (...arguments_:unknown[])=>unknown)(path,...rest);}) as typeof fs.readdirSync);
 let facts:Awaited<ReturnType<typeof readFleetStorageV1>>;try{facts=await readFleetStorageV1(context);}finally{spy.mockRestore();}
 expect(facts.storage?.images.length).toBeGreaterThan(0);
 expect(walked.filter(path=>/[\\/]\.cache[\\/]b([\\/]|$)/u.test(path)),"directories of the machine cache walked by the second read").toEqual([]);
},120000);

test("the fleet section lists every machine with load, pressure, queue, worker, offload and unit cache counters and the totals of the scheduler, in both languages, and satisfies the closed schema under both validators",()=>{
 const base=cases.find(row=>row.id===cases[0]!.id)!,machine=(origin:string,over:Record<string,unknown>={})=>({origin,platform:"macos-arm64-0123456789abcdef",pressure:"normal",load:1.5,cores:8,running:2,queued:0,headroom:true,ageSeconds:3,worker:"busy",offloadedIn:4,offloadedOut:1,refused:0,cacheHit:30,cacheMiss:10,...over});
 const facts={...plain(base.facts),fleet:{scheduler:"http://cache.g1.semio.internal:8981",expireSeconds:15,machines:[machine("g1-m02",{pressure:"critical",headroom:false,worker:"refusing",refused:3,load:null}),machine("g1-m01")],queued:2,executing:2,offloadRejected:5,refused:3,retried:1,cache:{hits:100,misses:20,puts:30},claims:{active:2,waiting:1,oldestSeconds:75}}} as unknown as FleetFactsV1;
 expect(validator("Facts")(facts)).toBe(true);expect(validateFleetFactsV1(facts)).toEqual([]);
 const result=summarizeFleetV1(facts,"en");
 expect(validator("Report")(result)).toBe(true);expect(validateFleetReportV1(result)).toEqual([]);
 expect(result.fleet).toMatchObject({machines:[{origin:"g1-m01"},{origin:"g1-m02"}],queued:2,executing:2,offloadedIn:8,offloadedOut:2,refused:3,declined:5,retried:1,cacheHit:60,cacheMiss:20,cache:{hits:100,misses:20,puts:30},claims:{active:2,waiting:1,oldestSeconds:75}});
 const english=formatFleetReportV1("en",result,20).filter(line=>line.includes("g1-m0")||line.startsWith("Fleet of machines")||line.includes("claims")||line.includes("cache server")),german=formatFleetReportV1("de",summarizeFleetV1(facts,"de"),20).filter(line=>line.includes("g1-m0")||line.startsWith("Maschinenflotte")||line.includes("Beanspruchungen")||line.includes("Cache-Server"));
 expect(english).toHaveLength(5);expect(german).toHaveLength(5);
 expect(english[0]).toContain("2 reporting, 2 queued and 2 executing");expect(german[0]).toContain("2 melden sich, 2 warten und 2 laufen");
 expect(english.join("\n")).toContain("g1-m02 macos-arm64-0123456789abcdef: pressure critical, load ?/8 cores");expect(english.join("\n")).toContain("worker refusing (no headroom)");expect(german.join("\n")).toContain("Worker refusing (kein Spielraum)");
 expect(english.join("\n")).toContain("claims: 2 active, 1 waiting, oldest 75 s");expect(english.join("\n")).not.toMatch(/\{[a-zA-Z]+\}/u);expect(german.join("\n")).not.toMatch(/\{[a-zA-Z]+\}/u);
 const bare=summarizeFleetV1(plain(base.facts),"en");expect(bare.fleet).toBeUndefined();
 const bad=plain(facts) as unknown as {fleet:{machines:{worker:string}[]}};bad.fleet.machines[0]!.worker="sleeping";expect(validator("Facts")(bad)).toBe(false);expect(validateFleetFactsV1(bad as never).length).toBeGreaterThan(0);
});

test("the fleet section says why a machine shares nothing: no other machine has its platform key (another OS or another compiler build), or it reports no Rust compiler, in both languages",()=>{
 const base=cases.find(row=>row.id===cases[0]!.id)!,machine=(origin:string,platform:string)=>({origin,platform,pressure:"normal",load:1.5,cores:8,running:0,queued:0,headroom:true,ageSeconds:3,worker:"idle",offloadedIn:0,offloadedOut:0,refused:0,cacheHit:0,cacheMiss:0});
 const facts=(machines:unknown[])=>({...plain(base.facts),fleet:{scheduler:"http://cache.g1.semio.internal:8981",expireSeconds:15,machines,queued:0,executing:0,offloadRejected:0,refused:0,retried:0}}) as unknown as FleetFactsV1;
 const mixed=facts([machine("g1-m01","macos-arm64-aaaaaaaaaaaaaaaa"),machine("g1-m02","macos-arm64-aaaaaaaaaaaaaaaa"),machine("g1-m03","macos-arm64-bbbbbbbbbbbbbbbb"),machine("g1-m04","linux-arm64-aaaaaaaaaaaaaaaa"),machine("g1-m08","windows-amd64-none")]);
 expect(validateFleetFactsV1(mixed)).toEqual([]);
 const english=formatFleetReportV1("en",summarizeFleetV1(mixed,"en"),40).join("\n"),german=formatFleetReportV1("de",summarizeFleetV1(mixed,"de"),40).join("\n");
 expect(english).toContain("g1-m03 shares its platform key with no other machine (other keys: linux-arm64-aaaaaaaaaaaaaaaa, macos-arm64-aaaaaaaaaaaaaaaa, windows-amd64-none)");
 expect(english).toContain("g1-m04 shares its platform key with no other machine");expect(english).toContain("g1-m08 reports no Rust compiler (windows-amd64-none)");
 expect(english).not.toContain("g1-m01 shares");expect(english).not.toContain("g1-m02 shares");
 expect(german).toContain("g1-m03 teilt seinen Plattformschlüssel mit keiner anderen Maschine");expect(german).toContain("g1-m08 meldet keinen Rust-Compiler");
 expect(english).not.toMatch(/\{[a-zA-Z]+\}/u);expect(german).not.toMatch(/\{[a-zA-Z]+\}/u);
 expect(formatFleetReportV1("en",summarizeFleetV1(facts([machine("g1-m01","macos-arm64-aaaaaaaaaaaaaaaa")]),"en"),40).join("\n")).not.toContain("shares its platform key");
});

test("the fleet section says per machine whose worker is absent why it works not, in the chosen language, and a machine with a working worker shows no reason",()=>{
 const base=cases.find(row=>row.id===cases[0]!.id)!,machine=(origin:string,over:Record<string,unknown>={})=>({origin,platform:"macos-arm64-aaaaaaaaaaaaaaaa",pressure:"normal",load:1.5,cores:8,running:0,queued:0,headroom:true,ageSeconds:3,worker:"idle",offloadedIn:0,offloadedOut:0,refused:0,cacheHit:0,cacheMiss:0,...over});
 const why={code:"no-toolchain",en:"the pinned Rust toolchain is not installed and could not be provisioned (nightly-2026-07-20 missing)",de:"die festgeschriebene Rust-Toolchain ist nicht installiert und konnte nicht bereitgestellt werden (nightly-2026-07-20 fehlt)"};
 const facts={...plain(base.facts),fleet:{scheduler:"http://cache.g1.semio.internal:8981",expireSeconds:15,machines:[machine("g1-m02",{worker:"absent",why}),machine("g1-m01"),machine("g1-m03",{worker:"absent"}),machine("g1-m04",{worker:"refusing",why:{code:"no-headroom",en:"the machine has no headroom for one more action (memory)",de:"die Maschine hat keinen Spielraum für eine weitere Aktion (memory)"}})],queued:0,executing:0,offloadRejected:0,refused:0,retried:0}} as unknown as FleetFactsV1;
 expect(validateFleetFactsV1(facts)).toEqual([]);expect(validator("Facts")(facts)).toBe(true);
 const english=formatFleetReportV1("en",summarizeFleetV1(facts,"en"),40),german=formatFleetReportV1("de",summarizeFleetV1(facts,"de"),40);
 expect(english.filter(line=>line.includes(" worker absent: "))).toEqual(["    g1-m02 worker absent: "+why.en]);
 expect(english).toContain("    g1-m04 worker refusing: the machine has no headroom for one more action (memory)");
 expect(german.filter(line=>line.includes(" Worker absent: "))).toEqual(["    g1-m02 Worker absent: "+why.de]);
 const bad=plain(facts) as unknown as {fleet:{machines:{why?:unknown}[]}};bad.fleet.machines[0]!.why={code:1};
 expect(validator("Facts")(bad)).toBe(false);expect(validateFleetFactsV1(bad as never).length).toBeGreaterThan(0);
});

test("unit keys that differ across origins for the same source are a finding naming the component, in both languages, under the closed schema",()=>{
 const base=cases.find(row=>row.id===cases[0]!.id)!,machine=(origin:string)=>({origin,platform:"macos-arm64-0123456789abcdef",pressure:"normal",load:1,cores:8,running:0,queued:0,headroom:true,ageSeconds:1,worker:"idle",offloadedIn:0,offloadedOut:0,refused:0,cacheHit:1,cacheMiss:1});
 const keys={origins:3,units:2,diffs:[{crate:"alpha",slot:"a".repeat(64),origins:["g1-m01","g1-m02","g1-m03"],differs:["platform"],sameSource:true},{crate:"beta",slot:"b".repeat(64),origins:["g1-m01","g1-m02"],differs:["source"],sameSource:false}],byComponent:{platform:1,source:1}};
 const facts={...plain(base.facts),fleet:{scheduler:"http://cache.g1.semio.internal:8981",expireSeconds:15,machines:[machine("g1-m01"),machine("g1-m02"),machine("g1-m03")],queued:0,executing:0,offloadRejected:0,refused:0,retried:0,keys}} as unknown as FleetFactsV1;
 expect(validator("Facts")(facts)).toBe(true);expect(validateFleetFactsV1(facts)).toEqual([]);
 for(const language of["en","de"] as const){
  const report=summarizeFleetV1(facts,language),lines=formatFleetReportV1(language,report,20).join("\n");
  expect(validator("Report")(report)).toBe(true);expect(validateFleetReportV1(report)).toEqual([]);
  expect(report.findings.filter(row=>row.id==="unit-key-machine-dependent").map(row=>[row.severity,row.subject,row.values.components])).toEqual([["warn","alpha","platform"]]);
  expect(lines).toContain("alpha");expect(lines).toContain("beta");expect(lines).not.toMatch(/\{[a-zA-Z]+\}/u);
 }
 expect(summarizeFleetV1(facts,"en").fleet!.keys).toEqual(keys);
});

test("a machine the fleet cache refuses is an alert and every machine lists why its units were not shared, in both languages, under the closed schema",()=>{
 const base=cases.find(row=>row.id===cases[0]!.id)!,machine=(origin:string,over:Record<string,unknown>={})=>({origin,platform:"macos-arm64-0123456789abcdef",pressure:"normal",load:1.5,cores:8,running:2,queued:0,headroom:true,ageSeconds:3,worker:"busy",offloadedIn:4,offloadedOut:1,refused:0,cacheHit:30,cacheMiss:10,...over});
 const facts={...plain(base.facts),fleet:{scheduler:"http://cache.g1.semio.internal:8981",expireSeconds:15,machines:[machine("g1-m02",{cacheSkipped:16,cacheSkipReasons:{"upload-expired-denied":12,"read-only":3,"path-dependent":1}}),machine("g1-m01",{cacheSkipped:2,cacheSkipReasons:{"upload-expired":2}}),machine("g1-m03")],queued:0,executing:0,offloadRejected:0,refused:0,retried:0}} as unknown as FleetFactsV1;
 expect(validator("Facts")(facts)).toBe(true);expect(validateFleetFactsV1(facts)).toEqual([]);
 for(const language of["en","de"] as const){
  const report=summarizeFleetV1(facts,language),lines=formatFleetReportV1(language,report,20).join("\n");
  expect(validator("Report")(report)).toBe(true);expect(validateFleetReportV1(report)).toEqual([]);
  expect(report.findings.filter(row=>row.id==="unit-cache-denied").map(row=>[row.severity,row.subject,row.values.count])).toEqual([["alert","g1-m02",12]]);
  expect(report.findings.filter(row=>row.id==="unit-cache-read-only").map(row=>[row.severity,row.subject,row.values.count])).toEqual([["warn","g1-m02",3]]);
  expect(report.summary.alerts).toBeGreaterThanOrEqual(1);
  expect(lines).toContain("upload-expired-denied 12, read-only 3, path-dependent 1");expect(lines).toContain("upload-expired 2");
  expect(lines).not.toMatch(/\{[a-zA-Z]+\}/u);
 }
 const quiet=summarizeFleetV1({...facts,fleet:{...facts.fleet!,machines:[machine("g1-m01")] as never}},"en");
 expect(quiet.findings.some(row=>row.id.startsWith("unit-cache-"))).toBe(false);expect(formatFleetReportV1("en",quiet,20).join("\n")).not.toContain("did not share");
});

test("every machine line names the code its agent runs and whether it is the code of the coordinator's tree, in both languages, under the closed schema",()=>{
 const base=cases.find(row=>row.id===cases[0]!.id)!,machine=(origin:string,over:Record<string,unknown>={})=>({origin,platform:"macos-arm64-0123456789abcdef",pressure:"normal",load:1.5,cores:8,running:2,queued:0,headroom:true,ageSeconds:3,worker:"busy",offloadedIn:4,offloadedOut:1,refused:0,cacheHit:30,cacheMiss:10,...over});
 const facts={...plain(base.facts),fleet:{scheduler:"http://cache.g1.semio.internal:8981",expireSeconds:15,treeCode:"aaaaaaaaaaaaaaaa",machines:[machine("g1-m01",{code:"aaaaaaaaaaaaaaaa"}),machine("g1-m02",{code:"bbbbbbbbbbbbbbbb"}),machine("g1-m03")],queued:0,executing:0,offloadRejected:0,refused:0,retried:0}} as unknown as FleetFactsV1;
 expect(validator("Facts")(facts)).toBe(true);expect(validateFleetFactsV1(facts)).toEqual([]);
 for(const [language,same,differs,unknown] of [["en","code aaaaaaaaaaaaaaaa (the tree's)","CODE bbbbbbbbbbbbbbbb differs from the tree's aaaaaaaaaaaaaaaa","code unknown"],["de","Code aaaaaaaaaaaaaaaa (der des Baums)","CODE bbbbbbbbbbbbbbbb weicht vom Baum aaaaaaaaaaaaaaaa ab","Code unbekannt"]] as const){
  const report=summarizeFleetV1(facts,language),lines=formatFleetReportV1(language,report,20);
  expect(validator("Report")(report)).toBe(true);expect(validateFleetReportV1(report)).toEqual([]);
  const row=(origin:string)=>lines.find(line=>line.includes(` ${origin} `))!;
  expect(row("g1-m01")).toContain(same);expect(row("g1-m02")).toContain(differs);expect(row("g1-m03")).toContain(unknown);
  expect(lines.join("\n")).not.toMatch(/\{[a-zA-Z]+\}/u);
 }
});

test("the one-machine visibility sections (host store, fleet cache retention, offload) carry the numbers of the facts, in both languages, under both validators, and an unreported retention is explicit",()=>{
 const visible=report("one-machine-visible-en"),german=cases.find(row=>row.id==="one-machine-visible-de")!;
 expect([visible.cas.usedBytes,visible.cas.usedBasisPoints,visible.cas.lastCollection.ageSeconds,visible.cas.leases.leasedBytes]).toEqual([22*2**30,6666,600,1.5*2**30]);
 expect([visible.retention.mode,visible.retention.ageSeconds,visible.retention.judged,visible.retention.stale,visible.retention.unreached,visible.retention.undeclared,visible.retention.retired]).toEqual(["dry-run",300,700,150,50,300,0]);
 expect([visible.offload.local,visible.offload.fleet,visible.offload.builds.map((row:any)=>row.pid),visible.offload.deadline,visible.offload.lost,visible.offload.silent,visible.offload.fellBack]).toEqual([6,40,[4000,4100],4,2,1,9]);
 const english=formatFleetReportV1("en",summarizeFleetV1(cases.find(row=>row.id==="one-machine-visible-en")!.facts,"en"),20),deutsch=formatFleetReportV1("de",summarizeFleetV1(german.facts,"de"),20);
 expect(english).toContain("Host store: 22.0 of 33.0 GiB used (66 %), cap 50.0 GiB, high water 29.7 GiB, low water 24.8 GiB");
 expect(english).toContain("  input leases: 120 live (1.5 GiB), 30 expired (0.5 GiB); last collection 10 min ago, 22.0 GiB left");
 expect(english).toContain("Fleet cache retention dry-run: swept 5 min ago; 1000 entries, 700 judged, 150 stale, 50 unreachable, 300 undeclared, 0 retired");
 expect(english).toContain("  stale by: edited 120, extern 30");expect(english).toContain("  refused at publish: would-refuse-stale-on-arrival 3");
 expect(english).toContain("Offload: 6 running locally, 40 on the fleet; 500 offloaded, 7 refused, 9 fell back (4 deadline, 2 lost operation, 1 silent)");
 expect(english).toContain("  build 4100: 2 locally, 30 on the fleet");
 expect(deutsch).toContain("Host-Speicher: 22.0 von 33.0 GiB belegt (66 %), Obergrenze 50.0 GiB, Hochwasser 29.7 GiB, Niedrigwasser 24.8 GiB");
 expect(deutsch).toContain("Flottencache-Aufbewahrung dry-run: geprüft vor 5 min; 1000 Einträge, 700 beurteilt, 150 veraltet, 50 unerreichbar, 300 ohne Deklaration, 0 entfernt");
 expect(deutsch).toContain("Auslagerung: 6 laufen lokal, 40 auf der Flotte; 500 ausgelagert, 7 abgelehnt, 9 lokal nachgeholt (4 Frist, 2 verlorene Operation, 1 still)");
 for(const line of [...english,...deutsch])expect(/\{[A-Za-z]+\}/u.test(line),line).toBe(false);
 const bare=cases.find(row=>row.id==="one-machine-unreported")!,lines=formatFleetReportV1("en",summarizeFleetV1(bare.facts,"en"),20);
 expect(bare.report.retention).toBeNull();expect(lines).toContain("Fleet cache retention: no report from the cache host");
 expect(lines).toContain("Host store: n/a of 0.0 GiB used (? %), cap 50.0 GiB, high water 0.0 GiB, low water 0.0 GiB");expect(lines).toContain("  input leases: 0 live (0.0 GiB), 5000 expired (? GiB); no collection recorded");
 const reports=validator("Report"),facts=validator("Facts"),mutations:Record<string,(value:any)=>void>={"unknown retention mode":value=>{value.fleet.retention.mode="loud";},"negative lease bytes":value=>{value.cas.leases.leasedBytes=-1;},"unknown offload property":value=>{value.offload.extra=1;},"build without pid":value=>{delete value.offload.builds[0].pid;}};
 for(const [name,mutate] of Object.entries(mutations)){const copy=plain(cases.find(row=>row.id==="one-machine-visible-en")!.facts);mutate(copy);expect(facts(copy),name).toBe(false);expect(validateFleetFactsV1(copy).length>0,name).toBe(true);}
 const badReport=plain(visible);badReport.retention.mode="loud";expect(reports(badReport)).toBe(false);expect(validateFleetReportV1(badReport).length>0).toBe(true);
});

test("the offload reader counts compile slots as local and light slots as running on the fleet per build and reads the counters with the deadline and lost-operation reasons; the host store reader reads the hint and the census without walking the objects or the lease ledger",async()=>{
 const {readFleetCasV1,readFleetOffloadV1}=await import("../📥️readers/🟦️.ts"),{bumpOffloadCountersV1}=await import("../../🌐️remote-execution/🚀️offload/📊️counters/🟦️.ts"),{writeCasCensusV1}=await import("../../🗄️cas/🧹️collector/🟦️.ts"),{casObjectPathV1}=await import("../../🗄️cas/🗃️store/🟦️.ts");
 const root=mkdtempSync(join(tmpdir(),"fleet-one-machine-")),now=Date.now();
 try{
  const unit=join(root,"unit"),slots=join(unit,"s");mkdirSync(slots,{recursive:true});
  for(const name of ["0.lock","light-0.lock","light-1.lock"])writeFileSync(join(slots,name),JSON.stringify({pid:process.pid,token:"t",atMs:now}));
  bumpOffloadCountersV1(root,{out:2,fellBack:1},"deadline");bumpOffloadCountersV1(root,{fellBack:1},"operation-lost");
  const offload=(await readFleetOffloadV1({root,environment:{SEMIO_UNIT_CACHE_ROOT:unit},language:"en",now:()=>now})).offload!;
  expect([offload.local,offload.fleet,offload.builds,offload.out,offload.fellBack,offload.reasons.deadline,offload.reasons["operation-lost"]]).toEqual([1,2,[{pid:process.ppid,local:1,fleet:2}],2,2,1,1]);
  expect(offload.spool,"an empty spool is depth zero").toMatchObject({depth:0});expect(offload.spool!.bound).toBeGreaterThanOrEqual(4096);
  const up=join(unit,"fl","up");mkdirSync(up,{recursive:true});for(const name of ["a.json","b.json","offer-c.json"])writeFileSync(join(up,name),"{}");writeFileSync(join(up,"note.tmp"),"x");
  const queued=(await readFleetOffloadV1({root,environment:{SEMIO_UNIT_CACHE_ROOT:unit},language:"en",now:()=>now})).offload!;
  expect(queued.spool,"the depth of the upload spool of the unit cache root, in jobs").toMatchObject({depth:3});
  expect(validateFleetFactsV1({version:1,atMs:now,workspace:root,offload:queued,sources:[{id:"offload",ok:true,milliseconds:1}]})).toEqual([]);
  expect(fleetTextV1("en","offload.spool",{depth:3,bound:4096})).toContain("3 units");expect(fleetTextV1("de","offload.spool",{depth:3,bound:4096})).toContain("3 Einheiten");
  expect(validateFleetFactsV1({version:1,atMs:now,workspace:root,offload,sources:[{id:"offload",ok:true,milliseconds:1}]})).toEqual([]);
  const home=join(root,"home"),store=join(home,".cache","semio","cas"),digest="a".repeat(64);
  mkdirSync(join(store,"v1"),{recursive:true});mkdirSync(join(casObjectPathV1(store,digest),".."),{recursive:true});writeFileSync(casObjectPathV1(store,digest),"0123456789");
  writeFileSync(join(store,"v1","usage.json"),JSON.stringify({version:1,bytes:12345,atMs:now-1000}));
  await writeCasCensusV1(store,{version:1,atMs:now-500,objects:1,bytes:10,classes:{pinned:{objects:0,bytes:0},held:{objects:0,bytes:0},leased:{objects:1,bytes:10},linked:{objects:0,bytes:0},expired:{objects:0,bytes:0},young:{objects:0,bytes:0},unreferenced:{objects:0,bytes:0}},holders:[]});
  const cas=(await readFleetCasV1({root,environment:{HOME:home},language:"en",now:()=>now})).cas!;
  expect([cas.root,cas.lastCollection,cas.leases]).toEqual([store,{atMs:now-1000,bytes:12345},{leased:1,expired:0,leasedBytes:10,expiredBytes:0}]);
  expect([cas.highWatermarkBytes,cas.lowWatermarkBytes]).toEqual([Math.floor(cas.effectiveCapBytes*0.9),Math.floor(cas.effectiveCapBytes*0.75)]);
  expect(validateFleetFactsV1({version:1,atMs:now,workspace:root,cas,sources:[{id:"cas",ok:true,milliseconds:1}]})).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const NOW=1_793_000_000_000;
const alarmOf=(action:"suspend"|"cancel"|"report",text:{en:string;de:string})=>({version:1,atMs:NOW-4000,observed:false,level:"danger",action,reason:"danger",id:"p1",pid:4242,kind:"cargo",owner:"agent-7",megabytes:3100,host:{availableMegabytes:600,guardMegabytes:2048,dangerMegabytes:1024,swapOutMegabytesPerSecond:30,pressure:"critical",headroomMegabytes:null,headroomGuardMegabytes:4096,headroomDangerMegabytes:2048},text});
const stateOf=(over:Record<string,unknown>={},host:Record<string,unknown>={})=>({version:1,atMs:NOW-5000,pid:77,mode:"enforce",code:"c0de",host:{totalMegabytes:16384,availableMegabytes:7000,swapUsedMegabytes:100,swapOutMegabytesPerSecond:0,compressedMegabytes:900,pressure:"normal",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null,...host},level:"calm",hold:false,guardMegabytes:2048,dangerMegabytes:1024,resumeMegabytes:3072,swapRoomMegabytes:null,headroomMegabytes:null,headroomGuardMegabytes:4096,headroomDangerMegabytes:2048,subjects:{count:2,megabytes:2500,byKind:{cargo:{count:2,megabytes:2500}}},suspended:[],counters:{suspended:0,resumed:0,cancelled:0,ticks:50},lastAlarms:[],overhead:{tickMillisecondsP50:2,tickMillisecondsP95:4.5,cpuBasisPoints:60,residentMegabytes:41},...over});
const memoryOf=(origin:string,value:unknown,at=NOW)=>workerMemoryOfV1(origin,value as never,at);
const workerRow=(origin:string,liveness:"ready"|"working"|"offline"="ready")=>({origin,platform:"macos",liveness,recorded:"running",stage:"tools",stageIndex:1,stageCount:5,attempts:1,recordAgeSeconds:3,heartbeatAgeSeconds:2,refusal:null});
const memoryFacts=(origins:Record<string,unknown>,local:unknown=null):FleetFactsV1=>({version:1,atMs:NOW,workspace:"/w",sources:[],workers:{offlineSeconds:600,rows:Object.keys(origins).map(origin=>workerRow(origin))},memory:{local:local as never,origins:origins as never}});
const ids=(facts:FleetFactsV1,language:FleetLanguageV1="en")=>summarizeFleetV1(facts,language).findings.map(row=>`${row.id}:${row.severity}:${row.subject}`);

test("worker memory: a calm record is shown per worker and raises no finding, the absent record, a missing file and a stale record raise worker-warden-absent",()=>{
 const facts=memoryFacts({calm:memoryOf("calm",stateOf()),none:null,off:memoryOf("off",{version:1,atMs:NOW-1000,absent:true}),old:memoryOf("old",stateOf({atMs:NOW-61_000}))});
 expect(validateFleetFactsV1(facts)).toEqual([]);
 const report=summarizeFleetV1(facts,"en");
 expect(validateFleetReportV1(report)).toEqual([]);
 expect(ids(facts)).toEqual(["worker-warden-absent:warn:none","worker-warden-absent:warn:off","worker-warden-absent:warn:old"]);
 const rows=report.workers!.rows as any[],calm=rows.find(row=>row.origin==="calm").memory;
 expect([calm.absent,calm.ageSeconds,calm.mode,calm.level,calm.availableMegabytes,calm.totalMegabytes,calm.swapUsedMegabytes,calm.swapOutMegabytesPerSecond,calm.compressedMegabytes,calm.pressure,calm.subjects,calm.subjectMegabytes,calm.suspended,calm.counters,calm.lastAlarm,calm.overhead]).toEqual([false,5,"enforce","calm",7000,16384,100,0,900,"normal",2,2500,0,{suspended:0,resumed:0,cancelled:0},null,{tickMillisecondsP95:4.5,cpuBasisPoints:60,residentMegabytes:41}]);
 expect(rows.find(row=>row.origin==="none").memory).toBeNull();
 expect(report.findings.find(row=>row.subject==="none")!.text).toBe("worker none lives but its warden is not visible: no memory record");
 expect(report.findings.find(row=>row.subject==="off")!.text).toContain("its warden writes no state");
 expect(report.findings.find(row=>row.subject==="old")!.text).toContain("last record 61 s old");
 expect(ids({...facts,workers:{offlineSeconds:600,rows:[workerRow("none","offline")]}})).toEqual(["worker-offline:warn:none"]);
 expect(ids({...facts,memory:undefined})).toEqual([]);
 const lines=formatFleetReportV1("en",report);
 expect(lines).toContain("  calm macos: ready, stage tools (1/5), attempt 1, record 3 s, machine 2 s; memory free 7000 MB, swap 100 MB, compressed 900 MB, calm (enforce), interventions s/r/c 0/0/0");
 expect(lines).toContain("  none macos: ready, stage tools (1/5), attempt 1, record 3 s, machine 2 s; memory -");
 expect(lines.some(line=>line.includes("memory no warden record"))).toBe(true);
 expect(lines.some(line=>line.includes("memory last record 61 s old"))).toBe(true);
});

test("worker memory: guard warns, danger with a cancel alarm alerts and names the process, owner and action of the last alarm in the chosen language",()=>{
 const guard=stateOf({level:"guard",lastAlarms:[],counters:{suspended:0,resumed:0,cancelled:0,ticks:9}},{availableMegabytes:1500,pressure:"warn"});
 const danger=stateOf({level:"danger",counters:{suspended:2,resumed:1,cancelled:1,ticks:90},lastAlarms:[alarmOf("suspend",{en:"old en",de:"old de"}),alarmOf("cancel",{en:"cancelled cargo pid 4242 of agent-7 (3100 MB)",de:"cargo PID 4242 von agent-7 abgebrochen (3100 MB)"})]},{availableMegabytes:600,pressure:"critical",swapUsedMegabytes:null});
 const facts=memoryFacts({g:memoryOf("g",guard),d:memoryOf("d",danger)});
 expect(ids(facts)).toEqual(["worker-memory-tight:alert:d","worker-warden-intervened:alert:d","worker-memory-tight:warn:g"]);
 const en=summarizeFleetV1(facts,"en").findings,de=summarizeFleetV1(facts,"de").findings;
 expect(en.find(row=>row.id==="worker-memory-tight"&&row.subject==="g")!.text).toBe("worker g is at memory level guard: 1500 MB available (guard 2048 MB, danger 1024 MB), swap 100 MB, pressure warn");
 expect(en.find(row=>row.id==="worker-memory-tight"&&row.subject==="d")!.text).toBe("worker d is at memory level danger: 600 MB available (guard 2048 MB, danger 1024 MB), swap ? MB, pressure critical");
 expect(en.find(row=>row.id==="worker-warden-intervened")!.text).toBe("worker d: its warden intervened (2 suspended, 1 resumed, 1 cancelled); last: cancelled cargo pid 4242 of agent-7 (3100 MB)");
 expect(de.find(row=>row.id==="worker-warden-intervened")!.text).toBe("Worker d: sein Wächter hat eingegriffen (2 angehalten, 1 fortgesetzt, 1 abgebrochen); zuletzt: cargo PID 4242 von agent-7 abgebrochen (3100 MB)");
 expect(de.find(row=>row.id==="worker-memory-tight"&&row.subject==="d")!.text).toBe("Worker d ist auf Speicherstufe Gefahr: 600 MB verfügbar (Wache 2048 MB, Gefahr 1024 MB), Swap ? MB, Druck critical");
 const suspendedOnly=memoryFacts({s:memoryOf("s",stateOf({counters:{suspended:1,resumed:1,cancelled:0,ticks:3},lastAlarms:[alarmOf("suspend",{en:"suspended x",de:"x angehalten"})]}))});
 expect(ids(suspendedOnly)).toEqual(["worker-warden-intervened:warn:s"]);
 expect(formatFleetReportV1("de",summarizeFleetV1(suspendedOnly,"de")).some(line=>line.includes("Speicher frei 7000 MB, Swap 100 MB, komprimiert 900 MB, ruhig (eingreifend), Eingriffe a/f/b 1/1/0"))).toBe(true);
});

test("worker memory: observe mode, high swap and a heavy warden are findings of their own, and a local warden of an origin without a worker row is judged too",()=>{
 const observe=stateOf({mode:"observe"},{swapUsedMegabytes:4096}),heavy=stateOf({overhead:{tickMillisecondsP50:3,tickMillisecondsP95:12.4,cpuBasisPoints:501,residentMegabytes:60.6}},{swapUsedMegabytes:4097});
 const facts=memoryFacts({o:memoryOf("o",observe),h:memoryOf("h",heavy)},memoryOf("coordinator",stateOf({level:"guard"},{availableMegabytes:1800})));
 expect(ids(facts)).toEqual(["worker-memory-tight:warn:coordinator","worker-swap-high:warn:h","worker-warden-overhead:warn:h","worker-warden-observing:info:o"]);
 const en=summarizeFleetV1(facts,"en"),de=summarizeFleetV1(facts,"de");
 expect(en.findings.find(row=>row.id==="worker-swap-high")!.text).toBe("worker h uses 4097 MB of swap, 25% of its 16384 MB memory");
 expect(de.findings.find(row=>row.id==="worker-warden-overhead")!.text).toBe("Worker h: sein Wächter braucht 5 % eines Kerns (Takt p95 12 ms, 61 MB resident)");
 expect(de.findings.find(row=>row.id==="worker-warden-observing")!.text).toBe("Worker o: sein Wächter beobachtet nur (Stufe ruhig); er entscheidet, handelt aber nicht");
 expect(en.memory.warden.origin).toBe("coordinator");
 expect(formatFleetReportV1("en",en).some(line=>line.startsWith("Warden coordinator: guard (enforce), free 1800 MB, swap 100 MB (out 0 MB/s), compressed 900 MB, pressure normal"))).toBe(true);
 expect(formatFleetReportV1("de",de).some(line=>line.startsWith("Wächter coordinator: Wache (eingreifend), frei 1800 MB"))).toBe(true);
 expect(validateFleetReportV1(en)).toEqual([]);expect(validateFleetReportV1(de)).toEqual([]);
});

test("the memory reader lists the origins of the store, reads memory.json bounded and tolerant and never throws on a missing, half-written, invalid or oversized file",async()=>{
 const root=mkdtempSync(join(tmpdir(),"fleet-memory-")),store=join(root,"store"),origins=join(store,"v1","origins"),{readFleetMemoryV1}=await import("../📥️readers/🟦️.ts");
 try{
  requireHostWardenV1("State",stateOf());requireHostWardenV1("Mirror",{version:1,atMs:NOW,absent:true});
  for(const name of ["fresh","absent","invalid","half","big","missing","observing"])mkdirSync(join(origins,name),{recursive:true});
  const full=JSON.stringify(stateOf({level:"guard"},{availableMegabytes:1500}));
  writeFileSync(join(origins,"fresh","memory.json"),full);
  writeFileSync(join(origins,"absent","memory.json"),JSON.stringify({version:1,atMs:NOW-9000,absent:true}));
  writeFileSync(join(origins,"invalid","memory.json"),JSON.stringify({...stateOf(),extra:1}));
  writeFileSync(join(origins,"half","memory.json"),full.slice(0,full.length>>1));
  writeFileSync(join(origins,"big","memory.json"),JSON.stringify(stateOf({code:"x".repeat(300_000)})));
  writeFileSync(join(origins,"observing","memory.json"),JSON.stringify(stateOf({mode:"observe"})));
  writeFileSync(join(origins,"stray.json"),"{}");
  const result=await readFleetMemoryV1({root,environment:{SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_ORIGIN:"nobody"} as NodeJS.ProcessEnv,language:"en",now:()=>NOW}),memory=result.memory!;
  expect(Object.keys(memory.origins)).toEqual(["absent","big","fresh","half","invalid","missing","observing"]);
  expect(memory.origins.fresh).toMatchObject({origin:"fresh",absent:false,level:"guard",availableMegabytes:1500,ageSeconds:5});
  expect(memory.origins.absent).toMatchObject({absent:true,ageSeconds:9,level:null,counters:null});
  for(const name of ["invalid","half","big","missing"])expect(memory.origins[name],name).toBeNull();
  expect(memory.origins.observing!.mode).toBe("observe");
  expect(validateFleetFactsV1({version:1,atMs:NOW,workspace:root,memory,sources:[{id:"memory",ok:true,milliseconds:1}]})).toEqual([]);
  const none=await readFleetMemoryV1({root,environment:{SEMIO_FLEET_STORE_ROOT:join(root,"nope")} as NodeJS.ProcessEnv,language:"en",now:()=>NOW});
  expect(none.memory!.origins).toEqual({});
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the newest alarm of a state is the last alarm, whatever the order of the list",()=>{
 const state=stateOf({counters:{suspended:1,resumed:0,cancelled:1,ticks:3},lastAlarms:[alarmOf("cancel",{en:"newest",de:"neu"}),{...alarmOf("suspend",{en:"older",de:"älter"}),atMs:NOW-9000}]});
 expect(memoryOf("x",state)!.lastAlarm).toMatchObject({action:"cancel",text:{en:"newest",de:"neu"},kind:"cargo",pid:4242,owner:"agent-7",megabytes:3100});
 expect(memoryOf("x",null)).toBeNull();
});

test("the tree difference counts every path whose state differs once, from the lines only one side shows",()=>{
 expect(treeDifferenceV1([],[])).toEqual({files:0,onlyThere:0,onlyHere:0});
 expect(treeDifferenceV1([" M a/x.rs"," M b/y.rs"],[" M b/y.rs"," D c/z.rs"])).toEqual({files:3,onlyThere:1,onlyHere:1});
 expect(treeDifferenceV1([" M a/x.rs"],[" D a/x.rs"])).toEqual({files:1,onlyThere:0,onlyHere:0});
});

test("a worker row shows how its tree differs from the coordinator in both languages, or that it is the same, stale or not comparable, and the tree keeps the closed schema",()=>{
 const tree=(extra:Record<string,unknown>)=>({reference:"g1-server",files:0,onlyThere:0,onlyHere:0,stale:false,verdict:"ok",...extra});
 const facts=(rows:Record<string,unknown>[]):FleetFactsV1=>({version:1,atMs:NOW,workspace:"/w",sources:[],workers:{offlineSeconds:600,rows:rows as never}});
 const recent={hours:12,ok:0,failed:1,stalled:28,timedOut:0,seconds:12_060};
 const rows=[{...workerRow("same"),tree:tree({}),recent},{...workerRow("apart"),tree:tree({files:8,onlyThere:8,stale:true,verdict:"lagging"})},{...workerRow("blind"),tree:null},{...workerRow("plain")},{...workerRow("gone"),tree:tree({files:4809,onlyThere:2757,onlyHere:2052,verdict:"stale"})}];
 const english=formatFleetReportV1("en",summarizeFleetV1(facts(rows),"en")),german=formatFleetReportV1("de",summarizeFleetV1(facts(rows),"de"));
 expect(english.find(line=>line.includes("same macos"))).toContain(", tree same as g1-server");
 expect(english.find(line=>line.includes("apart macos"))).toContain(", tree differs from g1-server by 8 file(s) (8 only here at the worker, 0 only at g1-server), replica says lagging, edits in flight, its view is still being refreshed");
 expect(english.find(line=>line.includes("blind macos"))).toContain(", tree not comparable (no readable view)");
 expect(english.find(line=>line.includes("plain macos"))).not.toContain("tree");
 expect(english.find(line=>line.includes("gone macos"))).toContain(", tree not compared: the replica's attestation is old, so its view (4809 differing line(s)) no longer describes the worker's tree");
 expect(english.find(line=>line.includes("gone macos"))).not.toContain("only here at the worker");
 expect(german.find(line=>line.includes("gone macos"))).toContain(", Baum nicht verglichen: der Nachweis der Replika ist alt");
 expect(english.find(line=>line.includes("same macos"))).toContain(", last 12 h in this stage: 1 failed, 28 stopped as stalled (3.4 h run)");
 expect(german.find(line=>line.includes("same macos"))).toContain(", letzte 12 h in dieser Stufe: 1 fehlgeschlagen, 28 als stillstehend beendet (");
 expect(english.find(line=>line.includes("plain macos"))).not.toContain("last 12 h");
 expect(german.find(line=>line.includes("apart macos"))).toContain(", Baum weicht von g1-server um 8 Datei(en) ab");
 expect(validateFleetReportV1(plain(summarizeFleetV1(facts(rows),"en")))).toEqual([]);
});

test("the worker tree reader compares the views the replicas publish: equal views are the same tree, differing lines are counted per path, a torn view is not comparable, and a machine outside the store gives nothing",async()=>{
 const root=mkdtempSync(join(tmpdir(),"fleet-tree-")),store=join(root,"store");
 try{
  const attestation=(origin:string,text:string|null)=>({version:1,originId:origin,groupId:"g1",platform:"macos",wallMs:NOW,hlc:`${NOW}-000000-${origin}`,sync:true,code:"0".repeat(64),cursors:{[origin]:{segment:1,bytes:1}},files:3,manifest:"1".repeat(64),head:null,indexTree:null,view:text===null?null:{staged:0,modified:text.split("\n").length,deleted:0,untracked:0,digest:createHash("sha256").update(text).digest("hex")},guards:null,latency:{publishMs:{p50:null,p95:null},applyMs:{p50:1,p95:1},samples:{publish:0,apply:1},tiers:{hot:{passes:1,lastMs:1,dutyPercent:1},sweep:{passes:1,lastMs:1,dutyPercent:1},file:{passes:1,lastMs:1,dutyPercent:1},walk:{passes:0,lastMs:null,dutyPercent:null},view:{passes:1,lastMs:1,dutyPercent:1}}},queue:{incoming:0}});
  const put=(origin:string,text:string|null,file:string|null=text)=>{const directory=join(store,"v1","origins",origin);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"attest.json"),JSON.stringify(attestation(origin,text)));if(file!==null)writeFileSync(join(directory,"view.txt"),file);};
  put("g1-server"," M a/x.rs\n M b/y.rs");put("g1-m01"," M a/x.rs\n M b/y.rs");put("g1-m02"," M a/x.rs\n M b/y.rs\n M c/z.rs");put("g1-m03"," M a/x.rs",null);
  const context=(origin:string|undefined)=>({root,environment:{...(origin===undefined?{}:{SEMIO_FLEET_ORIGIN:origin})} as NodeJS.ProcessEnv,language:"en" as const,now:()=>NOW});
  const found=(await readWorkerTreesV1(store,context("g1-server")))!;
  expect(found.get("g1-m01")).toEqual({reference:"g1-server",files:0,onlyThere:0,onlyHere:0,stale:false,verdict:expect.any(String)});
  expect(found.get("g1-m02")).toEqual({reference:"g1-server",files:1,onlyThere:1,onlyHere:0,stale:false,verdict:expect.any(String)});
  expect(found.get("g1-m03")).toBeNull();
  expect(found.has("g1-server")).toBe(false);
  expect(await readWorkerTreesV1(store,context("g1-unknown"))).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a worker row names the mapping grant queue of its graph read, in both languages: who waits, for what step, the lease queue head and the finished tallies",()=>{
 const root=mkdtempSync(join(tmpdir(),"fleet-mapping-")),file=join(root,"slice-memory.json"),now=Date.now();
 try{
  const queue={waiting:[{kind:"shared",step:"grant",waitedMilliseconds:61000},{kind:"shared",step:"slot",waitedMilliseconds:2000}],holders:{shared:1,exclusive:0,exclusiveHeldMilliseconds:0},ownerSlots:{retained:3,waiting:0,queued:0,maximum:32},head:{holderKind:"exclusive",holderPid:4242,holderCommand:"cargo build",holderSeconds:75,position:2,queued:3,ageMilliseconds:100},granted:{shared:{count:12,waitedMilliseconds:90000,maximumMilliseconds:61000},exclusive:{count:1,waitedMilliseconds:500,maximumMilliseconds:500}},exclusiveHeld:{count:1,waitedMilliseconds:7000,maximumMilliseconds:7000}};
  writeFileSync(file,JSON.stringify({sources:{owners:{taken:33,presence:{mappingQueue:queue}}}}));
  const mapping=readWorkerMappingV1(file,now,262144,600)!;
  expect(mapping).toMatchObject({taken:33,waiting:2,step:"grant",longestWaitedSeconds:61,holderKind:"exclusive",holderPid:4242,holderSeconds:75,queued:3,sharedGrants:12,sharedWaitedSeconds:90,sharedLongestSeconds:61,exclusiveHeldSeconds:7});
  expect(readWorkerMappingV1(file,now+620000,262144,600),"a report older than the freshness bound says nothing").toBeUndefined();
  expect(readWorkerMappingV1(file,now,10,600),"an oversized report says nothing").toBeUndefined();
  writeFileSync(file,"{torn");expect(readWorkerMappingV1(file,now,262144,600)).toBeUndefined();
  writeFileSync(file,JSON.stringify({sources:{owners:{taken:1,presence:{}}}}));expect(readWorkerMappingV1(file,now,262144,600),"an older worker code publishes no queue").toBeUndefined();
  const base=cases[0]!,fact=plain(base.facts) as unknown as FleetFactsV1,row={origin:"g1-m10",platform:"macos",liveness:"working",recorded:"running",stage:"workspace",stageIndex:5,stageCount:5,attempts:3,recordAgeSeconds:2,heartbeatAgeSeconds:1,refusal:null,mapping};
  const facts={...fact,workers:{offlineSeconds:600,rows:[row,{...row,origin:"g1-m11",mapping:{...mapping,waiting:0,step:null,holderKind:null,holderPid:null,holderCommand:null,holderSeconds:null,queued:null}}]}} as unknown as FleetFactsV1;
  expect(validator("Facts")(facts),JSON.stringify(validator("Facts").errors?.[0])).toBe(true);
  const report=summarizeFleetV1(facts,"en");expect(validator("Report")(report),JSON.stringify(validator("Report").errors?.[0])).toBe(true);
  const english=formatFleetReportV1("en",report,40).filter(line=>line.includes("g1-m1")),german=formatFleetReportV1("de",summarizeFleetV1(facts,"de"),40).filter(line=>line.includes("g1-m1"));
  expect(english[0]).toContain("mapping grant: 2 waiting for the lease grant, longest 61 s");
  expect(english[0]).toContain("head of the lease queue: exclusive holder, process 4242 (cargo build) for 75 s, 3 queued");
  expect(english[1]).toContain("mapping grant: nobody waiting (12 shared grants waited");
  expect(german[0]).toContain("Zuordnungs-Grant: 2 warten auf Lease-Zuteilung");expect(german[0]).toContain("exklusiver Halter, Prozess 4242");
  expect(english.join("\n")).not.toMatch(/\{[a-zA-Z]+\}/u);expect(german.join("\n")).not.toMatch(/\{[a-zA-Z]+\}/u);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("fleet status shows the host name state per origin: ok, duplicate (naming the others), named as another origin and unknown, with a bilingual hint, equal to the Python oracle",async()=>{
 const root=mkdtempSync(join(tmpdir(),"fleet-hostname-")),now=Date.now();
 const publish=(origin:string,hostLabel:string|null)=>{mkdirSync(join(root,"v1","origins",origin),{recursive:true});writeFileSync(join(root,"v1","origins",origin,"addresses.json"),JSON.stringify({version:1,origin,name:`semio-${origin}`,hostLabel,advertise:{enabled:false,duplicates:[],namedAs:null},addresses:[],observedAtMs:now,ttlMs:300000,generation:1}));};
 try{
  publish("g1-m01","ueli-mac-01");publish("g1-m08","uelis-mac-mini");publish("g1-m10","uelis-mac-mini");mkdirSync(join(root,"v1","origins","g1-m13"),{recursive:true});writeFileSync(join(root,"v1","origins","g1-m13","addresses.json"),JSON.stringify({version:1,origin:"g1-m13",hostLabel:"uelis-mac-mini-2"}));publish("g1-server","semio-g1-m08");publish("g1-m15",null);
  const found=await readFleetHostNamesV1(root,["g1-m01","g1-m08","g1-m10","g1-m13","g1-m15","g1-m16","g1-server"]);
  expect(Object.fromEntries(found)).toEqual({
   "g1-m01":{state:"ok",label:"ueli-mac-01",others:[]},
   "g1-m08":{state:"duplicate",label:"uelis-mac-mini",others:["g1-m10","g1-m13"]},
   "g1-m10":{state:"duplicate",label:"uelis-mac-mini",others:["g1-m08","g1-m13"]},
   "g1-m13":{state:"duplicate",label:"uelis-mac-mini-2",others:["g1-m08","g1-m10"]},
   "g1-m15":{state:"unknown",label:null,others:[]},
   "g1-m16":{state:"unknown",label:null,others:[]},
   "g1-server":{state:"named-as",label:"semio-g1-m08",others:["g1-m08"]}});
  const base=plain(cases[0]!.facts) as unknown as FleetFactsV1,rows=["g1-m01","g1-m08","g1-m15","g1-server"].map(origin=>({...workerRow(origin),hostName:found.get(origin)!}));
  const facts={...base,workers:{offlineSeconds:600,rows}} as unknown as FleetFactsV1;
  expect(validator("Facts")(facts),JSON.stringify(validator("Facts").errors?.[0])).toBe(true);
  const english=summarizeFleetV1(facts,"en"),german=summarizeFleetV1(facts,"de");
  expect(validator("Report")(english),JSON.stringify(validator("Report").errors?.[0])).toBe(true);
  expect(english.findings.filter(row=>row.id.startsWith("worker-host-name")).map(row=>`${row.id}:${row.severity}:${row.subject}`)).toEqual(["worker-host-name-duplicate:warn:g1-m08","worker-host-name-named-as:warn:g1-server"]);
  expect(english.findings.find(row=>row.subject==="g1-m08"&&row.id==="worker-host-name-duplicate")!.text).toBe("worker g1-m08: its host name uelis-mac-mini is also carried by g1-m10, g1-m13; run `repo fleet hostname apply` on that machine");
  expect(german.findings.find(row=>row.id==="worker-host-name-named-as")!.text).toBe("Worker g1-server: sein Rechnername semio-g1-m08 ist der Name von g1-m08; auf dieser Maschine `repo fleet hostname apply` ausführen");
  const lines=formatFleetReportV1("en",english,40);
  expect(lines.find(line=>line.startsWith("  g1-m01 "))).toContain(", host name ok (ueli-mac-01)");
  expect(lines.find(line=>line.startsWith("  g1-m08 "))).toContain(", host name uelis-mac-mini is also on g1-m10, g1-m13: run `repo fleet hostname apply` on this machine");
  expect(lines.find(line=>line.startsWith("  g1-m15 "))).toContain(", host name unknown (nothing published yet)");
  expect(lines.find(line=>line.startsWith("  g1-server "))).toContain(", host name semio-g1-m08 belongs to g1-m08: run `repo fleet hostname apply` on this machine");
  const german2=formatFleetReportV1("de",german,40);
  expect(german2.find(line=>line.startsWith("  g1-m08 "))).toContain(", Rechnername uelis-mac-mini liegt auch auf g1-m10, g1-m13: auf dieser Maschine `repo fleet hostname apply` ausführen");
  expect(english.workers!.rows.find(row=>row.origin==="g1-server")).toMatchObject({hostName:{state:"named-as",others:["g1-m08"]}});
  expect(lines.join("\n")).not.toMatch(/\{[a-zA-Z]+\}/u);
  const file=join(root,"facts.json");writeFileSync(file,JSON.stringify(facts));
  for(const language of ["en","de"] as const){
   const result=spawnSync(requireTestToolV1("python"),[oracle,"summarize",file,language],{encoding:"utf8",maxBuffer:1<<28});
   expect(result.status,result.stderr).toBe(0);
   expect(JSON.parse(result.stdout).findings).toEqual(plain((language==="en"?english:german).findings));
  }
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("repo fleet hostname apply prints the plan with the exact sudo command, asks on the terminal only, never takes --yes and runs the tool only after a yes",async()=>{
 const root=mkdtempSync(join(tmpdir(),"fleet-hostname-cli-"));
 try{
  mkdirSync(join(root,".cache","f"),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:"g1-zz-test",groupId:"g1"}));
  const calls:string[]=[],questions:string[]=[],drive=async(args:string[],answer:string,interactive=true,language="en")=>{const lines:string[]=[];const code=await runFleetCliV1(root,args,{SEMIO_LANGUAGE:language},line=>lines.push(line),{hostname:{interactive,platform:"darwin",answer:async question=>{questions.push(question);return answer;},run:async(command,commandArguments)=>{calls.push([command,...commandArguments].join(" "));return 0;}}});return {code,lines};};
  const declined=await drive(["hostname","apply"],"n");
  expect(declined.code).toBe(0);
  expect(declined.lines.join("\n")).toContain("This machine is g1-zz-test. Its host name will become semio-g1-zz-test with: sudo scutil --set LocalHostName semio-g1-zz-test");
  expect(declined.lines.join("\n")).toContain("Nothing was changed.");
  expect(questions).toEqual(["Rename this machine to semio-g1-zz-test? [y/N] "]);
  expect(calls).toEqual([]);
  expect((await drive(["hostname","apply","--lang","de"],"n",true,"de")).lines.join("\n")).toContain("Es wurde nichts geändert.");
  const refused:string[]=[],noTerminal=await runFleetCliV1(root,["hostname","apply"],{},line=>refused.push(line),{hostname:{interactive:false,platform:"darwin",run:async()=>{throw Error("must not run");}},refuse:text=>refused.push(text)});
  expect(noTerminal).toBe(2);expect(refused.join("\n")).toContain("needs a terminal");expect(refused.join("\n")).toContain("braucht ein Terminal");
  await expect(drive(["hostname","apply","--yes"],"y")).rejects.toThrow();
  await expect(drive(["hostname"],"y")).rejects.toThrow();
  expect(calls).toEqual([]);
  expect((await drive(["hostname","apply"],"y")).code).toBe(0);
  expect(calls).toEqual(["sudo scutil --set LocalHostName semio-g1-zz-test"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});
