import{pendingCratePathsV1}from"./🕓️pending-crates/🟨️.mjs";
import{readEdgeReadsV1,readEnvironmentPartsV1}from"./🧵️edge-reads/🟨️.mjs";
import{createBoundedCacheV1}from"../../🧊️bounded/🟨️.mjs";
import{createHash,randomBytes}from"node:crypto";
import{brotliCompressSync,brotliDecompressSync,constants as zlibConstants}from"node:zlib";
import{lstat,mkdir,open,readdir,readFile,rename,rm}from"node:fs/promises";
import{existsSync,readFileSync,readdirSync}from"node:fs";
import{dirname,join,sep}from"node:path";
import{createProjectFileIndexV1}from"../🧭️file-owner/🟨️.mjs";
import{graphStringInternerV1,internJsonStringsV1}from"../../🧠️memory/🟨️.mjs";
import{canonicalJsonV1,compareCodePointsV1,decodeFactIndexV1,encodeFactIndexV1,openFactIndexV1}from"./🧮️fact-codec/🟨️.mjs";
import{coverageOfV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"./🔭️coverage/🟨️.mjs";
import{INDEX_CHUNK_POLICY_V1,addOwnerRunsV1,bulkUpsertClassesV1,classesOfChunksV1,createChunkInternerV1}from"./🧱️chunk-classes/🟨️.mjs";
import{createFrontierV1,diffFrontiersV1,frontierFactHoldsV1,frontierHoldsV1,stableDirectoriesV1}from"./🧭️frontier/🟨️.mjs";
import{mergeIntoHeadV1}from"./🔀️merge/🟨️.mjs";
import{createClosureSettlerV1,sharedClosureSettlerV1}from"../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createCasStoreV1}from"../../🗄️cas/🗃️store/🟦️.ts";
import{createNodeStoreV1}from"../🧮️derived-nodes/💾️store/🟨️.mjs";
import{selectDoubtfulRowsV1}from"./🔎️doubt/🟨️.mjs";
import{derivedOwnerNameV1,derivedUpsertsV1,holdersOfDerivedV1,isDerivedOwnerV1,orphanedDerivedOwnersV1,scopeWithDerivedV1,splitStaleConsumersV1,suspectRowsV1,suspectsWithinV1,unsupportedDerivedV1,verifiableRowIdsV1}from"./✂️cutoff/🟨️.mjs";
import{GENERATION_POLICY_V1,P,HEX,sha256,isObject,unpack,aborted,fail,requireKeys,requireHex,requireCount,requireText}from"./🧱️guards/🟨️.mjs";
import{ROW_KEY_TEXT_LIMIT_V1,createFactIndexV1 as createFactIndexRowsV1,factConsumersV1,patchFactIndexV1,refineFactIndexV1,rowKeyV1,selectFactClassesV1,staleConsumersV1,staleFactRowsV1}from"./🧮️fact-rows/🟨️.mjs";
export{GENERATION_POLICY_V1,ROW_KEY_TEXT_LIMIT_V1,factConsumersV1,patchFactIndexV1,refineFactIndexV1,rowKeyV1,selectFactClassesV1,staleConsumersV1,staleFactRowsV1};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

export{canonicalJsonV1,compareCodePointsV1,createFrontierV1,diffFrontiersV1,frontierFactHoldsV1,frontierHoldsV1,stableDirectoriesV1};


/** 🔌️ Admits bounded consumer external roots without changing their sealed package identity. */
export function readGenerationExternalInputsV1(value){const rule=schema.definitions.ExternalInputs;if(!Array.isArray(value)||value.length>rule.maxItems||value.some(name=>{if(typeof name!=="string"||!name.length)return true;let length=0;for(const point of name)if(++length>rule.items.maxLength)return true;return false;}))throw fail("Bounded external input references required","GENERATION_EXTERNAL_INPUTS");return Object.freeze([...new Set(value)].sort(compareCodePointsV1));}
/** 🏷️ Resolves exact external keys before root npm package references, including scoped names. */
export function resolveGenerationExternalInputV1(nodes,name){if(Object.hasOwn(nodes??{},name))return name;const canonical="npm:"+name;return Object.hasOwn(nodes??{},canonical)?canonical:undefined;}

/** 📦️ Refuses canonical UTF8 and compressed documents beyond the same finite reader byte ceiling before publication. */
export function packGenerationDocumentV1(value,maximumBytes){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>P.maximumIndexBytes)throw fail("Finite generation document byte limit required");
 const text=canonicalJsonV1(value),expandedBytes=Buffer.byteLength(text,"utf8");if(expandedBytes>maximumBytes)throw fail("Stored document exceeds its finite limit ("+expandedBytes+" bytes expanded, limit "+maximumBytes+")");
 const bytes=brotliCompressSync(Buffer.from(text,"utf8"),{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:P.brotliQuality,[zlibConstants.BROTLI_PARAM_LGWIN]:P.brotliWindow,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:expandedBytes}});
 if(bytes.length>maximumBytes)throw fail("Stored document exceeds its finite limit ("+bytes.length+" bytes packed, limit "+maximumBytes+")");return bytes;
}
/** 🗜️ Packs the logical fact index into its binary document (`🧮️fact-codec`) and brotli: refuses an expanded or packed size beyond the finite reader byte ceiling before publication and answers the expanded size. */
export function packFactIndexV1(index,maximumBytes){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>P.maximumIndexBytes)throw fail("Finite generation document byte limit required");
 const raw=encodeFactIndexV1(index,{maximumBytes}),bytes=brotliCompressSync(raw,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:P.brotliQuality,[zlibConstants.BROTLI_PARAM_LGWIN]:P.brotliWindow,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:raw.length}});
 if(bytes.length>maximumBytes)throw fail("Stored document exceeds its finite limit ("+bytes.length+" bytes packed, limit "+maximumBytes+")");return{bytes,expandedBytes:raw.length};
}
/** 📖️ Reads a packed fact index within the finite reader ceiling straight from the binary document, without any intermediate text. */
export function unpackFactIndexV1(bytes,maximumBytes){return readFactIndexV1(decodeFactIndexV1(brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),{maximumBytes,share:text=>graphStringInternerV1.share(text)}));}
/** 🪶️ Opens a packed fact index for verification only: strings, facts and rows stay in the document until they are read (`🧮️fact-codec` `openFactIndexV1`). */
export function openPackedFactIndexV1(bytes,maximumBytes,name){return openFactIndexV1(brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),{maximumBytes,name,share:text=>graphStringInternerV1.share(text)});}

/** 📌️ Admits the closed pointer to the newest generation: a manifest digest, or null after an explicit invalidation. */
export function readGenerationPointerV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","generation","manifest","parent"],["reason","verified"],"generation pointer");
 if(value.version!==1)throw fail("Generation pointer version 1 required");
 requireCount(value.generation,"generation");
 if(value.manifest!==null)requireHex(value.manifest,"manifest");
 if(value.parent!==null)requireHex(value.parent,"parent");
 if(value.reason!==undefined)requireText(value.reason,"reason",P.reasonMaximumCharacters);
 if(value.verified!==undefined)readVerifiedStampV1(value.verified);
 return Object.freeze(value);
}

