import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,relative,resolve,sep} from "node:path";
import {acquireResourceLease} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {collectGarbageSingleFlightV1,reclaimUnitStorageV1,unitBudgetBytesV1} from "../../🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json";
import componentSchema from "../🧬️schema/🔣️.json";
import {UNIT_POLICY_V1 as policy,collectUnitsV1,leaseDirectoryOfStoreV1,measureUnitsV1,planUnitsV1,profileLeasePortV1,profileLeaseResourceV1,unitProfilesV1,unitTextV1,unitsCoveredV1,planUnitsByImageV1,type UnitImagesV1,type BuildUnitV1,type ProfileLeasePortV1} from "../🟦️.ts";
import {ownProcessIdentityV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔖️identity/🟦️.ts";

const checkout=resolve(import.meta.dir).split(sep+"🧰️framework")[0]!;
/** 🛡️ A test never reaches the store of the real checkout: refuses (loudly) any path inside it. */
const outsideCheckout=(path:string):string=>{const real=realpathSync(path);if(!relative(checkout,real).startsWith(".."))throw Error(`A test reached the live repository checkout ${checkout} through ${real}; give it a private workspace outside the checkout`);return path;};
/** 🏠️ A private per-test workspace outside the checkout (the first temporary location that lies outside it, since the runner may place TMPDIR inside the checkout) with its own `.cache/b` store and lease directory. */
const privateWorkspace=(prefix:string):Readonly<{root:string;store:string;leases:string}>=>{
 for(const base of [tmpdir(),"/tmp"]){
  if(!existsSync(base)||!relative(checkout,realpathSync(base)).startsWith(".."))continue;
  const root=mkdtempSync(join(realpathSync(base),prefix)),store=join(root,".cache/b/r/m"),leases=join(root,".cache/b/l/m/data");
  mkdirSync(leases,{recursive:true});mkdirSync(store,{recursive:true});outsideCheckout(store);return Object.freeze({root,store,leases});
 }
 throw Error(`No temporary directory outside the checkout ${checkout} exists for a private test workspace`);
};
const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(componentSchema,"units.json");
const policyUnitStamp="units.stamp",hour=3600000,python=(rows:readonly unknown[],mode:"plan"|"cover"|"images"="plan"):unknown[]=>{const run=Bun.spawnSync(["python3","-I",join(import.meta.dir,"🐍️.py"),join(import.meta.dir,"../🧬️schema/🔣️.json"),mode],{stdin:new TextEncoder().encode(JSON.stringify(rows))});expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);return JSON.parse(new TextDecoder().decode(run.stdout));};

test("the fixture is closed and the planner equals the independent Python oracle on every vector and on 400 seeded random states",()=>{
 const validate=ajv.compile({$ref:"units.json#/definitions/Vector"});for(const vector of fixture.vectors)expect(validate(vector),vector.name+JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.vectors)expect(planUnitsV1(vector.input as never) as unknown,vector.name).toEqual(vector.remove);
 expect(python(fixture.vectors.map(vector=>vector.input))).toEqual(fixture.vectors.map(vector=>vector.remove));
 let seed=20261024;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},now=2000*hour,states:{needBytes:number;nowMs:number;units:BuildUnitV1[]}[]=[];
 for(let index=0;index<400;index++){
  const units:BuildUnitV1[]=[];
  for(const profile of["prototype","debug"])for(const crate of["a","b","c","d"].filter(()=>next(3)>0)){for(let variant=0;variant<next(7);variant++)units.push({path:`/s/b/p/${profile}/build/${crate}/${variant.toString(16).padStart(16,"0")}`,crate,profile,bytes:next(500)*1048576,lastUsedMs:now-next(60*60)*60000,held:next(8)===0});}
  states.push({needBytes:next(3)?next(3000)*1048576:0,nowMs:now,units});
 }
 const expected=python(states);states.forEach((state,at)=>expect(planUnitsV1(state) as unknown,JSON.stringify(state).slice(0,200)).toEqual(expected[at]));
});

