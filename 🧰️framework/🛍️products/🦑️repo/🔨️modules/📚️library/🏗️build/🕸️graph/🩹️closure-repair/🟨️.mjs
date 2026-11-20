import{createHash}from"node:crypto";
import{existsSync}from"node:fs";
import{mkdir,mkdtemp,readFile,readdir,rm,writeFile}from"node:fs/promises";
import{tmpdir}from"node:os";
import{brotliDecompressSync}from"node:zlib";
import{join}from"node:path";

const KEY=/^[a-f0-9]{64}$/u,BATCH=64,TOKEN=/SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})/gu;
export const CLOSURE_NAMESPACES_V1=Object.freeze({SOURCE_CLOSURE:"build-input-closures",CARGO_LOCK:"build-lock-closures"});
export const CLOSURE_POLICY_V1=Object.freeze({maximumLeaves:20000,maximumLeafBytes:16777216,concurrency:8,fragments:"f"});

const recordsOf=async(root)=>{
 const folders=[join(root,"derived-nodes","n")],base=join(root,"inference","selections");
 for(const id of await readdir(base).catch(()=>[]))folders.push(join(base,id,"np"));
 return folders;
};
const mentions=async(path,key)=>{
 const bytes=await readFile(path).catch(()=>undefined);if(!bytes)return false;
 let text;try{text=brotliDecompressSync(bytes).toString("utf8");}catch{text=bytes.toString("latin1");}
 return text.includes(key);
};

/** 🩹️ Repairs the stale references to a build input closure leaf that no longer exists: removes every project inference record (`inference/selections/<id>/np`, with its fleet marker) and derived node record (`derived-nodes/n`) of the graph image `root` whose text names the leaf key, so the next build recomputes them and republishes the leaf. A removed record is only a miss; nothing else is touched. Answers the number of records removed. */
export async function purgeClosureReferencesV1({root,key,dryRun=false,cancelled=()=>false}){
 if(typeof key!=="string"||!KEY.test(key))throw Error("Closed closure key required");
 let removed=0;
 for(const folder of await recordsOf(root)){
  const names=(await readdir(folder).catch(()=>[])).filter(name=>!name.endsWith(".fleet")&&!name.endsWith(".tmp"));
  for(let at=0;at<names.length;at+=BATCH){
   if(cancelled())return removed;
   await Promise.all(names.slice(at,at+BATCH).map(async name=>{
    if(!await mentions(join(folder,name),key))return;
    removed++;if(dryRun)return;
    await rm(join(folder,name),{force:true});await rm(join(folder,name+".fleet"),{force:true});
   }));
  }
 }
 return removed;
}

/** 🧭️ The closure leaves a text names: `SEMIO_SOURCE_CLOSURE_<sha256>` and `SEMIO_CARGO_LOCK_<sha256>` tokens, as namespace and key, each once. */
export function closureReferencesOfTextV1(text){
 const found=new Map();
 for(const match of text.matchAll(TOKEN))found.set(match[2],CLOSURE_NAMESPACES_V1[match[1]]);
 return found;
}

/** 🚫️ The one failure of an absent closure leaf: it is not an ENOENT (the code is `closure-missing`), it carries the key, the namespace and the projects that name it, and the load answers it by obtaining the leaf or re-deriving those projects. */
export function closureMissingErrorV1({namespace,key,projects=[],cause}){
 return Object.assign(new Error("closure-missing:"+key+" "+namespace+" leaf is not on this machine",cause===undefined?undefined:{cause}),{code:"closure-missing",missingClosure:key,missingNamespace:namespace,missingProjects:projects});
}

/** 🗄️ Whether the leaf is on this machine. */
export function closureLeafPresentV1(directory,key){return existsSync(join(directory,key+".json"));}

/** 🧲️ Obtains one closure leaf, the only way a leaf that is not on this machine comes back: on this machine already (`present`), else fetched from the fleet cache, verified against its name (the key is the sha256 of its bytes) and bounded, and written by `put` (`refetched`), else `missing`. A fetch that fails, times out or returns other bytes is `missing`. A strict caller may verify held bytes before and after writing. */
export async function obtainClosureLeafV1({directory,namespace,key,fetch,put,signal,verify,maximumBytes=CLOSURE_POLICY_V1.maximumLeafBytes}){
 if(typeof key!=="string"||!KEY.test(key))throw Error("Closed closure key required");
 if(verify?await verify(namespace,key):closureLeafPresentV1(directory(namespace),key))return"present";
 if(!fetch||!put||signal?.aborted)return"missing";
 try{
  const bytes=await fetch(namespace,key,signal);
  if(!bytes||bytes.length<1||bytes.length>maximumBytes||createHash("sha256").update(bytes).digest("hex")!==key)return"missing";
  await put(namespace,key,bytes);
  const present=verify?await verify(namespace,key):closureLeafPresentV1(directory(namespace),key);
  return present?"refetched":"missing";
 }catch{return"missing";}
}

