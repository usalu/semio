import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";

const KEYS_V1=Object.freeze(["owners","paths","packages","modules","workspaces"]);
const compare=compareCodePointsV1;
const dirOf=path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):".";
const inside=(root,path)=>root==="."||path===root||path.startsWith(root+"/");
const ancestorsOf=path=>{const found=[path];for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1))found.push(path.slice(0,cut));if(path!==".")found.push(".");return found;};

/** 🧵️ The recorder of what the edges of each owner read from the environment while they are computed: the owners it resolved an import or a path dependency to (`owners`), the workspace-relative directories it looked up and found no owner for (`paths`), the package names (`packages`) and Go module names (`modules`) it looked up, the Cargo workspaces its manifests inherit from (`workspaces`) and whether it consulted the lock file (`lock`). `owner(name)` hands out the handle one owner records through; `finish(name)` answers the closed record of an owner (empty when it read nothing) or undefined for an owner never seen. */
export function createEdgeReadsRecorderV1(){
 const rows=new Map();
 const row=name=>{let found=rows.get(name);if(!found)rows.set(name,found={owners:new Set(),paths:new Set(),packages:new Set(),modules:new Set(),workspaces:new Set(),lock:false});return found;};
 return Object.freeze({
  owner(name){const found=row(name);return Object.freeze({owner:target=>{found.owners.add(target);},path:path=>{found.paths.add(path);},package:target=>{found.packages.add(target);},module:target=>{found.modules.add(target);},workspace:path=>{found.workspaces.add(path);},lock:()=>{found.lock=true;}});},
  finish(name){const found=rows.get(name);if(!found)return undefined;const record={};for(const key of KEYS_V1)if(found[key].size)record[key]=[...found[key]].sort(compare);if(found.lock)record.lock=true;return record;},
 });
}

/** 🙈️ The handle of an owner nobody records: every lookup is accepted and forgotten. */
export const NO_EDGE_READS_V1=Object.freeze({owner(){},path(){},package(){},module(){},workspace(){},lock(){}});

/** 📖️ Admits one closed record of the reads of an owner: sorted lists of bounded texts, `lock` only as true. */
export function readEdgeReadsV1(value,items=65536,length=4096){
 if(value===null||typeof value!=="object"||Array.isArray(value))throw Object.assign(Error("Closed edge reads required"),{code:"GENERATION_INVALID"});
 for(const key of Object.keys(value)){
  if(key==="lock"){if(value.lock!==true)throw Object.assign(Error("Edge reads lock must be true"),{code:"GENERATION_INVALID"});continue;}
  if(!KEYS_V1.includes(key))throw Object.assign(Error("Closed edge reads required: unknown "+key),{code:"GENERATION_INVALID"});
  const list=value[key];
  if(!Array.isArray(list)||!list.length||list.length>items||list.some((text,at)=>typeof text!=="string"||!text||text.length>length||(at>0&&compare(list[at-1],text)>=0)))throw Object.assign(Error("Sorted bounded edge reads required: "+key),{code:"GENERATION_INVALID"});
 }
 return value;
}

/** 🧭️ Admits the stored parts of an environment (the owner table is not stored, it is the owner rows of the manifest). */
export function readEnvironmentPartsV1(value){
 const bad=()=>Object.assign(Error("Closed environment parts required"),{code:"GENERATION_INVALID"}),pairs=list=>Array.isArray(list)&&list.every(row=>Array.isArray(row)&&row.length===2&&row.every(text=>typeof text==="string"));
 if(value===null||typeof value!=="object"||Array.isArray(value)||Object.keys(value).sort().join()!=="lock,modules,packages,taxonomy,workspaces")throw bad();
 if(!pairs(value.packages)||!pairs(value.modules)||!pairs(value.workspaces)||![value.lock,value.taxonomy].every(digest=>digest===null||typeof digest==="string"))throw bad();
 return value;
}

/** 🔍️ How two environments differ, in the units edges read them in: the owners whose row (name, root, native root) was added, removed or changed (`names`) with every root involved (`roots`, before and after), the package and module names whose owner changed, the directories of the workspace manifests that changed, whether the lock moved, and `all` when nothing can be said (no previous parts) or the taxonomy moved. A part list is `[key, value]` pairs; `owners` rows are `[name, root, native root or null]`. */
export function diffEnvironmentV1(previous,current){
 if(!previous||previous.taxonomy!==current.taxonomy)return{all:true,names:new Set(),roots:[],packages:new Set(),modules:new Set(),workspaces:[],lock:false};
 const names=new Set(),roots=new Set(),packages=new Set(),modules=new Set(),workspaces=new Set();
 const before=new Map(previous.owners.map(row=>[row[0],row])),after=new Map(current.owners.map(row=>[row[0],row]));
 for(const name of new Set([...before.keys(),...after.keys()])){
  const old=before.get(name),now=after.get(name);
  if(old&&now&&old[1]===now[1]&&old[2]===now[2])continue;
  names.add(name);for(const row of[old,now])if(row)for(const root of[row[1],row[2]])if(root!==null)roots.add(root);
 }
 const sideBySide=(list,into)=>{const map=new Map();for(const[key,value]of list)(map.get(key)??map.set(key,new Set()).get(key)).add(value);return map;};
 for(const[field,into]of[["packages",packages],["modules",modules]]){
  const a=sideBySide(previous[field]),b=sideBySide(current[field]);
  for(const key of new Set([...a.keys(),...b.keys()])){const x=[...a.get(key)??[]].sort().join("\0"),y=[...b.get(key)??[]].sort().join("\0");if(x!==y)into.add(key);}
 }
 const oldWorkspaces=new Map(previous.workspaces),newWorkspaces=new Map(current.workspaces);
 for(const path of new Set([...oldWorkspaces.keys(),...newWorkspaces.keys()]))if(oldWorkspaces.get(path)!==newWorkspaces.get(path))workspaces.add(dirOf(path));
 return{all:false,names,roots:[...roots].sort(compare),packages,modules,workspaces:[...workspaces].sort(compare),lock:previous.lock!==current.lock};
}

