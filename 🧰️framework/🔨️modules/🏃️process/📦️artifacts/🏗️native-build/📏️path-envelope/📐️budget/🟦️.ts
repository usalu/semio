import {spawnSync} from "node:child_process";
import {existsSync,readFileSync,readdirSync} from "node:fs";
import {dirname,join,posix,win32} from "node:path";
import {runtimeParseTomlV1} from "../../../../../⚙️runtime/📄️parse/🟦️.ts";
import {cargoCompilerIntermediatePathsV1} from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
const budget=schema.definitions.Budget.const,policy=schema.definitions.Policy.const;
/** 🌳️ One canonical checkout root at the longest legal spelling of its account name. */
export type CanonicalRootV1=Readonly<{id:string;root:string;flavor:"posix"|"win32"}>;
/** 📦️ One Cargo package: its library name, the names of its other units and the targets that compile it. */
export type BudgetPackageV1=Readonly<{name:string;lib:string|null;others:readonly string[];zones:readonly string[]}>;
/** 🚨️ One output or checkout path with its length. */
export type BudgetPathV1=Readonly<{subject:string;root:string;target:string;profile:string;length:number;path:string}>;
/** 🌳️ The longest checkout root of every supported platform. */
export function canonicalRootsV1():readonly CanonicalRootV1[]{return budget.roots.map(row=>({id:row.id,root:row.template.replace("{account}","a".repeat(budget.accountCharacters)),flavor:row.flavor as "posix"|"win32"}));}
/** 🛤️ The compiler lane directory with the longest lane identifier below one root. */
export function laneDirectoryV1(root:CanonicalRootV1):string{return(root.flavor==="win32"?win32:posix).join(root.root,...budget.cacheDirectory.split("/"),budget.laneDirectory,"q"+"0".repeat(budget.laneCharacters-1));}
/** 🗂️ The files that exist below one worktree and that version control tracks or would track (ignored files excluded). */
export function trackedFilesV1(workspace:string):readonly string[]{const run=spawnSync("git",["ls-files","-z","--cached","--others","--exclude-standard"],{cwd:workspace,maxBuffer:1<<30});if(run.status!==0)throw Error("Tracked file list required");return[...new Set(run.stdout.toString("utf8").split("\0").filter(Boolean))].filter(file=>existsSync(join(workspace,file)));}
const manifest=(text:string):Record<string,any>|undefined=>{try{return runtimeParseTomlV1(text);}catch{return undefined;}};
const sources=(directory:string,folder:string)=>{const full=join(directory,folder);return existsSync(full)?readdirSync(full,{withFileTypes:true}).filter(entry=>entry.isFile()&&entry.name.endsWith(".rs")).map(entry=>entry.name.slice(0,-3)):[];};
/** 🕸️ Reads every tracked local package and every locked registry package into unit names and target zones (the closure of normal and build dependencies below the zone roots). */
export function readBudgetPackagesV1(workspace:string,files:readonly string[]):readonly BudgetPackageV1[]{
 const documents:{path:string;doc:Record<string,any>}[]=[],aliases=new Map<string,string>(),local=new Map<string,{lib:string|null;others:Set<string>;dependencies:Set<string>;roots:Set<string>}>();
 for(const path of files.filter(file=>file.split("/").at(-1)==="Cargo.toml")){const doc=manifest(readFileSync(join(workspace,path),"utf8"));if(!doc)continue;documents.push({path,doc});for(const [key,value] of Object.entries<any>(doc.workspace?.dependencies??{}))aliases.set(key,value?.package??key);}
 for(const {path,doc} of documents){
  const name=doc.package?.name as string|undefined;if(!name)continue;
  const directory=join(workspace,dirname(path)),row=local.get(name)??{lib:null,others:new Set<string>(),dependencies:new Set<string>(),roots:new Set<string>()};local.set(name,row);
  if(doc.lib||existsSync(join(directory,"src","lib.rs")))row.lib=doc.lib?.name??name;
  for(const entry of[...(doc.bin??[]),...(doc.test??[]),...(doc.example??[]),...(doc.bench??[])])if(entry.name)row.others.add(entry.name);
  if(existsSync(join(directory,"src","main.rs")))row.others.add(name);
  if(existsSync(join(directory,"build.rs")))row.others.add("build-script-build");
  for(const folder of["tests","examples","benches",join("src","bin")])for(const stem of sources(directory,folder))row.others.add(stem);
  for(const table of[doc.dependencies,doc["build-dependencies"],...Object.values<any>(doc.target??{}).flatMap(entry=>[entry.dependencies,entry["build-dependencies"]])])for(const [key,value] of Object.entries<any>(table??{}))row.dependencies.add(value?.package??(value?.workspace?aliases.get(key)??key:key));
  for(const zone of budget.zones)if(zone.roots.kind==="crate-type"&&(doc.lib?.["crate-type"]??[]).includes(zone.roots.value)||zone.roots.kind==="path"&&path.includes(zone.roots.value))row.roots.add(zone.id);
 }
 const membership=new Map<string,string[]>();
 for(const zone of budget.zones){const closure=new Set<string>(),stack=[...local].filter(([,row])=>row.roots.has(zone.id)).map(([name])=>name);while(stack.length){const name=stack.pop()!;if(closure.has(name))continue;closure.add(name);stack.push(...(local.get(name)?.dependencies??[]));}for(const name of closure)membership.set(name,[...(membership.get(name)??[]),zone.id]);}
 const registry=new Set<string>();
 for(const path of files.filter(file=>file.endsWith("Cargo.lock")))for(const match of readFileSync(join(workspace,path),"utf8").matchAll(/\[\[package\]\]\nname = "([^"]+)"\nversion = "[^"]+"\n(source = "[^"]+")?/gu))if(match[2]&&!local.has(match[1]!))registry.add(match[1]!);
 const everything=budget.zones.map(zone=>zone.id);
 return Object.freeze([...[...local].map(([name,row])=>({name,lib:row.lib,others:[...row.others].sort(),zones:membership.get(name)??[]})),...[...registry].map(name=>({name,lib:name,others:["build-script-build"],zones:everything}))].sort((a,b)=>a.name<b.name?-1:1));
}
/** 📐️ The longest output path per root, target and profile, and every output path over the bound, below the lane directory of each canonical root. */
export function cargoBudgetV1(packages:readonly BudgetPackageV1[]):Readonly<{table:readonly BudgetPathV1[];offenders:readonly BudgetPathV1[]}>{
 const table:BudgetPathV1[]=[],offenders:BudgetPathV1[]=[];
 for(const root of canonicalRootsV1()){const directory=laneDirectoryV1(root);
  for(const target of budget.targets)for(const profile of budget.profiles){
   let worst:BudgetPathV1={subject:"",root:root.id,target:target.id,profile,length:0,path:""};
   for(const row of packages){
    if(target.zone&&!row.zones.includes(target.zone))continue;
    const units=[...(row.lib!==null?[{name:row.lib,kind:"library"}]:[]),...(target.zone?[]:row.others.map(name=>({name,kind:"test"})))];
    for(const unit of units){
     let longest="";for(const path of cargoCompilerIntermediatePathsV1(directory,[{packageName:row.name,name:unit.name,platform:target.directory||null,profileName:profile,kind:unit.kind,operation:"build"}],root.flavor))if(path.length>longest.length)longest=path;
     const found={subject:row.name+(unit.kind==="test"?" ("+unit.name+")":""),root:root.id,target:target.id,profile,length:longest.length,path:longest};
     if(found.length>worst.length)worst=found;if(found.length>policy.maximumCharacters)offenders.push(found);
    }
   }
   table.push(worst);
  }}
 return Object.freeze({table:Object.freeze(table),offenders:Object.freeze(offenders.sort((a,b)=>b.length-a.length||(a.subject<b.subject?-1:1)))});
}
/** 🌲️ The longest path below the trees the bootstrap generates, relative to the worktree, as measured on this machine; `undefined` when none of the trees exists. */
export function generatedTreeLongestV1(workspace:string):Readonly<{length:number;path:string}>|undefined{
 let longest:{length:number;path:string}|undefined;
 const walk=(directory:string,relative:string)=>{let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}for(const entry of entries){const path=relative+"/"+entry.name;if(!longest||path.length>longest.length)longest={length:path.length,path};if(entry.isDirectory()&&!entry.isSymbolicLink())walk(join(directory,entry.name),path);}};
 for(const tree of budget.generatedTrees)if(existsSync(join(workspace,tree)))walk(join(workspace,tree),tree);
 return longest;
}
/** 🚧️ The canonical roots below which one generated path exceeds the bound. */
export function generatedPathOffendersV1(longest:Readonly<{length:number;path:string}>):readonly BudgetPathV1[]{return canonicalRootsV1().flatMap(root=>root.root.length+1+longest.length>policy.maximumCharacters?[{subject:longest.path,root:root.id,target:"generated",profile:"-",length:root.root.length+1+longest.length,path:root.root+"/"+longest.path}]:[]);}
/** 🚧️ Every tracked file whose path below the longest canonical root exceeds the bound. */
export function sourcePathOffendersV1(files:readonly string[]):readonly BudgetPathV1[]{
 const offenders:BudgetPathV1[]=[];
 for(const root of canonicalRootsV1())for(const file of files){const length=root.root.length+1+file.length;if(length>policy.maximumCharacters)offenders.push({subject:file,root:root.id,target:"checkout",profile:"-",length,path:root.root+"/"+file});}
 return Object.freeze(offenders.sort((a,b)=>b.length-a.length));
}
