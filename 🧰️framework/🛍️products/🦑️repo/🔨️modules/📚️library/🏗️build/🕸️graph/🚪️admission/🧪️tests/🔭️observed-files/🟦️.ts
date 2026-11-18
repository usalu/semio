/** 🔭️ Contract: a file of another owner that an owner only reads as an input fact (the Cargo manifests a plugin catalog consumer hashes) is a member of the closure and invalidates it, but admits no owner; only a dependency edge or an admitting file admits. The closure of a small selection stays small however many owners the catalog holds. */
import{expect,test}from"bun:test";
import{closeBuildOwnerAdmissionV1,readBuildOwnerAdmissionV1,readBuildOwnerRegistryV1,type BuildOwnerAdmissionFactsV1,type BuildOwnerRegistryV1}from"../../🟦️.ts";

type Facts=BuildOwnerAdmissionFactsV1&{observedFiles?:string[]};
const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
const owner=(name:string,root:string)=>({name,root,manifest:root+"/📋️project.json",packages:[] as string[]});
const registryOf=(owners:ReturnType<typeof owner>[]):BuildOwnerRegistryV1=>readBuildOwnerRegistryV1({version:1,owners,catalogProviders:[]});
const close=(registry:BuildOwnerRegistryV1,selected:string[],facts:(name:string,targets:readonly string[])=>Facts)=>closeBuildOwnerAdmissionV1({kind:"owners",projects:selected,exclude:[],targets:["build"]},registry,async(o,targets)=>{const value=facts(o.name,targets);return{...value,project:{name:o.name,targets:{build:{}}}} as never;});

test("a catalog consumer observes 200 foreign manifests: they are members and invalidate, the closure is the consumer alone",async()=>{
 const crates=Array.from({length:200},(_,at)=>owner("crate"+at,"catalog/crate"+at)),registry=registryOf([owner("plugin","plugins/p"),...crates]);
 const manifests=crates.map(crate=>crate.root+"/Cargo.toml");
 const admission=await close(registry,["plugin"],(name,targets)=>({project:{name,targets:{}},files:[registry.owners.find(o=>o.name===name)!.manifest],observedFiles:name==="plugin"&&targets.length?manifests:[],dependencies:[]}));
 expect(admission.owners).toEqual(["plugin"]);
 expect(manifests.every(path=>admission.files.includes(path))).toBe(true);
 expect((admission as {observedFiles?:string[]}).observedFiles).toEqual([...manifests].sort());
 expect(readBuildOwnerAdmissionV1(admission)).toBe(admission);
});

test("a file that is also an admitting file of another owner admits that owner and is not reported as merely observed",async()=>{
 const registry=registryOf([owner("plugin","plugins/p"),owner("crate0","catalog/crate0"),owner("crate1","catalog/crate1")]);
 const admission=await close(registry,["plugin"],(name,targets)=>({project:{name,targets:{}},files:[registry.owners.find(o=>o.name===name)!.manifest,...name==="plugin"&&targets.length?["catalog/crate0/Cargo.toml"]:[]],observedFiles:name==="plugin"&&targets.length?["catalog/crate0/Cargo.toml","catalog/crate1/Cargo.toml"]:[],dependencies:[]}));
 expect(admission.owners).toEqual(["crate0","plugin"]);
 expect((admission as {observedFiles?:string[]}).observedFiles).toEqual(["catalog/crate1/Cargo.toml"]);
 expect(admission.files).toContain("catalog/crate1/Cargo.toml");
});

test("a dependency edge admits whatever the observed files say",async()=>{
 const registry=registryOf([owner("plugin","plugins/p"),owner("dep","lib/dep"),owner("crate","catalog/crate")]);
 const admission=await close(registry,["plugin"],(name,targets)=>({project:{name,targets:{}},files:[registry.owners.find(o=>o.name===name)!.manifest],observedFiles:name==="plugin"&&targets.length?["catalog/crate/Cargo.toml"]:[],dependencies:name==="plugin"&&targets.length?["dep"]:[]}));
 expect(admission.owners).toEqual(["dep","plugin"]);
});

test("200 seeded worlds: the closure equals the start owner, its dependencies and the owners of its admitting files only, and every observed file is a member",async()=>{
 const random=seeded(14112026);let observedTotal=0,admittedTotal=0;
 for(let world=0;world<200;world++){
  const count=3+Math.floor(random()*25),owners=Array.from({length:count},(_,at)=>owner("o"+at,"t/"+at)),registry=registryOf(owners),pick=()=>"o"+Math.floor(random()*count);
  const plan=new Map(owners.map(o=>[o.name,{deps:Array.from({length:Math.floor(random()*3)},pick).filter(name=>name!==o.name),files:[...new Set(Array.from({length:Math.floor(random()*3)},()=>"t/"+Math.floor(random()*count)+"/f.ts").filter(path=>!path.startsWith(o.root+"/")))],seen:[...new Set(Array.from({length:Math.floor(random()*8)},()=>"t/"+Math.floor(random()*count)+"/Cargo.toml"))]}]));
  const start=pick(),admission=await close(registry,[start],(name,targets)=>{const row=plan.get(name)!;return{project:{name,targets:{}},files:[registry.owners.find(o=>o.name===name)!.manifest,...targets.length?row.files:[]],observedFiles:targets.length?row.seen:[],dependencies:targets.length?[...new Set(row.deps)]:[]};});
  const expected=new Set([start]),row=plan.get(start)!;for(const next of[...row.deps,...row.files.map(path=>"o"+path.split("/")[1])])expected.add(next);
  expect(admission.owners,"world "+world).toEqual([...expected].sort());
  const observed=(admission as {observedFiles?:string[]}).observedFiles??[];
  for(const path of plan.get(start)!.seen)expect(admission.files).toContain(path);
  for(const path of observed){expect(admission.files).toContain(path);}
  observedTotal+=observed.length;admittedTotal+=expected.size;
 }
 expect(observedTotal).toBeGreaterThan(200);expect(admittedTotal).toBeGreaterThan(400);
});
