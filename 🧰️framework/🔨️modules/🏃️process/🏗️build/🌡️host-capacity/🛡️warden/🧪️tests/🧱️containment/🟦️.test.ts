import {expect,test} from "bun:test";
import {mkdtempSync,readFileSync,writeFileSync,mkdirSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {decideHostWardenV1,foreignWardenHoldersV1,grantedWardenSubjectsV1,HOST_WARDEN_POLICY_V1,type HostWardenHostV1,type HostWardenPolicyV1,type HostWardenRowV1,type HostWardenSubjectV1} from "../../🟦️.ts";
import {foreignHostMegabytesV1,readHostWardenGrantsV1,readHostWardenStateV1,runHostWardenLoopV1,spawnHostWardenSuccessorV1,type HostWardenPortsV1} from "../../🔁️loop/🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";

/** 🧱️ The containment of the whole machine: grants enforced whatever their process runs, memory outside the grants counted, a guard that never has a gap, and the swap exhaustion of the 2026-11-27 incident.
 * @see ../🐍️.py (the independent oracle: `--granted`, `--foreign`, `--decide`) */
const oracle=(mode:string):unknown=>{
 const done=Bun.spawnSync([testPythonV1()??Bun.which("python3")??"python3","-I",fileURLToPath(new URL("../🐍️.py",import.meta.url)),mode]);
 expect(done.exitCode).toBe(0);
 return JSON.parse(done.stdout.toString());
};

test("granted subjects and the foreign holders equal the independent oracle and the stored expectation",()=>{
 const granted=oracle("--granted") as unknown[],foreign=oracle("--foreign") as unknown[];
 fixture.granted.forEach((row,index)=>{
  const result=grantedWardenSubjectsV1(row.rows as unknown as HostWardenRowV1[],row.subjects as unknown as HostWardenSubjectV1[],row.grants,row.nowMs,undefined,new Map(Object.entries((row as {suspendedSince?:Record<string,number>}).suspendedSince??{})));
  expect(JSON.parse(JSON.stringify(result)),row.name).toEqual(granted[index]);
  expect(granted[index],row.name).toEqual((row as {expect:unknown}).expect);
 });
 fixture.foreign.forEach((row,index)=>{
  const result=foreignWardenHoldersV1(row.rows as unknown as HostWardenRowV1[],row.subjects as never,row.heavyMegabytes,new Set(row.exclude));
  expect(result,row.name).toEqual(foreign[index] as typeof result);
 });
 const [first]=fixture.granted;
 const result=grantedWardenSubjectsV1(first!.rows as unknown as HostWardenRowV1[],first!.subjects as unknown as HostWardenSubjectV1[],first!.grants,first!.nowMs);
 expect(result.map(subject=>[subject.pid,subject.kind,subject.grantMegabytes]),"dead and reused pids book nothing").toEqual([[70,"cargo",2000],[60,"granted",3600]]);
 expect(result.find(subject=>subject.pid===60)!.megabytes,"the whole tree of the grant is the subject").toBe(4150);
});

test("the swap that is nearly used up is a danger by itself (17:30 incident), a mostly empty or ample swap is not",()=>{
 const base={totalMegabytes:16384,availableMegabytes:3000,swapUsedMegabytes:3060,swapOutMegabytesPerSecond:11,compressedMegabytes:4500,pressure:"normal",swapFreeMegabytes:160,swapVolumeFreeMegabytes:35000,swapFileMegabytes:1024} as HostWardenHostV1;
 const level=(host:HostWardenHostV1)=>decideHostWardenV1(HOST_WARDEN_POLICY_V1,host,[],{level:"calm",calmTicks:0});
 expect(level(base)).toMatchObject({level:"danger",trigger:"swap-free",hold:true});
 expect(level({...base,swapFreeMegabytes:900})).toMatchObject({level:"guard",trigger:"swap-free"});
 expect(level({...base,swapFreeMegabytes:1670,swapUsedMegabytes:1400}).level,"the resting state of this machine: 1400 of 3072 MB used").toBe("calm");
 expect(level({...base,swapUsedMegabytes:4,swapFreeMegabytes:16}).level).toBe("calm");
 expect(level({...base,swapFreeMegabytes:null,swapUsedMegabytes:null}).level).toBe("calm");
 const compressed=(megabytes:number,live:Partial<HostWardenHostV1>={pressure:"warn"})=>level({...base,swapFreeMegabytes:2000,swapUsedMegabytes:1000,compressedMegabytes:megabytes,...live}).level;
 expect([compressed(4800),compressed(4950),compressed(5900),compressed(8260)],"with kernel pressure warn: 4.8 GB guard (the pressure alone), 4.9 GB guard, 5.9 GB danger, 8.26 GB (the incident peak) danger").toEqual(["guard","guard","danger","danger"]);
 expect([compressed(4800,{}),compressed(8260,{}),compressed(11000,{})],"sticky compressed memory with available memory, no swap-out and normal pressure (18:40-19:10) is calm").toEqual(["calm","calm","calm"]);
 expect(compressed(8260,{swapOutMegabytesPerSecond:300})).toBe("danger");
});

test("a subject far above its grant is suspended before danger once the host has no spare memory, and is only counted while memory is spare",()=>{
 const host={totalMegabytes:16384,availableMegabytes:2300,swapUsedMegabytes:500,swapOutMegabytesPerSecond:0,compressedMegabytes:1000,pressure:"normal",swapFreeMegabytes:2500,swapVolumeFreeMegabytes:35000} as HostWardenHostV1;
 const subject={id:"g",pid:5,kind:"granted",rank:70,elapsedSeconds:300,megabytes:11600,suspended:false,suspendedSeconds:0,canSuspend:true,canCancel:true,lastResort:true,grantMegabytes:3600} as HostWardenSubjectV1;
 const decide=(sample:HostWardenHostV1,item=subject)=>decideHostWardenV1(HOST_WARDEN_POLICY_V1,sample,[item],{level:"calm",calmTicks:3});
 expect(decide(host)).toMatchObject({level:"guard",trigger:"grant",actions:[{action:"suspend",id:"g",reason:"guard"}]});
 expect(decide({...host,availableMegabytes:9000}).actions,"spare memory: admission already counts what it holds").toEqual([]);
 expect(decide(host,{...subject,megabytes:4000}).actions,"inside the tolerance").toEqual([]);
 expect(decide({...host,availableMegabytes:600},{...subject,suspended:true,suspendedSeconds:HOST_WARDEN_POLICY_V1.pauseDeadlineSeconds}).actions).toContainEqual({action:"cancel",id:"g",reason:"danger"});
});

const rows=(extra:readonly HostWardenRowV1[]):readonly HostWardenRowV1[]=>[{pid:1,parent:0,group:1,memoryKilobytes:1000,stopped:false,elapsedSeconds:9999,start:"S1",command:"/sbin/launchd"},...extra];
const row=(pid:number,parent:number,megabytes:number,command:string,stopped=false):HostWardenRowV1=>({pid,parent,group:pid,memoryKilobytes:megabytes*1024,stopped,elapsedSeconds:600,start:`S${pid}`,command});
const calm:HostWardenHostV1={totalMegabytes:16384,availableMegabytes:9000,swapUsedMegabytes:500,swapOutMegabytesPerSecond:0,compressedMegabytes:1000,pressure:"normal",swapFreeMegabytes:2500,swapVolumeFreeMegabytes:35000};
const policy:HostWardenPolicyV1={...HOST_WARDEN_POLICY_V1,calmMilliseconds:1,tightMilliseconds:1};
const portsOf=(directory:string,table:()=>readonly HostWardenRowV1[],sample:()=>HostWardenHostV1,signals:string[],written:string[]):HostWardenPortsV1=>({directory,now:Date.now,write:line=>written.push(line),mode:()=>"enforce",sample:async()=>sample(),table:async()=>table(),signal:(pid,name)=>{signals.push(`${name}.${pid}`);},sleep:async()=>{await Bun.sleep(2);}});
const tables={roots:["^/never"],never:[],kinds:[]};

test("MOUNTED: a grant on a process no kind knows is throttled when it runs over its reservation and the host is tight; the state counts what runs outside the grants",async()=>{
 const base=mkdtempSync(join(tmpdir(),"warden-containment-")),directory=join(base,"warden"),signals:string[]=[],written:string[]=[];
 mkdirSync(directory,{recursive:true});mkdirSync(join(base,"grants"));
 writeFileSync(join(base,"grants","a.json"),JSON.stringify({id:"a",pid:60,reservedMegabytes:3600,operation:"command"}));
 writeFileSync(join(base,"grants","a.attached"),"not a grant");
 expect(readHostWardenGrantsV1(directory).map(grant=>grant.id)).toEqual(["a"]);
 const table=rows([row(60,1,50,"/usr/bin/runner"),row(61,60,11500,"/usr/bin/vitest-worker"),row(80,1,3000,"/Applications/Hog.app/Hog")]);
 const tight={...calm,availableMegabytes:2300};
 const result=await runHostWardenLoopV1({ports:portsOf(directory,()=>table,()=>tight,signals,written),policy,tables,maxTicks:1,code:"containment"});
 expect(result.counters.suspended).toBe(1);
 expect(signals.filter(line=>line.startsWith("SIGSTOP")),"the tree of the grant, parent first").toEqual(["SIGSTOP.60","SIGSTOP.61"]);
 expect(signals.some(line=>line.endsWith(".80")),"the foreign process is counted, never touched").toBe(false);
 const state=readHostWardenStateV1(base)!;
 expect(state.foreign).toEqual({megabytes:3000,processes:1,heavy:1});
 expect(state.grants).toEqual({count:1,megabytes:3600});
 expect(foreignHostMegabytesV1(base)).toBe(3000);
 expect(state.trigger).toBe("grant");
 expect(written.some(line=>line.includes("above its grant of 3600 MB"))&&written.some(line=>line.includes("über seiner Zusage")||line.includes("seiner Zusage von 3600")),"the reason is printed in both languages").toBe(true);
});

test("MOUNTED: the guard does not end for idleness while a grant is live or a foreign process holds a gigabyte, and ends once both are gone",async()=>{
 const base=mkdtempSync(join(tmpdir(),"warden-idle-")),directory=join(base,"warden");
 mkdirSync(directory,{recursive:true});mkdirSync(join(base,"grants"));
 const quick={...policy,idleMilliseconds:30};
 writeFileSync(join(base,"grants","a.json"),JSON.stringify({id:"a",pid:60,reservedMegabytes:3600}));
 const live=await runHostWardenLoopV1({ports:portsOf(directory,()=>rows([row(60,1,50,"/usr/bin/runner")]),()=>calm,[],[]),policy:quick,tables,maxTicks:40,code:"idle"});
 expect(live.reason,"a live grant keeps the guard").toBe("limit");
 const heavy=await runHostWardenLoopV1({ports:portsOf(join(base,"other"),()=>rows([row(80,1,1500,"/Applications/Hog.app/Hog")]),()=>calm,[],[]),policy:quick,tables,maxTicks:40,code:"idle"});
 expect(heavy.reason,"a heavy foreign process keeps the guard").toBe("limit");
 const none=await runHostWardenLoopV1({ports:portsOf(join(base,"third"),()=>rows([row(80,1,100,"/bin/small")]),()=>calm,[],[]),policy:quick,tables,maxTicks:200,code:"idle"});
 expect(none.reason).toBe("idle");
});

test("MOUNTED: a warden whose sources changed starts its successor before it ends, and a successor waits for the lock instead of giving up",async()=>{
 const base=mkdtempSync(join(tmpdir(),"warden-handover-")),directory=join(base,"warden");
 mkdirSync(directory,{recursive:true});
 let successors=0,digest="old";
 const ports=portsOf(directory,()=>rows([row(60,1,50,"/usr/bin/runner")]),()=>calm,[],[]);
 writeFileSync(join(base,"grants.keep"),"");mkdirSync(join(base,"grants"),{recursive:true});writeFileSync(join(base,"grants","a.json"),JSON.stringify({id:"a",pid:60,reservedMegabytes:100}));
 const old=await runHostWardenLoopV1({ports,policy:{...policy,codeCheckMilliseconds:1},tables,code:"old",sources:()=>{digest="new";return digest;},successor:()=>{successors++;expect(readFileSync(join(directory,"lock.json"),"utf8"),"the lock is still held when the successor starts").toContain(String(process.pid));}});
 expect(old.reason).toBe("code");expect(successors).toBe(1);
 const spawned:string[][]=[];
 spawnHostWardenSuccessorV1({directory:base,spawn:(_command,args)=>{spawned.push([...args]);}});
 expect(spawned[0]!.slice(1,3),"the successor names the pid it takes over from").toEqual(["--handover",String(process.pid)]);
 writeFileSync(join(directory,"lock.json"),JSON.stringify({pid:process.pid,start:""}));
 const waiting=runHostWardenLoopV1({ports,policy,tables,maxTicks:2,code:"new",handover:process.pid});
 await Bun.sleep(150);
 await import("node:fs").then(fs=>fs.rmSync(join(directory,"lock.json")));
 expect((await waiting).reason,"the successor took the lock once it was released").toBe("limit");
});

test("the footprint tool failing as a whole because one process ended is asked again process by process",async()=>{
 const {observeTreeMegabytesV1}=await import("../../../📡️sampling/🟦️.ts");
 const table="  10     1   1000\n  11    10   2000\n  12    10   3072\n";
 const run=async(command:string,args:readonly string[]):Promise<string>=>{
  if(command==="ps")return table;
  const pids=args.filter((_,index)=>args[index-1]==="-p");
  if(pids.includes("12")){const error=Object.assign(Error("Command failed: footprint"),{stdout:""});throw error;}
  return pids.map(pid=>`proc [${pid}]: 64-bit    Footprint: ${Number(pid)*1048576} B (16384 bytes per page)\n`).join("");
 };
 expect(await observeTreeMegabytesV1(10,run as never,"darwin",undefined,undefined,"footprint",undefined,undefined,null),"10 MB + 11 MB from the tool, the ended one keeps its resident set of 3000 KB").toBe(24);
});

test("regression 18:40-19:10: sticky compressed memory on a healthy host (4-5 GB available, swap-out 0, pressure normal) never suspends or cancels, and a long suspended worker is resumed, not cancelled",()=>{
 const healthy={totalMegabytes:16384,availableMegabytes:4297,swapUsedMegabytes:1400,swapOutMegabytesPerSecond:0,compressedMegabytes:7200,pressure:"normal",swapFreeMegabytes:1670,swapVolumeFreeMegabytes:35000,swapFileMegabytes:1024} as HostWardenHostV1;
 const worker=(extra:Partial<HostWardenSubjectV1>)=>({id:"v",pid:7,kind:"vitest-worker",rank:52,elapsedSeconds:2000,megabytes:2500,suspended:false,suspendedSeconds:0,canSuspend:true,canCancel:true,capMegabytes:8192,...extra}) as HostWardenSubjectV1;
 const running=decideHostWardenV1(HOST_WARDEN_POLICY_V1,healthy,[worker({})],{level:"calm",calmTicks:5});
 expect(running).toMatchObject({level:"calm",hold:false,actions:[]});
 for(const level of ["guard","danger"] as const)for(const swapped of [HOST_WARDEN_POLICY_V1.suspendMaximumSeconds,HOST_WARDEN_POLICY_V1.suspendHardMaximumSeconds+1]){
  const decision=decideHostWardenV1(HOST_WARDEN_POLICY_V1,healthy,[worker({suspended:true,suspendedSeconds:swapped,nonresidentMegabytes:0})],{level,calmTicks:HOST_WARDEN_POLICY_V1.calmTicks});
  expect(decision.actions.some(action=>action.action==="cancel"),`${level} ${swapped}s`).toBe(false);
 }
 const incident={...healthy,availableMegabytes:2980,swapUsedMegabytes:3060,swapFreeMegabytes:160,swapOutMegabytesPerSecond:705,compressedMegabytes:8260,pressure:"warn"} as HostWardenHostV1;
 expect(decideHostWardenV1(HOST_WARDEN_POLICY_V1,incident,[worker({})],{level:"calm",calmTicks:0}).level,"the 17:30:20 counters still fire").toBe("danger");
});