/** 📖️ Reads one closure leaf through `read`; an absent leaf is first obtained (`obtain`) and read again, and stays a typed {@link closureMissingErrorV1} (`projects` may be a function, asked only on a miss), never an ENOENT. */
export async function readClosureLeafV1({namespace,key,read,obtain,projects}){
 try{return read();}
 catch(error){
  if(error?.code!=="ENOENT")throw error;
  if(obtain&&await obtain(namespace,key)!=="missing")try{return await read();}catch(again){if(again?.code!=="ENOENT")throw again;}
  throw closureMissingErrorV1({namespace,key,projects:typeof projects==="function"?projects():projects,cause:error});
 }
}

/** 🩹️ The closure leaves a set of generation fragments name, each with the fragments that name it: `read(name)` answers the brotli bytes of fragment `name` (or nothing), `memo` remembers the leaves of fragments seen before (a fragment is content addressed, so its leaves never change). */
export async function closuresOfFragmentsV1({names,read,memo=new Map(),concurrency=CLOSURE_POLICY_V1.concurrency,cancelled=()=>false}){
 const named=new Map();let next=0;
 await Promise.all(Array.from({length:Math.min(concurrency,names.length)},async()=>{
  while(next<names.length&&!cancelled()){
   const name=names[next++];let leaves=memo.get(name);
   if(!leaves){
    const bytes=await read(name);if(!bytes)continue;
    let text;try{text=brotliDecompressSync(bytes).toString("utf8");}catch{text=Buffer.from(bytes).toString("latin1");}
    leaves=[...closureReferencesOfTextV1(text)];memo.set(name,leaves);
   }
   for(const[key,namespace]of leaves){const row=named.get(key)??named.set(key,{namespace,fragments:new Set()}).get(key);row.fragments.add(name);}
  }
 }));
 return named;
}

/** 👥️ The owners that name each leaf of `named`, from the owner table of a manifest (`owners[name].fragment`). */
export function closureOwnersV1(named,owners){
 const byFragment=new Map();
 for(const[name,row]of Object.entries(owners)){const list=byFragment.get(row.fragment)??byFragment.set(row.fragment,[]).get(row.fragment);list.push(name);}
 return new Map([...named].map(([key,row])=>[key,{namespace:row.namespace,projects:[...row.fragments].flatMap(fragment=>byFragment.get(fragment)??[]).sort()}]));
}

/** 📣️ The one log line of a leaf that was not on this machine. */
export function closureMissingLineV1({key,count,outcome}){return`[graph] closure-missing ${key} named by ${count} projects: ${outcome}`;}

/** 🧰️ Settles one absent closure leaf inside a request: the leaf is obtained (`refetched`), else the projects that name it are marked stale by removing their records (`purge`) so the normal inference path derives them again (`re-derived`, or `asked of the fleet` when `heavy(projects)` says the fleet must derive them). Logs the line. */
export async function settleClosureLeafV1({key,namespace,projects=[],obtain,purge,heavy,say=line=>process.stderr.write(line+"\n")}){
 const outcome=await obtain(namespace,key);
 let result;
 if(outcome==="present"||outcome==="refetched")result="refetched";
 else{await purge(key);result=heavy&&projects.length&&await heavy(projects)?"asked of the fleet":"re-derived";}
 say(closureMissingLineV1({key,count:projects.length,outcome:result}));
 return result;
}

/** 💾️ Writes a verified leaf under the writer custody of the graph store through the content addressed adoption (`adoptStagedGraphLeavesV1`), staged outside the worktree. */
export function closureLeafPutterV1(workspaceRoot){
 return async(namespace,key,bytes)=>{
  const{adoptStagedGraphLeavesV1}=await import("../../📥️inputs/🟨️.mjs"),staging=await mkdtemp(join(tmpdir(),"semio-closure-"));
  try{await mkdir(join(staging,namespace),{recursive:true});await writeFile(join(staging,namespace,key+".json"),bytes);adoptStagedGraphLeavesV1(workspaceRoot,staging,[namespace+"/"+key+".json"]);}
  finally{await rm(staging,{recursive:true,force:true});}
 };
}
