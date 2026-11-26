/** 🛡️ Contract: the host warden acts on running processes by one pure law (suspend, resume, cancel by the memory the host can still give out), classifies its subjects from the process table, and keeps its promises on real processes.
 * @see ./🐍️.py
 * @see ./🧫️fixtures/🔣️.json
 * @see ../🧬️schema/🔣️.json */
import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,symlinkSync,writeFileSync,existsSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {randomBytes} from "node:crypto";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import {readHostCapacityPolicyV1} from "../../📏️decision/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json";
import {repoPlaywrightBrowsersPath} from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts";
import {HOST_WARDEN_TABLE_V1,parseWardenCpuV1,HOST_WARDEN_ANCHORS_V1,HOST_WARDEN_POLICY_V1,HOST_WARDEN_TABLES_V1,buildHostWardenAlarmV1,classifyWardenSubjectsV1,decideHostWardenV1,discoverWardenWorkspacesV1,expandWardenPatternV1,headroomHostWardenV1,parseWardenSwapV1,wardenBrowsersV1,formatHostWardenAlarmV1,isHostWardenV1,triggerHostWardenV1,parseWardenElapsedV1,parseWardenProcessTableV1,planWardenSignalsV1,requireHostWardenV1,wardenSubjectIdV1,formatHostWardenCancelFailureV1,formatHostWardenCancelReasonV1,formatHostWardenHoldV1,formatHostWardenStatusV1,hostWardenHoldsV1,swapDepthHostWardenV1,topWardenOthersV1,type HostWardenHostV1,type HostWardenStateV1,type HostWardenKindV1,type HostWardenPolicyV1,type HostWardenRowV1,type HostWardenSubjectV1,type HostWardenTablesV1} from "../🟦️.ts";
import {HostWardenCancelledErrorV1,chargeWardenRowsV1,currentHostWardenMirrorV1,defaultHostWardenPortsV1,readHostWardenCancelsBelowV1,requireNoHostWardenCancelBelowV1,readWardenSwapV1,ensureHostWardenV1,hostWardenCodeDigestV1,readHostWardenCancelV1,readHostWardenCancelsSinceV1,readHostWardenMirrorV1,readHostWardenStateV1,runHostWardenLoopV1,wardenProcessStartV1,type HostWardenPortsV1} from "../🔁️loop/🟦️.ts";

const python=testPythonV1()??Bun.which("python3")??"python3",oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url)),decoder=new TextDecoder();
const model=(mode:string):any=>{const done=Bun.spawnSync([python,"-I",oracle,mode]);if(done.exitCode!==0)throw Error(decoder.decode(done.stderr));return JSON.parse(decoder.decode(done.stdout));};
const roots:string[]=[],fresh=():string=>{const root=mkdtempSync(join(tmpdir(),"warden-"));roots.push(root);return root;};
const children:{pid:number;kill:(signal?:number|NodeJS.Signals)=>void}[]=[];
const reap=async():Promise<void>=>{for(const child of children.splice(0)){try{process.kill(child.pid,"SIGCONT");}catch{}try{child.kill("SIGKILL");}catch{}}await Bun.sleep(150);};
afterAll(async()=>{await reap();for(const root of roots)rmSync(root,{recursive:true,force:true});});
const repoRoot=fileURLToPath(import.meta.url).split("/🧰️framework/")[0]!;
const mutable=<T>(value:unknown):T=>value as T;

