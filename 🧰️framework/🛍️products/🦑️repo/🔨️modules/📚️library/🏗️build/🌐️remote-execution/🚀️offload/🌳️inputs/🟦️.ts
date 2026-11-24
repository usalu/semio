import {lstatSync,readdirSync,readFileSync,statSync} from "node:fs";
import {join,relative,sep} from "node:path";
import {OFFLOAD_POLICY_V1} from "../📐️policy/🟦️.ts";

const limits=OFFLOAD_POLICY_V1.limits;
/** 🌳️ The files of one directory tree by path relative to its root, in a stable order, bounded by the policy; `undefined` beyond the bounds. Directories named in `ignored` and links to directories are not entered. */
export function walkFilesV1(root:string,ignored:readonly string[],maximumFiles:number=limits.maximumInputFiles,maximumBytes:number=limits.maximumInputBytes):ReadonlyMap<string,string>|undefined{
 const found=new Map<string,string>();let bytes=0,overflow=false;
 const walk=(directory:string):void=>{
  let entries;try{entries=readdirSync(directory,{withFileTypes:true});}catch{return;}
  for(const entry of entries.sort((left,right)=>left.name<right.name?-1:1)){
   if(overflow)return;
   const path=join(directory,entry.name);
   if(entry.isDirectory()){if(!ignored.includes(entry.name))walk(path);continue;}
   let size:number;
   if(entry.isFile())size=statSync(path).size;
   else if(entry.isSymbolicLink()){try{const stat=statSync(path);if(!stat.isFile())continue;size=stat.size;}catch{continue;}}
   else continue;
   bytes+=size;
   if(found.size>=maximumFiles||bytes>maximumBytes){overflow=true;return;}
   found.set(relative(root,path).split(sep).join("/"),path);
  }
 };
 walk(root);
 return overflow?undefined:found;
}

const ARTIFACT=/^lib(.+)-([0-9a-f]{16})\.(rlib|rmeta|so|dylib|dll)$/u,SIDECAR=".semio-deps";
type RecordedDependenciesV1=Readonly<{svh:string;dependencies:readonly (readonly [string,string])[]}>;
function recordedDependenciesV1(text:string):RecordedDependenciesV1|undefined{
 try{
  const value=JSON.parse(text) as {svh?:unknown;dependencies?:unknown};
  if(typeof value.svh!=="string"||!Array.isArray(value.dependencies))return undefined;
  const rows:(readonly [string,string])[]=[];
  for(const row of value.dependencies){if(!Array.isArray(row)||typeof row[0]!=="string"||typeof row[1]!=="string")return undefined;rows.push([row[0],row[1]]);}
  return{svh:value.svh,dependencies:rows};
 }catch{return undefined;}
}
/** 🔗️ The artifacts one compile loads, never the search directory. The compiler picks a dependency by crate name and version hash (svh), not by file name, so a dependency counts as loaded when an artifact of its name carries its svh, whatever extra-filename that artifact has (a restored unit keeps the extra-filename of the build it came from, and a lane that rebuilt the same crate names it differently). The edges are the unit cache's own record of each artifact (`<rmeta>.semio-deps`: its svh and the name, extra-filename and svh of every crate it was built against), followed from the direct externs: a dependency is taken by its extra-filename when an artifact of that name exists, else by name and svh through the recorded svh of the candidates (a single candidate without a record is taken as well, several without a record all). An artifact without a record falls back to the extra-filenames in its metadata (every file of a selected artifact names the dependencies it was built against). A unit that only emits a library (`rlibs` false) reads dependencies from their rmeta alone, so their rlibs stay behind unless named directly; a unit that links needs both. The metadata of an artifact is read from its rmeta (the rlib only when no rmeta exists), a directory or file that is gone is skipped, and only the number of selected artifacts is bounded: bytes are bounded where they travel, on what the content store lacks. */
export function dependencyClosureV1(directories:readonly string[],direct:readonly string[],read:(path:string)=>Buffer=path=>readFileSync(path),rlibs:boolean=true):ReadonlyMap<string,readonly string[]>|undefined{
 const bySuffix=new Map<string,Map<string,string[]>>(),byName=new Map<string,Set<string>>(),named=new Set(direct.map(path=>path.split(/[\\/]/u).at(-1)!));
 for(const directory of directories){
  let names:string[];try{names=readdirSync(directory);}catch{continue;}
  for(const name of names){
   const match=ARTIFACT.exec(name);if(!match)continue;
   const origins=bySuffix.get(match[2]!)??new Map<string,string[]>();
   const files=origins.get(directory)??[];
   files.push(name);origins.set(directory,files);bySuffix.set(match[2]!,origins);
   const suffixes=byName.get(match[1]!)??new Set<string>();suffixes.add(match[2]!);byName.set(match[1]!,suffixes);
  }
 }
 const selected=new Map<string,string[]>(),chosen=new Set<string>(),queue:string[]=[],recorded=new Map<string,RecordedDependenciesV1|null>();
 const take=(suffix:string):void=>{if(chosen.has(suffix)||!bySuffix.has(suffix))return;chosen.add(suffix);queue.push(suffix);};
 const recordOf=(suffix:string):RecordedDependenciesV1|null=>{
  const known=recorded.get(suffix);if(known!==undefined)return known;
  let found:RecordedDependenciesV1|null=null;
  for(const [directory,files] of bySuffix.get(suffix)??[]){
   const source=files.find(file=>file.endsWith(".rmeta"))??files.find(file=>file.endsWith(".rlib"));
   if(source===undefined)continue;
   try{found=recordedDependenciesV1(read(join(directory,source+SIDECAR)).toString("utf8"))??null;}catch{found=null;}
   if(found!==null)break;
  }
  recorded.set(suffix,found);return found;
 };
 const resolve=(name:string,extra:string,svh:string):void=>{
  if(bySuffix.has(extra)){take(extra);return;}
  const candidates=[...byName.get(name)??[]];
  if(candidates.length===0)return;
  const exact=candidates.filter(suffix=>recordOf(suffix)?.svh===svh);
  if(exact.length>0){for(const suffix of exact)take(suffix);return;}
  const unrecorded=candidates.filter(suffix=>recordOf(suffix)===null);
  for(const suffix of unrecorded)take(suffix);
 };
 for(const name of named){const match=ARTIFACT.exec(name);if(match)take(match[2]!);}
 while(queue.length>0){
  if(chosen.size>limits.maximumClosureFiles)return undefined;
  const suffix=queue.shift()!,edges=recordOf(suffix);
  for(const [directory,files] of bySuffix.get(suffix)!){
   const metadata=files.some(file=>file.endsWith(".rmeta")),current=selected.get(directory)??[];
   for(const file of files){
    if(file.endsWith(".rlib")&&!rlibs&&metadata&&!named.has(file))continue;
    current.push(file);
    if(edges!==null||metadata&&!file.endsWith(".rmeta"))continue;
    let text:string;try{text=read(join(directory,file)).toString("latin1");}catch{continue;}
    for(const token of text.matchAll(/[0-9a-f]{16}/gu))take(token[0]);
   }
   selected.set(directory,current);
  }
  if(edges!==null)for(const [spec,svh] of edges.dependencies){
   const cut=spec.lastIndexOf("-");
   if(cut>0)resolve(spec.slice(0,cut),spec.slice(cut+1),svh);
  }
 }
 return new Map([...selected].map(([directory,files])=>[directory,files.sort()]));
}