/** 📜️ Admits one closed immutable generation manifest. */
export function readGenerationManifestV1(bytes){
 const value=JSON.parse(Buffer.from(bytes).toString("utf8"));
 requireKeys(value,["version","generation","parent","identity","implementation","env","membership","frontier","factIndex","global","owners","built"],["envParts"],"generation manifest");
 if(value.envParts!==undefined)readEnvironmentPartsV1(value.envParts);
 if(value.version!==1)throw fail("Generation manifest version 1 required");
 requireCount(value.generation,"generation");
 if(value.parent!==null)requireHex(value.parent,"parent");
 for(const key of["identity","implementation","env","membership","factIndex","global"])requireHex(value[key],key);
 if(value.frontier!==null)requireHex(value.frontier,"frontier");
 if(!isObject(value.owners)||Object.keys(value.owners).length>P.maximumOwners)throw fail("Bounded owner table required");
 for(const[name,row]of Object.entries(value.owners)){
  requireText(name,"owner name");requireKeys(row,["root","kind","fragment","bytes","edgesEnv","targets"],["config","nativeRoot","tag","reads"],"generation owner");if(row.reads!==undefined)readEdgeReadsV1(row.reads);
  requireText(row.root,"owner root");if(!["project","unavailable"].includes(row.kind))throw fail("Closed owner kind required");
  requireHex(row.fragment,"fragment");requireHex(row.edgesEnv,"edgesEnv");requireCount(row.bytes,"fragment bytes",P.maximumFragmentBytes);
  if(!Array.isArray(row.targets)||row.targets.some(target=>typeof target!=="string"))throw fail("Target name list required");
  if(row.config!==undefined)requireText(row.config,"owner config");if(row.nativeRoot!==undefined)requireText(row.nativeRoot,"owner native root");if(row.tag!==undefined)requireText(row.tag,"owner tag",64);
 }
 requireKeys(value.built,["kind","base","dirty"],[],"generation provenance");
 if(!["cold","delta","refresh"].includes(value.built.kind))throw fail("Closed provenance kind required");
 if(value.built.base!==null)requireCount(value.built.base,"base generation");
 if(!Array.isArray(value.built.dirty)||value.built.dirty.some(name=>typeof name!=="string"))throw fail("Dirty owner list required");
 return Object.freeze(value);
}

/** 🧩️ Admits one closed owner fragment: the neutral node, its outgoing edges and the environment those edges were computed under. */
export function readGenerationFragmentV1(value){
 requireKeys(value,["version","name","root","kind","node","edges","edgesEnv","targets"],["config","nativeRoot"],"generation fragment");
 if(value.version!==1)throw fail("Generation fragment version 1 required");
 requireText(value.name,"fragment name");requireText(value.root,"fragment root");requireHex(value.edgesEnv,"edgesEnv");
 if(!["project","unavailable"].includes(value.kind)||!Array.isArray(value.edges))throw fail("Closed fragment required");
 return value;
}

function readFactIndexV1(value){
 requireKeys(value,["version","owners","rows","classes"],[],"fact index");
 if(value.version!==1||!Array.isArray(value.owners)||!Array.isArray(value.rows)||!Array.isArray(value.classes))throw fail("Closed fact index required");
 const owners=value.owners.length,rows=value.rows.length;
 for(const cls of value.classes){
  requireKeys(cls,["r","o"],[],"fact class");
  if(!Array.isArray(cls.r)||!Array.isArray(cls.o)||cls.r.some(id=>!Number.isSafeInteger(id)||id<0||id>=rows)||cls.o.some(id=>!Number.isSafeInteger(id)||id<0||id>=owners))throw fail("Fact class references out of range");
 }
 return value;
}

export function createFactIndexV1(ownerRows){
 let rows=ownerRows;
 for(const list of ownerRows.values())if(typeof list==="string"){
  rows=new Map();
  for(const[name,value]of ownerRows)rows.set(name,typeof value==="string"?JSON.parse(readFileSync(value,"utf8")):value);
  break;
 }
 return createFactIndexRowsV1(rows);
}
/** 🗂️ The filesystem port on real files: a missing file reads undefined, writes are atomic by rename, a content addressed write that finds its file does nothing. */
export function createNodeFilePortV1(){
 return Object.freeze({
  async read(path){try{return await readFile(path);}catch(error){if(error?.code==="ENOENT"||error?.code==="ENOTDIR")return undefined;throw error;}},
  async write(path,bytes,{ifAbsent=false}={}){
   if(ifAbsent){try{await lstat(path);return false;}catch(error){if(error?.code!=="ENOENT")throw error;}}
   await mkdir(dirname(path),{recursive:true});
   const temporary=path+"."+randomBytes(8).toString("hex")+".tmp";
   try{const file=await open(temporary,"wx");try{await file.writeFile(bytes);}finally{await file.close();}await rename(temporary,path);}finally{await rm(temporary,{force:true});}
   return true;
  },
  async names(directory){try{return await readdir(directory);}catch(error){if(error?.code==="ENOENT")return[];throw error;}},
  async list(directory){
   let names;try{names=await readdir(directory);}catch(error){if(error?.code==="ENOENT")return[];throw error;}
   const rows=await Promise.all(names.map(async name=>{try{const status=await lstat(join(directory,name));return status.isFile()?{name,modifiedMs:status.mtimeMs}:undefined;}catch(error){if(error?.code==="ENOENT")return undefined;throw error;}}));
   return rows.filter(Boolean);
  },
  async remove(path){await rm(path,{force:true});},
 });
}

const defaultSleep=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason);return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason);},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

async function mapLimit(items,limit,visit,signal){
 const out=new Array(items.length);let next=0;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{while(next<items.length){aborted(signal);const at=next++;out[at]=await visit(items[at],at);}}));
 return out;
}

/** 🚧️ The withheld owners of a generation that do not hold the source that withholds them: an untagged `unavailable` owner names the file that does not parse (`config`), and it stays valid only while the bytes of that file are the ones it was withheld for, so a generation that carries no fact about that file (a withholding recorded before its facts were kept) can never notice the repair and its owner must be inferred again. Sources outside the workspace and tagged owners (rediscovered as a group) are not judged. The judgement is global: a repair is bounded by the withheld owners and one build heals all of them, instead of one build per layer of the closure they open. */
export function unfoundedWithheldOwnersV1(index,owners,separator=sep){
 const open=new Map(),kinds=new Set(["file","analysis","digest"]);
 for(const[name,row]of Object.entries(owners)){
  if(row.kind!=="unavailable"||row.tag||typeof row.config!=="string"||row.config.startsWith(".."))continue;
  const at=index.owners.indexOf(name);if(at>=0)open.set(at,[name,row.config]);
 }
 for(const cls of index.classes){
  if(!open.size)break;
  const here=cls.o.filter(at=>open.has(at));if(!here.length)continue;
  const configs=new Set(here.map(at=>open.get(at)[1]));
  for(const id of cls.r){
   const row=index.rows[id];if(!kinds.has(row.kind))continue;
   const path=separator==="/"?row.path:row.path.split(separator).join("/");
   for(const config of configs)if(path===config||path.endsWith("/"+config)){for(const at of here)if(open.get(at)?.[1]===config)open.delete(at);configs.delete(config);}
   if(!configs.size)break;
  }
 }
 return[...open.values()].map(([name])=>name);
}