test("the schema is closed and every kind names a pattern, a rank, actions and a source that exists in the repository",()=>{
 const kinds=schema.definitions.Kinds.const;
 expect(kinds.map(kind=>kind.name)).toEqual(["remote-unit","local-unit","pool-child","farm-worker","warm-deriver","heavy-question","browser","headless-browser","test","test-binary","slice-worker","cargo","stage"]);
 expect(kinds.map(kind=>kind.rank)).toEqual([10,20,25,30,35,40,45,46,50,55,60,65,90]);
 expect(kinds.map(kind=>kind.rank),"kinds are listed by ascending cost").toEqual([...kinds.map(kind=>kind.rank)].sort((a,b)=>a-b));
 for(const kind of kinds){
  expect(existsSync(join(repoRoot,kind.source)),`${kind.name}: ${kind.source}`).toBe(true);
  if(!kind.pattern.startsWith("^"))expect(kind.source.includes(kind.pattern),`${kind.name} pattern is a fragment of its source`).toBe(true);
  expect(()=>new RegExp(kind.pattern,"u")).not.toThrow();
 }
 expect(kinds.find(kind=>kind.name==="stage")).toMatchObject({actions:["cancel"],signal:"SIGTERM",scope:"tree"});
 const capped=Object.fromEntries(kinds.filter(kind=>"capMegabytes" in kind).map(kind=>[kind.name,(kind as {capMegabytes:number}).capMegabytes]));
 expect(capped).toEqual({"remote-unit":10240,"local-unit":10240,"pool-child":6144,"farm-worker":4096,"warm-deriver":6144,"heavy-question":8192,browser:4096,"headless-browser":4096,test:8192,"test-binary":8192,"slice-worker":6144});
 expect(kinds.filter(kind=>"capCritical" in kind).map(kind=>kind.name),"the build-critical kinds are never the first response to a cap").toEqual(["remote-unit","local-unit","pool-child","slice-worker"]);
 expect(kinds.filter(kind=>"ceiling" in kind).map(kind=>[kind.name,(kind as {ceiling:string}).ceiling]),"the kinds that run compile units carry a runtime ceiling of the admission law").toEqual([["remote-unit","unit"],["local-unit","unit"]]);
 expect(HOST_WARDEN_POLICY_V1).toMatchObject({swapUsedGuardBasisPoints:3500,swapUsedDangerBasisPoints:5500,swapShareGuardBasisPoints:4500,swapShareDangerBasisPoints:6000,compressedGuardBasisPoints:3500,compressedDangerBasisPoints:5000,capMaximumBasisPoints:6000,capMilliseconds:5000,capProbeMaximum:8,holdMinimumMegabytes:512,othersMaximum:5,ceilingNearBasisPoints:7500});
 for(const kind of kinds.filter(candidate=>"capMegabytes" in candidate))expect(kind.actions,`${kind.name}: a capped kind can be cancelled`).toContain("cancel");
 expect(kinds.find(kind=>kind.name==="pool-child")).toMatchObject({rank:25,actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"});
 expect(kinds.find(kind=>kind.name==="warm-deriver")).toMatchObject({rank:35,actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"});
 expect(kinds.find(kind=>kind.name==="browser")).toMatchObject({rank:45,actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"});
 expect(kinds.find(kind=>kind.name==="test-binary")).toMatchObject({rank:55,actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"});
 expect(kinds.find(kind=>kind.name==="cargo")).toMatchObject({rank:65,actions:["cancel"],signal:"SIGTERM",scope:"tree"});
 expect(Math.min(...kinds.filter(kind=>["browser","test-binary","cargo"].includes(kind.name)).map(kind=>kind.rank)),"new kinds cost more to redo than a compile unit").toBeGreaterThan(kinds.find(kind=>kind.name==="local-unit")!.rank);
 expect(kinds.find(kind=>kind.name==="cargo")!.rank,"cancelling cargo loses every unit of its tree, so it ranks after its units and its test binaries").toBeGreaterThan(kinds.find(kind=>kind.name==="test-binary")!.rank);
 expect(HOST_WARDEN_POLICY_V1).toMatchObject({guardBasisPoints:1000,guardMinimumMegabytes:1024,guardMaximumMegabytes:4096,dangerBasisPoints:500,dangerMinimumMegabytes:512,dangerMaximumMegabytes:2048,resumeBasisPoints:1500,swapActiveMegabytesPerSecond:16,swapDiskReserveMegabytes:4096,swapRoomMaximumBasisPoints:5000,headroomGuardBasisPoints:2500,headroomGuardMinimumMegabytes:2048,headroomGuardMaximumMegabytes:8192,headroomDangerBasisPoints:1250,headroomDangerMinimumMegabytes:1024,headroomDangerMaximumMegabytes:4096,calmTicks:3,suspendMaximumSeconds:120,maximumSuspendsPerTick:4,maximumCancelsPerTick:2,keepRunning:1,calmMilliseconds:2000,tightMilliseconds:500,idleMilliseconds:600000,codeCheckMilliseconds:60000,lastAlarms:8,alarmsMaximumBytes:1048576,cancelRecordMaximumAgeMilliseconds:3600000,stateFile:"state.json",lockFile:"lock.json",alarmsFile:"alarms.jsonl",modeFile:"mode",protectFile:"protect.json",cancelledDirectory:"cancelled",directory:"warden",logFile:"warden.log",logMaximumBytes:4194304,defaultMode:"enforce",ensureMilliseconds:15000});
 for(const pattern of[...schema.definitions.Roots.const,...schema.definitions.Never.const,...schema.definitions.Identities.const])expect(()=>new RegExp(pattern,"u")).not.toThrow();
 for(const pattern of[...schema.definitions.Identities.const,...kinds.map(kind=>kind.pattern)])expect(()=>new RegExp(expandWardenPatternV1(pattern,["/w/s"])!,"u"),pattern).not.toThrow();
});

test("a declared final link is the dearest compile unit and waits longer suspended, the danger cancel stays: the policy carries heavySuspendMaximumSeconds above suspendMaximumSeconds",()=>{
 expect(HOST_WARDEN_POLICY_V1.heavySuspendMaximumSeconds).toBeGreaterThan(HOST_WARDEN_POLICY_V1.suspendMaximumSeconds);
 const row=(name:string)=>fixture.decisions.find(item=>item.name.startsWith(name))!;
 expect(row("heavy: a young declared final link is kept running").expect.actions).toEqual([{action:"suspend",id:"p102",reason:"guard"}]);
 expect(row("heavy: the danger line still cancels").expect.actions).toEqual([{action:"cancel",id:"p101",reason:"danger"}]);
 expect(row("heavy: a declared final link suspended 130").expect.actions).toEqual([{action:"resume",id:"p102",reason:"overdue"}]);
});

test("every decision equals the independent Python oracle and the expectation stored in the fixture",()=>{
 const expected=model("--decide");
 expect(fixture.decisions.length).toBeGreaterThanOrEqual(70);
 fixture.decisions.forEach((row,index)=>{
  const policy={...HOST_WARDEN_POLICY_V1,...("policy" in row?row.policy:{})} as HostWardenPolicyV1;
  const decision=decideHostWardenV1(policy,mutable<HostWardenHostV1>(row.host),mutable<HostWardenSubjectV1[]>(row.subjects),mutable(row.previous));
  expect(JSON.parse(JSON.stringify(decision)),row.name).toEqual(expected[index]);
  expect(expected[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
  expect(isHostWardenV1("Decision",decision),row.name).toBe(true);
 });
 const seen=new Set(fixture.decisions.flatMap(row=>(row as {expect:{actions:{action:string;reason:string}[]}}).expect.actions.map(action=>`${action.action}.${action.reason}`)));
 for(const key of["suspend.guard","suspend.danger","cancel.danger","cancel.overdue","resume.overdue","resume.calm","cancel.cap"])expect(seen.has(key),key).toBe(true);
 const levels=new Set(fixture.decisions.map(row=>(row as {expect:{level:string}}).expect.level));
 expect([...levels].sort()).toEqual(["calm","danger","guard"]);
});

test("the ladder lines clamp on small and large hosts as the law says",()=>{
 const none:HostWardenSubjectV1[]=[],calm={level:"calm" as const,calmTicks:0};
 const lines=(total:number)=>{const d=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{totalMegabytes:total,availableMegabytes:total,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:0,pressure:"normal",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null},none,calm);return[d.guardMegabytes,d.dangerMegabytes,d.resumeMegabytes];};
 expect(lines(4096)).toEqual([1024,512,1024]);
 expect(lines(16384)).toEqual([1638,819,2457]);
 expect(lines(131072)).toEqual([4096,2048,19660]);
});

test("every classification equals the independent Python oracle and the expectation stored in the fixture",()=>{
 const expected=model("--classify");
 expect(fixture.classifications.length).toBeGreaterThanOrEqual(6);
 fixture.classifications.forEach((row,index)=>{
  const tables=("tables" in row?row.tables:HOST_WARDEN_TABLES_V1) as HostWardenTablesV1,options=("options" in row?row.options:{}) as {suspendedSince?:Record<string,number>;nowMs?:number;protect?:string[];within?:number;prefixes?:string[]};
  const subjects=classifyWardenSubjectsV1(mutable<HostWardenRowV1[]>(row.rows),tables,{...options,...options.suspendedSince?{suspendedSince:new Map(Object.entries(options.suspendedSince))}:{}} as never);
  expect(JSON.parse(JSON.stringify(subjects)),row.name).toEqual(expected[index]);
  expect(expected[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
  for(const subject of subjects)expect(isHostWardenV1("Subject",subject),row.name).toBe(true);
 });
 const kinds=new Set(fixture.classifications.flatMap(row=>(row as {expect:{kind:string}[]}).expect.map(subject=>subject.kind)));
 for(const kind of schema.definitions.Kinds.const)expect(kinds.has(kind.name),`a case classifies ${kind.name}`).toBe(true);
});

test("classification facts: ids carry the start digest, never beats kinds, the Ready stage below the sidecar is a subject, megabytes follow the nearest subject",()=>{
 const find=(name:string)=>fixture.classifications.find(row=>row.name.startsWith(name))!;
 const reused=(find("a pid reused") as {expect:{id:string;pid:number}[]}).expect;
 expect(reused[0]!.id).toBe(wardenSubjectIdV1(151,"Tue Nov 24 04:00:00 2026"));
 expect(reused[0]!.id).not.toBe(reused[1]!.id);
 expect((find("never patterns") as {expect:{kind:string;pid:number}[]}).expect.map(subject=>[subject.pid,subject.kind])).toEqual([[42,"stage"]]);
 expect((find("megabytes") as {expect:{pid:number;megabytes:number}[]}).expect.map(subject=>[subject.pid,subject.megabytes])).toEqual([[71,350],[74,310]]);
 expect((find("the first matching kind") as {expect:{kind:string}[]}).expect[0]!.kind).toBe("heavy-question");
 expect((find("a stopped subject") as {expect:{suspendedSeconds:number}[]}).expect[0]!.suspendedSeconds).toBe(250);
});

test("the process table parser reads real macOS text and a Linux sample: columns, stopped state, elapsed forms, start text, multi-token commands",()=>{
 const mac=parseWardenProcessTableV1(fixture.tables.darwin);
 expect(mac.map(row=>row.pid)).toEqual([1,2908,9362,26616,88190]);
 expect(mac[0]).toEqual({pid:1,parent:0,group:1,memoryKilobytes:12144,stopped:false,elapsedSeconds:((10*24+9)*60+4)*60+35,start:"Fri Nov 13 17:42:44 2026",command:"/sbin/launchd"});
 expect(mac[2]).toMatchObject({pid:9362,parent:1,group:9362,memoryKilobytes:16656,elapsedSeconds:((24+5)*60+11)*60+39,start:"Sun Nov 22 21:35:40 2026"});
 expect(mac[2]!.command).toContain("/.cache/b/t/m/bun/");
 expect(mac[3]).toMatchObject({pid:26616,parent:26264,group:97578,memoryKilobytes:679024,elapsedSeconds:36});
 expect(mac[3]!.command.startsWith("rustc --crate-name semio_framework_kernel_identity")).toBe(true);
 expect(mac[3]!.command).toContain("🧰️framework");
 expect(mac[4]).toMatchObject({pid:88190,elapsedSeconds:23*60+10,stopped:false});
 const linux=parseWardenProcessTableV1(fixture.tables.linux);
 expect(linux.map(row=>[row.pid,row.stopped,row.elapsedSeconds])).toEqual([[1,false,((12*24+3)*60+4)*60+5],[4242,true,(1*60+2)*60+3],[4243,false,5],[4250,false,59],[4300,true,((2*24+1)*60)*60]]);
 expect(linux[2]).toMatchObject({start:"Tue Nov  3 02:43:24 2026",command:"[sh] <defunct>"});
 expect(linux[1]!.command).toBe("/home/dev/.cache/b/t/m/bun/abc/bun /home/dev/ws/x/script.ts repo test");
 expect(parseWardenElapsedV1("2-01:00:00")).toBe(2*86400+3600);
 expect(parseWardenElapsedV1("garbage")).toBe(0);
 expect(parseWardenProcessTableV1("not a row\n  12 1 12 100 S 00:01 Mon Jan  1 00:00:00 2026 /x continued\n")).toHaveLength(1);
 for(const row of[...mac,...linux])expect(isHostWardenV1("ProcessRow",row)).toBe(true);
 const subjects=classifyWardenSubjectsV1(linux,HOST_WARDEN_TABLES_V1,{});
 expect(subjects.map(subject=>[subject.pid,subject.kind,subject.suspended])).toEqual([[4300,"local-unit",true]]);
});

test("a signal plan reaches the tree parent first, leaves out the warden itself, its ancestors and never processes",()=>{
 const row=(pid:number,parent:number,command:string,group=pid):HostWardenRowV1=>({pid,parent,group,memoryKilobytes:1,stopped:false,elapsedSeconds:1,start:"s",command});
 const rows=[row(1,0,"init"),row(10,1,"top"),row(11,10,"a"),row(12,11,"b"),row(13,10,"/x/🛰️sidecar/🟦️.ts agent"),row(14,10,"c")];
 expect(planWardenSignalsV1(rows,{pid:10,scope:"tree"})).toEqual([10,11,14,12]);
 expect(planWardenSignalsV1(rows,{pid:10,scope:"tree"},HOST_WARDEN_TABLES_V1,new Set([14]))).toEqual([10,11,12]);
 expect(planWardenSignalsV1(rows,{pid:11,scope:"process"})).toEqual([11]);
 expect(planWardenSignalsV1([row(20,1,"g",7),row(21,20,"h",7),row(22,1,"o",8)],{pid:20,scope:"group"})).toEqual([20,21]);
 expect(planWardenSignalsV1(rows,{pid:999,scope:"tree"})).toEqual([]);
});

test("the linker of a compile unit belongs to its compiler: the macOS driver and ld below a pinned rustc add to its megabytes, are no subjects and are reached by its tree signal, the unit ceiling is the final one of a test executable",()=>{
 const row=(pid:number,parent:number,memoryKilobytes:number,command:string):HostWardenRowV1=>({pid,parent,group:pid,memoryKilobytes,stopped:false,elapsedSeconds:60,start:"s"+pid,command});
 const rows=[row(1,0,1024,"/sbin/launchd"),row(70,1,40960,"/w/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun /w/x/script.ts rustc --crate-name curation_unit --test"),row(71,70,1048576,"rustc --crate-name curation_unit --edition=2021 --emit=dep-info,link --test -C debuginfo=line-tables-only"),row(72,71,8192,"cc -arch arm64 -m64 /t/curation_unit.rcgu.o -o /t/deps/curation_unit-5447f24b5ffef750 -Wl,-dead_strip"),row(73,72,4928000,"/Library/Developer/CommandLineTools/usr/bin/ld -demangle -arch arm64 -dead_strip -o /t/deps/curation_unit-5447f24b5ffef750")];
 const subjects=classifyWardenSubjectsV1(rows);
 expect(subjects.map(subject=>[subject.pid,subject.kind])).toEqual([[71,"local-unit"]]);
 expect(subjects[0]!.megabytes).toBe(Math.floor((1048576+8192+4928000)/1024));
 expect(subjects[0]!.ceiling).toEqual(expect.objectContaining({unit:"curation_unit|host",kind:"final"}));
 expect(planWardenSignalsV1(rows,{pid:71,scope:"tree"})).toEqual([71,72,73]);
});

test("an alarm names the action, the process, its owner, its megabytes and the host numbers in English and German",()=>{
 const host:HostWardenHostV1={totalMegabytes:16384,availableMegabytes:1500,swapUsedMegabytes:100,swapOutMegabytesPerSecond:20,compressedMegabytes:2000,pressure:"warn",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null};
 const alarm=buildHostWardenAlarmV1({atMs:1,observed:false,level:"guard",action:{action:"suspend",id:"7.abcd",reason:"guard"},subject:{pid:7,kind:"local-unit",megabytes:683,owner:"bun repo build"},host,decision:{guardMegabytes:1638,dangerMegabytes:819,headroomMegabytes:null,headroomGuardMegabytes:4096,headroomDangerMegabytes:2048}});
 expect(alarm.text.en).toBe("[warden] suspended local-unit pid 7 (bun repo build, 683 MB): available memory is below the guard line; host 1500 MB available (guard 1638 MB, danger 819 MB), swap 20 MB/s, pressure warn");
 expect(alarm.text.de).toBe("[Wächter] angehalten local-unit pid 7 (bun repo build, 683 MB): der verfügbare Speicher liegt unter der Wachlinie; Host 1500 MB verfügbar (Wache 1638 MB, Gefahr 819 MB), Swap 20 MB/s, Druck Warnung");
 expect(isHostWardenV1("Alarm",alarm)).toBe(true);
 const observed=buildHostWardenAlarmV1({atMs:1,observed:true,level:"danger",action:{action:"cancel",id:"7.abcd",reason:"danger"},subject:{pid:7,kind:"stage",megabytes:1},host:{...host,swapOutMegabytesPerSecond:null,pressure:"critical"},decision:{guardMegabytes:1,dangerMegabytes:1,headroomMegabytes:null,headroomGuardMegabytes:1,headroomDangerMegabytes:1}});
 expect(observed.observed).toBe(true);expect(observed.owner).toBe("");
 expect(formatHostWardenAlarmV1("en",observed)).toContain("swap unknown, pressure critical [observe mode: nothing was signalled]");
 expect(formatHostWardenAlarmV1("de",observed)).toContain("Swap unbekannt, Druck kritisch [Beobachtungsmodus: nichts wurde signalisiert]");
});

test("the alarm text names the trigger that decided the level, and agrees with the level of the decision",()=>{
 const base:HostWardenHostV1={totalMegabytes:16384,availableMegabytes:2000,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:null,pressure:"normal",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null};
 const words=(host:HostWardenHostV1,level:"guard"|"danger")=>{
  const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,host,[],{level:"calm",calmTicks:0});
  expect(decision.level).toBe(level);
  const alarm=buildHostWardenAlarmV1({atMs:1,observed:false,level,action:{action:"suspend",id:"7.abcd",reason:level},subject:{pid:7,kind:"stage",megabytes:1,owner:"o"},host,decision});
  return{trigger:triggerHostWardenV1(level,{availableMegabytes:host.availableMegabytes,guardMegabytes:decision.guardMegabytes,dangerMegabytes:decision.dangerMegabytes,pressure:host.pressure,headroomMegabytes:decision.headroomMegabytes,headroomGuardMegabytes:decision.headroomGuardMegabytes,headroomDangerMegabytes:decision.headroomDangerMegabytes}),en:alarm.text.en,de:alarm.text.de};
 };
 const critical=words({...base,availableMegabytes:2000,pressure:"critical"},"danger");
 expect(critical.trigger).toBe("pressure");expect(critical.en).toContain("memory pressure is critical");expect(critical.en).not.toContain("below the danger line");expect(critical.de).toContain("der Speicherdruck ist kritisch");
 expect(decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...base,availableMegabytes:9000,pressure:"critical"},[],{level:"calm",calmTicks:0}).level,"critical pressure alone is critical").toBe("danger");
 const low=words({...base,availableMegabytes:500,pressure:"critical"},"danger");
 expect(low.trigger).toBe("available");expect(low.en).toContain("available memory is below the danger line");expect(low.de).toContain("unter der Gefahrenlinie");
 const warn=words({...base,availableMegabytes:2000,pressure:"warn"},"guard");
 expect(warn.trigger).toBe("pressure");expect(warn.en).toContain("memory pressure is warn");expect(warn.de).toContain("der Speicherdruck ist Warnung");
 const swap=words({...base,availableMegabytes:2000,swapOutMegabytesPerSecond:50},"guard");
 expect(swap.trigger).toBe("swap");expect(swap.en).toContain("swap-out rate is high");expect(swap.de).toContain("Swap-Auslagerungsrate");
 const guard=words({...base,availableMegabytes:1000},"guard");
 expect(guard.trigger).toBe("available");expect(guard.en).toContain("below the guard line");
});

test("a third-party schema validator (python jsonschema, when installed) agrees with the validator of the repository on every record of the fixtures",()=>{
 const third=[testPythonV1(),Bun.which("python3"),Bun.which("python")].find(candidate=>candidate!==undefined&&candidate!==null&&Bun.spawnSync([candidate,"-c","import jsonschema"]).exitCode===0),probe={exitCode:third===undefined?1:0};
 if(probe.exitCode!==0){console.log("[skip] python jsonschema is not installed; the repository validator alone checked the records");return;}
 const records=[...fixture.decisions.map(row=>["Decision",(row as {expect:unknown}).expect]),...fixture.decisions.flatMap(row=>[["Host",row.host],...row.subjects.map(subject=>["Subject",subject])]),...fixture.classifications.flatMap(row=>[...row.rows.map(item=>["ProcessRow",item]),...(row as {expect:unknown[]}).expect.map(subject=>["Subject",subject])])];
 const code="import json,sys,jsonschema\ndoc=json.load(open(sys.argv[1]))\nrecords=json.load(sys.stdin)\nout=[]\nfor name,value in records:\n    try:\n        jsonschema.validate(value,{'$ref':'#/definitions/'+name,'definitions':doc['definitions']},cls=jsonschema.Draft7Validator)\n        out.append(True)\n    except jsonschema.ValidationError:\n        out.append(False)\nprint(json.dumps(out))";
 const done=Bun.spawnSync([third!,"-c",code,fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url))],{stdin:Buffer.from(JSON.stringify(records))});
 expect(done.exitCode,decoder.decode(done.stderr)).toBe(0);
 const theirs=JSON.parse(decoder.decode(done.stdout)) as boolean[];
 expect(theirs).toEqual(records.map(([name,value])=>isHostWardenV1(name as "Host",value)));
 expect(theirs.every(Boolean)).toBe(true);
 const broken=[["Decision",{level:"calm"}],["Host",{totalMegabytes:0}],["Action",{action:"kill",id:"x",reason:"guard"}],["Subject",{id:"a"}]] as const;
 const verdict=Bun.spawnSync([third!,"-c",code,fileURLToPath(new URL("../🧬️schema/🔣️.json",import.meta.url))],{stdin:Buffer.from(JSON.stringify(broken))});
 expect(JSON.parse(decoder.decode(verdict.stdout))).toEqual([false,false,false,false]);
 for(const[name,value]of broken)expect(isHostWardenV1(name,value)).toBe(false);
});

const token=randomBytes(4).toString("hex");
const hog=(tag:string)=>`const buffer=Buffer.alloc(40*1048576,1);setInterval(()=>{buffer[0]=1;},1000);/*warden-test-${token}-${tag}*/`;
const kind=(name:string,rank:number,tag:string):HostWardenKindV1=>({name,rank,pattern:`warden-test-${token}-${tag}\\*/`,actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"});
const testTables:HostWardenTablesV1={roots:[`warden-test-${token}`],never:[],kinds:[kind("cheap",10,"a"),kind("middle",20,"b"),kind("dear",30,"c")]};
const startChild=(tag:string)=>{const child=Bun.spawn([process.execPath,"-e",hog(tag)],{stdout:"ignore",stderr:"ignore",stdin:"ignore"});children.push(child);return child;};
const stateOf=(pid:number):string=>{const done=Bun.spawnSync(["ps","-o","state=","-p",String(pid)]);return decoder.decode(done.stdout).trim();};
const host=(available:number,pressure:"normal"|"warn"|"critical"="normal",swap:number|null=0):HostWardenHostV1=>({totalMegabytes:4096,availableMegabytes:available,swapUsedMegabytes:0,swapOutMegabytesPerSecond:swap,compressedMegabytes:0,pressure,swapFreeMegabytes:null,swapVolumeFreeMegabytes:null});
const realTable=async():Promise<readonly HostWardenRowV1[]>=>{const done=Bun.spawn(["ps","-A","-ww","-o","pid=,ppid=,pgid=,rss=,state=,etime=,lstart=,args="],{stdout:"pipe",env:{...process.env,LC_ALL:"en_US.UTF-8"}});return parseWardenProcessTableV1(await new Response(done.stdout).text());};
const quickPolicy:HostWardenPolicyV1={...HOST_WARDEN_POLICY_V1,calmMilliseconds:1,tightMilliseconds:1};
const scripted=(directory:string,phases:readonly HostWardenHostV1[],written:string[],afterTick:(tick:number)=>Promise<void>=async()=>undefined,mode:()=>"observe"|"enforce"=()=>"enforce"):HostWardenPortsV1=>{
 let index=0,tick=0;
 const allowed=()=>new Set(children.map(child=>child.pid));
 return{directory,now:Date.now,write:line=>written.push(line),mode,
  sample:async()=>phases[Math.min(index++,phases.length-1)]!,
  table:realTable,
  signal:(pid,name)=>{if(!allowed().has(pid))throw Error(`the test refuses to signal pid ${pid}`);process.kill(pid,name);},
  sleep:async()=>{tick++;await Bun.sleep(60);await afterTick(tick);}};
};

test("MOUNTED: real children are suspended, resumed and killed by the ladder; records, alarms, state and the lock hold",async()=>{
 await reap();
 const base=fresh(),directory=join(base,"warden"),written:string[]=[],c=startChild("c");await Bun.sleep(1300);const b=startChild("b");await Bun.sleep(1300);const a=startChild("a"),gone=(state:string)=>state===""||state.startsWith("Z");
 await Bun.sleep(700);
 const states:Record<number,Record<string,string>>={},second:{reason?:string}={};
 const phases=[host(3000),host(515),host(3000),host(3000),host(3000),host(505,"critical"),host(505,"critical"),host(3000)];
 const ports=scripted(directory,phases,written,async tick=>{
  states[tick]={a:stateOf(a.pid),b:stateOf(b.pid),c:stateOf(c.pid)};
  if(tick===2)Object.assign(second,await runHostWardenLoopV1({ports:{...scripted(directory,[host(3000)],[]),directory},policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:1}));
 });
 const result=await runHostWardenLoopV1({ports,policy:{...quickPolicy,guardMinimumMegabytes:520,pauseDeadlineSeconds:0},tables:testTables,within:process.pid,maxTicks:8,code:"mounted"});
 expect(result.reason).toBe("limit");expect(result.ticks).toBe(8);
 expect(states[1]!.a.startsWith("T"),"calm: nothing is stopped").toBe(false);
 expect(states[2]!.a.startsWith("T"),"guard: the cheapest subject is really stopped (state T)").toBe(true);
 expect(states[2]!.c.startsWith("T"),"the most expensive subject keeps running").toBe(false);
 expect(states[4]!.a.startsWith("T"),"still stopped before the third calm tick").toBe(true);
 expect(states[5]!.a.startsWith("T"),"resumed on the third calm tick").toBe(false);
 expect(states[5]!.a,"it is alive again").not.toBe("");
 expect(states[6]!.a.startsWith("T"),"danger: the cheapest subject is paused first, not killed").toBe(true);
 expect(gone(states[6]!.a)).toBe(false);
 expect(gone(states[7]!.a),"danger that pausing did not relieve: the paused subject is cancelled").toBe(true);
 expect(states[8]!.c.startsWith("T")).toBe(false);
 expect(second.reason,"a second warden of the directory refuses the lock").toBe("locked");
 expect(stateOf(b.pid).startsWith("T"),"leaving resumes what this run suspended").toBe(false);
 expect(stateOf(b.pid)).not.toBe("");
 const records=readdirSync(join(directory,"cancelled")).filter(name=>name.startsWith(`${a.pid}.`));
 expect(records).toHaveLength(1);
 const cancel=JSON.parse(readFileSync(join(directory,"cancelled",records[0]!),"utf8"));
 expect(requireHostWardenV1("Cancel",cancel)).toMatchObject({pid:a.pid,kind:"cheap",reason:"danger"});
 expect(cancel.megabytes).toBeGreaterThan(3);
 expect(readHostWardenCancelV1(a.pid,base)).toMatchObject({pid:a.pid,kind:"cheap",reason:"danger"});
 expect(readHostWardenCancelV1(b.pid,base)).toBeNull();
 const lines=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(lines.map(line=>`${line.action}.${line.kind}.${line.reason}`)).toEqual(["suspend.cheap.guard","resume.cheap.calm","suspend.cheap.danger","cancel.cheap.danger","suspend.middle.danger"]);
 for(const line of lines){expect(requireHostWardenV1("Alarm",line)).toBeTruthy();expect(line.observed).toBe(false);expect(line.text.en).toContain(`pid ${line.pid}`);expect(line.text.de).toContain(`pid ${line.pid}`);expect(line.megabytes).toBeGreaterThan(3);}
 expect(written.filter(line=>line.startsWith("[warden] suspended cheap")).length).toBe(2);
 expect(written.filter(line=>line.startsWith("[Wächter] angehalten cheap")).length).toBe(2);
 expect(written.some(line=>line.startsWith("[warden] started"))&&written.some(line=>line.startsWith("[Wächter] gestartet"))).toBe(true);
 const state=JSON.parse(readFileSync(join(directory,"state.json"),"utf8"));
 expect(requireHostWardenV1("State",state)).toMatchObject({pid:process.pid,mode:"enforce",code:"mounted",level:"calm",counters:{suspended:3,resumed:1,cancelled:1,ticks:8},suspended:[]});
 expect(state.subjects.byKind.dear.count).toBe(1);
 expect(state.lastAlarms).toHaveLength(5);
 expect(state.overhead.tickMillisecondsP95).toBeGreaterThan(0);
 expect(readHostWardenStateV1(base)?.pid).toBe(process.pid);
 expect(existsSync(join(directory,"lock.json")),"the lock is released").toBe(false);
});

test("MOUNTED: a fresh loop resumes what a dead predecessor left suspended and takes over its stale lock",async()=>{
 await reap();
 const directory=fresh(),written:string[]=[],d=startChild("a"),e=startChild("b");
 await Bun.sleep(600);
 process.kill(d.pid,"SIGSTOP");process.kill(e.pid,"SIGSTOP");
 await Bun.sleep(200);
 expect(stateOf(d.pid).startsWith("T")).toBe(true);
 const startD=wardenProcessStartV1(d.pid),rows=await realTable(),rowD=rows.find(row=>row.pid===d.pid)!;
 expect(rowD.start).toBe(startD);
 const state={version:1,atMs:Date.now()-60000,pid:999999,mode:"enforce",code:"dead",host:host(3000),level:"guard",hold:true,guardMegabytes:1024,dangerMegabytes:512,resumeMegabytes:1024,swapRoomMegabytes:null,headroomMegabytes:null,headroomGuardMegabytes:2048,headroomDangerMegabytes:1024,subjects:{count:2,megabytes:100,byKind:{cheap:{count:1,megabytes:50}}},suspended:[{id:wardenSubjectIdV1(d.pid,startD),pid:d.pid,start:startD,kind:"cheap",sinceMs:Date.now()-50000},{id:"1.deadbeef",pid:1,start:"never started",kind:"cheap",sinceMs:1}],counters:{suspended:1,resumed:0,cancelled:0,ticks:5},lastAlarms:[],overhead:{tickMillisecondsP50:1,tickMillisecondsP95:2,cpuBasisPoints:1,residentMegabytes:1}};
 requireHostWardenV1("State",state);
 writeFileSync(join(directory,"state.json"),JSON.stringify(state));
 writeFileSync(join(directory,"lock.json"),JSON.stringify({pid:999999,start:"Thu Jan  1 00:00:00 1970"}));
 const result=await runHostWardenLoopV1({ports:scripted(directory,[host(3000)],written),policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:1,code:"fresh"});
 expect(result.reason).toBe("limit");
 await Bun.sleep(100);
 expect(stateOf(d.pid).startsWith("T"),"the subject the predecessor suspended is resumed").toBe(false);
 expect(stateOf(e.pid).startsWith("T"),"a process nobody recorded stays as it is").toBe(true);
 expect(written.some(line=>line.startsWith("[warden] resumed cheap pid ")&&line.includes(String(d.pid)))).toBe(true);
 expect(written.some(line=>line.startsWith("[Wächter] cheap pid ")&&line.includes("fortgesetzt"))).toBe(true);
 expect(written.some(line=>line.includes("holds the lock"))).toBe(false);
 process.kill(e.pid,"SIGCONT");
});

test("MOUNTED: observe mode decides and alarms once per decision but signals nothing; a protected path excludes the subjects",async()=>{
 await reap();
 const directory=fresh(),written:string[]=[],h=startChild("c");await Bun.sleep(1300);const g=startChild("b");await Bun.sleep(1300);const f=startChild("a");
 await Bun.sleep(600);
 const phases=[host(300,"critical"),host(300,"critical"),host(300,"critical")];
 const result=await runHostWardenLoopV1({ports:scripted(directory,phases,written,async()=>undefined,()=>"observe"),policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:3,code:"observe"});
 expect(result.counters).toEqual({suspended:0,resumed:0,cancelled:0});
 for(const child of[f,g,h]){expect(stateOf(child.pid).startsWith("T")).toBe(false);expect(stateOf(child.pid)).not.toBe("");}
 const lines=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(lines.map(line=>`${line.action}.${line.kind}`),"three identical ticks announce each decision once").toEqual(["suspend.cheap","suspend.middle"]);
 for(const line of lines)expect(line.observed).toBe(true);
 expect(readdirSync(join(directory,"cancelled"))).toEqual([]);
 expect(written.some(line=>line.includes("[observe mode: nothing was signalled]"))&&written.some(line=>line.includes("[Beobachtungsmodus: nichts wurde signalisiert]"))).toBe(true);
 const guarded=fresh(),again:string[]=[];
 writeFileSync(join(guarded,"protect.json"),JSON.stringify([`warden-test-${token}-b`,`warden-test-${token}-a`,`warden-test-${token}-c`]));
 const none=await runHostWardenLoopV1({ports:scripted(guarded,[host(100,"critical")],again),policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:2,code:"protect"});
 expect(none.counters).toEqual({suspended:0,resumed:0,cancelled:0});
 for(const child of[f,g,h])expect(stateOf(child.pid)).not.toBe("");
 expect(existsSync(join(guarded,"alarms.jsonl"))).toBe(false);
});

test("MOUNTED: the loop ends by itself when no subject was seen for the idle time and when its sources changed",async()=>{
 const idle=await runHostWardenLoopV1({ports:scripted(fresh(),[host(3000)],[]),policy:{...quickPolicy,idleMilliseconds:50},tables:{roots:["nothing-matches-this"],never:[],kinds:[]},maxTicks:50,code:"idle"});
 expect(idle.reason).toBe("idle");
 const digest="two",changed=await runHostWardenLoopV1({ports:scripted(fresh(),[host(3000)],[]),policy:{...quickPolicy,codeCheckMilliseconds:1},tables:testTables,within:process.pid,maxTicks:50,code:"one",sources:()=>digest});
 expect(changed.reason).toBe("code");
 const controller=new AbortController();
 const stopped=await runHostWardenLoopV1({ports:scripted(fresh(),[host(3000)],[],async tick=>{if(tick===2)controller.abort();}),policy:quickPolicy,tables:testTables,within:process.pid,signal:controller.signal,code:"signal"});
 expect(stopped.reason).toBe("signal");
});

test("ensure is cheap and idempotent: no spawn while a fresh alive holder runs the same code, a spawn otherwise, never a throw",()=>{
 const directory=fresh(),spawned:{args:readonly string[]}[]=[],run=(command:string,args:readonly string[])=>{spawned.push({args});void command;};
 const first=ensureHostWardenV1({directory,code:"x",spawn:run,memoise:false});
 expect(first).toEqual({started:true,reason:"spawned"});
 expect(spawned).toHaveLength(1);
 expect(spawned[0]!.args[0]!.endsWith("⌨️main/🟦️.ts")).toBe(true);
 expect(spawned[0]!.args.slice(1)).toEqual(["--directory",directory]);
 const dir=join(directory,"warden");
 const state={version:1,atMs:Date.now(),pid:process.pid,mode:"enforce",code:"x",host:host(3000),level:"calm",hold:false,guardMegabytes:1,dangerMegabytes:1,resumeMegabytes:1,swapRoomMegabytes:null,headroomMegabytes:null,headroomGuardMegabytes:1,headroomDangerMegabytes:1,subjects:{count:0,megabytes:0,byKind:{}},suspended:[],counters:{suspended:0,resumed:0,cancelled:0,ticks:1},lastAlarms:[],overhead:{tickMillisecondsP50:1,tickMillisecondsP95:1,cpuBasisPoints:1,residentMegabytes:1}};
 mkdirSync(dir,{recursive:true});writeFileSync(join(dir,"state.json"),JSON.stringify(state));
 expect(ensureHostWardenV1({directory,code:"x",spawn:run,memoise:false})).toEqual({started:false,reason:"running"});
 expect(ensureHostWardenV1({directory,code:"y",spawn:run,memoise:false})).toEqual({started:false,reason:"replacing"});
 expect(spawned).toHaveLength(1);
 expect(readHostWardenStateV1(directory)?.pid).toBe(process.pid);
 expect(readHostWardenStateV1(directory,Date.now()+HOST_WARDEN_POLICY_V1.staleMilliseconds+5)).toBeNull();
 writeFileSync(join(dir,"state.json"),JSON.stringify({...state,atMs:Date.now()-HOST_WARDEN_POLICY_V1.staleMilliseconds-1}));
 expect(ensureHostWardenV1({directory,code:"x",spawn:run,memoise:false}).started).toBe(true);
 writeFileSync(join(dir,"state.json"),JSON.stringify({...state,pid:999999}));
 expect(ensureHostWardenV1({directory,code:"x",spawn:run,memoise:false}).started).toBe(true);
 expect(ensureHostWardenV1({directory,code:"x",spawn:()=>{throw Error("boom");},memoise:false})).toEqual({started:false,reason:"failed"});
 expect(hostWardenCodeDigestV1()).toMatch(/^[0-9a-f]{16}$/u);
 expect(hostWardenCodeDigestV1()).toBe(hostWardenCodeDigestV1());
 const cancelled=join(dir,"cancelled");mkdirSync(cancelled,{recursive:true});
 const record={version:1,pid:4242,start:"s",kind:"cheap",owner:"o",command:"rustc /x/work/t1/r/a.rs",megabytes:5,atMs:Date.now()-1000,reason:"danger",ancestors:[1]};
 writeFileSync(join(cancelled,"4242.deadbeef.json"),JSON.stringify(record));
 writeFileSync(join(cancelled,"4243.deadbeef.json"),JSON.stringify({...record,pid:4243,atMs:Date.now()-HOST_WARDEN_POLICY_V1.cancelRecordMaximumAgeMilliseconds-5}));
 expect(readHostWardenCancelV1(4242,directory)).toMatchObject({pid:4242,reason:"danger"});
 expect(readHostWardenCancelV1(4243,directory)).toBeNull();
 expect(readHostWardenCancelV1(1,directory)).toBeNull();
});

test("ensure memoises: the digest is read once per process and a second ensure within ensureMilliseconds returns without reading any file",()=>{
 const directory=fresh(),spawned:number[]=[],run=()=>{spawned.push(1);},t=Date.now();
 expect(ensureHostWardenV1({directory,code:"m",spawn:run,nowMs:t})).toEqual({started:true,reason:"spawned"});
 rmSync(join(directory,"warden"),{recursive:true,force:true});
 expect(ensureHostWardenV1({directory,code:"m",spawn:run,nowMs:t+HOST_WARDEN_POLICY_V1.ensureMilliseconds-1})).toEqual({started:false,reason:"running"});
 expect(existsSync(join(directory,"warden"))).toBe(false);
 expect(spawned).toHaveLength(1);
 expect(ensureHostWardenV1({directory,code:"m",spawn:run,nowMs:t+HOST_WARDEN_POLICY_V1.ensureMilliseconds})).toEqual({started:true,reason:"spawned"});
 expect(ensureHostWardenV1({directory,code:"n",spawn:run,nowMs:t+HOST_WARDEN_POLICY_V1.ensureMilliseconds})).toEqual({started:true,reason:"spawned"});
 expect(spawned).toHaveLength(3);
 expect(ensureHostWardenV1({directory,spawn:run,nowMs:t+1}).started).toBe(true);
 expect(ensureHostWardenV1({directory,spawn:run,nowMs:t+2})).toEqual({started:false,reason:"running"});
});

test("the rollout switch: a missing or invalid mode file answers the authored defaultMode, a valid one wins",()=>{
 const base=fresh(),ports=defaultHostWardenPortsV1({hostPolicy:readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity),directory:base});
 expect(schema.definitions.Policy.const.defaultMode).toBe("enforce");
 expect(HOST_WARDEN_POLICY_V1.defaultMode).toBe("enforce");
 expect(ports.mode()).toBe(HOST_WARDEN_POLICY_V1.defaultMode);
 mkdirSync(join(base,"warden"),{recursive:true});
 writeFileSync(join(base,"warden","mode"),"banana");
 expect(ports.mode()).toBe(HOST_WARDEN_POLICY_V1.defaultMode);
 writeFileSync(join(base,"warden","mode"),"enforce\n");
 expect(ports.mode()).toBe("enforce");
 writeFileSync(join(base,"warden","mode"),"observe");
 expect(ports.mode()).toBe("observe");
 expect(defaultHostWardenPortsV1({hostPolicy:readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity),directory:base,observe:true}).mode()).toBe("observe");
});

test("cancel records carry the command; readers list the records since a time; the mirror is the state as written or the absent record and validates",()=>{
 const directory=fresh(),dir=join(directory,"warden"),cancelled=join(dir,"cancelled"),now=Date.now();mkdirSync(cancelled,{recursive:true});
 const record={version:1,pid:7001,start:"s",kind:"remote-unit",owner:"o",command:"rustc /w/work/task-a/r/x.rs",megabytes:900,atMs:now-5000,reason:"danger",ancestors:[7000,1]};
 writeFileSync(join(cancelled,"7001.aaaaaaaa.json"),JSON.stringify(record));
 writeFileSync(join(cancelled,"7002.bbbbbbbb.json"),JSON.stringify({...record,pid:7002,command:"rustc /w/work/task-b/r/y.rs",atMs:now-9000}));
 writeFileSync(join(cancelled,"7003.cccccccc.json"),JSON.stringify({...record,pid:7003,atMs:now-1000,command:undefined}));
 expect(requireHostWardenV1("Cancel",record)).toMatchObject({command:"rustc /w/work/task-a/r/x.rs"});
 expect(isHostWardenV1("Cancel",{...record,command:"x".repeat(513)})).toBe(false);
 expect(readHostWardenCancelsSinceV1(now-6000,directory,now).map(entry=>entry.pid)).toEqual([7001]);
 expect(readHostWardenCancelsSinceV1(now-20000,directory,now).map(entry=>entry.pid)).toEqual([7001,7002]);
 expect(readHostWardenCancelsSinceV1(now,directory,now)).toEqual([]);
 expect(isHostWardenV1("Cancel",{...record,ancestors:undefined})).toBe(false);
 const absent=currentHostWardenMirrorV1(directory,now);
 expect(absent).toEqual({version:1,atMs:now,absent:true});
 expect(requireHostWardenV1("Mirror",absent)).toEqual(absent);
 const state={version:1,atMs:now,pid:process.pid,mode:"observe",code:"x",host:host(3000),level:"calm",hold:false,guardMegabytes:1,dangerMegabytes:1,resumeMegabytes:1,swapRoomMegabytes:null,headroomMegabytes:null,headroomGuardMegabytes:1,headroomDangerMegabytes:1,subjects:{count:0,megabytes:0,byKind:{}},suspended:[],counters:{suspended:0,resumed:0,cancelled:0,ticks:1},lastAlarms:[],overhead:{tickMillisecondsP50:1,tickMillisecondsP95:1,cpuBasisPoints:1,residentMegabytes:1}};
 writeFileSync(join(dir,"state.json"),JSON.stringify(state));
 const mirrored=currentHostWardenMirrorV1(directory,now);
 expect(mirrored).toEqual(state);
 const path=join(directory,"memory.json");
 writeFileSync(path,JSON.stringify(mirrored));expect(readHostWardenMirrorV1(path)).toEqual(state);
 writeFileSync(path,JSON.stringify(absent));expect(readHostWardenMirrorV1(path)).toEqual(absent);
 writeFileSync(path,JSON.stringify({...absent,absent:false}));expect(readHostWardenMirrorV1(path)).toBeNull();
 writeFileSync(path,JSON.stringify({...state,extra:1}));expect(readHostWardenMirrorV1(path)).toBeNull();
 expect(readHostWardenMirrorV1(join(directory,"none.json"))).toBeNull();
});

test("the real process table of this machine parses, holds this process and finds no subject below it that the authored tables name",async()=>{
 const rows=await realTable(),mine=rows.find(row=>row.pid===process.pid);
 expect(rows.length).toBeGreaterThan(10);
 expect(mine).toBeDefined();
 expect(mine!.memoryKilobytes).toBeGreaterThan(1000);
 expect(mine!.start).toBe(wardenProcessStartV1(process.pid));
 for(const row of rows)expect(isHostWardenV1("ProcessRow",row),String(row.pid)).toBe(true);
 const subjects=classifyWardenSubjectsV1(rows,HOST_WARDEN_TABLES_V1,{within:process.pid});
 expect(subjects.filter(subject=>subject.kind==="stage")).toEqual([]);
});

test("the default ports read this machine: a host sample that validates, a two-step process table whose command lines come from the cache the second time",async()=>{
 const base=fresh(),ports=defaultHostWardenPortsV1({hostPolicy:readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity),directory:base});
 expect(ports.directory).toBe(join(base,"warden"));
 const sample=await ports.sample();
 expect(isHostWardenV1("Host",sample)).toBe(true);
 expect(sample.availableMegabytes).toBeGreaterThan(0);
 expect(sample.availableMegabytes).toBeLessThanOrEqual(sample.totalMegabytes);
 if(process.platform==="darwin"){
  expect(typeof sample.compressedMegabytes).toBe("number");
  expect(typeof sample.swapFreeMegabytes,"free swap of sysctl vm.swapusage").toBe("number");
  const disk=Bun.spawnSync(["df","-k","/System/Volumes/Data"]),columns=decoder.decode(disk.stdout).trim().split("\n").at(-1)!.trim().split(/\s+/u);
  expect(Math.abs(sample.swapVolumeFreeMegabytes!-Math.floor(Number(columns[3])/1024)),"the swap volume shares its container with the data volume").toBeLessThan(8192);
 }
 if(process.platform==="linux")expect(sample.swapVolumeFreeMegabytes).toBeNull();
 const first=await ports.table(),second=await ports.table(),full=await realTable();
 for(const rows of[first,second]){
  const mine=rows.find(row=>row.pid===process.pid);
  expect(mine?.command.length,"this process has a command line").toBeGreaterThan(0);
  expect(mine!.start).toBe(wardenProcessStartV1(process.pid));
 }
 const commands=new Map(full.map(row=>[row.pid,row.command]));
 let compared=0;for(const row of second){const reference=commands.get(row.pid);if(reference!==undefined&&row.command!==""&&row.elapsedSeconds>=5&&!reference.startsWith("(")){expect(row.command).toBe(reference);compared++;}}
 expect(compared).toBeGreaterThan(10);
 expect(ports.mode()).toBe(HOST_WARDEN_POLICY_V1.defaultMode);
 mkdirSync(join(base,"warden"),{recursive:true});
 writeFileSync(join(base,"warden","mode"),"observe");
 expect(ports.mode()).toBe("observe");
});

test("every workspace discovery and every swap reading equals the independent Python oracle and the stored expectation",()=>{
 const found=model("--discover"),swaps=model("--swap");
 expect(fixture.discoveries.length).toBeGreaterThanOrEqual(3);expect(fixture.swaps.length).toBeGreaterThanOrEqual(9);
 fixture.discoveries.forEach((row,index)=>{
  expect(JSON.parse(JSON.stringify(discoverWardenWorkspacesV1(mutable<HostWardenRowV1[]>(row.rows),row.seed))),row.name).toEqual(found[index]);
  expect(found[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
 });
 fixture.swaps.forEach((row,index)=>{
  expect(JSON.parse(JSON.stringify(parseWardenSwapV1(row.platform,row.texts as never))),row.name).toEqual(swaps[index]);
  expect(swaps[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
 });
 expect(new Set(fixture.swaps.map(row=>row.platform))).toEqual(new Set(["darwin","linux","win32"]));
});

test("the browser anchor of the schema is the browser path the repository computes (ensureRepositoryBrowserPathV1 defaults to it), below any workspace",()=>{
 expect(HOST_WARDEN_ANCHORS_V1.browsers).toHaveLength(1);
 expect(wardenBrowsersV1([repoRoot])).toEqual([repoPlaywrightBrowsersPath(repoRoot)]);
 const second="/Users/other/ws";
 expect(wardenBrowsersV1([repoRoot,second])).toEqual([repoPlaywrightBrowsersPath(repoRoot),repoPlaywrightBrowsersPath(second)]);
 expect(expandWardenPatternV1("^<browsers>/x",[])).toBeNull();
 expect(expandWardenPatternV1("^<workspace>/x",["/a.b","/c"])).toBe("^(?:/a\\.b|/c)/x");
 expect(expandWardenPatternV1("^plain",[])).toBe("^plain");
});

test("identity facts: shell-launched pinned toolchain and cargo test binaries are subjects, cargo ranks after its units, the user's tools and the Claude app never are",()=>{
 const find=(name:string)=>(fixture.classifications.find(row=>row.name.startsWith(name))as unknown as {expect:{pid:number;kind:string;rank:number;megabytes:number;owner:string}[]}).expect;
 const shell=find("identity: pinned toolchain");
 expect(shell.map(subject=>[subject.pid,subject.kind])).toEqual([[30,"cargo"],[31,"local-unit"],[32,"local-unit"],[33,"local-unit"],[34,"local-unit"]]);
 expect(shell.find(subject=>subject.kind==="cargo")!.rank).toBeGreaterThan(shell.find(subject=>subject.kind==="local-unit")!.rank);
 expect(find("identity: the rustc, cargo and Homebrew")).toEqual([]);
 expect(find("identity: cargo test and bench").map(subject=>[subject.pid,subject.kind])).toEqual([[30,"cargo"],[31,"test-binary"],[32,"test-binary"],[33,"test-binary"]]);
 const browser=find("identity: the repository browser runtime");
 expect(browser.map(subject=>[subject.pid,subject.kind])).toEqual([[40,"browser"],[43,"browser"]]);
 expect(browser[0]!.megabytes,"the renderer and the crash handler count into the main browser process").toBe(Math.floor((150000+600000+9000)/1024));
 expect(find("identity: cargo and compile tools of any toolchain").map(subject=>[subject.pid,subject.kind])).toEqual([[30,"cargo"],[31,"local-unit"],[33,"local-unit"]]);
 expect(find("identity anchors")).toEqual([]);
 expect(find("identity: protected").map(subject=>subject.pid)).toEqual([33]);
 expect(find("identity: a rustc below a root").map(subject=>subject.owner)).toEqual(["bun x/script.ts repo build","rustc --crate-name b src/lib.rs"]);
 const order=(rows:{rank:number}[])=>rows.map(row=>row.rank);
 expect(order(shell).slice(1).every(rank=>rank===20)).toBe(true);
});

test("the headroom law: available memory plus the swap room, bounded by the free disk of the swap volume minus the reserve and by the share of the physical memory",()=>{
 const base:HostWardenHostV1={totalMegabytes:16384,availableMegabytes:3000,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:null,pressure:"normal",swapFreeMegabytes:1000,swapVolumeFreeMegabytes:1500};
 const room=(over:Partial<HostWardenHostV1>)=>headroomHostWardenV1(HOST_WARDEN_POLICY_V1,{...base,...over});
 expect(room({})).toEqual({swapRoomMegabytes:1000,headroomMegabytes:4000,headroomGuardMegabytes:4096,headroomDangerMegabytes:2048});
 expect(room({swapVolumeFreeMegabytes:4096}).swapRoomMegabytes,"a free disk equal to the reserve grows nothing").toBe(1000);
 expect(room({swapVolumeFreeMegabytes:4096+500}).swapRoomMegabytes).toBe(1500);
 expect(room({swapVolumeFreeMegabytes:61000}).swapRoomMegabytes,"bounded by half of the physical memory").toBe(8192);
 expect(room({swapVolumeFreeMegabytes:null}).swapRoomMegabytes,"a swap that cannot grow keeps its free part").toBe(1000);
 expect(room({swapFreeMegabytes:null}).headroomMegabytes).toBeNull();
 const level=(over:Partial<HostWardenHostV1>)=>decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...base,...over},[],{level:"calm",calmTicks:0});
 expect(level({}).level,"available above the guard line, headroom below its line: the old law said calm").toBe("guard");
 expect(level({swapVolumeFreeMegabytes:61000}).level).toBe("calm");
 expect(level({availableMegabytes:1900,swapFreeMegabytes:0,swapVolumeFreeMegabytes:0}).level).toBe("danger");
 expect(level({swapFreeMegabytes:null}).level,"a host without swap keeps the available-memory law").toBe("calm");
 expect(level({}).swapRoomMegabytes).toBe(1000);
 for(const total of[2048,4096,8192,16384,32768,65536,131072]){
  const calm=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{totalMegabytes:total,availableMegabytes:0,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:null,pressure:"normal",swapFreeMegabytes:0,swapVolumeFreeMegabytes:10**7},[],{level:"calm",calmTicks:0});
  const guard=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{totalMegabytes:total,availableMegabytes:0,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:null,pressure:"normal",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null},[],{level:"calm",calmTicks:0}).guardMegabytes;
  const healthy=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{totalMegabytes:total,availableMegabytes:guard,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:null,pressure:"normal",swapFreeMegabytes:0,swapVolumeFreeMegabytes:10**7},[],{level:"calm",calmTicks:0});
  expect(calm.level,`${total} MB: no available memory is danger`).toBe("danger");
  expect(healthy.level,`${total} MB: a roomy disk never escalates what the available-memory law calls calm`).toBe("calm");
 }
});

test("the alarm names the headroom as the trigger and prints it in both languages; a headroom trigger never says 'available memory is below the guard line'",()=>{
 const host:HostWardenHostV1={totalMegabytes:16384,availableMegabytes:3000,swapUsedMegabytes:1856,swapOutMegabytesPerSecond:0,compressedMegabytes:null,pressure:"normal",swapFreeMegabytes:1000,swapVolumeFreeMegabytes:1500};
 const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,host,[],{level:"calm",calmTicks:0});
 expect(decision.level).toBe("guard");
 const alarm=buildHostWardenAlarmV1({atMs:1,observed:false,level:"guard",action:{action:"suspend",id:"7.abcd",reason:"guard"},subject:{pid:7,kind:"local-unit",megabytes:683,owner:"zsh"},host,decision});
 expect(alarm.text.en).toBe("[warden] suspended local-unit pid 7 (zsh, 683 MB): available memory plus swap that can still grow is below the headroom guard line; host 3000 MB available (guard 1638 MB, danger 819 MB), swap 0 MB/s, pressure normal, headroom 4000 MB (guard 4096 MB, danger 2048 MB)");
 expect(alarm.text.de).toBe("[Wächter] angehalten local-unit pid 7 (zsh, 683 MB): der verfügbare Speicher plus noch wachsender Swap liegt unter der Wachlinie der Reserve; Host 3000 MB verfügbar (Wache 1638 MB, Gefahr 819 MB), Swap 0 MB/s, Druck normal, Reserve 4000 MB (Wache 4096 MB, Gefahr 2048 MB)");
 expect(isHostWardenV1("Alarm",alarm)).toBe(true);
 const danger=decideHostWardenV1(HOST_WARDEN_POLICY_V1,{...host,availableMegabytes:1900,swapFreeMegabytes:0,swapVolumeFreeMegabytes:0},[],{level:"calm",calmTicks:0});
 const text=buildHostWardenAlarmV1({atMs:1,observed:false,level:"danger",action:{action:"cancel",id:"7.abcd",reason:"danger"},subject:{pid:7,kind:"cargo",megabytes:1,owner:"zsh"},host:{...host,availableMegabytes:1900,swapFreeMegabytes:0,swapVolumeFreeMegabytes:0},decision:danger}).text;
 expect(text.en).toContain("below the headroom danger line");expect(text.en).not.toContain("available memory is below");expect(text.de).toContain("Gefahrenlinie der Reserve");
});

test("MOUNTED: a pinned rustc and a cargo test binary started from a shell (no root at all in the tables) are subjects and are suspended by the ladder; a rustc outside the pinned path of the workspace is not touched",async()=>{
 await reap();
 const workspace=fresh(),bin=join(workspace,".cache","b","t","m","r","h0","toolchains","nightly-test","bin"),deps=join(workspace,"target","debug","deps"),foreignBin=join(fresh(),"bin"),sleep=Bun.which("sleep")!;
 mkdirSync(bin,{recursive:true});mkdirSync(deps,{recursive:true});mkdirSync(foreignBin,{recursive:true});
 const unitPath=join(bin,"rustc"),binaryPath=join(deps,"semio_core-0123456789abcdef"),foreignPath=join(foreignBin,"rustc");
 for(const path of[unitPath,binaryPath,foreignPath])symlinkSync(sleep,path);
 const spawnAs=(path:string)=>{const child=Bun.spawn([path,"60"],{stdout:"ignore",stderr:"ignore",stdin:"ignore"});children.push(child);return child;};
 const unit=spawnAs(unitPath),binary=spawnAs(binaryPath),foreign=spawnAs(foreignPath),directory=fresh(),written:string[]=[],states:Record<number,Record<string,string>>={};
 await Bun.sleep(500);
 const ports=scripted(directory,[host(1000),host(1000),host(1000)],written,async tick=>{
  if(tick===1)for(let attempt=0;attempt<20&&!stateOf(binary.pid).startsWith("T");attempt++)await Bun.sleep(10);
  states[tick]={unit:stateOf(unit.pid),binary:stateOf(binary.pid),foreign:stateOf(foreign.pid)};
 });
 const result=await runHostWardenLoopV1({ports,policy:quickPolicy,tables:{...HOST_WARDEN_TABLES_V1,roots:[]},workspaces:[workspace],within:process.pid,maxTicks:3,code:"identity"});
 expect(result.reason).toBe("limit");
 expect(states[2]!.binary.startsWith("T"),"the cargo test binary is stopped although no root is among its ancestors, the non-critical subject goes first").toBe(true);
 expect(states[2]!.unit.startsWith("T"),"the build-critical pinned rustc keeps running").toBe(false);
 expect(states[2]!.foreign.startsWith("T"),"a rustc outside the pinned toolchain path is never touched").toBe(false);
 const state=JSON.parse(readFileSync(join(directory,"state.json"),"utf8"));
 expect(requireHostWardenV1("State",state).subjects.byKind["local-unit"].count).toBe(1);
 expect(state.subjects.byKind["test-binary"].count).toBe(1);
 expect(Object.keys(state.subjects.byKind).sort()).toEqual(["local-unit","test-binary"]);
 const lines=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(lines.map(line=>`${line.action}.${line.kind}`)).toEqual(["suspend.test-binary"]);
 expect(lines[0].owner.includes("semio_core-0123456789abcdef"),"a subject outside every root is its own owner").toBe(true);
 expect(stateOf(binary.pid).startsWith("T"),"leaving resumes what this run suspended").toBe(false);
});

test("the swap reader runs the commands of the schema per platform and a failing command leaves its number null",async()=>{
 const calls:string[]=[],texts:Record<string,string>={"sysctl -n vm.swapusage":"total = 3072.00M  used = 1832.19M  free = 1239.81M  (encrypted)","df -k /System/Volumes/VM":fixture.swaps[0]!.texts.disk!};
 const run=async(command:string,args:readonly string[])=>{const key=[command,...args].join(" ");calls.push(key);const text=texts[key];if(text===undefined)throw Error("no such command");return text;};
 expect(await readWardenSwapV1("darwin",{run,read:async()=>null})).toEqual({swapFreeMegabytes:1239,swapVolumeFreeMegabytes:60487});
 expect(calls).toEqual(["sysctl -n vm.swapusage","df -k /System/Volumes/VM"]);
 calls.length=0;delete texts["df -k /System/Volumes/VM"];texts["df -k /System/Volumes/Data"]=fixture.swaps[1]!.texts.disk!;
 expect(await readWardenSwapV1("darwin",{run,read:async()=>null})).toEqual({swapFreeMegabytes:1239,swapVolumeFreeMegabytes:2307});
 expect(calls).toEqual(["sysctl -n vm.swapusage","df -k /System/Volumes/VM","df -k /System/Volumes/Data"]);
 expect(await readWardenSwapV1("darwin",{run:async()=>{throw Error("x");},read:async()=>null})).toEqual({swapFreeMegabytes:null,swapVolumeFreeMegabytes:null});
 expect(await readWardenSwapV1("linux",{run,read:async path=>path==="/proc/meminfo"?fixture.swaps[5]!.texts.meminfo!:null})).toEqual({swapFreeMegabytes:4882,swapVolumeFreeMegabytes:null});
 expect(await readWardenSwapV1("win32",{run,read:async()=>null})).toEqual({swapFreeMegabytes:null,swapVolumeFreeMegabytes:null});
});

const memoryCase=(available:number,over:Partial<HostWardenHostV1>={}):HostWardenHostV1=>({totalMegabytes:16384,availableMegabytes:available,swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,compressedMegabytes:0,pressure:"normal",swapFreeMegabytes:null,swapVolumeFreeMegabytes:null,...over});

test("swap and compressed depth: lines are shares of the physical memory, a fixed swap is also read as a share of itself, a growing swap is not",()=>{
 const policy=HOST_WARDEN_POLICY_V1;
 expect(swapDepthHostWardenV1(policy,memoryCase(9000))).toEqual({depth:0,compressed:0,source:null});
 expect(swapDepthHostWardenV1(policy,memoryCase(9000,{swapUsedMegabytes:5735}))).toEqual({depth:1,compressed:0,source:"swap-used"});
 expect(swapDepthHostWardenV1(policy,memoryCase(9000,{swapUsedMegabytes:9012}))).toEqual({depth:2,compressed:0,source:"swap-used"});
 expect(swapDepthHostWardenV1(policy,memoryCase(9000,{compressedMegabytes:8192}))).toEqual({depth:2,compressed:2,source:"compressed"});
 expect(swapDepthHostWardenV1(policy,memoryCase(9000,{swapUsedMegabytes:3000,swapFreeMegabytes:1500}))).toEqual({depth:2,compressed:0,source:"swap-share"});
 expect(swapDepthHostWardenV1(policy,memoryCase(9000,{swapUsedMegabytes:3000,swapFreeMegabytes:1500,swapVolumeFreeMegabytes:60000}))).toEqual({depth:0,compressed:0,source:null});
 expect(swapDepthHostWardenV1(policy,memoryCase(9000,{swapUsedMegabytes:null,compressedMegabytes:null}))).toEqual({depth:0,compressed:0,source:null});
});

test("the alarm names a deep swap or compressed memory as the trigger, and a cap kill names the cap, in both languages",()=>{
 const words=(host:HostWardenHostV1,level:"guard"|"danger")=>{
  const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,host,[],{level:"calm",calmTicks:0});
  expect(decision.level).toBe(level);
  const alarm=buildHostWardenAlarmV1({atMs:1,observed:false,level,action:{action:"suspend",id:"7.abcd",reason:level},subject:{pid:7,kind:"stage",megabytes:1,owner:"o"},host,decision});
  expect(isHostWardenV1("Alarm",alarm)).toBe(true);
  return{trigger:alarm.trigger,en:alarm.text.en,de:alarm.text.de};
 };
 const swap=words(memoryCase(2300,{swapUsedMegabytes:6000}),"guard");
 expect(swap.trigger).toBe("swap-used");expect(swap.en).toContain("swap in use is above its line");expect(swap.de).toContain("der belegte Swap liegt über seiner Linie");
 const compressed=words(memoryCase(1500,{compressedMegabytes:8500}),"danger");
 expect(compressed.trigger).toBe("compressed");expect(compressed.en).toContain("compressed memory is above its danger line");expect(compressed.de).toContain("der komprimierte Speicher liegt über seiner Gefahrenlinie");
 const share=words(memoryCase(1500,{swapUsedMegabytes:3000,swapFreeMegabytes:1500}),"danger");
 expect(share.trigger).toBe("swap-share");expect(share.en).toContain("share of the swap in use");
 const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,memoryCase(9000),[{id:"9.x",pid:9,kind:"browser",rank:45,elapsedSeconds:5,megabytes:5000,suspended:false,suspendedSeconds:0,canSuspend:true,canCancel:true,capMegabytes:4096}],{level:"calm",calmTicks:0});
 expect(decision.actions).toEqual([{action:"cancel",id:"9.x",reason:"cap",capMegabytes:4096}]);
 const alarm=buildHostWardenAlarmV1({atMs:1,observed:false,level:"calm",action:decision.actions[0]!,subject:{pid:9,kind:"browser",megabytes:5000,owner:"o"},host:memoryCase(9000),decision});
 expect(alarm).toMatchObject({reason:"cap",capMegabytes:4096});expect(alarm.trigger).toBeUndefined();
 expect(alarm.text.en).toContain("its memory is over the cap of 4096 MB for browser");expect(alarm.text.de).toContain("sein Speicher liegt über der Obergrenze von 4096 MB für browser");
 expect(isHostWardenV1("Alarm",alarm)).toBe(true);
});

test("every list of largest foreign consumers and every hold verdict equals the independent Python oracle and the stored expectation",()=>{
 const others=model("--others"),holds=model("--holds");
 expect(fixture.others.length).toBeGreaterThanOrEqual(3);expect(fixture.holds.length).toBeGreaterThanOrEqual(11);
 fixture.others.forEach((row,index)=>{
  const found=topWardenOthersV1(mutable<HostWardenRowV1[]>(row.rows),mutable<{pid:number}[]>(row.subjects),row.count,(row as {prefixes?:string[]}).prefixes??[]);
  expect(JSON.parse(JSON.stringify(found)),row.name).toEqual(others[index]);
  expect(others[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
 });
 fixture.holds.forEach((row,index)=>{
  const found=hostWardenHoldsV1(HOST_WARDEN_POLICY_V1,mutable<{hold:boolean;atMs:number}|null>(row.state),row.needMegabytes,row.nowMs,(row as {operation?:string}).operation);
  expect(found,row.name).toBe(holds[index]);
  expect(holds[index],`stored expectation of ${row.name}`).toBe((row as {expect:boolean}).expect);
 });
});

test("the status lines and the hold line speak the levels as calm, tight and critical in both languages, with the trigger and the foreign consumers",()=>{
 const host=memoryCase(2300,{swapUsedMegabytes:6000,compressedMegabytes:1200});
 const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,host,[],{level:"calm",calmTicks:0});
 const alarm=buildHostWardenAlarmV1({atMs:5,observed:false,level:"guard",action:{action:"suspend",id:"7.abcd",reason:"guard"},subject:{pid:7,kind:"browser",megabytes:900,owner:"o"},host,decision});
 const state:HostWardenStateV1={version:1,atMs:10,pid:1,mode:"enforce",code:"c",host,level:"guard",hold:true,trigger:"swap-used",others:[{pid:60,megabytes:1464,processes:3,command:"Big"}],guardMegabytes:1638,dangerMegabytes:819,resumeMegabytes:2457,swapRoomMegabytes:null,headroomMegabytes:null,headroomGuardMegabytes:4096,headroomDangerMegabytes:2048,subjects:{count:0,megabytes:0,byKind:{}},suspended:[],counters:{suspended:1,resumed:0,cancelled:2,ticks:9},lastAlarms:[alarm],overhead:{tickMillisecondsP50:1,tickMillisecondsP95:2,cpuBasisPoints:3,residentMegabytes:4}};
 expect(isHostWardenV1("State",state)).toBe(true);
 const en=formatHostWardenStatusV1("en",state),de=formatHostWardenStatusV1("de",state);
 expect(en[0]).toContain("tight (enforce)");expect(en[0]).toContain("swap in use is above its line");expect(en[0]).toContain("swap in use 6000 MB");expect(en[0]).toContain("suspended 1, resumed 0, cancelled 2");
 expect(en[1]).toContain("last action: [warden] suspended browser");expect(en[2]).toContain("Big (pid 60, 3 processes, 1464 MB)");
 expect(de[0]).toContain("knapp (enforce)");expect(de[2]).toContain("Big (pid 60, 3 Prozesse, 1464 MB)");
 expect(formatHostWardenStatusV1("en",{...state,level:"danger"})[0]).toContain("critical (enforce)");
 expect(formatHostWardenStatusV1("de",{...state,level:"calm",hold:false,trigger:null})[0]).toContain("ruhig (enforce), kein Auslöser");
 expect(formatHostWardenStatusV1("en",null)).toEqual(["Memory guard: no warden is running on this machine"]);
 expect(formatHostWardenHoldV1("en","cargo build",state)).toBe("[warden] cargo build waits: host memory is tight (swap in use is above its line and available memory is below the resume line); nothing heavy starts until it is calm (available 2300 MB, swap in use 6000 MB, compressed 1200 MB)");
 expect(formatHostWardenHoldV1("de","cargo build",{...state,level:"danger",trigger:"available"})).toContain("der Hostspeicher ist kritisch");
});

const rowOf=(pid:number,parent:number,megabytes:number,command:string):HostWardenRowV1=>({pid,parent,group:pid,memoryKilobytes:megabytes*1024,stopped:false,elapsedSeconds:600,start:`Tue Nov 24 06:${String(pid%60).padStart(2,"0")}:00 2026`,command});
const watchdogTables:HostWardenTablesV1={roots:["^/fake/root"],never:[],kinds:[{name:"browser",rank:45,pattern:"^/fake/chromium/(?!.*\\s--type=)",actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree",capMegabytes:1000},{name:"probe",rank:50,pattern:"^/fake/probe",actions:["suspend","cancel"],signal:"SIGTERM",scope:"tree"}]};
const watchdog=async(rows:readonly HostWardenRowV1[],options:Readonly<{footprint?:(pid:number)=>Promise<number|null>;mode?:"observe"|"enforce";available?:number}>={})=>{
 const base=fresh(),directory=join(base,"warden"),written:string[]=[],signals:string[]=[];
 const ports:HostWardenPortsV1={directory,now:Date.now,write:line=>written.push(line),mode:()=>options.mode??"enforce",sample:async()=>memoryCase(options.available??9000),table:async()=>rows,signal:(pid,name)=>{signals.push(`${name}:${pid}`);},sleep:async()=>undefined,...options.footprint===undefined?{}:{footprint:options.footprint}};
 const result=await runHostWardenLoopV1({ports,tables:watchdogTables,policy:{...quickPolicy,capMilliseconds:0},maxTicks:1,code:"fake",platform:"linux",workspaces:[]});
 const alarms=(()=>{try{return readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").filter(Boolean).map(line=>JSON.parse(line));}catch{return[];}})();
 return{result,signals,alarms,written,directory,base};
};
const fakeTable=(browserMegabytes:number,renderMegabytes:number):readonly HostWardenRowV1[]=>[rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),rowOf(11,10,browserMegabytes,"/fake/chromium/chrome --headless"),rowOf(12,11,renderMegabytes,"/fake/chromium/chrome --type=renderer"),rowOf(13,10,300,"/fake/probe --serve")];

test("watchdog cancels an overdue stopped tree whose page-in cost and guard floor exceed available memory",async()=>{
 const base=fresh(),directory=join(base,"warden"),signals:string[]=[];
 const tables:HostWardenTablesV1={roots:["^/fake/root"],never:[],kinds:[{name:"slice-worker",rank:60,pattern:"^/fake/slice",actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree",capMegabytes:6144,capCritical:true}]};
 const raw=[rowOf(10,1,50,"/fake/root"),{...rowOf(11,10,3800,"/fake/slice"),stopped:true},{...rowOf(12,11,500,"/fake/helper"),stopped:true}];
 const charge:Readonly<Record<number,number>>={11:4700,12:900};
 const darwin={footprint:(pid:number)=>(charge[pid]??50)*1048576};
 const linux=await chargeWardenRowsV1("linux",raw,{minimumKilobytes:0,read:async()=> "Pss: 1024 kB\nSwapPss: 4096 kB"});
 expect(linux[1]).toMatchObject({memoryKilobytes:5120,nonresidentKilobytes:4096});
 const rows=await chargeWardenRowsV1("darwin",raw,{darwin});
 expect(rows[1]).toMatchObject({memoryKilobytes:4700*1024,nonresidentKilobytes:900*1024});
 expect(rows[2]).toMatchObject({memoryKilobytes:900*1024,nonresidentKilobytes:400*1024});
 let now=Date.now();
 const ports:HostWardenPortsV1={directory,now:()=>now,write:()=>undefined,mode:()=>"enforce",sample:async()=>memoryCase(2863,{pressure:"warn",swapOutMegabytesPerSecond:3.75}),table:async()=>rows,signal:(pid,name)=>{signals.push(`${name}:${pid}`);},sleep:async()=>{now+=121000;}};
 await runHostWardenLoopV1({ports,tables,maxTicks:2,code:"fake",sources:async()=>"fake",platform:"linux",workspaces:[]});
 expect(signals).toEqual(["SIGKILL:11","SIGKILL:12"]);
 expect(readHostWardenCancelsBelowV1(10,now-242000,base,now)).toMatchObject([{reason:"overdue",megabytes:5600}]);
});

test("watchdog: a process class over its declared cap is killed with its tree, recorded and reported, without any memory pressure",async()=>{
 const run=await watchdog(fakeTable(500,700));
 expect(run.signals).toEqual(["SIGKILL:11","SIGKILL:12"]);
 expect(run.alarms).toHaveLength(1);
 expect(run.alarms[0]).toMatchObject({action:"cancel",reason:"cap",kind:"browser",pid:11,capMegabytes:1000,megabytes:1200,observed:false,level:"calm"});
 expect(run.alarms[0].text.en).toContain("its memory is over the cap of 1000 MB for browser");
 expect(run.alarms[0].text.de).toContain("Obergrenze von 1000 MB");
 expect(run.result.counters.cancelled).toBe(1);
 const records=readdirSync(join(run.directory,"cancelled"));
 expect(records).toHaveLength(1);
 expect(requireHostWardenV1("Cancel",JSON.parse(readFileSync(join(run.directory,"cancelled",records[0]!),"utf8")))).toMatchObject({pid:11,kind:"browser",reason:"cap",megabytes:1200});
 expect(run.written.some(line=>line.includes("over the cap of 1000 MB"))&&run.written.some(line=>line.includes("Obergrenze von 1000 MB"))).toBe(true);
 expect(readHostWardenStateV1(run.base)?.counters.cancelled).toBe(1);
});

test("watchdog: below the cap nothing is signalled, and a kind without a cap is never capped however large",async()=>{
 const small=await watchdog(fakeTable(500,400));
 expect(small.signals).toEqual([]);expect(small.alarms).toEqual([]);
 const large=await watchdog([...fakeTable(500,400),rowOf(14,10,9000,"/fake/probe --huge")]);
 expect(large.signals,"the probe kind declares no cap").toEqual([]);
});

test("watchdog: the footprint (what the OS compressed or swapped out of the tree) counts, not only the resident set",async()=>{
 const probed:number[]=[];
 const compressed=await watchdog(fakeTable(100,100),{footprint:async pid=>{probed.push(pid);return pid===11?5000:null;}});
 expect(probed).toEqual([11]);
 expect(compressed.signals).toEqual(["SIGKILL:11","SIGKILL:12"]);
 expect(compressed.alarms[0]).toMatchObject({reason:"cap",megabytes:5000,capMegabytes:1000});
 const lower=await watchdog(fakeTable(500,700),{footprint:async()=>100});
 expect(lower.alarms[0],"a footprint below the resident set never lowers the measure").toMatchObject({reason:"cap",megabytes:1200});
 const failing=await watchdog(fakeTable(500,700),{footprint:async()=>{throw Error("footprint refused");}});
 expect(failing.alarms[0],"a failing footprint falls back to the resident set").toMatchObject({reason:"cap",megabytes:1200});
});

test("watchdog: observe mode reports the cap breach once and signals nothing",async()=>{
 const run=await watchdog(fakeTable(500,700),{mode:"observe"});
 expect(run.signals).toEqual([]);
 expect(run.alarms).toHaveLength(1);
 expect(run.alarms[0]).toMatchObject({reason:"cap",observed:true});
 expect(readdirSync(join(run.directory,"cancelled"))).toEqual([]);
});

test("the state names the trigger and the largest consumers the warden does not own, and never signals them",async()=>{
 const rows=[...fakeTable(100,100),rowOf(70,1,2000,"/Applications/Hog.app/Hog"),rowOf(71,70,1000,"/Applications/Hog.app/Helper")];
 const run=await watchdog(rows,{available:2300});
 const state=readHostWardenStateV1(run.base)!;
 expect(isHostWardenV1("State",state)).toBe(true);
 expect(state.level).toBe("calm");expect(state.trigger).toBeNull();expect(state.others).toEqual([]);
 const tight=await watchdog(rows,{available:1000});
 const held=readHostWardenStateV1(tight.base)!;
 expect(held).toMatchObject({level:"guard",hold:true,trigger:"available"});
 expect(held.others![0]).toEqual({pid:70,megabytes:3000,processes:2,command:"Hog"});
 expect(tight.signals.every(entry=>!entry.endsWith(":70")&&!entry.endsWith(":71")),"foreign processes are never signalled").toBe(true);
});

test("watchdog: a build-critical kind over its cap is reported once, other work is suspended first and it is killed only at danger",async()=>{
 const criticalTables:HostWardenTablesV1={roots:["^/fake/root"],never:[],kinds:[{name:"slice",rank:60,pattern:"^/fake/slice",actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree",capMegabytes:1000,capCritical:true},{name:"other",rank:45,pattern:"^/fake/other",actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"},{name:"other2",rank:46,pattern:"^/fake/second",actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree"}]};
 const rows=[rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),rowOf(21,10,1800,"/fake/slice worker"),rowOf(22,10,600,"/fake/other job"),rowOf(23,10,200,"/fake/second job")];
 const run=async(available:number,ticks:number)=>{
  const base=fresh(),directory=join(base,"warden"),signals:string[]=[];
  const ports:HostWardenPortsV1={directory,now:Date.now,write:()=>undefined,mode:()=>"enforce",sample:async()=>memoryCase(available),table:async()=>rows,signal:(pid,name)=>{signals.push(`${name}:${pid}`);},sleep:async()=>undefined};
  await runHostWardenLoopV1({ports,tables:criticalTables,policy:{...quickPolicy,capMilliseconds:0,keepRunning:0},maxTicks:ticks,code:"fake",platform:"linux",workspaces:[]});
  const alarms=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").filter(Boolean).map(line=>JSON.parse(line));
  return{signals,alarms,state:readHostWardenStateV1(base)!};
 };
 const calm=await run(9000,3);
 expect(calm.alarms.filter(alarm=>alarm.action==="report"),"reported once over three ticks").toHaveLength(1);
 expect(calm.alarms.find(alarm=>alarm.action==="report")).toMatchObject({reason:"cap",kind:"slice",capMegabytes:1000,megabytes:1800});
 expect(calm.alarms.find(alarm=>alarm.action==="report").text.en).toContain("crossed the cap of 1000 MB for slice");
 expect(calm.alarms.find(alarm=>alarm.action==="report").text.de).toContain("überschritten");
 expect(calm.signals.some(entry=>entry.startsWith("SIGKILL")),"nothing is killed at a calm host").toBe(false);
 expect(calm.signals.includes("SIGSTOP:22"),"the non-critical work is suspended first").toBe(true);
 expect(calm.signals.some(entry=>entry.endsWith(":21")&&entry.startsWith("SIGSTOP")),"the critical subject keeps running").toBe(false);
 expect(calm.state).toMatchObject({level:"guard",trigger:"cap",hold:true});
 const danger=await run(300,1);
 expect(danger.alarms.some(alarm=>alarm.action==="cancel"&&alarm.reason==="cap"&&alarm.kind==="slice"),"at danger the crossing is killed").toBe(true);
});

const unitTables:HostWardenTablesV1={roots:["^/fake/root"],never:[],kinds:[{name:"unit",rank:20,pattern:"^/fake/rustc",actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree",capMegabytes:10240,capCritical:true,ceiling:"unit"}]};
const ceilingRun=async(rows:readonly HostWardenRowV1[],options:Readonly<{mode?:"observe"|"enforce";available?:number;pressure?:"normal"|"warn"|"critical";ticks?:number}>={})=>{
 const base=fresh(),directory=join(base,"warden"),written:string[]=[],signals:string[]=[];
 const ports:HostWardenPortsV1={directory,now:Date.now,write:line=>written.push(line),mode:()=>options.mode??"enforce",sample:async()=>memoryCase(options.available??9000,{pressure:options.pressure??"normal"}),table:async()=>rows,signal:(pid,name)=>{signals.push(`${name}:${pid}`);},sleep:async()=>undefined};
 const result=await runHostWardenLoopV1({ports,tables:unitTables,policy:{...quickPolicy,capMilliseconds:0},maxTicks:options.ticks??1,code:"fake",platform:"linux",workspaces:[]});
 const alarms=(()=>{try{return readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").filter(Boolean).map(line=>JSON.parse(line));}catch{return[];}})();
 return{result,signals,alarms,written,directory,base};
};
const unitRows=(unitMegabytes:number,crate="semio_plain",target="wasm32-wasip2",types="lib"):readonly HostWardenRowV1[]=>[rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),rowOf(11,10,unitMegabytes,`/fake/rustc --crate-name ${crate} --edition=2021 /w/lib.rs --crate-type ${types} --target ${target}`),rowOf(12,11,100,"/fake/linker --version")];

test("ceiling: an unlisted unit above its budget is only reported on a calm host (tight, nothing killed) and cancelled when the host is critical, with a typed bilingual alarm that names the unit, its peak and its ceiling and tells to carve it",async()=>{
 const calm=await ceilingRun(unitRows(1700));
 expect(calm.signals,"the budget is a carving signal, not a kill rule").toEqual([]);
 expect(calm.alarms).toHaveLength(1);
 expect(calm.alarms[0]).toMatchObject({action:"report",reason:"ceiling",kind:"unit",pid:11,megabytes:1800,level:"guard",ceiling:{unit:"semio_plain|wasm32-wasip2",kind:"library",megabytes:1536,listed:false}});
 expect(isHostWardenV1("Alarm",calm.alarms[0])).toBe(true);
 expect(calm.alarms[0].text.en).toContain("[warden] reported unit pid 11");
 expect(readdirSync(join(calm.directory,"cancelled"))).toEqual([]);
 expect(calm.result.counters.cancelled).toBe(0);
 expect(readHostWardenStateV1(calm.base)).toMatchObject({level:"guard",trigger:"cap",hold:true});
 const repeated=await ceilingRun(unitRows(1700),{ticks:3});
 expect(repeated.alarms,"reported once however many ticks").toHaveLength(1);
 const run=await ceilingRun(unitRows(1700),{available:300});
 expect(run.signals).toEqual(["SIGKILL:11","SIGKILL:12"]);
 expect(run.alarms).toHaveLength(1);
 expect(run.alarms[0]).toMatchObject({action:"cancel",reason:"ceiling",kind:"unit",pid:11,megabytes:1800,level:"danger",ceiling:{unit:"semio_plain|wasm32-wasip2",kind:"library",megabytes:1536,listed:false}});
 expect(isHostWardenV1("Alarm",run.alarms[0])).toBe(true);
 expect(run.alarms[0].text.en).toContain("unit semio_plain|wasm32-wasip2 peaked at 1800 MB, above its library ceiling of 1536 MB");
 expect(run.alarms[0].text.en).toContain("not on the carving list: carve it");
 expect(run.alarms[0].text.de).toContain("Einheit semio_plain|wasm32-wasip2 erreichte 1800 MB und liegt über ihrer Obergrenze Bibliothek von 1536 MB");
 expect(run.alarms[0].text.de).toContain("schneiden");
 const records=readdirSync(join(run.directory,"cancelled"));
 expect(records).toHaveLength(1);
 const record=requireHostWardenV1<{ancestors:number[];ceiling:object;reason:string}>("Cancel",JSON.parse(readFileSync(join(run.directory,"cancelled",records[0]!),"utf8")));
 expect(record).toMatchObject({pid:11,reason:"ceiling",ancestors:[10,1],ceiling:{unit:"semio_plain|wasm32-wasip2",megabytes:1536,listed:false}});
 expect(run.result.counters.cancelled).toBe(1);
 expect((await ceilingRun(unitRows(1436))).signals,"1536 MB in total (the unit and its linker) is at the ceiling: nothing happens").toEqual([]);
});

test("ceiling: a carving-listed unit gets its declared budget: reported above it only, with the declared and the measured megabytes in the words",async()=>{
 const sourcing=(megabytes:number)=>unitRows(megabytes,"semio_hub_sourcing");
 expect((await ceilingRun(sourcing(3000))).signals,"3100 MB in total is within the declared ceiling").toEqual([]);
 const run=await ceilingRun(sourcing(3200));
 expect(run.alarms[0]).toMatchObject({action:"report",reason:"ceiling",ceiling:{unit:"semio_hub_sourcing|wasm32-wasip2",megabytes:3100,listed:true,measuredMegabytes:2824}});
 expect(run.alarms[0].text.en).toContain("peaked at 3300 MB, above its declared carving budget of 3100 MB (measured 2824 MB)");
 expect(run.alarms[0].text.de).toContain("erreichte 3300 MB und liegt über ihrem erklärten Carving-Budget von 3100 MB (gemessen 2824 MB)");
 const hub=(megabytes:number)=>unitRows(megabytes,"semio_hub_sourcing_component","wasm32-wasip2","cdylib");
 expect((await ceilingRun(hub(3996))).alarms,"4096 MB in total is within the default final ceiling").toEqual([]);
 expect((await ceilingRun(hub(3997))).alarms[0].ceiling).toEqual({unit:"semio_hub_sourcing_component|wasm32-wasip2",kind:"final",megabytes:4096,listed:false});
});

test("ceiling: observe mode reports the breach once and signals nothing; at critical the unit is cancelled before any cap or shedding",async()=>{
 const observed=await ceilingRun(unitRows(1700),{mode:"observe",ticks:3});
 expect(observed.signals).toEqual([]);
 expect(observed.alarms).toHaveLength(1);
 expect(observed.alarms[0]).toMatchObject({observed:true,reason:"ceiling"});
 const tight=await ceilingRun(unitRows(1700),{available:700,ticks:1});
 expect(tight.alarms.map(alarm=>[alarm.action,alarm.reason])).toEqual([["cancel","ceiling"]]);
});

test("build-script cancellation keeps the artifact directory beyond the stored command text",async()=>{
 const output="/work with spaces/target/prototype/build/beta-9a23",rows=unitRows(4500,"build_script_build","wasm32-wasip2","bin");
 const run=await ceilingRun(rows.map(row=>row.pid===11?{...row,command:row.command+" --cfg test".repeat(60)+` --out-dir "${output}" -C opt-level=0`}:row),{available:300});
 const record=readHostWardenCancelsBelowV1(10,Date.now()-60_000,run.base)[0];
 expect(record?.buildScriptOutput).toBe(output);
 expect(record?.command).toHaveLength(512);
 expect(record?.command).not.toContain(output);
});

test("ceiling: the typed failure of the cancelled build is found by the pid above the cancelled process, carries both languages and names every cancelled unit",async()=>{
 const run=await ceilingRun(unitRows(1700),{available:300});
 const since=Date.now()-60000;
 expect(readHostWardenCancelsBelowV1(10,since,run.base).map(record=>record.pid)).toEqual([11]);
 expect(readHostWardenCancelsBelowV1(1,since,run.base).map(record=>record.pid),"the whole ancestry counts").toEqual([11]);
 expect(readHostWardenCancelsBelowV1(99,since,run.base),"another build is not hit").toEqual([]);
 expect(readHostWardenCancelsBelowV1(10,Date.now()+1000,run.base),"a cancel before the build started is not its own").toEqual([]);
 expect(()=>requireNoHostWardenCancelBelowV1(99,since,run.base)).not.toThrow();
 let caught:unknown;try{requireNoHostWardenCancelBelowV1(10,since,run.base);}catch(error){caught=error;}
 expect(caught).toBeInstanceOf(HostWardenCancelledErrorV1);
 const failure=caught as HostWardenCancelledErrorV1;
 expect(failure).toMatchObject({code:"host-warden-cancelled",name:"HostWardenCancelledErrorV1"});
 expect(failure.cancels.map(record=>record.pid)).toEqual([11]);
 expect(failure.message).toContain("the host warden cancelled 1 of its processes");
 expect(failure.message).toContain("unit semio_plain|wasm32-wasip2 peaked at 1800 MB, above its library ceiling of 1536 MB");
 expect(failure.message).toContain("Nothing is retried");
 expect(failure.message).toContain("der Host-Wächter hat 1 seiner Prozesse abgebrochen");
 expect(failure.message).toContain("Es wird nichts wiederholt");
 const reasons=(record:Parameters<typeof formatHostWardenCancelReasonV1>[1])=>formatHostWardenCancelReasonV1("en",record);
 expect(reasons({reason:"cap",kind:"browser",megabytes:5000,capMegabytes:4096})).toBe("its memory is over the cap of 4096 MB for browser");
 expect(reasons({reason:"overdue",kind:"unit",megabytes:1})).toBe("suspended for too long");
 expect(formatHostWardenCancelReasonV1("de",{reason:"danger",kind:"unit",megabytes:1})).toBe("der Host hatte kritisch wenig Speicher");
 expect(formatHostWardenCancelFailureV1("en",[]).split("\n")).toHaveLength(2);
});

test("ceiling: a suspended unit is cancelled only at a critical host (below it, an overdue one is resumed) and the caller finds the typed failure",async()=>{
 const rows=unitRows(900).map(row=>row.pid===11?{...row,stopped:true}:row);
 const base=fresh(),directory=join(base,"warden"),written:string[]=[],signals:string[]=[];
 let now=1_000_000;
 const ports:HostWardenPortsV1={directory,now:()=>now,write:line=>written.push(line),mode:()=>"enforce",sample:async()=>memoryCase(300),table:async()=>rows.filter(row=>!signals.includes(`SIGKILL:${row.pid}`)),signal:(pid,name)=>{signals.push(`${name}:${pid}`);},sleep:async()=>{now+=61000;}};
 const result=await runHostWardenLoopV1({ports,tables:unitTables,policy:{...quickPolicy,suspendMaximumSeconds:120,keepRunning:0},maxTicks:4,code:hostWardenCodeDigestV1(),platform:"linux",workspaces:[]});
 expect(result.counters.cancelled).toBe(1);
 expect(signals.filter(signal=>signal.startsWith("SIGKILL:11")),"killed once").toHaveLength(1);
 const alarms=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(alarms.map(alarm=>`${alarm.action}.${alarm.reason}`)).toEqual(["cancel.danger"]);
 expect(alarms[0].text.en).toContain("available memory is below the danger line");
});

test("MOUNTED: a real unit over its ceiling is killed with its tree at a critical host, its record names the unit and its parent, the build owner finds it",async()=>{
 const tag="unit",child=Bun.spawn([process.execPath,"-e",`${hog(tag)}/* --crate-name warden_probe --crate-type lib --target wasm32-wasip2 */`],{stdout:"ignore",stderr:"ignore",stdin:"ignore"});children.push(child);
 await Bun.sleep(400);
 const tables:HostWardenTablesV1={roots:[`warden-test-${token}`],never:[],kinds:[{name:"unit",rank:20,pattern:`warden-test-${token}-${tag}`,actions:["suspend","cancel"],signal:"SIGKILL",scope:"tree",capMegabytes:10240,capCritical:true,ceiling:"unit"}]};
 const ceilings={workerClass:"tiny",classes:{tiny:{hostMegabytes:4096,libraryMegabytes:20,finalMegabytes:30,finalBackend:"serial"}},carving:[],families:[]};
 const base=fresh(),directory=join(base,"warden"),written:string[]=[],begun=Date.now();
 const ports:HostWardenPortsV1={directory,now:Date.now,write:line=>written.push(line),mode:()=>"enforce",sample:async()=>host(300),table:realTable,signal:(pid,name)=>{if(!children.some(entry=>entry.pid===pid))throw Error(`the test refuses to signal pid ${pid}`);process.kill(pid,name);},sleep:async()=>{await Bun.sleep(60);}};
 const result=await runHostWardenLoopV1({ports,policy:quickPolicy,tables,ceilings,within:process.pid,maxTicks:4,code:"mounted",platform:process.platform});
 await Bun.sleep(150);
 expect(result.counters.cancelled).toBe(1);
 expect(stateOf(child.pid),"the killed child is gone").toBe("");
 expect(written.some(line=>line.includes("unit warden_probe|wasm32-wasip2 peaked at")&&line.includes("above its library ceiling of 20 MB"))).toBe(true);
 const cancels=readHostWardenCancelsBelowV1(process.pid,begun,base);
 expect(cancels.map(record=>[record.pid,record.reason])).toEqual([[child.pid,"ceiling"]]);
 expect(cancels[0]!.ancestors).toContain(process.pid);
 expect(()=>requireNoHostWardenCancelBelowV1(process.pid,begun,base)).toThrow(HostWardenCancelledErrorV1);
},30000);

test("damping: the loop suspends one subject at most flapMaximumSuspensions times within the window, whatever the host keeps saying",async()=>{
 const rows:readonly HostWardenRowV1[]=[rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),rowOf(11,10,300,"/fake/rustc --crate-name semio_a --crate-type lib"),rowOf(13,10,900,"/fake/rustc --crate-name semio_b --crate-type lib")];
 const base=fresh(),directory=join(base,"warden"),signals:string[]=[];
 const ports:HostWardenPortsV1={directory,now:Date.now,write:()=>undefined,mode:()=>"enforce",sample:async()=>memoryCase(1500),table:async()=>rows,signal:(pid,name)=>{signals.push(`${name}:${pid}`);},sleep:async()=>undefined};
 await runHostWardenLoopV1({ports,tables:unitTables,policy:quickPolicy,maxTicks:7,code:"fake",platform:"linux",workspaces:[]});
 expect(signals.filter(signal=>signal==="SIGSTOP:13"),"the cheaper victim is suspended twice, then damped").toHaveLength(HOST_WARDEN_POLICY_V1.flapMaximumSuspensions);
 expect(signals.filter(signal=>signal==="SIGSTOP:11"),"the dearest keeps running").toHaveLength(0);
});

test("warn alone is no measured signal: the level holds admissions and nothing is suspended or cancelled, with 3921 MB available and a 3 GB final running",()=>{
 const subject:HostWardenSubjectV1={id:"p1",pid:1,kind:"local-unit",rank:20,elapsedSeconds:4400,megabytes:2983,suspended:false,suspendedSeconds:0,canSuspend:true,canCancel:true,capMegabytes:10240,capCritical:true};
 const warn=decideHostWardenV1(HOST_WARDEN_POLICY_V1,memoryCase(3921,{pressure:"warn",swapFreeMegabytes:4000,swapVolumeFreeMegabytes:60000}),[subject,{...subject,id:"p2",pid:2,elapsedSeconds:5,megabytes:300}],{level:"calm",calmTicks:0});
 expect(warn).toMatchObject({level:"guard",hold:true,trigger:"pressure",actions:[]});
 const measured=decideHostWardenV1(HOST_WARDEN_POLICY_V1,memoryCase(3921,{pressure:"warn",swapOutMegabytesPerSecond:40,swapFreeMegabytes:4000,swapVolumeFreeMegabytes:60000}),[subject,{...subject,id:"p2",pid:2,elapsedSeconds:5,megabytes:300}],{level:"calm",calmTicks:0});
 expect(measured.actions.map(action=>[action.action,action.id]),"the young unit goes first, the long running one is the dearest and stays").toEqual([["suspend","p2"]]);
});

test("the memory a table row carries is what the kernel charges: every charge case equals the independent Python oracle and the stored expectation",async()=>{
 const charges=model("--charge");
 expect(fixture.charges.length).toBeGreaterThanOrEqual(5);
 for(const [index,row] of fixture.charges.entries()){
  const full=mutable<HostWardenRowV1[]>(row.rows.map(item=>({...item,parent:1,group:item.pid,stopped:false,elapsedSeconds:60,start:"s",command:"c"})));
  const darwin={footprint:(pid:number)=>(row.darwinBytes as Record<string,number|null>)[String(pid)]??null},read=async(path:string)=>(row.smaps as Record<string,string>)[path.split("/")[2]!]??null;
  const found=await chargeWardenRowsV1(row.platform as NodeJS.Platform,full,{darwin,read,minimumKilobytes:row.minimumKilobytes});
  expect(found.map(item=>item.memoryKilobytes),row.name).toEqual(charges[index]);
  expect(charges[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
  expect(found.every(item=>isHostWardenV1("ProcessRow",item)),row.name).toBe(true);
 }
});

test("a browser tree that holds 3 GB but is resident with 5 GB stays under its 4096 MB cap once the table carries the footprint",async()=>{
 const base="/Users/u/ws/.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright/chromium-1248/chrome-mac-arm64/Chromium.app/Contents";
 const row=(pid:number,parent:number,megabytes:number,command:string):HostWardenRowV1=>({pid,parent,group:pid,memoryKilobytes:megabytes*1024,stopped:false,elapsedSeconds:600,start:"Tue Nov 24 06:"+pid,command});
 const rows=[row(1,0,1,"/sbin/launchd"),row(21,1,3,"/bin/zsh"),row(40,21,300,base+"/MacOS/Chromium --headless --remote-debugging-pipe"),row(41,40,2900,base+"/Frameworks/Chromium Framework.framework/Helpers/Chromium Helper (Renderer).app/Contents/MacOS/Chromium Helper (Renderer) --type=renderer"),row(42,40,1850,base+"/Frameworks/Chromium Framework.framework/Helpers/Chromium Helper (GPU).app/Contents/MacOS/Chromium Helper (GPU) --type=gpu-process")];
 const options={workspaces:["/Users/u/ws"],within:21},before=classifyWardenSubjectsV1(rows,HOST_WARDEN_TABLES_V1,options).find(subject=>subject.kind==="browser");
 expect(before?.megabytes,"resident set").toBe(5050);expect(before!.megabytes).toBeGreaterThan(before!.capMegabytes!);
 const megabytes:Record<number,number>={40:250,41:1800,42:1000},charged=await chargeWardenRowsV1("darwin",rows,{darwin:{footprint:pid=>megabytes[pid]===undefined?null:megabytes[pid]!*1048576}});
 const after=classifyWardenSubjectsV1(charged,HOST_WARDEN_TABLES_V1,options).find(subject=>subject.kind==="browser");
 expect(after?.megabytes,"footprint").toBe(3050);expect(after!.megabytes).toBeLessThan(after!.capMegabytes!);expect(after!.capMegabytes).toBe(4096);
});

test("on macOS the real table carries what the kernel charges this process, read natively and equal to the footprint tool",async()=>{
 if(process.platform!=="darwin")return;
 const child=Bun.spawn(["sleep","30"]);children.push(child);
 const rows=await chargeWardenRowsV1("darwin",(await realTable()).filter(row=>row.pid===child.pid||row.pid===process.pid));
 const tool=Bun.spawnSync(["footprint","-f","bytes","--noCategories","-p",String(child.pid)]),match=/phys_footprint:\s+(\d+)\s+B/u.exec(decoder.decode(tool.stdout)),mine=rows.find(row=>row.pid===child.pid)!;
 expect(match).not.toBeNull();expect(mine.memoryKilobytes).toBe(Math.floor(Number(match![1])/1024));
 expect(rows.find(row=>row.pid===process.pid)!.memoryKilobytes).toBeGreaterThan(1000);
});

test("the CPU time column: macOS and Linux forms parse to seconds like an independent Python reading, a row without the column keeps none",()=>{
 const texts=["339:28.28","0:00.01","1:02:03.50","00:00:03","1-02:03:04","12-00:00:00"],expected=JSON.parse(decoder.decode(Bun.spawnSync([python,"-I","-c","import json,re,sys\nout=[]\nfor t in sys.argv[1:]:\n    d=0\n    if '-' in t:\n        d,t=t.split('-');d=int(d)\n    p=t.split(':')\n    if len(p)==2:p=[0]+p\n    out.append(d*86400+int(p[0])*3600+int(p[1])*60+float(p[2]))\nprint(json.dumps(out))",...texts]).stdout)) as number[];
 expect(texts.map(parseWardenCpuV1)).toEqual(expected);
 expect(parseWardenCpuV1("garbage")).toBe(0);
 const rows=parseWardenProcessTableV1(["    1     0     1   7248 Ss   01-02:10:49  12:51.90 Wed Nov 25 01:16:30 2026 /sbin/launchd","  102     1   102   3504 Ss   01-02:10:38 339:28.28 Wed Nov 25 01:16:41 2026 /usr/sbin/syslogd -x","  200     1   200   3504 Ss   10:38 Wed Nov 25 01:16:41 2026 /no/time/column"].join("\n"));
 expect(rows.map(row=>[row.pid,row.cpuSeconds,row.elapsedSeconds,row.command])).toEqual([[1,12*60+51.9,((24+2)*60+10)*60+49,"/sbin/launchd"],[102,339*60+28.28,((24+2)*60+10)*60+38,"/usr/sbin/syslogd -x"],[200,undefined,638,"/no/time/column"]]);
 expect(HOST_WARDEN_TABLE_V1.quickArguments.join(" ")).toContain("etime=,time=,lstart=");
 for(const row of rows)expect(isHostWardenV1("ProcessRow",row)).toBe(true);
});

test("the loop adds the CPU a classified process gained between two ticks to a cumulative total: the first tick is a baseline, an exited child keeps its share, a young newcomer counts whole, an old newcomer does not",async()=>{
 const process_=(pid:number,parent:number,command:string,cpu:number,elapsed:number):HostWardenRowV1=>({...rowOf(pid,parent,10,command),cpuSeconds:cpu,elapsedSeconds:elapsed});
 const ticks:(readonly HostWardenRowV1[])[]=[
  [rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),process_(11,10,"/fake/chromium/chrome --headless",50,9000),process_(12,11,"/fake/chromium/chrome --type=renderer",20,9000)],
  [rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),process_(11,10,"/fake/chromium/chrome --headless",52,9002),process_(12,11,"/fake/chromium/chrome --type=renderer",21,9002),process_(13,11,"/fake/chromium/chrome --type=gpu",3,1)],
  [rowOf(1,0,1,"/sbin/launchd"),rowOf(10,1,50,"/fake/root run"),process_(11,10,"/fake/chromium/chrome --headless",55,9004),process_(13,11,"/fake/chromium/chrome --type=gpu",4,3),process_(14,11,"/fake/chromium/chrome --type=utility",900,3600)],
 ];
 const base=fresh(),directory=join(base,"warden"),totals:(number|undefined)[]=[];let tick=0;
 const ports:HostWardenPortsV1={directory,now:Date.now,write:()=>undefined,mode:()=>"observe",sample:async()=>memoryCase(9000),table:async()=>ticks[Math.min(tick,ticks.length-1)]!,signal:()=>undefined,sleep:async()=>{totals.push((JSON.parse(readFileSync(join(directory,"state.json"),"utf8")) as {subjects:{cpuSeconds?:number}}).subjects.cpuSeconds);tick++;}};
 await runHostWardenLoopV1({ports,tables:watchdogTables,policy:{...quickPolicy,capMilliseconds:0},maxTicks:3,code:"cpu",platform:"linux",workspaces:[]});
 expect(totals).toEqual([0,6,10]);
 const none=await watchdog(fakeTable(500,200));
 expect((JSON.parse(readFileSync(join(none.directory,"state.json"),"utf8")) as {subjects:Record<string,unknown>}).subjects).not.toHaveProperty("cpuSeconds");
});
