import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {readHealthV1} from "../../🛡️resilience/🟦️.ts";
import {readTopologyV1} from "../../🗺️topology/🟦️.ts";
import {runReplicaCommandV1} from "../⌨️command/🟦️.ts";
import {runV1,seedV1,statusV1,type ReplicaEventV1,type ReplicaOptionsV1} from "../🟦️.ts";
import type {SidecarSettingsV1} from "../🧷️sidecars/🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`sidecars-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const until=async(condition:()=>boolean|Promise<boolean>,milliseconds=20000):Promise<void>=>{const end=Date.now()+milliseconds;while(!await condition()){if(Date.now()>end)throw Error("Timed out");await Bun.sleep(20);}};
const put=(root:string,path:string,text:string):void=>{const file=join(root,...path.split("/"));mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);};
const git={present:true,head:"refs/heads/main",commit:"a".repeat(40),indexTree:"c".repeat(40)};
const fast={pollMilliseconds:10,scanSeconds:1,debounceMilliseconds:5,persistMilliseconds:0,attestMilliseconds:100,restartStableMilliseconds:100,restartJitterMilliseconds:0,minimumUptimeMilliseconds:0,viewMilliseconds:300,viewDebounceMilliseconds:50,logFiles:[] as string[]};
const child=readFileSync(join(import.meta.dir,"..","🧷️sidecars","🧪️tests","🧬️child.ts"),"utf8");

function machine(name:string,sidecars:SidecarSettingsV1|undefined,store:string){
 const worktree=temp(name),codeRoot=temp(`${name}-code`);
 put(codeRoot,"x/🟦️.ts","export const a=1;\n");put(worktree,"child.ts",child);
 const options:ReplicaOptionsV1={worktree,storeRoot:store,originId:name,groupId:"g1",...fast,codeRoot,gitState:async()=>git,gitView:async()=>null,selfCheck:async()=>({ok:true}),...sidecars===undefined?{}:{sidecars}};
 return {options,events:[] as ReplicaEventV1[],controller:new AbortController(),done:undefined as Promise<void>|undefined};
}
const settings=(children:SidecarSettingsV1["children"]):SidecarSettingsV1=>({enabled:true,startAfterSeconds:0,graceSeconds:2,restart:{initialMilliseconds:100,maximumMilliseconds:400,stableMilliseconds:60_000},children});
const kid=(name:string,mode:string)=>({name,script:"child.ts",arguments:[mode],roles:["member","server"] as ("member"|"server")[],requires:"none" as const,enabled:true});
const alive=(pid:number):boolean=>{try{process.kill(pid,0);return true;}catch{return false;}};

test("the topology declares the build agent for every machine with a platform peer and the scheduler for the group server, and refuses a malformed sidecar setting",()=>{
 const topology=readTopologyV1();
 expect(topology.sidecars?.enabled).toBe(true);
 expect(topology.sidecars?.children.map(item=>[item.name,item.arguments,item.roles,item.requires])).toEqual([["build-agent",["agent"],["member","server"],"none"],["build-scheduler",["scheduler"],["server"],"none"],["build-cache",["cache"],["server"],"none"]]);
 for(const item of topology.sidecars!.children)expect(existsSync(join(import.meta.dir,"..","..","..","..","..","..","..",...item.script.split("/"))),item.script).toBe(true);
 const broken=JSON.parse(JSON.stringify(topology)) as {sidecars:{children:{requires:string}[]}};broken.sidecars.children[0]!.requires="always";
 expect(()=>readTopologyV1(broken)).toThrow(/Closed fleet topology/u);
});

test("the daemon starts its sidecars once it is healthy, shows them in health.json and in status, restarts a crashed one with back-off without touching replication, and stops them with itself",async()=>{
 const store=temp("store"),who=machine("s1",settings([kid("good","stay"),kid("crashy","crash")]),store);
 put(who.options.worktree,"a.txt","alpha\n");await seedV1(who.options);
 who.done=runV1(who.options,who.controller.signal,event=>who.events.push(event));
 await until(async()=>{const health=await readHealthV1(who.options.worktree);return health?.sidecars?.find(item=>item.name==="good")?.state==="running"&&(health.sidecars.find(item=>item.name==="crashy")?.restarts??0)>=2;});
 const health=(await readHealthV1(who.options.worktree))!,good=health.sidecars!.find(item=>item.name==="good")!,pid=good.pid!;
 expect(alive(pid)).toBe(true);expect(health.sidecars!.find(item=>item.name==="crashy")).toMatchObject({lastExit:"exit 3"});
 put(who.options.worktree,"b.txt","written while a sidecar crashes\n");
 await until(()=>existsSync(join(store,"v1","origins","s1","segments","000000000000.jsonl"))&&readFileSync(join(store,"v1","origins","s1","segments","000000000000.jsonl"),"utf8").includes("b.txt"),30000);
 const report=await statusV1(who.options);expect(report.health?.sidecars?.map(item=>item.name)).toEqual(["good","crashy"]);
 const lines:string[]=[];
 await runReplicaCommandV1(["status","--worktree",who.options.worktree,"--store",store,"--lang","en"],{root:who.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>{lines.push(line);}});
 expect(lines.some(line=>/Sidecar good: running \(process \d+/u.test(line)),lines.join("\n")).toBe(true);expect(lines.some(line=>/Sidecar crashy: (failed, restarts with back-off|running)/u.test(line)),lines.join("\n")).toBe(true);
 const german:string[]=[];
 await runReplicaCommandV1(["status","--worktree",who.options.worktree,"--store",store,"--lang","de"],{root:who.options.worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>{german.push(line);}});
 expect(german.some(line=>/Sidecar good: läuft \(Prozess \d+/u.test(line)),german.join("\n")).toBe(true);
 who.controller.abort();await who.done;
 expect(alive(pid),"the sidecar ends with the daemon").toBe(false);
});

test("a daemon without a sidecar setting starts nothing, and a daemon whose health is not ok starts nothing yet",async()=>{
 const store=temp("store2"),plain=machine("s2",undefined,store);put(plain.options.worktree,"a.txt","alpha\n");await seedV1(plain.options);
 plain.done=runV1(plain.options,plain.controller.signal,event=>plain.events.push(event));
 await until(async()=>(await readHealthV1(plain.options.worktree))!==undefined);
 expect((await readHealthV1(plain.options.worktree))?.sidecars).toBeUndefined();
 plain.controller.abort();await plain.done;
 const late=machine("s3",{...settings([kid("good","stay")]),startAfterSeconds:3},store);put(late.options.worktree,"a.txt","alpha\n");await seedV1(late.options);
 late.done=runV1(late.options,late.controller.signal,event=>late.events.push(event));
 await until(async()=>(await readHealthV1(late.options.worktree))?.sidecars?.[0]?.state==="waiting");
 expect((await readHealthV1(late.options.worktree))?.sidecars?.[0]).toMatchObject({state:"waiting",pid:null});
 await until(async()=>(await readHealthV1(late.options.worktree))?.sidecars?.[0]?.state==="running",15000);
 late.controller.abort();await late.done;
});