/** 🧠️ The answers of the derived questions of one store root: the node store under `root` (unless `durable` is false) and the settler that keeps the graph of the answers in memory, so the readers and the builder of one load ask each question once. */
export function createAnswersV1({root,fs=createNodeFilePortV1(),durable=true,workspaceRoot}){
 if(durable&&workspaceRoot!==undefined){
  const settler=sharedClosureSettlerV1({workspaceRoot,cacheRoot:dirname(root),blobs:createCasStoreV1()});
  return Object.freeze({nodes:settler.store,settler});
 }
 const nodes=durable?createNodeStoreV1({root,fs:{read:path=>fs.read(path),write:(path,bytes)=>fs.write(path,bytes),mkdir:async()=>undefined,remove:path=>fs.remove(path),list:async directory=>(await fs.list(directory)).map(file=>({path:join(directory,file.name),mtimeMs:file.modifiedMs}))}}):undefined;
 return Object.freeze({nodes,settler:createClosureSettlerV1({store:nodes})});
}

/**
 * 🛠️ Creates the generation store over `root`: readers never take a lease; a builder publishes the next immutable generation with one compare-and-swap of the pointer under an exclusive hold that covers one small read and one rename.
 * `holds(rows, signal)` answers whether every observation row still holds; `membership(previous, signal)` answers `{digest, frontier, changes}` against the previous frontier (`changes` is meaningful only with a previous frontier); `compute(spec, signal)` refreshes exactly the dirty owners.
 */
