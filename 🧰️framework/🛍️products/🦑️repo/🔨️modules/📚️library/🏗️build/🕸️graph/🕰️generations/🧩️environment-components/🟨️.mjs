import{createHash}from"node:crypto";
import{compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";

const H=value=>createHash("sha256").update(JSON.stringify(value)).digest("hex"),cp=compareCodePointsV1;
const within=(root,path)=>root==="."||path===root||path.startsWith(root+"/");
const related=(a,b)=>within(a,b)||within(b,a);
const dirOf=path=>path.includes("/")?path.slice(0,path.lastIndexOf("/")):".";
const ancestorsOf=path=>{const found=[path];for(let cut=path.lastIndexOf("/");cut>0;cut=path.lastIndexOf("/",cut-1))found.push(path.slice(0,cut));if(path!==".")found.push(".");return found;};
const lower=(sorted,key)=>{let low=0,high=sorted.length;while(low<high){const mid=(low+high)>>>1;if(cp(sorted[mid][0],key)<0)low=mid+1;else high=mid;}return low;};

/** 🚩️ Whether an owner's edges are judged by the components of the environment they read (the policy flag `componentEdgeValidity`, or `SEMIO_GRAPH_COMPONENT_EDGES=1` in the environment). */
export const componentEdgeValidityOnV1=(policy,environment={})=>policy?.componentEdgeValidity===true||environment.SEMIO_GRAPH_COMPONENT_EDGES==="1";

/** 📇️ Indexes environment parts `{owners:[[name,root,native|null]],packages,modules,workspaces:[[manifest,digest]],lock,taxonomy}` once; `digest(key)` answers the digest of one component, memoised, `tableDigest` the digest of the whole table. */
export function indexEnvironmentV1(parts){
 const rows=new Map(parts.owners.map(row=>[row[0],row])),rowDigest=new Map(parts.owners.map(row=>[row[0],H(row)])),entries=[];
 for(const[name,root,native]of parts.owners){entries.push([root,name]);if(native!==null&&native!==root)entries.push([native,name]);}
 entries.sort((a,b)=>cp(a[0],b[0])||cp(a[1],b[1]));
 const byLocation=new Map();for(const entry of entries)(byLocation.get(entry[0])??byLocation.set(entry[0],[]).get(entry[0])).push(entry);
 const multi=list=>{const map=new Map();for(const[key,value]of list)(map.get(key)??map.set(key,[]).get(key)).push(value);return map;},packages=multi(parts.packages),modules=multi(parts.modules);
 const workspaces=parts.workspaces.map(([path,digest])=>[dirOf(path),digest]).sort((a,b)=>cp(a[0],b[0])||cp(a[1],b[1]));
 const strictlyInside=root=>{const start=root==="."?"":root+"/",found=[];for(let at=lower(entries,start);at<entries.length&&entries[at][0].startsWith(start);at++)if(entries[at][0]!==root)found.push(entries[at]);return found;};
 const entryDigest=list=>list.length?H(list.map(([location,name])=>[location,name,rowDigest.get(name)])):"-",memo=new Map();
 const compute=key=>{
  const kind=key[0],argument=key.slice(2);
  if(key==="T")return String(parts.taxonomy);
  if(key==="L")return parts.lock===null||parts.lock===undefined?"-":String(parts.lock);
  if(kind==="o")return rowDigest.get(argument)??"-";
  if(kind==="t")return entryDigest(strictlyInside(argument));
  if(kind==="c"){const found=[];for(const ancestor of ancestorsOf(argument))found.push(...byLocation.get(ancestor)??[]);found.push(...strictlyInside(argument));found.sort((a,b)=>cp(a[0],b[0])||cp(a[1],b[1]));return entryDigest(found);}
  if(kind==="p"||kind==="m"){const list=(kind==="p"?packages:modules).get(argument);return list?H([...list].sort(cp)):"-";}
  if(kind==="w"){const list=workspaces.filter(([directory])=>directory===argument).map(row=>row[1]).sort(cp);return list.length?H(list):"-";}
  if(kind==="W"){const list=workspaces.filter(([directory])=>related(directory,argument));return list.length?H(list):"-";}
  throw Error("Closed component kind required: "+key);
 };
 return Object.freeze({rows,digest:key=>{let found=memo.get(key);if(found===undefined)memo.set(key,found=compute(key));return found;},tableDigest:H([parts.taxonomy,parts.lock??null,parts.owners,parts.packages,parts.modules,parts.workspaces])});
}

/** 🔑️ The component keys the edges of an owner read, sorted: the taxonomy, its own row and territory, every resolved owner with its territory, the lookups that found no owner, packages, modules, workspace directories, the related workspaces of its roots when it inherits from one, the lock when it consulted it; `*` (the whole table) when nothing was recorded. */
export function recordKeysV1(row,reads,index){
 if(!reads)return["*"];
 const keys=new Set(["T","o|"+row.name]),roots=[row.root,row.nativeRoot].filter(root=>root);
 for(const root of roots)keys.add("t|"+root);
 for(const target of reads.owners??[]){keys.add("o|"+target);const found=index.rows.get(target);for(const root of found?[found[1],found[2]]:[])if(root)keys.add("t|"+root);}
 for(const path of reads.paths??[])keys.add("c|"+path);
 for(const name of reads.packages??[])keys.add("p|"+name);
 for(const name of reads.modules??[])keys.add("m|"+name);
 for(const directory of reads.workspaces??[])keys.add("w|"+directory);
 if(reads.workspaces?.length)for(const root of roots)keys.add("W|"+root);
 if(reads.lock)keys.add("L");
 return[...keys].sort(cp);
}

/** 🧾️ The digest of exactly the components of `keys` under `index`; `*` digests the whole table. */
export const readsDigestV1=(keys,index)=>keys.length===1&&keys[0]==="*"?H(["*",index.tableDigest]):H(keys.map(key=>[key,index.digest(key)]));

/** 🧩️ The judge of owner rows under one environment: `digestOf(name,row)` is the digest of the components the row's recorded reads name, `stamp(name,row)` the `{edgesReads}` a row is written with, `holds(name,row)` is true exactly when the row carries `edgesReads` and the components it read still have that digest. A row is stale under an environment change iff a component it read changed, whatever the number of changes in between. */
export function createComponentValidityV1(parts){
 const index=indexEnvironmentV1(parts),digestOf=(name,row)=>readsDigestV1(recordKeysV1({name,root:row.root,nativeRoot:row.nativeRoot},row.reads,index),index);
 return Object.freeze({digestOf,stamp:(name,row)=>({edgesReads:digestOf(name,row)}),holds:(name,row)=>typeof row.edgesReads==="string"&&digestOf(name,row)===row.edgesReads,tableDigest:index.tableDigest});
}

/** 🚦️ The judge of the build under the policy flag, or undefined when the flag is off or the environment parts are unknown (a build without them judges by the stamp alone). */
export const componentEdgesV1=(policy,environment,parts)=>parts&&componentEdgeValidityOnV1(policy,environment)?createComponentValidityV1(parts):undefined;
