/** 🧭️ Contract: the indexed source ownership of the admission closure answers exactly what the linear scan answers and what an independent Python model answers, and it makes the closure of a workspace with thousands of owners and ten thousands of files cost the files, not the files times the owners. */
import{expect,test}from"bun:test";
import{dirname,join}from"node:path";
import{fileURLToPath}from"node:url";
import{testPythonV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{buildSourceOwnerV1,closeBuildOwnerAdmissionV1,createSourceOwnerResolverV1,buildRelativeImportProjectV1,registryOwnerIndexesV1,readBuildOwnerRegistryV1,type BuildOwnerRegistryV1}from"../../🟦️.ts";

const python=testPythonV1(),oracle=join(dirname(fileURLToPath(import.meta.url)),"🐍️.py");
type World={owners:{name:string;root:string;nativeRoots?:string[]}[];paths:string[]};
const model=(worlds:readonly World[])=>{
 const run=Bun.spawnSync([python!,"-I",oracle],{stdin:Buffer.from(JSON.stringify({worlds}))});
 expect(run.exitCode).toBe(0);return JSON.parse(run.stdout.toString()).answers as (string|null)[][];
};
const seeded=(seed:number)=>()=>{seed=(seed*1664525+1013904223)>>>0;return seed/4294967296;};
const registryOf=(owners:World["owners"]):BuildOwnerRegistryV1=>({version:1,owners:owners.map(owner=>({...owner,manifest:(owner.root==="."?"":owner.root+"/")+"📋️project.json",packages:[]})),catalogProviders:[]});

test("the workspace root owns what no deeper root owns, a native root owns its subtree, equal-length roots keep the smallest name",()=>{
 const owners=[{name:"workspace",root:"."},{name:"b",root:"r",nativeRoots:["r/native"]},{name:"a",root:"x"},{name:"inner",root:"r/native/deep"}],resolve=createSourceOwnerResolverV1(registryOf(owners));
 const paths=["top.ts","r/f.ts","r/native/f.rs","r/native/deep/f.rs","x/y","xx/y","r","r/nat"];
 expect(paths.map(path=>resolve(path)??null)).toEqual(["workspace","b","b","inner","a","workspace","b","b"]);
 expect(paths.map(path=>buildSourceOwnerV1(registryOf(owners),path)??null)).toEqual(paths.map(path=>resolve(path)??null));
 expect(model([{owners,paths}])).toEqual([paths.map(path=>resolve(path)??null)]);
});

test("120 seeded worlds: the index, the linear scan and the independent model agree on every path",()=>{
 const random=seeded(17112026),words=["a","b","ab","c","🧰️x","🔨️y","dir","m n"];
 const directory=()=>Array.from({length:1+Math.floor(random()*4)},()=>words[Math.floor(random()*words.length)]!).join("/");
 const worlds:World[]=[],indexed:(string|null)[][]=[];let owned=0,unowned=0;
 for(let world=0;world<120;world++){
  const used=new Set<string>(),fresh=()=>{let root=directory();while(used.has(root))root=directory();used.add(root);return root;};
  const owners=Array.from({length:1+Math.floor(random()*30)},(_,at)=>{const root=random()<0.08&&!used.has(".")?(used.add("."),"."):fresh();return{name:"p"+Math.floor(random()*40)+"-"+at%3,root,...random()<0.3?{nativeRoots:[fresh()]}:{}};});
  const paths=Array.from({length:60},()=>directory()+(random()<0.7?"/file"+Math.floor(random()*5)+".ts":"")),registry=registryOf(owners),resolve=createSourceOwnerResolverV1(registry),answers=paths.map(path=>resolve(path)??null);
  expect(answers).toEqual(paths.map(path=>buildSourceOwnerV1(registry,path)??null));
  worlds.push({owners,paths});indexed.push(answers);for(const answer of answers)answer===null?unowned++:owned++;
 }
 expect(model(worlds)).toEqual(indexed);
 expect(owned).toBeGreaterThan(500);expect(unowned).toBeGreaterThan(500);
});

test("the closure of 6000 owners and one owner with 60000 files costs the files, not the files times the owners",async()=>{
 const owners=Array.from({length:6000},(_,at)=>({name:"o"+at,root:"tree/"+at%60+"/o"+at,manifest:"tree/"+at%60+"/o"+at+"/📋️project.json",packages:[] as string[]})),registry=readBuildOwnerRegistryV1({version:1,owners,catalogProviders:[]});
 const wide=Array.from({length:60000},(_,at)=>"tree/"+at%60+"/o"+(at%6000)+"/dir"+at%7+"/file"+at+".ts"),started=process.cpuUsage();
 const admission=await closeBuildOwnerAdmissionV1({kind:"all"},registry,async owner=>({project:{name:owner.name,targets:{}},files:owner.name==="o0"?wide:[],dependencies:[]}));
 const used=process.cpuUsage(started),seconds=(used.user+used.system)/1e6;
 expect(admission.owners.length).toBe(6000);
 expect(admission.files.length).toBeGreaterThan(60000);
 expect(seconds).toBeLessThan(4);
},120_000);

test("the owner lookups of an observation are built once per registry and answer what the scans over the owner lists answer",()=>{
 const random=seeded(2611),words=["a","b","ab","🧰️x","dir"],directory=()=>Array.from({length:1+Math.floor(random()*3)},()=>words[Math.floor(random()*words.length)]!).join("/");
 const used=new Set<string>(),fresh=()=>{let root=directory();while(used.has(root))root=directory();used.add(root);return root;};
 const owners=Array.from({length:25},(_,at)=>({name:"p"+at,root:fresh(),packages:["@p/"+at],...at%4===0?{nativeRoots:[fresh()]}:{}}));
 const registry=registryOf(owners.map(({packages:_packages,...owner})=>owner));registry.owners.forEach((owner,at)=>{owner.packages=owners[at]!.packages;});
 const first=registryOwnerIndexesV1(registry);
 expect(registryOwnerIndexesV1(registry)).toBe(first);
 expect(first.byPackage.get("@p/3")).toBe("p3");expect(first.byPackage.size).toBe(25);
 const rows=registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({name:owner.name,root})));
 for(const row of rows)expect(first.byRoot.get(row.root)).toBe(row.name);
 for(let at=0;at<300;at++){const path=directory()+"/file"+at+".ts";expect(first.relativeProject(path)).toBe(buildRelativeImportProjectV1(rows,path));}
});
