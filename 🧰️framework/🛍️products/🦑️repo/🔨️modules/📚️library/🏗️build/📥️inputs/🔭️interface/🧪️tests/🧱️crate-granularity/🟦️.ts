import {expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,readFileSync,readdirSync} from "node:fs";
import {homedir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import {nativeDependencyRoots,wire} from "../../../../🕸️graph/📥️inference/🦀️native-manifests/🟨️.mjs";

/** 🧱️ Crate granularity law: the dependency crates a native task names (the closure of path dependencies the graph builds its `{input,projects}` lists from) equal the closure `cargo metadata` resolves for the same crate, for a stride sample of 52 crates of the framework workspace. The oracle is Cargo's own reading of the manifests, so workspace inheritance, renames and target tables are Cargo's. */
setDefaultTimeout(240000);
const root=(()=>{let at=import.meta.dir;while(!existsSync(join(at,"AGENTS.md"))&&dirname(at)!==at)at=dirname(at);return process.env.SEMIO_TEST_CHECKOUT??at;})();
const cargo=(()=>{
 try{
  const channel=/^channel\s*=\s*"([^"]+)"/mu.exec(readFileSync(join(root,"rust-toolchain.toml"),"utf8"))?.[1];if(channel===undefined)return undefined;
  const base=join(process.env.RUSTUP_HOME??join(homedir(),".rustup"),"toolchains"),name=readdirSync(base).find(entry=>entry.startsWith(channel+"-"));
  return name!==undefined&&existsSync(join(base,name,"bin","cargo"))?join(base,name,"bin","cargo"):undefined;
 }catch{return undefined;}
})();
const real=cargo===undefined?test.skip:test;
type Dependency={name:string;kind:string|null;path?:string};
const packages=new Map<string,Dependency[]>(),loaded=new Set<string>();
function load(manifest:string):void{
 if(loaded.has(manifest))return;
 const run=spawnSync(cargo!,["metadata","--offline","--no-deps","--format-version","1","--config",'build.rustc="rustc"',"--config",'build.rustc-wrapper=""',"--manifest-path",manifest],{cwd:root,encoding:"utf8",maxBuffer:1<<28,env:{...process.env,RUSTC_WRAPPER:""}});
 loaded.add(manifest);
 if(run.status!==0)return;
 for(const row of JSON.parse(run.stdout).packages as {manifest_path:string;dependencies:Dependency[]}[]){loaded.add(row.manifest_path);packages.set(dirname(row.manifest_path),row.dependencies.filter(dependency=>typeof dependency.path==="string"));}
}
function closure(directory:string):Set<string>{
 const seen=new Set<string>(),pending=[directory];
 while(pending.length){
  const at=pending.pop()!;if(seen.has(at))continue;seen.add(at);
  if(!packages.has(at))load(join(at,"Cargo.toml"));
  for(const dependency of packages.get(at)??[])if(dependency.kind===null||dependency.kind==="build")pending.push(dependency.path!);
 }
 return seen;
}
wire({sourceAnalyses:{prepareSourceAnalysesV1:()=>undefined,RUST_SOURCE_CACHE:new Map(),RUST_INPUT_SNAPSHOTS:new WeakMap()},coordinates:{buildPath:(path:string)=>path.split("\\").join("/"),SCRIPT_BASENAME:"script.ts"},targetPolicy:{targetExecutionKindV1:()=>undefined,executionTargetV1:()=>({})}});

real("the dependency crates of a stride sample of 52 framework crates equal the closure Cargo resolves",()=>{
 const owners=(JSON.parse(readFileSync(join(root,"📋️build-owners.json"),"utf8")).owners as {root:string}[]).map(row=>row.root).filter(path=>path.startsWith("🧰️framework/")&&existsSync(join(root,path,"Cargo.toml"))).sort();
 expect(owners.length).toBeGreaterThan(1000);
 const stride=Math.floor(owners.length/52),sample=owners.filter((_,index)=>index%stride===0).slice(0,52);
 expect(sample.length).toBeGreaterThanOrEqual(50);
 let compared=0;const mismatches:string[]=[];
 for(const crate of sample){
  const absolute=join(root,crate),ours=new Set(nativeDependencyRoots(crate,root,false,new Map(),new Map()).map(path=>resolve(root,path))),theirs=closure(absolute);
  if(!packages.has(absolute)){continue;}
  compared++;
  const missing=[...theirs].filter(path=>!ours.has(path)).map(path=>relative(root,path)),extra=[...ours].filter(path=>!theirs.has(path)).map(path=>relative(root,path));
  if(missing.length||extra.length)mismatches.push(crate+" missing "+missing.slice(0,2).join(",")+" extra "+extra.slice(0,2).join(","));
 }
 expect(compared).toBeGreaterThanOrEqual(50);
 expect(mismatches).toEqual([]);
});
