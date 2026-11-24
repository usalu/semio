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
import {HOST_WARDEN_ANCHORS_V1,HOST_WARDEN_POLICY_V1,HOST_WARDEN_TABLES_V1,buildHostWardenAlarmV1,classifyWardenSubjectsV1,decideHostWardenV1,discoverWardenWorkspacesV1,expandWardenPatternV1,headroomHostWardenV1,parseWardenSwapV1,wardenBrowsersV1,formatHostWardenAlarmV1,isHostWardenV1,triggerHostWardenV1,parseWardenElapsedV1,parseWardenProcessTableV1,planWardenSignalsV1,requireHostWardenV1,wardenSubjectIdV1,type HostWardenHostV1,type HostWardenKindV1,type HostWardenPolicyV1,type HostWardenRowV1,type HostWardenSubjectV1,type HostWardenTablesV1} from "../🟦️.ts";
import {currentHostWardenMirrorV1,defaultHostWardenPortsV1,readWardenSwapV1,ensureHostWardenV1,hostWardenCodeDigestV1,readHostWardenCancelV1,readHostWardenCancelsSinceV1,readHostWardenMirrorV1,readHostWardenStateV1,runHostWardenLoopV1,wardenProcessStartV1,type HostWardenPortsV1} from "../🔁️loop/🟦️.ts";

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
 expect(kinds.map(kind=>kind.name)).toEqual(["remote-unit","local-unit","pool-child","farm-worker","warm-deriver","heavy-question","browser","test","test-binary","slice-worker","cargo","stage"]);
 expect(kinds.map(kind=>kind.rank)).toEqual([10,20,25,30,35,40,45,50,55,60,65,90]);
 expect(kinds.map(kind=>kind.rank),"kinds are listed by ascending cost").toEqual([...kinds.map(kind=>kind.rank)].sort((a,b)=>a-b));
 for(const kind of kinds){
  expect(existsSync(join(repoRoot,kind.source)),`${kind.name}: ${kind.source}`).toBe(true);
  if(!kind.pattern.startsWith("^"))expect(kind.source.includes(kind.pattern),`${kind.name} pattern is a fragment of its source`).toBe(true);
  expect(()=>new RegExp(kind.pattern,"u")).not.toThrow();
 }
 expect(kinds.find(kind=>kind.name==="stage")).toMatchObject({actions:["cancel"],signal:"SIGTERM",scope:"tree"});
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