test("the policy keeps what a live build needs: a held unit never goes however old and a unit younger than the minimum age never goes",()=>{
 expect(policy.heldGraceMilliseconds).toBeGreaterThan(policy.minimumAgeMilliseconds);expect(policy.idleMilliseconds).toBeGreaterThan(policy.heldGraceMilliseconds);expect(policy.keepNewestPerCrate).toBeGreaterThanOrEqual(2);
 const now=500*hour,units:BuildUnitV1[]=[0,1,2,3].map(index=>({path:`/s/b/p/prototype/build/x/${index}`,crate:"x",profile:"prototype",bytes:1,lastUsedMs:now-(index===0?policy.minimumAgeMilliseconds/2:policy.heldGraceMilliseconds*(index+1)),held:false}));
 expect(planUnitsV1({needBytes:1e12,nowMs:now,units}).map(row=>row.path)).toEqual(["/s/b/p/prototype/build/x/3","/s/b/p/prototype/build/x/2","/s/b/p/prototype/build/x/1"]);
 expect(planUnitsV1({needBytes:1e12,nowMs:now,units:units.map(unit=>({...unit,held:true}))})).toEqual([]);
});

test("the units a running build covers are the newest variants and the units used in the active window, equal to the Python oracle on every vector and on 300 seeded random profiles",()=>{
 const validate=ajv.compile({$ref:"units.json#/definitions/Cover"});for(const vector of fixture.covers)expect(validate(vector),vector.name+JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.covers)expect([...unitsCoveredV1(vector.units,vector.nowMs)].sort(),vector.name).toEqual(vector.covered);
 expect(python(fixture.covers,"cover")).toEqual(fixture.covers.map(vector=>vector.covered));
 let seed=20261127;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},now=2000*hour,states:{nowMs:number;units:{path:string;lastUsedMs:number}[]}[]=[];
 for(let index=0;index<300;index++){const units:{path:string;lastUsedMs:number}[]=[];for(const crate of["a","b","c"].filter(()=>next(4)>0))for(let variant=0;variant<next(8);variant++)units.push({path:`/s/b/p/prototype/build/${crate}/${variant.toString(16).padStart(16,"0")}`,lastUsedMs:now-next(80*60)*60000});states.push({nowMs:now,units});}
 const expected=python(states,"cover");states.forEach((state,at)=>expect([...unitsCoveredV1(state.units,state.nowMs)].sort(),JSON.stringify(state).slice(0,200)).toEqual(expected[at] as string[]));
 expect(policy.activeBuildMilliseconds).toBeGreaterThan(policy.heldGraceMilliseconds);expect(policy.activeBuildMilliseconds).toBeLessThan(policy.idleMilliseconds);
});

function unit(store:string,profile:string,crate:string,hash:string,bytes:number,ageHours:number){
 const path=join(store,profile,"build",crate,hash);mkdirSync(join(path,"fingerprint"),{recursive:true});mkdirSync(join(path,"out"),{recursive:true});
 writeFileSync(join(path,"fingerprint","invoked.timestamp"),"x");writeFileSync(join(path,".lock"),"");writeFileSync(join(path,"out","lib.rlib"),Buffer.alloc(bytes,1));
 const when=(Date.now()-ageHours*hour)/1000;for(const target of[join(path,"out","lib.rlib"),join(path,"fingerprint","invoked.timestamp"),join(path,"fingerprint"),join(path,"out"),join(path,".lock"),path])utimesSync(target,when,when);
 return path;
}

const imagesOf=(vector:{mounts:readonly{prefix:string;id:string}[];needs:Readonly<Record<string,number|undefined>>;budgets?:Readonly<Record<string,number|undefined>>}):UnitImagesV1=>({of:path=>vector.mounts.filter(mount=>path.startsWith(mount.prefix+"/")).sort((left,right)=>right.prefix.length-left.prefix.length)[0]?.id??"home",needBytes:id=>vector.needs[id]??0,budgetBytes:id=>vector.budgets?.[id],satisfied:()=>false});