/**
 * 🎯️ The owners whose edges may differ under the new environment: `rows` maps (a Map) each candidate to `{root, nativeRoot?, reads?}` as the previous generation stored it, `tables` lists name-to-`[root, native root or null]` Maps of the old and the new owner table (for the roots of the owners an owner resolved to). A candidate without recorded reads is affected. Otherwise it is affected when
 * - an owner it resolved to was added, removed or changed, or a changed root lies inside its own roots or inside the territory of an owner it resolved to (a new owner there takes over files and paths) or is related to a directory it found no owner for;
 * - a package or module name it looked up changed its owner, a workspace manifest it inherits from, or one above or inside the roots of an owner that reads Cargo manifests, changed, or the lock moved and it consulted the lock.
 * A path is inside a root when it equals the root or lies below it; the root `.` contains everything.
 */
export function edgeConsumersV1({diff,rows,tables}){
 const all=[...rows.keys()];
 if(diff.all)return new Set(all);
 const affected=new Set(),mark=names=>{for(const name of names??[])affected.add(name);};
 const byRoot=new Map(),cargoRoot=new Map(),consumersOf=new Map(),missIndex=new Map(),packageIndex=new Map(),moduleIndex=new Map(),workspaceIndex=new Map(),locked=[];
 const push=(map,key,name)=>{const list=map.get(key);if(list)list.push(name);else map.set(key,[name]);};
 for(const name of all){
  const row=rows.get(name);
  if(!row.reads){affected.add(name);continue;}
  for(const root of[row.root,row.nativeRoot])if(root){push(byRoot,root,name);if(row.reads.workspaces)push(cargoRoot,root,name);}
  for(const target of row.reads.owners??[])push(consumersOf,target,name);
  for(const path of row.reads.paths??[])push(missIndex,path,name);
  for(const target of row.reads.packages??[])push(packageIndex,target,name);
  for(const target of row.reads.modules??[])push(moduleIndex,target,name);
  for(const path of row.reads.workspaces??[])push(workspaceIndex,path,name);
  if(row.reads.lock)locked.push(name);
 }
 for(const name of diff.names)mark(consumersOf.get(name));
 for(const target of diff.packages)mark(packageIndex.get(target));
 for(const target of diff.modules)mark(moduleIndex.get(target));
 if(diff.lock)mark(locked);
 const resolvedRoots=new Map();
 for(const[target]of consumersOf)resolvedRoots.set(target,new Set([rows.get(target)?.root,rows.get(target)?.nativeRoot,...tables.flatMap(table=>table.get(target)??[])].filter(root=>root)));
 const territories=new Map();
 for(const[target,list]of resolvedRoots)for(const root of list)push(territories,root,target);
 const sortedMisses=[...missIndex.keys()].sort(compare);
 const below=(map,sorted,root)=>{
  if(root==="."){for(const key of sorted)mark(map.get(key));return;}
  mark(map.get(root));
  const start=root+"/";let low=0,high=sorted.length;
  while(low<high){const middle=(low+high)>>>1;if(compare(sorted[middle],start)<0)low=middle+1;else high=middle;}
  for(;low<sorted.length&&sorted[low].startsWith(start);low++)mark(map.get(sorted[low]));
 };
 for(const root of diff.roots){
  for(const ancestor of ancestorsOf(root)){
   mark(byRoot.get(ancestor));
   for(const target of territories.get(ancestor)??[])mark(consumersOf.get(target));
   mark(missIndex.get(ancestor));
  }
  below(missIndex,sortedMisses,root);
 }
 const cargoRoots=[...cargoRoot.keys()].sort(compare);
 for(const directory of diff.workspaces){
  mark(workspaceIndex.get(directory));
  for(const ancestor of ancestorsOf(directory))mark(cargoRoot.get(ancestor));
  below(cargoRoot,cargoRoots,directory);
 }
 return new Set(all.filter(name=>affected.has(name)));
}
