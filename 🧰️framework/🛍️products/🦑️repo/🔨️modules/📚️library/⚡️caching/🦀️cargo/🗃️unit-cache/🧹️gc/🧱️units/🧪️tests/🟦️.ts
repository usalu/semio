import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {acquireResourceLease} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {collectGarbageSingleFlightV1,reclaimUnitStorageV1} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import componentSchema from "../🧬️schema/🔣️.json";
import {UNIT_POLICY_V1 as policy,collectUnitsV1,leaseDirectoryOfStoreV1,measureUnitsV1,planUnitsV1,profileLeasePortV1,profileLeaseResourceV1,unitProfilesV1,unitTextV1,type BuildUnitV1,type ProfileLeasePortV1} from "../🟦️.ts";

const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(componentSchema,"units.json");
const policyUnitStamp="units.stamp",hour=3600000,python=(rows:readonly unknown[]):unknown[]=>{const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"../🧬️schema/🔣️.json")],{stdin:new TextEncoder().encode(JSON.stringify(rows))});expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);return JSON.parse(new TextDecoder().decode(run.stdout));};

test("the fixture is closed and the planner equals the independent Python oracle on every vector and on 400 seeded random states",()=>{
 const validate=ajv.compile({$ref:"units.json#/definitions/Vector"});for(const vector of fixture.vectors)expect(validate(vector),vector.name+JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.vectors)expect(planUnitsV1(vector.input as never) as unknown,vector.name).toEqual(vector.remove);
 expect(python(fixture.vectors.map(vector=>vector.input))).toEqual(fixture.vectors.map(vector=>vector.remove));
 let seed=20261024;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},now=2000*hour,states:{needBytes:number;nowMs:number;units:BuildUnitV1[]}[]=[];
 for(let index=0;index<400;index++){
  const units:BuildUnitV1[]=[];
  for(const profile of["prototype","debug"])for(const crate of["a","b","c","d"].filter(()=>next(3)>0)){const busy=next(2)===0;for(let variant=0;variant<next(7);variant++)units.push({path:`/s/b/p/${profile}/build/${crate}/${variant.toString(16).padStart(16,"0")}`,crate,profile,bytes:next(500)*1048576,lastUsedMs:now-next(60*60)*60000,held:next(8)===0,busy:profile==="debug"?busy:!busy});}
  states.push({needBytes:next(3)?next(3000)*1048576:0,nowMs:now,units});
 }
 const expected=python(states);states.forEach((state,at)=>expect(planUnitsV1(state) as unknown,JSON.stringify(state).slice(0,200)).toEqual(expected[at]));
});

test("the policy keeps what a live build needs: a busy profile never loses its newest variants and a unit younger than the minimum age never goes",()=>{
 expect(policy.busyMinimumAgeMilliseconds).toBeGreaterThan(policy.minimumAgeMilliseconds);expect(policy.idleMilliseconds).toBeGreaterThan(policy.busyMinimumAgeMilliseconds);expect(policy.keepNewestPerCrate).toBeGreaterThanOrEqual(2);
 const now=500*hour,units:BuildUnitV1[]=[0,1,2,3].map(index=>({path:`/s/b/p/prototype/build/x/${index}`,crate:"x",profile:"prototype",bytes:1,lastUsedMs:now-(index===0?policy.minimumAgeMilliseconds/2:policy.busyMinimumAgeMilliseconds*(index+1)),held:false,busy:true}));
 expect(planUnitsV1({needBytes:1e12,nowMs:now,units}).map(row=>row.path)).toEqual(["/s/b/p/prototype/build/x/3","/s/b/p/prototype/build/x/2"]);
 expect(planUnitsV1({needBytes:1e12,nowMs:now,units:units.map(unit=>({...unit,busy:false}))}).map(row=>row.path)).toEqual(["/s/b/p/prototype/build/x/3","/s/b/p/prototype/build/x/2","/s/b/p/prototype/build/x/1"]);
});

function unit(store:string,profile:string,crate:string,hash:string,bytes:number,ageHours:number){
 const path=join(store,profile,"build",crate,hash);mkdirSync(join(path,"fingerprint"),{recursive:true});mkdirSync(join(path,"out"),{recursive:true});
 writeFileSync(join(path,"fingerprint","invoked.timestamp"),"x");writeFileSync(join(path,".lock"),"");writeFileSync(join(path,"out","lib.rlib"),Buffer.alloc(bytes,1));
 const when=(Date.now()-ageHours*hour)/1000;for(const target of[join(path,"out","lib.rlib"),join(path,"fingerprint","invoked.timestamp"),join(path,"fingerprint"),join(path,"out"),join(path,".lock"),path])utimesSync(target,when,when);
 return path;
}