test("a split partition is planned image by image: the plan equals the independent Python oracle on every vector and on 300 seeded random partitions, and one image is the plain plan",()=>{
 const validate=ajv.compile({$ref:"units.json#/definitions/ImagePlan"});for(const vector of fixture.images)expect(validate(vector),vector.name+JSON.stringify(validate.errors)).toBe(true);
 for(const vector of fixture.images)expect(planUnitsByImageV1({nowMs:vector.nowMs,units:vector.units,images:imagesOf(vector)}) as unknown,vector.name).toEqual(vector.remove);
 expect(python(fixture.images,"images")).toEqual(fixture.images.map(vector=>vector.remove));
 let seed=20261128;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%limit;},now=2000*hour,states:{nowMs:number;units:BuildUnitV1[];mounts:{prefix:string;id:string}[];needs:Record<string,number>;budgets:Record<string,number>}[]=[];
 for(let index=0;index<300;index++){
  const mounts=["a","b","c"].filter(()=>next(3)>0).map(id=>({prefix:"/s/u/l/"+id,id})),units:BuildUnitV1[]=[],needs:Record<string,number>={},budgets:Record<string,number>={};
  for(const image of["a","b","c","home"]){
   const base=image==="home"?"/s/b/p":"/s/u/l/"+image;for(const crate of["x","y"].filter(()=>next(3)>0))for(let variant=0;variant<next(6);variant++)units.push({path:`${base}/prototype/build/${crate}/${variant.toString(16).padStart(16,"0")}`,crate,profile:"prototype",bytes:next(300)*1048576,lastUsedMs:now-next(70*60)*60000,held:next(8)===0});
   if(next(2))needs[image]=next(1500)*1048576;if(next(4)===0)budgets[image]=next(900)*1048576;
  }
  states.push({nowMs:now,units,mounts,needs,budgets});
 }
 const expected=python(states,"images");states.forEach((state,at)=>expect(planUnitsByImageV1({nowMs:state.nowMs,units:state.units,images:imagesOf(state)}) as unknown,JSON.stringify(state).slice(0,200)).toEqual(expected[at]));
 for(const state of states){const single={mounts:[],needs:{home:state.needs.home??0},budgets:state.budgets.home===undefined?{}:{home:state.budgets.home}};expect(planUnitsByImageV1({nowMs:state.nowMs,units:state.units,images:imagesOf(single)}) as unknown).toEqual(planUnitsV1({needBytes:single.needs.home,...state.budgets.home===undefined?{}:{budgetBytes:state.budgets.home},nowMs:state.nowMs,units:state.units}));}
});

test("every Cargo compile launch of the repository asks Cargo to touch the fingerprint of each unit it plans, the evidence by which the collector tells the units a held profile's builds use",()=>{
 for(const site of["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts","🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts"]){
  const text=readFileSync(join(checkout,site),"utf8");expect(text.includes('CARGO_UNSTABLE_MTIME_ON_USE:'),site).toBe(true);expect(/CARGO_UNSTABLE_MTIME_ON_USE: ?"true"/u.test(text),site).toBe(true);
 }
});