export function createGenerationStoreV1({root,fs=createNodeFilePortV1(),lease,holds,doubt,derive,answers=createAnswersV1({root,fs}),resources,now=()=>Date.now(),sleep=defaultSleep,stamp=P.stampVerification}){
 if(typeof root!=="string"||!root||typeof holds!=="function"||!resources?.facts)throw fail("Generation store root, fact check and facts resource required");
 const{nodes,settler}=answers;
 const stats={holdCalls:0,rowsChecked:0,computed:0,published:0,casConflicts:0,merged:0,mergeConflicts:0,publishHeldMs:[],indexLoads:0,fragmentReads:0,indexViews:0};
 const at=(directory,digest,extension)=>join(root,directory,digest+extension),pointerPath=join(root,P.pointer),indexCache=createBoundedCacheV1({name:"generation:indexes",maxEntries:P.indexCacheEntries,maxBytes:P.indexCacheBytes,sizeOf:index=>index.stats?index.stats().bytes:index.rows.length*P.indexRowBytes,onEvict:(_key,index)=>index.drop?.()});
 const manifestPath=digest=>at(P.manifests,digest,".json"),fragmentPath=digest=>at(P.fragments,digest,".json.br"),indexPath=digest=>at(P.indexes,digest,".facts.br"),frontierPath=digest=>at(P.frontiers,digest,".json.br");
 const hold=async(rows,signal)=>{stats.holdCalls++;stats.rowsChecked+=rows.length;return holds(rows,signal);};
 const readPointer=async()=>{const bytes=await fs.read(pointerPath);return bytes?readGenerationPointerV1(bytes):undefined;};

 /** 🔖️ Reads the newest sealed generation without any lease; a manifest retired between the two reads is followed by re-reading the pointer, so a result is always one complete generation. */
 async function read(signal){
  for(let attempt=0;attempt<8;attempt++){
   aborted(signal);const pointer=await readPointer();if(!pointer)return undefined;
   if(pointer.manifest===null)return{pointer,manifest:undefined};
   const bytes=await fs.read(manifestPath(pointer.manifest));
   if(!bytes){if(attempt===7)return{pointer,manifest:undefined,missing:true};await sleep(5*(attempt+1),signal);continue;}
   if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==pointer.manifest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");
   let manifest;try{manifest=readGenerationManifestV1(bytes);}catch{return{pointer,manifest:undefined};}if(manifest.generation!==pointer.generation)throw fail("Generation manifest differs from its pointer","GENERATION_CORRUPT");
   return{pointer,manifest};
  }
  throw fail("Generation manifest unreadable","GENERATION_UNREADABLE");
 }
 /** 🗞️ Reads one sealed generation by the digest of its manifest although a newer pointer replaced it; a retired manifest answers undefined. */
 async function readManifestAt(digest,signal){
  aborted(signal);requireHex(digest,"manifest");const bytes=await fs.read(manifestPath(digest));if(!bytes)return undefined;
  if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==digest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");
  try{return readGenerationManifestV1(bytes);}catch{return undefined;}
 }
 async function readIndex(digest){
  const cached=indexCache.get(digest);if(cached)return cached;
  const bytes=await fs.read(indexPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Fact index content differs from its name","GENERATION_CORRUPT");
  const index=unpackFactIndexV1(bytes,P.maximumIndexBytes);stats.indexLoads++;
  indexCache.set(digest,index);return index;
 }
 async function readIndexView(digest){
  const key="view:"+digest,cached=indexCache.get(key)??indexCache.get(digest);if(cached)return cached;
  const bytes=await fs.read(indexPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Fact index content differs from its name","GENERATION_CORRUPT");
  const index=openPackedFactIndexV1(bytes,P.maximumIndexBytes,"fact-index:"+digest.slice(0,12));stats.indexViews++;
  indexCache.set(key,index);return index;
 }
 async function readFrontier(digest){
  if(digest===null)return undefined;const bytes=await fs.read(frontierPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Frontier content differs from its name","GENERATION_CORRUPT");return unpack(bytes,P.maximumIndexBytes);
 }
 async function readFragment(row){
  const bytes=await fs.read(fragmentPath(row.fragment));if(!bytes)return undefined;stats.fragmentReads++;
  if(sha256(bytes)!==row.fragment)throw fail("Fragment content differs from its name","GENERATION_CORRUPT");
  return readGenerationFragmentV1(internJsonStringsV1(unpack(bytes,P.maximumFragmentBytes),graphStringInternerV1));
 }
 /** 🌐️ Reads the global fragment of one manifest: the external nodes, the cross-owner edges and the parse failures. */
 async function readGlobal(manifest){const bytes=await fs.read(fragmentPath(manifest.global));if(!bytes||sha256(bytes)!==manifest.global)throw fail("Global fragment is missing","GENERATION_INCOMPLETE");return unpack(bytes,P.maximumFragmentBytes).value??{};}
 /** 📇️ Lists the projects (name, root, target names) and the unavailable projects of one sealed generation from the manifest and the few unavailable fragments, without reading any project node. */
 async function catalog(current,{signal}={}){
  const manifest=current?.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const rows=Object.entries(manifest.owners),global=await readGlobal(manifest);
  const loaded=await mapLimit(rows.filter(([,row])=>row.kind==="unavailable"),P.readConcurrency,async([name,row])=>{const fragment=await readFragment(row);if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment.node;},signal);
  return{projects:rows.filter(([,row])=>row.kind==="project").map(([name,row])=>({name,root:row.root,targets:row.targets})),unavailable:[...(global.unavailable??[]),...loaded].sort((a,b)=>compareCodePointsV1(a.root,b.root))};
 }
 const keep=async(directory,bytes,extension)=>{const digest=sha256(bytes);await fs.write(at(directory,digest,extension),bytes,{ifAbsent:true});return{digest,bytes:bytes.length};};
 const store=(directory,value,extension,limit)=>keep(directory,packGenerationDocumentV1(value,limit),extension);
 const writeManifest=async manifest=>{const bytes=Buffer.from(canonicalJsonV1(readGenerationManifestV1(Buffer.from(canonicalJsonV1(manifest)))));if(bytes.length>P.maximumManifestBytes)throw fail("Generation manifest exceeds its finite limit");const digest=sha256(bytes);await fs.write(manifestPath(digest),bytes,{ifAbsent:true});return digest;};

 /**
  * 🔍️ Verifies the facts of the selected owners (every owner without a selection) and maps the observed membership changes to their owners through the roots of the manifest.
  * `changes` undefined means the caller cannot diff membership: the plan is cold.
  */
 async function plan({current,identity,changes,journalDirty,only,skipEnvironment=false,signal,progress}){
  const cold=reason=>({kind:"cold",reason,dirty:new Set(),added:[],removed:[],pending:[],envStale:true,checked:0});
  if(!current?.manifest)return cold(current?(current.missing?"manifest":"invalidated"):"empty");
  const manifest=current.manifest;
  if(manifest.identity!==identity)return cold("identity");
  if(changes===undefined)return cold("frontier");
  const index=await readIndexView(manifest.factIndex);if(!index)return cold("index");
  const present=new Set(await fs.names(join(root,P.fragments)));if(!present.has(manifest.global+".json.br"))return cold("global");
  const dirty=new Set(),added=[],removed=new Set(),reasons=[],owners=Object.entries(manifest.owners),ownerOf=createProjectFileIndexV1(Object.fromEntries(owners.map(([name,row])=>[name,{root:row.root}])));
  const configs=new Map(owners.filter(([,row])=>row.config!==undefined).map(([name,row])=>[row.config,name]));
  let envStale=false;
  const pending=pendingCratePathsV1(changes,ownerOf);
  for(const change of changes){
   const known=configs.get(change.path);
   if(pending.has(change.path))continue;
   if(change.config&&change.kind==="added"){added.push(change.path);const prior=ownerOf(change.path);if(prior!==undefined&&manifest.owners[prior].config!==change.path&&change.path.startsWith(manifest.owners[prior].root+"/")&&!change.path.slice(manifest.owners[prior].root.length+1).includes("/"))removed.add(prior);}
   else if(change.config&&change.kind==="removed"&&known)removed.add(known);
   else if(change.config&&known)dirty.add(known);
   else if(change.directory&&change.kind!=="removed")continue;
   else{const owner=ownerOf(change.path);if(owner!==undefined)dirty.add(owner);else{envStale=true;if(change.kind!=="removed")added.push(change.path);}}
  }
  for(const[name,row]of owners)if((!only||only.has(name))&&!present.has(row.fragment+".json.br"))dirty.add(name);
  const unfounded=unfoundedWithheldOwnersV1(index,manifest.owners);for(const name of unfounded)dirty.add(name);
  const scope=only?new Set(skipEnvironment?only:[...only,P.envOwner]):undefined,selection=selectFactClassesV1(index,scope?scopeWithDerivedV1(index,scope):undefined);
  const verifiable=verifiableRowIdsV1(index,selection.ids),candidates=doubt?await selectDoubtfulRowsV1(index,verifiable,doubt,{signal,progress,chunk:P.doubtChunkRows}):verifiable;
  const{stale}=await staleFactRowsV1(index,candidates,hold,{signal,progress,...doubt?{budget:Infinity}:{}});
  await doubt?.settled?.({rows:index.rows,ids:verifiable,stale});
  const{direct,suspects:reached}=splitStaleConsumersV1(index,selection.classes,stale),suspects=scope?suspectsWithinV1(index,reached,scope):reached,settled=await settler.settle({pseudoRows:suspectRowsV1(index,suspects),derive,signal,progress,hold,...journalDirty?{dirty:journalDirty}:{}}),changedHolders=holdersOfDerivedV1(index,settled.changed.keys());
  await settler.release?.();
  for(const name of direct){if(name===P.envOwner)envStale=true;else if(manifest.owners[name])dirty.add(name);}
  for(const holders of changedHolders.values())for(const name of holders)if(manifest.owners[name])dirty.add(name);
  for(const change of changes){
   if(reasons.length>=P.reasonRows)break;if(change.directory&&change.kind!=="removed"||pending.has(change.path))continue;
   const owner=configs.get(change.path)??ownerOf(change.path);reasons.push({change:{path:change.path,kind:change.kind},owners:[owner??P.envOwner]});
  }
  for(const id of[...stale].sort((a,b)=>a-b)){
   if(reasons.length>=P.reasonRows)break;
   const names=[];
   for(const at of selection.classes){const cls=index.classes[at];if(cls.r.includes(id))for(const owner of cls.o){const name=index.owners[owner];if(!isDerivedOwnerV1(name)&&!names.includes(name))names.push(name);if(names.length>=3)break;}if(names.length>=3)break;}
   if(!names.length)continue;
   reasons.push({row:index.rows[id],owners:names});
  }
  for(const[name,row]of settled.changed){if(reasons.length>=P.reasonRows)break;const names=[...changedHolders.get(name)??[]].sort(compareCodePointsV1).slice(0,3);if(names.length)reasons.push({row,owners:names});}
  for(const name of removed)dirty.delete(name);
  const scoped=only?new Set([...[...dirty].filter(name=>only.has(name)),...unfounded]):dirty,clean=!scoped.size&&!added.length&&!removed.size&&!envStale;
  const staleKinds={};for(const id of stale){const kind=index.rows[id].kind;staleKinds[kind]=(staleKinds[kind]??0)+1;}
  return{kind:clean?"clean":"delta",dirty:scoped,added,removed:[...removed],pending:[...pending].sort(compareCodePointsV1),envStale,checked:selection.ids.length,stale:stale.size,doubtful:candidates.length,staleKinds,reasons,cutoff:{questions:suspects.size,held:settled.held.size,kept:settled.kept.size,changed:settled.changed.size,failed:settled.failed,reused:settled.reused},refresh:settled.held};
 }

 /** 🤝️ Swaps the pointer for the next generation only if it still names the generation the builder started from; the exclusive hold covers one small read and one rename and nothing else. */
 async function publish({manifestDigest,expectedGeneration,reason,verified,signal}){
  const body=async()=>{
   const started=now(),pointer=await readPointer();
   if((pointer?.generation??-1)!==expectedGeneration){stats.casConflicts++;stats.publishHeldMs.push(now()-started);return{published:false,pointer};}
   const next={version:1,generation:expectedGeneration+1,manifest:manifestDigest,parent:pointer?.manifest??null,...(reason?{reason}:{}),...(verified?{verified}:{})};
   await fs.write(pointerPath,Buffer.from(canonicalJsonV1(next)));stats.published++;stats.publishHeldMs.push(now()-started);return{published:true,pointer:next};
  };
  return lease?lease.exclusive(resources.facts,body,{signal,timeoutMs:P.publishWaitMilliseconds}):body();
 }

 /** 🏗️ Builds and publishes the next generation from the newest one: the plan names the dirty owners, `compute` refreshes exactly those and unchanged fragments are carried over by digest. The verification view of the fact index (the rows the plan read stay materialized in it) is released before `compute` runs, so the inference never carries it; a later plan in this process opens it again. */
 async function build({identity,implementation,membership,compute,prime,signal,progress,hint,only,observedAtMs=now(),coverage,claim}){
  await prime?.(signal);
  const current=await read(signal),usable=current?.manifest&&current.manifest.identity===identity?current.manifest:undefined;
  const reuse=hint&&usable&&current.pointer.generation===hint.generation&&now()-hint.atMs<=P.hintMaximumAgeMilliseconds;
  let previous=usable&&!reuse?{manifest:usable,frontier:await readFrontier(usable.frontier)}:undefined,live=reuse?hint.live:await membership(previous?.frontier?previous:undefined,signal);
  const found=reuse?hint.plan:await plan({current,identity,changes:previous?.frontier?live.changes:undefined,...previous?.frontier&&live.dirty?{journalDirty:live.dirty}:{},only,skipEnvironment:coverage!==undefined,signal,progress});
  if(found.kind==="clean"){
   if(usable.membership===live.digest&&!found.refresh?.size)return{current,built:false,published:false,plan:found,computed:0};
   return{...await refreshMetadata({current,live,signal,held:found.refresh}),built:false,plan:found,computed:0};
  }
  const cold=found.kind==="cold",owners0=cold?{}:usable.owners,names=cold?[]:[...found.dirty].sort(compareCodePointsV1);
  const want=claim&&await claim.extend({owners:[...names,...found.removed],structural:cold||found.envStale||found.added.length>0,signal});
  if(want&&!want.held)return{current,built:false,published:false,computed:0,plan:found,needsBuild:true,pending:{generation:current?.pointer.generation??-1,roots:{},...want.request}};
  aborted(signal);
  const spec={cold,owners:names,discover:found.added,removed:found.removed,changed:cold?[]:(live.changes??[]).filter(change=>!change.directory).map(change=>change.path),previousEnv:cold?null:usable.env,previousParts:cold?undefined:usable.envParts,envStale:found.envStale,generation:(current?.pointer.generation??-1)+1,coldReason:cold?found.reason:undefined,reasons:found.reasons??[],staleRows:found.stale??0,staleKinds:found.staleKinds,
   stubs:Object.fromEntries(Object.entries(owners0).map(([name,row])=>[name,{root:row.root,kind:row.kind,targets:row.targets,...(row.nativeRoot?{nativeRoot:row.nativeRoot}:{}),...(row.config?{config:row.config}:{}),...(row.tag?{tag:row.tag}:{}),...(row.reads?{reads:row.reads}:{})}]))};
  previous=undefined;indexCache.clear();
  const result=await compute(spec,signal);
  stats.computed+=result.owners.length;aborted(signal);
  const envDigest=result.env.digest,owners={...owners0},dropped=new Set(found.removed),upserts=new Map(),append=new Map(),returned=new Set(),stored=[];
  for(const name of dropped)delete owners[name];
  for(const owner of result.owners){
   aborted(signal);
   if(owner.partial==="edges"){
    const old=owners[owner.name];if(!old)throw fail("Edge refresh requires a published owner","GENERATION_INCOMPLETE");
    stored.push(async()=>{const fragment=await readFragment(old);if(!fragment)throw fail("Edge refresh requires the published fragment","GENERATION_INCOMPLETE");const saved=await store(P.fragments,{...fragment,edges:owner.edges??await result.reEdge(owner.name,fragment),edgesEnv:envDigest},".json.br",P.maximumFragmentBytes),{reads:_,...base}=old,reads=owner.reads??result.readsOf?.(owner.name);owners[owner.name]={...base,fragment:saved.digest,bytes:saved.bytes,edgesEnv:envDigest,...(reads?{reads}:{})};});
    if(owner.rows?.length)append.set(owner.name,owner.rows);
    continue;
   }
   returned.add(owner.name);
   stored.push(async()=>{
    const body={version:1,name:owner.name,root:owner.root,kind:owner.kind,node:owner.node,edges:owner.edges,edgesEnv:envDigest,targets:owner.targets,...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{})};
    const saved=await store(P.fragments,body,".json.br",P.maximumFragmentBytes);
    owners[owner.name]={root:owner.root,kind:owner.kind,fragment:saved.digest,bytes:saved.bytes,edgesEnv:envDigest,targets:owner.targets,...(owner.config?{config:owner.config}:{}),...(owner.nativeRoot?{nativeRoot:owner.nativeRoot}:{}),...(owner.tag?{tag:owner.tag}:{}),...(owner.reads?{reads:owner.reads}:{})};
   });
   upserts.set(owner.name,owner.rows??[]);
  }
  await mapLimit(stored,P.readConcurrency,job=>job(),signal);
  for(const name of names)if(!returned.has(name)&&!dropped.has(name)){delete owners[name];dropped.add(name);}
  for(const[name,row]of Object.entries(owners))if(row.tag&&result.replaceTags?.includes(row.tag)&&!returned.has(name)){delete owners[name];dropped.add(name);}
  for(const name of returned)dropped.delete(name);
  const renamed=new Map();
  for(const name of returned){const row=owners[name];if(row&&!row.tag&&row.kind==="project")renamed.set(row.root,name);}
  for(const[name,row]of Object.entries(owners))if(!returned.has(name)&&!row.tag&&row.kind==="project"&&renamed.has(row.root)){delete owners[name];dropped.add(name);}
  for(const name of result.carried??[])if(owners[name]&&!returned.has(name))owners[name]={...owners[name],edgesEnv:envDigest};
  for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==envDigest)throw fail("Every owner needs edges of the published environment: "+name+" ("+JSON.stringify({kind:row.kind,tag:row.tag??null,edges:String(row.edgesEnv??"none").slice(0,12),environment:String(envDigest).slice(0,12),root:row.root,dirty:names.includes(name),returned:returned.has(name),dropped:dropped.has(name),carried:(result.carried??[]).includes(name),sameRootReturned:[...returned].filter(other=>owners[other]?.root===row.root)})+")","GENERATION_INCOMPLETE");
  upserts.set(P.envOwner,result.env.rows??[]);
  for(const[name,rows]of derivedUpsertsV1(result.derived??[],found.refresh))upserts.set(name,rows);if(result.derivedSpill&&existsSync(join(result.derivedSpill,"derived")))for(const name of readdirSync(join(result.derivedSpill,"derived")))if(name.endsWith(".json"))upserts.set(name.slice(0,-5),join(result.derivedSpill,"derived",name));
  let index=cold?createFactIndexV1(upserts):patchFactIndexV1(await readIndex(usable.factIndex),{drop:[...dropped],upserts,append});
  const orphans=orphanedDerivedOwnersV1(index);if(orphans.length)index=patchFactIndexV1(index,{drop:orphans});
  const unsupported=unsupportedDerivedV1(index);if(unsupported.length)throw fail("Derived facts need a supporting computation: "+unsupported.slice(0,3).map(row=>row.path.slice(0,96)).join(", "),"GENERATION_INCOMPLETE");

  const packedIndex=packFactIndexV1(index,P.maximumIndexBytes),indexStored=await keep(P.indexes,packedIndex.bytes,".facts.br");
  const globalStored=await store(P.fragments,{version:1,name:"@global",value:result.global??null},".json.br",P.maximumFragmentBytes);
  const frontierStored=await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes);
  const expectedGeneration=current?.pointer.generation??-1;
  const{owners:_table,...envParts}=result.env.parts??{},next={version:1,generation:expectedGeneration+1,parent:current?.pointer.manifest??null,identity,implementation,env:envDigest,membership:live.digest,frontier:frontierStored.digest,factIndex:indexStored.digest,global:globalStored.digest,...(result.env.parts?{envParts}:{}),owners,built:{kind:cold?"cold":"delta",base:cold?null:usable.generation,dirty:[...returned].sort(compareCodePointsV1)}};
  const manifestDigest=await writeManifest(next);aborted(signal);
  const verified=stamp?mergeVerifiedStampV1({owners:Object.keys(owners).sort(compareCodePointsV1),scope:cold?undefined:only,observedAtMs,carried:cold?undefined:coverage}):undefined;
  let outcome=await publish({manifestDigest,expectedGeneration,verified,signal}),done=outcome.published?{next,index,packed:packedIndex,kept:indexStored}:undefined;
  if(!done&&!cold){index=undefined;const merged=await mergeIntoHeadV1({read,readIndex,writeManifest,publish,patch:patchFactIndexV1,pack:value=>packFactIndexV1(value,P.maximumIndexBytes),keep:bytes=>keep(P.indexes,bytes,".facts.br"),key:rowKeyV1,stats},{base:current,mine:next,patch:{drop:[...dropped],upserts,append},verified,signal});if(merged.outcome){outcome=merged.outcome;done=merged;}}
  if(!done)return{current:await read(signal),built:true,published:false,plan:found,computed:result.owners.length};
  indexCache.set(done.kept.digest,done.index);
  await settler.markIndexed({held:found.refresh,dropped:(result.derived??[]).map(fact=>derivedOwnerNameV1(fact.row))}).catch(()=>undefined);
  await doubt?.published?.({rows:done.index.rows,observedAt:observedAtMs}).catch(()=>false);
  if(await(claim?claim.alone({signal}).catch(()=>false):true))await retire({signal}).catch(error=>{if(signal?.aborted)throw error;});
  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,published:true,plan:found,computed:result.owners.length,carried:result.carried?.length??0,...(done.theirs?{merged:done.theirs.length}:{}),index:{rows:done.index.rows.length,owners:done.index.owners.filter(name=>!isDerivedOwnerV1(name)).length,expandedBytes:done.packed.expandedBytes,packedBytes:done.kept.bytes}};
 }

 /** 🧷️ Publishes the same owners under a refreshed membership frontier, so the next verification is a stat pass again; losing the swap is harmless. */
 async function refreshMetadata({current,live,signal,held}){
  const manifest=current.manifest,frontier=live?await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes):undefined;
  let factIndex=manifest.factIndex,patched;
  if(held?.size){const old=await readIndex(manifest.factIndex);if(old){patched=patchFactIndexV1(old,{upserts:derivedUpsertsV1([],held)});indexCache.delete(manifest.factIndex);factIndex=(await keep(P.indexes,packFactIndexV1(patched,P.maximumIndexBytes).bytes,".facts.br")).digest;}}
  const next={...manifest,generation:manifest.generation+1,parent:current.pointer.manifest,membership:live?live.digest:manifest.membership,frontier:frontier?frontier.digest:manifest.frontier,factIndex,built:{kind:"refresh",base:manifest.generation,dirty:[]}};
  const digest=await writeManifest(next);
  try{const outcome=await publish({manifestDigest:digest,expectedGeneration:current.pointer.generation,verified:current.pointer.verified,signal});if(outcome.published){if(patched){indexCache.set(factIndex,patched);await settler.markIndexed({held}).catch(()=>undefined);}return{current:{pointer:outcome.pointer,manifest:next},published:true};}}
  catch(error){if(signal?.aborted)throw error;}
  return{current:await read(signal)??current,published:false};
 }

 /** 🔁️ Brings the caller to a generation that holds. `current` serves the newest generation of this identity without verification or waiting; `fresh` verifies (only the closure `only` when given), serves a verified generation without any lease and builds otherwise, inside `builder` (which hands the build its slice claim; see `./🔐️custody`) or the builder lease, where a holder that finds the work done by a peer returns that generation untouched and a build whose swap a peer won merges onto the peer's generation (see `./🔀️merge`). */
 async function refresh({identity,implementation,policy="fresh",only,membership,compute,prime,signal,progress,builder,requestedAt}){
  if(!P.policies.includes(policy))throw fail("Closed graph policy required");
  const started=now(),first=await read(signal),usable=first?.manifest&&first.manifest.identity===identity;
  if(policy==="current"&&usable)return{current:first,served:"current",built:false,published:false,computed:0};
  let hint,scope=only,coverage,pending;
  if(usable&&requestedAt!==undefined&&first.pointer.verified!==undefined){
   const owners=Object.keys(first.manifest.owners).sort(compareCodePointsV1);
   coverage=coverageOfV1({stamp:first.pointer.verified,owners,implementation,manifestImplementation:first.manifest.implementation,requestedAtMs:requestedAt});
   if(coverage){const open=uncoveredOfV1({needed:only,owners,coverage});if(!open.size)return{current:first,served:"covered",built:false,published:false,computed:0};scope=open;}
  }
  if(usable){
   const previous={manifest:first.manifest,frontier:await readFrontier(first.manifest.frontier)},live=await membership(previous.frontier?previous:undefined,signal);
   const verdict=await plan({current:first,identity,changes:previous.frontier?live.changes:undefined,...previous.frontier&&live.dirty?{journalDirty:live.dirty}:{},only:scope,skipEnvironment:coverage!==undefined,signal,progress});
   if(verdict.kind==="clean"){
    if(!scope&&live.digest!==first.manifest.membership||verdict.refresh?.size){const refreshed=await refreshMetadata({current:first,live:!scope&&live.digest!==first.manifest.membership?live:undefined,signal,held:verdict.refresh});return{...refreshed,served:"verified",built:false,computed:0,plan:verdict};}
    return{current:first,served:"verified",built:false,published:false,computed:0,plan:verdict};
   }
   if(verdict.kind==="delta")pending={structural:verdict.envStale||verdict.added.length>0||verdict.removed.length>0,generation:first.pointer.generation,dirty:[...verdict.dirty].sort(compareCodePointsV1),roots:Object.fromEntries([...verdict.dirty].filter(name=>first.manifest.owners[name]).map(name=>[name,first.manifest.owners[name].root]))};
   if(verdict.kind==="delta"&&previous.frontier)hint={generation:first.pointer.generation,plan:verdict,live,atMs:now()};
  }
  const body=claim=>build({identity,implementation,membership,compute,prime,signal,progress,hint,only:scope,observedAtMs:started,coverage,claim});
  for(let attempt=0;attempt<8;attempt++){
   const result=await(builder?builder(body,pending):lease?lease.exclusive(resources.builder,body,{signal}):body());
   if(result.published||!result.built)return{...result,served:"built"};
  }
  throw fail("Generation keeps changing while it is built","GENERATION_CONFLICT");
 }

 /** 🧺️ Collects the closure of `roots` through the fragments of one generation. `neighbors(fragment)` names the projects a fragment reaches (the targets of its edges by default); only the fragments of the closure are read. */
 async function closureOf(current,roots,{neighbors,signal}={}){
  const manifest=current?.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const seen=new Set();let frontier=[...new Set(roots)];
  while(frontier.length){
   const batch=[...new Set(frontier.filter(name=>!seen.has(name)))];frontier=[];
   for(const name of batch){if(!manifest.owners[name])throw fail("Unknown project in the closure: "+name,"GENERATION_UNKNOWN_OWNER");seen.add(name);}
   const loaded=await mapLimit(batch,P.readConcurrency,async name=>{const fragment=await readFragment(manifest.owners[name]);if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment;},signal);
   for(const fragment of loaded)for(const target of(neighbors?neighbors(fragment):fragment.edges.map(edge=>edge.target))){if(target==="*"){for(const name of Object.keys(manifest.owners))if(!seen.has(name))frontier.push(name);}else if(manifest.owners[target]&&!seen.has(target))frontier.push(target);}
  }
  return seen;
 }

 /**
  * 🌊️ Streams the graph of exactly the owners in `names` (every owner without it) fragment by fragment: `nodes()` yields `[name, node]` in code point order while at most `readConcurrency` fragments are resident, so a consumer that drops what it was given keeps the resident state independent of the number of owners; `finish()` answers the rest (dependencies, external nodes, unavailable projects, generation, membership) once the nodes were consumed. Nodes and edges come from fragments only, so unrelated owners are never read; the external nodes are those the selected owners' edges reach plus those their inputs (target inputs and named input groups) name in `externalDependencies` (a task hash needs the node of every external dependency it lists, edge or not).
  */
 function assembleStream(current,{names,externalInputs=[],compare=(a,b)=>a<b?-1:a>b?1:0,signal,progress}={}){
  const manifest=current.manifest;if(!manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
  const wanted=(names?[...new Set(names)]:Object.keys(manifest.owners)).sort(compareCodePointsV1),keep=names?new Set(wanted):undefined;
  for(const name of wanted)if(!manifest.owners[name])throw fail("Unknown project in the closure: "+name,"GENERATION_UNKNOWN_OWNER");
  const inputRoots=readGenerationExternalInputsV1(externalInputs),unavailable=[],edges=new Map(),roots=[],referenced=new Set(inputRoots);let global,drained=false,started=false;
  const referenceExternalInputs=value=>{if(Array.isArray(value)){for(const item of value)referenceExternalInputs(item);}else if(value&&typeof value==="object"){for(const[key,item]of Object.entries(value)){if(key==="externalDependencies"&&Array.isArray(item)){for(const name of item)if(typeof name==="string")referenced.add(name);}else referenceExternalInputs(item);}}};
  async function*nodes(){
   if(started)throw fail("The node stream of an assembly is read once","GENERATION_STREAM_USED");started=true;
   const globalBytes=await fs.read(fragmentPath(manifest.global));if(!globalBytes||sha256(globalBytes)!==manifest.global)throw fail("Global fragment is missing","GENERATION_INCOMPLETE");
   global=unpack(globalBytes,P.maximumFragmentBytes).value??{};
   for(const name of inputRoots)if(resolveGenerationExternalInputV1(global.externalNodes,name)===undefined)throw fail("Unknown external input in the sealed generation: "+name,"GENERATION_UNKNOWN_EXTERNAL");
   await progress?.({phase:"assembly",done:0,total:wanted.length});
   const pending=[];let next=0,done=0;
   const launch=()=>{while(pending.length<P.readConcurrency&&next<wanted.length){const name=wanted[next++],job=readFragment(manifest.owners[name]).then(fragment=>{if(!fragment)throw fail("Fragment of "+name+" is missing","GENERATION_INCOMPLETE");return fragment;});job.catch(()=>{});pending.push(job);}};
   launch();
   while(pending.length){
    aborted(signal);const fragment=await pending.shift();launch();
    roots.push(fragment.root);
    for(const edge of fragment.edges)if(!keep||keep.has(edge.target)||global.externalNodes?.[edge.target]!==undefined)edges.set(JSON.stringify(edge),edge);
    if(fragment.kind==="project"){referenceExternalInputs(fragment.node);yield[fragment.name,fragment.node];}else unavailable.push(fragment.node);
    await progress?.({phase:"assembly",done:++done,total:wanted.length});
   }
   drained=true;
  }
  async function finish(){
   if(!drained)throw fail("The node stream must be consumed before the assembly is finished","GENERATION_STREAM_OPEN");
   const external=new Set();
   if(keep){
    for(const edge of edges.values())if(global.externalNodes?.[edge.target]!==undefined)external.add(edge.target);
    for(const name of referenced){const canonical=resolveGenerationExternalInputV1(global.externalNodes,name);if(canonical!==undefined)external.add(canonical);}
    let grown=true;
    while(grown){grown=false;for(const edge of global.edges??[])if((keep.has(edge.source)||external.has(edge.source))&&(keep.has(edge.target)||global.externalNodes?.[edge.target]!==undefined)){edges.set(JSON.stringify(edge),edge);if(global.externalNodes?.[edge.target]!==undefined&&!external.has(edge.target)){external.add(edge.target);grown=true;}}}
   }else for(const edge of global.edges??[])edges.set(JSON.stringify(edge),edge);
   const sorted=[...edges.entries()].sort(([a],[b])=>compare(a,b)).map(([,edge])=>edge);
   const externalNodes=Object.fromEntries(Object.entries(global.externalNodes??{}).filter(([key])=>!keep||external.has(key)));
   const rows=[...(global.unavailable??[]),...unavailable].filter(row=>!keep||roots.some(root=>row.root===root||row.root.startsWith(root+"/")));
   return{dependencies:sorted,externalNodes,unavailable:rows.sort((a,b)=>compareCodePointsV1(a.root,b.root)),generation:manifest.generation,membership:manifest.membership};
  }
  return Object.freeze({nodes,finish});
 }

 /** 🔧️ Assembles the graph of exactly the owners in `names` (every owner without it) into one object: the streamed nodes collected in order, then the rest. */
 async function assemble(current,options){
  const stream=assembleStream(current,options),projects={};
  for await(const[name,node]of stream.nodes())projects[name]=node;
  return{projects,...await stream.finish()};
 }

 /** 📸️ Reads one complete graph without any lease: a fragment retired while the snapshot is read restarts it from the newest pointer, so the result is always the graph of one sealed generation. */
 async function snapshot({roots,neighbors,compare,signal,progress}={}){
  for(let attempt=0;attempt<8;attempt++){
   const current=await read(signal);if(!current?.manifest)throw fail("A sealed generation is required","GENERATION_EMPTY");
   try{const names=roots?[...await closureOf(current,roots,{neighbors,signal})]:undefined;return{current,graph:await assemble(current,{names,compare,signal,progress})};}
   catch(error){if(error?.code!=="GENERATION_INCOMPLETE")throw error;await sleep(5*(attempt+1),signal);}
  }
  throw fail("Generation fragments keep disappearing","GENERATION_UNREADABLE");
 }

 /** 🎯️ Brings the caller to a generation that holds for the closure of `roots`: only the facts those projects consume are verified, so an edit elsewhere neither waits for a builder nor is refreshed for this caller. */
 async function refreshClosure({roots,neighbors,closure,identity,implementation,policy="fresh",membership,compute,signal,progress,builder,requestedAt}){
  const closeOf=current=>closure?closure(current,{signal}):closureOf(current,roots,{neighbors,signal});
  const first=await read(signal);let names,unborn=false;
  if(first?.manifest&&first.manifest.identity===identity){try{names=await closeOf(first);}catch(error){if(error?.code==="GENERATION_UNKNOWN_OWNER"){names=new Set();unborn=true;}else if(error?.code!=="GENERATION_INCOMPLETE")throw error;}}
  let verified=names,result=await refresh({identity,implementation,policy,only:names,membership,compute,signal,progress,builder,requestedAt});
  if(result.needsBuild)return result;
  let built=result.built,published=result.published,computed=result.computed;
  for(let round=0;;round++){
   const closure=verified&&!unborn&&result.current.pointer.generation===first.pointer.generation?verified:await closeOf(result.current);
   if(policy==="current"||!verified||round>=8||[...closure].every(name=>verified.has(name)))return{...result,built,published,computed,...(built?{served:"built"}:{}),names:closure};
   verified=new Set([...verified,...closure]);result=await refresh({identity,implementation,policy,only:verified,membership,compute,signal,progress,builder,requestedAt});
   if(result.needsBuild)return result;built=built||result.built;published=published||result.published;computed+=result.computed;
  }
 }

 /** 🧹️ Removes documents that no retained generation names once they are older than the grace window (every file of the index directory is an index document, whatever format its generation used); a failure only costs disk. */
 async function retire({signal}={}){
  aborted(signal);const pointer=await readPointer();if(!pointer)return;
  const files=await fs.list(join(root,P.manifests)),rows=[];
  for(const file of files){aborted(signal);if(!/^[a-f0-9]{64}\.json$/u.test(file.name))continue;const digest=file.name.slice(0,64),bytes=await fs.read(manifestPath(digest));if(!bytes)throw fail("Generation manifest disappeared before retirement","GENERATION_INCOMPLETE");if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==digest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");rows.push({digest,manifest:readGenerationManifestV1(bytes)});}
  if(pointer.manifest!==null&&!rows.some(row=>row.digest===pointer.manifest))throw fail("Current generation manifest missing before retirement","GENERATION_INCOMPLETE");
  rows.sort((a,b)=>b.manifest.generation-a.manifest.generation);
  const kept=rows.filter((row,index)=>index<P.retainGenerations||row.digest===pointer.manifest),live=new Set([pointer.manifest]),fragments=new Set(),indexes=new Set(),frontiers=new Set();
  for(const row of kept){live.add(row.digest);indexes.add(row.manifest.factIndex);fragments.add(row.manifest.global);if(row.manifest.frontier)frontiers.add(row.manifest.frontier);for(const owner of Object.values(row.manifest.owners))fragments.add(owner.fragment);}
  const cutoff=now()-P.graceMilliseconds;
  const sweep=async(directory,keep,extension)=>{for(const file of await fs.list(join(root,directory))){aborted(signal);if(file.name.endsWith(extension)&&!keep.has(file.name.slice(0,64))&&file.modifiedMs<cutoff)await fs.remove(join(root,directory,file.name));}};
  await sweep(P.manifests,live,".json");await sweep(P.fragments,fragments,".json.br");await sweep(P.indexes,indexes,"");await sweep(P.frontiers,frontiers,".json.br");await nodes?.prune({now:now()});
 }

 /** ♻️ Marks the newest generation invalid without deleting anything; the next fresh reader builds. */
 async function invalidate({reason="invalidated",signal}={}){
  const pointer=await readPointer(),digest=null;
  const outcome=await publish({manifestDigest:digest,expectedGeneration:pointer?.generation??-1,reason,signal});
  return outcome.published;
 }

 return Object.freeze({dropIndexes:()=>indexCache.clear(),read,readManifestAt,readIndex,readIndexView,readFrontier,readFragment,plan,build,refresh,refreshClosure,closureOf,assemble,assembleStream,snapshot,retire,invalidate,publish,stats,readGlobal,catalog});
}

/** 🔒️ A reference exclusive lease over lock files for tests and non repository runtimes: acquisition is `open(wx)`, a dead owner's lock is replaced, waiting polls with backoff and honours the abort signal. */
export function createFileLeasePortV1(directory,{staleMilliseconds=P.leaseStaleMilliseconds,now=()=>Date.now(),sleep=defaultSleep,onHold}={}){
 const alive=pid=>{try{process.kill(pid,0);return true;}catch(error){return error?.code==="EPERM";}};
 return Object.freeze({
  async exclusive(resource,body,{signal,timeoutMs}={}){
   const path=join(directory,sha256(resource)+".lock"),token=randomBytes(12).toString("hex"),started=now();let delay=2;
   await mkdir(directory,{recursive:true});
   for(;;){
    aborted(signal);
    try{const file=await open(path,"wx");try{await file.writeFile(JSON.stringify({pid:process.pid,token,since:now()}));}finally{await file.close();}break;}
    catch(error){
     if(error?.code!=="EEXIST")throw error;
     try{const held=JSON.parse(await readFile(path,"utf8"));if(!alive(held.pid)||now()-held.since>staleMilliseconds){const again=JSON.parse(await readFile(path,"utf8"));if(again.token===held.token)await rm(path,{force:true});continue;}}catch{}
     if(timeoutMs!==undefined&&now()-started>timeoutMs)throw fail("Resource lease timed out: "+resource,"LEASE_TIMEOUT");
     await sleep(delay,signal);delay=Math.min(delay*2,P.pollMaximumMilliseconds);
    }
   }
   const acquired=now();
   try{return await body();}finally{try{const held=JSON.parse(await readFile(path,"utf8"));if(held.token===token)await rm(path,{force:true});}catch{}onHold?.({resource,heldMs:now()-acquired,waitedMs:acquired-started});}
  },
 });
}