test("every decision equals the independent Python oracle and the expectation stored in the fixture",()=>{
 const expected=model("--decide");
 expect(fixture.decisions.length).toBeGreaterThanOrEqual(24);
 fixture.decisions.forEach((row,index)=>{
  const policy={...HOST_WARDEN_POLICY_V1,...("policy" in row?row.policy:{})} as HostWardenPolicyV1;
  const decision=decideHostWardenV1(policy,mutable<HostWardenHostV1>(row.host),mutable<HostWardenSubjectV1[]>(row.subjects),mutable(row.previous));
  expect(JSON.parse(JSON.stringify(decision)),row.name).toEqual(expected[index]);
  expect(expected[index],`stored expectation of ${row.name}`).toEqual((row as {expect:unknown}).expect);
  expect(isHostWardenV1("Decision",decision),row.name).toBe(true);
 });
 const seen=new Set(fixture.decisions.flatMap(row=>(row as {expect:{actions:{action:string;reason:string}[]}}).expect.actions.map(action=>`${action.action}.${action.reason}`)));
 for(const key of["suspend.guard","suspend.danger","cancel.danger","cancel.overdue","resume.overdue","resume.calm"])expect(seen.has(key),key).toBe(true);
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
 expect(mac[0]).toEqual({pid:1,parent:0,group:1,rssKilobytes:12144,stopped:false,elapsedSeconds:((10*24+9)*60+4)*60+35,start:"Fri Nov 13 17:42:44 2026",command:"/sbin/launchd"});
 expect(mac[2]).toMatchObject({pid:9362,parent:1,group:9362,rssKilobytes:16656,elapsedSeconds:((24+5)*60+11)*60+39,start:"Sun Nov 22 21:35:40 2026"});
 expect(mac[2]!.command).toContain("/.cache/b/t/m/bun/");
 expect(mac[3]).toMatchObject({pid:26616,parent:26264,group:97578,rssKilobytes:679024,elapsedSeconds:36});
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
 const row=(pid:number,parent:number,command:string,group=pid):HostWardenRowV1=>({pid,parent,group,rssKilobytes:1,stopped:false,elapsedSeconds:1,start:"s",command});
 const rows=[row(1,0,"init"),row(10,1,"top"),row(11,10,"a"),row(12,11,"b"),row(13,10,"/x/🛰️sidecar/🟦️.ts agent"),row(14,10,"c")];
 expect(planWardenSignalsV1(rows,{pid:10,scope:"tree"})).toEqual([10,11,14,12]);
 expect(planWardenSignalsV1(rows,{pid:10,scope:"tree"},HOST_WARDEN_TABLES_V1,new Set([14]))).toEqual([10,11,12]);
 expect(planWardenSignalsV1(rows,{pid:11,scope:"process"})).toEqual([11]);
 expect(planWardenSignalsV1([row(20,1,"g",7),row(21,20,"h",7),row(22,1,"o",8)],{pid:20,scope:"group"})).toEqual([20,21]);
 expect(planWardenSignalsV1(rows,{pid:999,scope:"tree"})).toEqual([]);
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
 const base=fresh(),directory=join(base,"warden"),written:string[]=[],a=startChild("a"),b=startChild("b"),c=startChild("c"),gone=(state:string)=>state===""||state.startsWith("Z");
 await Bun.sleep(700);
 const states:Record<number,Record<string,string>>={},second:{reason?:string}={};
 const phases=[host(3000),host(1000),host(3000),host(3000),host(3000),host(3000,"critical"),host(3000)];
 const ports=scripted(directory,phases,written,async tick=>{
  states[tick]={a:stateOf(a.pid),b:stateOf(b.pid),c:stateOf(c.pid)};
  if(tick===2)Object.assign(second,await runHostWardenLoopV1({ports:{...scripted(directory,[host(3000)],[]),directory},policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:1}));
 });
 const result=await runHostWardenLoopV1({ports,policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:7,code:"mounted"});
 expect(result.reason).toBe("limit");expect(result.ticks).toBe(7);
 expect(states[1]!.a.startsWith("T"),"calm: nothing is stopped").toBe(false);
 expect(states[2]!.a.startsWith("T"),"guard: the cheapest subject is really stopped (state T)").toBe(true);
 expect(states[2]!.c.startsWith("T"),"the most expensive subject keeps running").toBe(false);
 expect(states[4]!.a.startsWith("T"),"still stopped before the third calm tick").toBe(true);
 expect(states[5]!.a.startsWith("T"),"resumed on the third calm tick").toBe(false);
 expect(states[5]!.a,"it is alive again").not.toBe("");
 expect(gone(states[6]!.a),"danger: the cheapest subject is really killed").toBe(true);
 expect(states[6]!.b.startsWith("T"),"danger: the next one is suspended").toBe(true);
 expect(states[7]!.c.startsWith("T")).toBe(false);
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
 expect(lines.map(line=>`${line.action}.${line.kind}.${line.reason}`)).toEqual(["suspend.cheap.guard","resume.cheap.calm","cancel.cheap.danger","suspend.middle.danger"]);
 for(const line of lines){expect(requireHostWardenV1("Alarm",line)).toBeTruthy();expect(line.observed).toBe(false);expect(line.text.en).toContain(`pid ${line.pid}`);expect(line.text.de).toContain(`pid ${line.pid}`);expect(line.megabytes).toBeGreaterThan(3);}
 expect(written.filter(line=>line.startsWith("[warden] suspended cheap")).length).toBe(1);
 expect(written.filter(line=>line.startsWith("[Wächter] angehalten cheap")).length).toBe(1);
 expect(written.some(line=>line.startsWith("[warden] started"))&&written.some(line=>line.startsWith("[Wächter] gestartet"))).toBe(true);
 const state=JSON.parse(readFileSync(join(directory,"state.json"),"utf8"));
 expect(requireHostWardenV1("State",state)).toMatchObject({pid:process.pid,mode:"enforce",code:"mounted",level:"calm",counters:{suspended:2,resumed:1,cancelled:1,ticks:7},suspended:[]});
 expect(state.subjects.byKind.dear.count).toBe(1);
 expect(state.lastAlarms).toHaveLength(4);
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
 const directory=fresh(),written:string[]=[],f=startChild("a"),g=startChild("b"),h=startChild("c");
 await Bun.sleep(600);
 const phases=[host(300,"critical"),host(300,"critical"),host(300,"critical")];
 const result=await runHostWardenLoopV1({ports:scripted(directory,phases,written,async()=>undefined,()=>"observe"),policy:quickPolicy,tables:testTables,within:process.pid,maxTicks:3,code:"observe"});
 expect(result.counters).toEqual({suspended:0,resumed:0,cancelled:0});
 for(const child of[f,g,h]){expect(stateOf(child.pid).startsWith("T")).toBe(false);expect(stateOf(child.pid)).not.toBe("");}
 const lines=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(lines.map(line=>`${line.action}.${line.kind}`),"three identical ticks announce each decision once").toEqual(["cancel.cheap","cancel.middle"]);
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
 const record={version:1,pid:4242,start:"s",kind:"cheap",owner:"o",command:"rustc /x/work/t1/r/a.rs",megabytes:5,atMs:Date.now()-1000,reason:"danger"};
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
 const record={version:1,pid:7001,start:"s",kind:"remote-unit",owner:"o",command:"rustc /w/work/task-a/r/x.rs",megabytes:900,atMs:now-5000,reason:"danger"};
 writeFileSync(join(cancelled,"7001.aaaaaaaa.json"),JSON.stringify(record));
 writeFileSync(join(cancelled,"7002.bbbbbbbb.json"),JSON.stringify({...record,pid:7002,command:"rustc /w/work/task-b/r/y.rs",atMs:now-9000}));
 writeFileSync(join(cancelled,"7003.cccccccc.json"),JSON.stringify({...record,pid:7003,atMs:now-1000,command:undefined}));
 expect(requireHostWardenV1("Cancel",record)).toMatchObject({command:"rustc /w/work/task-a/r/x.rs"});
 expect(isHostWardenV1("Cancel",{...record,command:"x".repeat(513)})).toBe(false);
 expect(readHostWardenCancelsSinceV1(now-6000,directory,now).map(entry=>entry.pid)).toEqual([7001]);
 expect(readHostWardenCancelsSinceV1(now-20000,directory,now).map(entry=>entry.pid)).toEqual([7001,7002]);
 expect(readHostWardenCancelsSinceV1(now,directory,now)).toEqual([]);
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
 expect(mine!.rssKilobytes).toBeGreaterThan(1000);
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
 const ports=scripted(directory,[host(1000),host(1000),host(1000)],written,async tick=>{states[tick]={unit:stateOf(unit.pid),binary:stateOf(binary.pid),foreign:stateOf(foreign.pid)};});
 const result=await runHostWardenLoopV1({ports,policy:quickPolicy,tables:{...HOST_WARDEN_TABLES_V1,roots:[]},workspaces:[workspace],within:process.pid,maxTicks:3,code:"identity"});
 expect(result.reason).toBe("limit");
 expect(states[2]!.unit.startsWith("T"),"the pinned rustc is stopped although no root is among its ancestors").toBe(true);
 expect(states[2]!.binary.startsWith("T"),"the most expensive subject keeps running").toBe(false);
 expect(states[2]!.foreign.startsWith("T"),"a rustc outside the pinned toolchain path is never touched").toBe(false);
 const state=JSON.parse(readFileSync(join(directory,"state.json"),"utf8"));
 expect(requireHostWardenV1("State",state).subjects.byKind["local-unit"].count).toBe(1);
 expect(state.subjects.byKind["test-binary"].count).toBe(1);
 expect(Object.keys(state.subjects.byKind).sort()).toEqual(["local-unit","test-binary"]);
 const lines=readFileSync(join(directory,"alarms.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));
 expect(lines.map(line=>`${line.action}.${line.kind}`)).toEqual(["suspend.local-unit"]);
 expect(lines[0].owner.startsWith("rustc"),"a subject outside every root is its own owner").toBe(true);
 expect(stateOf(unit.pid).startsWith("T"),"leaving resumes what this run suspended").toBe(false);
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