test("a unit a build only reuses is used when its fingerprint file is touched (Cargo -Z mtime-on-use), though nothing else in it changed, and a unit of a held profile stays then however old it was built",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-touch-")),store=join(root,"m"),leased={leases:async()=>"busy" as const,lock:()=>({release:()=>undefined})};
 try{
  const reused=unit(store,"b/p/prototype","serde","r1",4096,100),idle=unit(store,"b/p/prototype","serde","r2",4096,101),third=unit(store,"b/p/prototype","serde","r3",4096,102),fourth=unit(store,"b/p/prototype","serde","r4",4096,103),touched=new Date();
  writeFileSync(join(reused,"fingerprint","lib-serde"),"");utimesSync(join(reused,"fingerprint"),new Date(Date.now()-100*hour),new Date(Date.now()-100*hour));utimesSync(join(reused,"fingerprint","lib-serde"),touched,touched);
  const rows=await measureUnitsV1(unitProfilesV1(store),{nowMs:Date.now(),busy:()=>true,lock:()=>"missing"});
  expect(Date.now()-rows.find(row=>row.path===reused)!.lastUsedMs).toBeLessThan(60000);expect(rows.filter(row=>row.held).map(row=>row.path).sort()).toEqual([reused,idle].sort());
  const result=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**9,...leased});expect(result.removed.map(row=>row.path).sort()).toEqual([third,fourth].sort());expect(existsSync(reused)&&existsSync(idle)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a unit a build touches between measuring and removing stays, however old it was",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-race-")),store=join(root,"m");
 try{
  const victim=unit(store,"b/p/prototype","serde","v1",4096,103),gone=unit(store,"b/p/prototype","serde","v2",4096,102),newest=[unit(store,"b/p/prototype","serde","v3",4096,101),unit(store,"b/p/prototype","serde","v4",4096,100)],old=new Date(Date.now()-103*hour);
  writeFileSync(join(victim,"fingerprint","lib-serde"),"");for(const target of[join(victim,"fingerprint","lib-serde"),join(victim,"fingerprint")])utimesSync(target,old,old);
  let probes=0;const lock=(path:string)=>{if(path===join(victim,".lock")&&++probes===2){const now=new Date();utimesSync(join(victim,"fingerprint","lib-serde"),now,now);}return {release:()=>undefined};};
  const result=await collectUnitsV1(store,{dryRun:false,needBytes:()=>0,leases:async()=>({release:()=>undefined}),lock});
  expect(probes).toBe(2);expect(existsSync(victim),"touched meanwhile").toBe(true);expect(existsSync(gone),"untouched variant beside it").toBe(false);expect(newest.every(existsSync)).toBe(true);expect(result.removed.map(row=>row.path)).toEqual([gone]);expect(result.held).toBe(1);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("profiles are found below the declared bases at two levels, units need a fingerprint, and measuring reads sizes and last change from the files",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m");
 try{
  const first=unit(store,"b/p/prototype","serde","aaaa",65536,5),second=unit(store,"b/p/wasm32-unknown-unknown/prototype","serde","bbbb",131072,30);unit(store,"u/l/d/prototype","tokio","cccc",4096,1);
  mkdirSync(join(store,"b/p/prototype/build/serde/notaunit/out"),{recursive:true});mkdirSync(join(store,"b/p/prototype/incremental"),{recursive:true});
  expect(unitProfilesV1(store).map(row=>relative(store,row.profileDirectory).split(sep).join("/"))).toEqual(["b/p/prototype","b/p/wasm32-unknown-unknown/prototype","u/l/d/prototype"]);
  expect(unitProfilesV1(store).map(row=>row.profile)).toEqual(["prototype","prototype","prototype"]);expect(unitProfilesV1(store)[1]!.buildDirectory).toBe(join(store,"b/p"));
  const now=Date.now(),rows=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>false,lock:()=>"missing"});
  expect(rows.length).toBe(3);const byPath=new Map(rows.map(row=>[row.path,row]));
  expect(byPath.get(first)!.bytes).toBeGreaterThanOrEqual(65536);expect(Math.abs(now-byPath.get(first)!.lastUsedMs-5*hour)).toBeLessThan(120000);expect(Math.abs(now-byPath.get(second)!.lastUsedMs-30*hour)).toBeLessThan(120000);
  expect(rows.every(row=>!row.held)).toBe(true);
  const unsupported=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>false,lock:()=>"unsupported"});expect(unsupported.map(row=>row.held)).toEqual([false,false,true]);
  const leased=await measureUnitsV1(unitProfilesV1(store),{nowMs:now,busy:()=>true,lock:()=>"missing"});expect(leased.every(row=>row.held),"a profile compilers hold keeps the units its builds use (one variant each is the newest)").toBe(true);
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