test("profiles are found below the declared bases at two levels, units need a fingerprint, and measuring reads sizes and last change from the files",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m");
 try{
  const first=unit(store,"b/p/prototype","serde","aaaa",65536,5),second=unit(store,"b/p/wasm32-unknown-unknown/prototype","serde","bbbb",131072,30);unit(store,"u/l/d/b/prototype","tokio","cccc",4096,1);
  mkdirSync(join(store,"b/p/prototype/build/serde/notaunit/out"),{recursive:true});mkdirSync(join(store,"b/p/prototype/incremental"),{recursive:true});
  expect(unitProfilesV1(store).map(row=>row.profileDirectory.slice(store.length+1))).toEqual(["b/p/prototype","b/p/wasm32-unknown-unknown/prototype","u/l/d/b/prototype"]);
  expect(unitProfilesV1(store).map(row=>row.profile)).toEqual(["prototype","prototype","prototype"]);expect(unitProfilesV1(store)[1]!.buildDirectory).toBe(join(store,"b/p"));
  const now=Date.now(),rows=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>false,lock:()=>"missing"});
  expect(rows.length).toBe(3);const byPath=new Map(rows.map(row=>[row.path,row]));
  expect(byPath.get(first)!.bytes).toBeGreaterThanOrEqual(65536);expect(Math.abs(now-byPath.get(first)!.lastUsedMs-5*hour)).toBeLessThan(120000);expect(Math.abs(now-byPath.get(second)!.lastUsedMs-30*hour)).toBeLessThan(120000);
  expect(rows.every(row=>!row.held&&!row.busy)).toBe(true);
  const unsupported=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>true,lock:()=>"unsupported"});expect(unsupported.map(row=>row.held)).toEqual([false,false,true]);expect(unsupported.every(row=>row.busy)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a collection frees stale units under an exclusive profile lease, keeps a unit a running compiler locks, removes emptied crate folders and speaks English and German",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m");
 try{
  const stale=[unit(store,"b/p/prototype","serde","a1",65536,40),unit(store,"b/p/prototype","serde","a2",65536,35),unit(store,"b/p/prototype","serde","a3",65536,30)],live=unit(store,"b/p/prototype","serde","a4",65536,3),newest=unit(store,"b/p/prototype","serde","a5",65536,2),alone=[unit(store,"b/p/prototype","lone","b1",4096,60),unit(store,"b/p/prototype","lone","b2",4096,59),unit(store,"b/p/prototype","lone","b3",4096,58)];
  const taken:string[]=[],leases:ProfileLeasePortV1=async profile=>{taken.push("acquire:"+profile.profile);return {release:()=>{taken.push("release:"+profile.profile);}};},lines:string[]=[];
  const locked=new Set([join(stale[1]!,".lock")]),lock=(path:string)=>locked.has(path)?"held" as const:{release:()=>undefined};
  const dry=await collectUnitsV1(store,{dryRun:true,needBytes:()=>0,leases,lock,report:{language:"en",write:line=>lines.push(line)}});
  expect(dry.removed.map(row=>row.path).sort()).toEqual([alone[0]!,stale[0]!,stale[2]!].sort());expect(existsSync(stale[0]!)).toBe(true);expect(lines.some(line=>line.includes("would remove build unit"))).toBe(true);
  const german:string[]=[],result=await collectUnitsV1(store,{dryRun:false,needBytes:()=>0,leases,lock,report:{language:"de",write:line=>german.push(line)}});
  expect(existsSync(stale[0]!)&&existsSync(stale[2]!)).toBe(false);expect(existsSync(stale[1]!),"locked unit stays").toBe(true);expect(existsSync(live)&&existsSync(newest)).toBe(true);expect(existsSync(alone[0]!)).toBe(false);expect(existsSync(alone[2]!)).toBe(true);
  expect(result.removed.length).toBe(3);expect(result.held).toBe(1);expect(result.busyProfiles).toBe(0);expect(german.some(line=>line.includes("entfernt"))).toBe(true);expect(german.at(-1)).toContain("Build-Einheiten");
  const sweep=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**12,leases,lock:()=>({release:()=>undefined})});expect(existsSync(join(store,"b/p/prototype/build/lone")),"emptied crate folder goes").toBe(false);expect(sweep.removed.length).toBeGreaterThan(3);expect(taken.filter(row=>row.startsWith("acquire")).length).toBeGreaterThanOrEqual(2);expect(taken.filter(row=>row.startsWith("release")).length).toBe(taken.filter(row=>row.startsWith("acquire")).length);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a profile that compilers hold loses only superseded units idle for the busy minimum, and pressure stops once the image has its free space",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m");
 try{
  const old=unit(store,"b/p/prototype","a","1",65536,40),mid=unit(store,"b/p/prototype","a","2",65536,10),fresh=unit(store,"b/p/prototype","a","3",65536,4),newest=unit(store,"b/p/prototype","a","4",65536,1);
  const busy=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**9,leases:async()=>"busy",lock:()=>({release:()=>undefined}),report:{language:"en",write:()=>undefined}});
  expect(busy.busyProfiles).toBe(1);expect(busy.removed.map(row=>row.path)).toEqual([old,mid]);expect([fresh,newest].every(existsSync)).toBe(true);
  const again=unit(store,"b/p/prototype","b","1",65536,20),second=unit(store,"b/p/prototype","b","2",65536,15),third=unit(store,"b/p/prototype","b","3",65536,10),fourth=unit(store,"b/p/prototype","b","4",65536,5);let calls=0;
  const idle=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**9,satisfied:()=>++calls>1,leases:async()=>({release:()=>undefined}),lock:()=>({release:()=>undefined})});
  expect(idle.removed.length).toBeLessThan(6);expect(existsSync(again)).toBe(false);expect(fourth&&second&&third).toBeTruthy();
  const stopped=new AbortController();stopped.abort();await expect(collectUnitsV1(store,{dryRun:false,needBytes:()=>0,signal:stopped.signal,leases:async()=>({release:()=>undefined}),lock:()=>({release:()=>undefined})})).rejects.toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a real compiler lock keeps its unit, a real exclusive build lease makes a profile busy, a free one is taken without waiting and released",async()=>{
 if(!["darwin","linux"].includes(process.platform))return;
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,".🧬semio/🦑️repo/⚡️cache/rust/fixed/m"),leases=join(root,".cache/b/l/m/data");
 try{
  mkdirSync(leases,{recursive:true});mkdirSync(store,{recursive:true});expect(leaseDirectoryOfStoreV1(store)).toBe(leases);expect(leaseDirectoryOfStoreV1(join(tmpdir(),"nowhere-"+process.pid))).toBeUndefined();
  const held=unit(store,"b/p/prototype","serde","a1",65536,50),free=unit(store,"b/p/prototype","serde","a2",65536,45);unit(store,"b/p/prototype","serde","a3",65536,40);unit(store,"b/p/prototype","serde","a4",65536,39);unit(store,"b/p/prototype","serde","a5",65536,38);
  const holder=Bun.spawn(["python3","-I","-c","import fcntl,sys,time;f=open(sys.argv[1],'r+');fcntl.flock(f,fcntl.LOCK_EX);print('locked',flush=True);time.sleep(60)",join(held,".lock")],{stdout:"pipe"});
  try{
   await holder.stdout.getReader().read();
   const profile=unitProfilesV1(store)[0]!,port=profileLeasePortV1(store);
   const other=await acquireResourceLease({directory:leases,resource:profileLeaseResourceV1(profile),mode:"shared",signal:new AbortController().signal,timeoutMs:0});
   expect(await port(profile)).toBe("busy");other.release();
   const result=await collectUnitsV1(store,{dryRun:false,needBytes:()=>0});
   expect(result.held).toBe(1);expect(existsSync(held)).toBe(true);expect(existsSync(free)).toBe(false);expect(result.busyProfiles).toBe(0);
   const probe=await port(profile);expect(probe).not.toBe("busy");if(probe!=="busy")probe.release();
   expect(readdirSync(join(store,"b/p/prototype/build/serde")).sort()).toEqual(["a1","a4","a5"]);
  }finally{holder.kill();}
  expect(await profileLeasePortV1(join(tmpdir(),"nowhere-"+process.pid))(unitProfilesV1(store)[0]!)).toBe("busy");
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("the single-flight collector removes superseded build units of the image beside incremental sessions, at most once per unit interval unless the image is under pressure, and the reclaim before a build frees units too",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m"),cache=join(store,"u");
 try{
  mkdirSync(join(cache,"locks"),{recursive:true});
  const old=[unit(store,"b/p/prototype","serde","a1",65536,60),unit(store,"b/p/prototype","serde","a2",65536,55),unit(store,"b/p/prototype","serde","a3",65536,50)],keep=[unit(store,"b/p/prototype","serde","a4",65536,30),unit(store,"b/p/prototype","serde","a5",65536,29)];
  const first=await collectGarbageSingleFlightV1(cache,1<<30,{nowMs:Date.now(),volume:()=>({volumeBytes:2**40,freeBytes:2**39})});
  expect(first.running).toBe(false);if(!first.running)expect(first.applied.units).toBeGreaterThan(0);
  expect(old.every(path=>!existsSync(path)||path===old[2])).toBe(true);expect(keep.every(existsSync)).toBe(true);expect(existsSync(join(cache,policyUnitStamp))).toBe(true);
  const stale=unit(store,"b/p/prototype","serde","a6",65536,100);
  const second=await collectGarbageSingleFlightV1(cache,1<<30,{nowMs:Date.now(),volume:()=>({volumeBytes:2**40,freeBytes:2**39})});if(!second.running)expect(second.applied.units,"within the unit interval and without pressure nothing is scanned").toBe(0);expect(existsSync(stale)).toBe(true);
  const gained=await reclaimUnitStorageV1(store,1,{});expect(gained).toBeGreaterThanOrEqual(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("every message exists in both languages with the same placeholders",()=>{
 const messages=componentSchema.definitions.Messages.const as unknown as Record<string,{en:string;de:string}>,holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 for(const[key,row]of Object.entries(messages)){expect(holes(row.de),key).toEqual(holes(row.en));expect(unitTextV1(key as never,"en")).not.toBe(unitTextV1(key as never,"de"));}
});
