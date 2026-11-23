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

const ARTIFACT=/^lib(.+)-([0-9a-f]{16})\.(rlib|rmeta|so|dylib|dll)$/u;
/** 🔗️ The artifacts one compile loads, never the search directory: every file of a selected artifact names the dependencies it was built against (the extra-filename in its metadata), and the union of those names is followed from the direct externs. A unit that only emits a library (`rlibs` false) reads dependencies from their rmeta alone, so their rlibs stay behind unless named directly; a unit that links needs both. The metadata of an artifact is read from its rmeta (the rlib only when no rmeta exists), a directory or file that is gone is skipped, and only the number of selected artifacts is bounded: bytes are bounded where they travel, on what the content store lacks. */
export function dependencyClosureV1(directories:readonly string[],direct:readonly string[],read:(path:string)=>Buffer=path=>readFileSync(path),rlibs:boolean=true):ReadonlyMap<string,readonly string[]>|undefined{
 const bySuffix=new Map<string,Map<string,string[]>>(),named=new Set(direct.map(path=>path.split(/[\\/]/u).at(-1)!));
 for(const directory of directories){
  let names:string[];try{names=readdirSync(directory);}catch{continue;}
  for(const name of names){
   const match=ARTIFACT.exec(name);if(!match)continue;
   const origins=bySuffix.get(match[2]!)??new Map<string,string[]>();
   const files=origins.get(directory)??[];
   files.push(name);origins.set(directory,files);bySuffix.set(match[2]!,origins);
  }
 }
 const selected=new Map<string,string[]>(),chosen=new Set<string>(),queue:string[]=[];
 const take=(suffix:string):void=>{if(chosen.has(suffix)||!bySuffix.has(suffix))return;chosen.add(suffix);queue.push(suffix);};
 for(const name of named){const match=ARTIFACT.exec(name);if(match)take(match[2]!);}
 while(queue.length>0){
  if(chosen.size>limits.maximumClosureFiles)return undefined;
  const suffix=queue.shift()!;
  for(const [directory,files] of bySuffix.get(suffix)!){
   const metadata=files.some(file=>file.endsWith(".rmeta")),current=selected.get(directory)??[];
   for(const file of files){
    if(file.endsWith(".rlib")&&!rlibs&&metadata&&!named.has(file))continue;
    current.push(file);
    if(metadata&&!file.endsWith(".rmeta"))continue;
    let text:string;try{text=read(join(directory,file)).toString("latin1");}catch{continue;}
    for(const token of text.matchAll(/[0-9a-f]{16}/gu))take(token[0]);
   }
   selected.set(directory,current);
  }
 }
 return new Map([...selected].map(([directory,files])=>[directory,files.sort()]));
}