test("a profile that compilers hold loses only the units no running build uses, and pressure stops once the image has its free space",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m");
 try{
  const old=unit(store,"b/p/prototype","a","1",65536,40),mid=unit(store,"b/p/prototype","a","2",65536,10),fresh=unit(store,"b/p/prototype","a","3",65536,4),newest=unit(store,"b/p/prototype","a","4",65536,1);
  const busy=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**9,leases:async()=>"busy",lock:()=>({release:()=>undefined}),report:{language:"en",write:()=>undefined}});
  expect(busy.busyProfiles).toBe(1);expect(busy.removed.map(row=>row.path)).toEqual([old,mid]);expect([old,mid].some(existsSync),"the variants no build uses go").toBe(false);expect([fresh,newest].every(existsSync),"the units the build uses stay").toBe(true);expect(busy.held).toBe(2);
  const again=unit(store,"b/p/prototype","b","1",65536,20),second=unit(store,"b/p/prototype","b","2",65536,15),third=unit(store,"b/p/prototype","b","3",65536,10),fourth=unit(store,"b/p/prototype","b","4",65536,5);let calls=0;
  const idle=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**9,satisfied:()=>++calls>1,leases:async()=>({release:()=>undefined}),lock:()=>({release:()=>undefined})});
  expect(idle.removed.length).toBeLessThan(6);expect(existsSync(again)).toBe(false);expect(fourth&&second&&third).toBeTruthy();
  const stopped=new AbortController();stopped.abort();await expect(collectUnitsV1(store,{dryRun:false,needBytes:()=>0,signal:stopped.signal,leases:async()=>({release:()=>undefined}),lock:()=>({release:()=>undefined})})).rejects.toThrow();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a collector that runs while a compiler holds the build lease of the profile leaves the units a build uses, idle, unlocked, over budget and under pressure alike, takes the variants no build uses, and takes the rest once the compiler is gone",async()=>{
 if(!["darwin","linux"].includes(process.platform))return;
 const {root,store,leases}=privateWorkspace("units-holder-");
 try{
  mkdirSync(join(store,"u","locks"),{recursive:true});
  const paths=["a","b","c","d","e"].map((hash,index)=>unit(store,"b/p/prototype","pack-http",hash,65536,200-index)),profile=unitProfilesV1(store)[0]!;
  const compiler=await acquireResourceLease({directory:leases,resource:profileLeaseResourceV1(profile),mode:"shared",signal:new AbortController().signal,timeoutMs:0});
  try{
   const dry=await collectUnitsV1(store,{dryRun:true,needBytes:()=>10**12,budgetBytes:0});expect(dry.removed.map(row=>row.path).sort()).toEqual(paths.slice(0,3).sort());expect(dry.held).toBe(2);expect(dry.busyProfiles).toBe(1);
   const applied=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**12,budgetBytes:0});expect(applied.removed.map(row=>row.path).sort()).toEqual(paths.slice(0,3).sort());expect(paths.slice(0,3).some(existsSync)).toBe(false);expect(paths.slice(3).every(existsSync),"the two newest variants stay").toBe(true);
   await reclaimUnitStorageV1(store,10**12,{capacityBytes:1});expect(paths.slice(3).every(existsSync),"the reclaim before a build leaves them too").toBe(true);
   const swept=await collectGarbageSingleFlightV1(join(store,"u"),1,{nowMs:Date.now()+365*24*hour,volume:()=>({volumeBytes:2**40,freeBytes:1})});if(!swept.running)expect(swept.applied.units).toBe(0);expect(paths.slice(3).every(existsSync),"the pressure sweep leaves them too").toBe(true);
  }finally{compiler.release();}
  const gone=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**12,budgetBytes:0});expect(gone.removed.length).toBe(2);expect(paths.some(existsSync)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
},60000);

test("a real compiler lock keeps its unit, a real exclusive build lease makes a profile busy, a free one is taken without waiting and released",async()=>{
 if(!["darwin","linux"].includes(process.platform))return;
 const {root,store,leases}=privateWorkspace("units-"),nowhere=privateWorkspace("nowhere-");rmSync(join(nowhere.root,".cache"),{recursive:true,force:true});
 try{
  expect(leaseDirectoryOfStoreV1(store)).toBe(leases);expect(leaseDirectoryOfStoreV1(join(nowhere.root,"store"))).toBeUndefined();
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
  expect(await profileLeasePortV1(join(nowhere.root,"store"))(unitProfilesV1(store)[0]!)).toBe("busy");
 }finally{rmSync(root,{recursive:true,force:true});rmSync(nowhere.root,{recursive:true,force:true});}
},60000);

test("the single-flight collector removes superseded build units of the image beside incremental sessions, at most once per unit interval unless the image is under pressure, and the reclaim before a build frees units too",async()=>{
 const {root,store}=privateWorkspace("units-"),cache=join(store,"u");
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

test("units of the native debug lane (u/l/d/b/debug/build/<crate>/<hash>) are retired oldest first once together they exceed their budget, without any free-space pressure and without a lock backend",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-lane-")),store=join(root,"m"),free={leases:async()=>({release:()=>undefined}),lock:()=>"unsupported" as const};
 try{
  const lane="u/l/d/b/debug",paths=["x","y","z"].flatMap((crate,index)=>[8,7,6,5].map(age=>unit(store,lane,crate,"h"+age,65536,age+index/10)));
  expect(unitProfilesV1(store).map(row=>relative(store,row.profileDirectory).split(sep).join("/"))).toEqual([lane]);
  const measured=await measureUnitsV1(unitProfilesV1(store),{nowMs:Date.now(),busy:()=>false,lock:()=>"unsupported"}),total=measured.reduce((sum,row)=>sum+row.bytes,0),each=total/paths.length;
  expect(measured).toHaveLength(paths.length);expect(measured.every(row=>!row.held)).toBe(true);
  const idle=await collectUnitsV1(store,{dryRun:true,needBytes:()=>0,budgetBytes:total,...free});expect(idle.removed).toHaveLength(0);
  const plan=await collectUnitsV1(store,{dryRun:true,needBytes:()=>0,budgetBytes:Math.floor(total-each*2.5),...free});expect(plan.removed.map(row=>row.path)).toEqual([join(store,lane,"build/z/h8"),join(store,lane,"build/y/h8"),join(store,lane,"build/x/h8")].sort((left,right)=>measured.find(row=>row.path===left)!.lastUsedMs-measured.find(row=>row.path===right)!.lastUsedMs));
  expect(plan.removed.every(row=>row.reason==="pressure")).toBe(true);
  const applied=await collectUnitsV1(store,{dryRun:false,needBytes:()=>0,budgetBytes:Math.floor(total-each*2.5),satisfied:()=>true,...free});
  expect(applied.removed.map(row=>row.path).sort()).toEqual(plan.removed.map(row=>row.path).sort());for(const row of applied.removed)expect(existsSync(row.path)).toBe(false);
  const rest=await collectUnitsV1(store,{dryRun:false,needBytes:()=>0,budgetBytes:0,...free});expect(rest.removed.length).toBe(paths.length-applied.removed.length);expect(readdirSync(join(store,lane,"build"))).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the explicit reclaim retires lane units beyond the unit budget of the given capacity without any free-space need, and keeps them without a capacity",async()=>{
 const {root,store}=privateWorkspace("units-reclaim-");
 try{
  mkdirSync(join(store,"u"),{recursive:true});const lane="u/l/d/b/debug",old=[unit(store,lane,"x","h8",65536,8),unit(store,lane,"x","h7",65536,7)],young=[unit(store,lane,"x","h6",65536,6),unit(store,lane,"x","h5",65536,5)];
  await reclaimUnitStorageV1(store,0,{});for(const path of[...old,...young])expect(existsSync(path)).toBe(true);
  await reclaimUnitStorageV1(store,0,{capacityBytes:Math.ceil(2.5*65536*2**20/unitBudgetBytesV1(2**20))});for(const path of old)expect(existsSync(path)).toBe(false);for(const path of young)expect(existsSync(path)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("every message exists in both languages with the same placeholders",()=>{
 const messages=componentSchema.definitions.Messages.const as unknown as Record<string,{en:string;de:string}>,holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(row=>row[1]).sort();
 for(const[key,row]of Object.entries(messages)){expect(holes(row.de),key).toEqual(holes(row.en));expect(unitTextV1(key as never,"en")).not.toBe(unitTextV1(key as never,"de"));}
});

test("canonical lane discovery separates build profiles from marker namespaces and preserves explicit target ownership",()=>{const root=mkdtempSync(join(tmpdir(),"lane-scan-"));try{for(const path of fixture.discovery.directories)mkdirSync(join(root,path),{recursive:true});expect(new Ajv({strict:false}).validate({...componentSchema, $ref:"#/definitions/Discovery"},fixture.discovery)).toBe(true);const found=unitProfilesV1(root).map(row=>join(row.profileDirectory,"build"));expect(found.map(path=>path.slice(root.length+1).replaceAll("\\","/")).sort()).toEqual(fixture.discovery.expected);expect(existsSync(join(root,"u/l/d1/v/prototype/build"))).toBe(true);expect(existsSync(join(root,"u/l/d1/m/prototype/build"))).toBe(true);}finally{rmSync(root,{recursive:true,force:true});}});

test("every measured unit is grouped under its profile on any path separator, and a unit outside the declared profiles is reported by its own profile directory instead of crashing",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-")),store=join(root,"m");
 try{
  const old=unit(store,"b/p/prototype","a","1",65536,40),newest=unit(store,"b/p/prototype","a","2",65536,1),result=await collectUnitsV1(store,{dryRun:false,needBytes:()=>10**9,leases:async()=>({release:()=>undefined}),lock:()=>({release:()=>undefined})});
  expect(result.removed.map(row=>row.path)).toContain(old);expect(existsSync(old)).toBe(false);expect(newest.length).toBeGreaterThan(0);expect(Object.keys(result.profiles).every(key=>!key.includes("\\"))).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a lane with a live holder loses no unit however old, idle, over budget or unlocked, and an idle lane beside it still does (the vanished proc-macro2 build script unit)",async()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"units-live-")),store=join(root,"m"),free={leases:async()=>({release:()=>undefined}),lock:()=>"unsupported" as const};
 try{
  const mine="u/l/p488470/prototype",other="u/l/p111111/prototype",stale=["a","b","c"].flatMap(hash=>["proc-macro2","serde"].map(crate=>unit(store,mine,crate,hash,65536,40))),spare=unit(store,other,"proc-macro2","a",65536,40);
  const marker=join(store,"u/l/p488470/v",process.pid+"-abc123");mkdirSync(join(store,"u/l/p488470/v"),{recursive:true});writeFileSync(marker,JSON.stringify({...ownProcessIdentityV1()}));
  const options={needBytes:()=>1e12,budgetBytes:0,...free},dry=await collectUnitsV1(store,{dryRun:true,...options});
  expect(dry.removed.map(row=>row.path)).toEqual([spare]);
  const applied=await collectUnitsV1(store,{dryRun:false,...options});
  expect(applied.removed.map(row=>row.path)).toEqual([spare]);expect(existsSync(spare)).toBe(false);for(const path of stale)expect(existsSync(path)).toBe(true);
  expect(applied.held).toBe(stale.length);
  rmSync(marker);
  const released=await collectUnitsV1(store,{dryRun:false,...options});expect(released.removed).toHaveLength(stale.length);for(const path of stale)expect(existsSync(path)).toBe(false);
  const flipping=unit(store,mine,"quote","x",65536,40),late=await collectUnitsV1(store,{dryRun:false,...options,live:()=>true});expect(late.removed).toHaveLength(0);expect(existsSync(flipping)).toBe(true);
 }finally{rmSync(root,{recursive:true,force:true});}
});
