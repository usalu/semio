import{createGenerationPlanV1}from"./🧭️plan/🟨️.mjs";
export{unfoundedWithheldOwnersV1}from"./🧭️plan/🟨️.mjs";
import{readEdgeReadsV1,readEnvironmentPartsV1}from"./🧵️edge-reads/🟨️.mjs";
import{createBoundedCacheV1}from"../../🧊️bounded/🟨️.mjs";
import{orderOwnersV1,readPriorityV1,writePendingV1}from"./🧭️steering/🟨️.mjs";
import{createHash,randomBytes}from"node:crypto";
import{brotliCompressSync,brotliDecompressSync,constants as zlibConstants}from"node:zlib";
import{lstat,mkdir,open,readdir,readFile,rename,rm}from"node:fs/promises";
import{dirname,join}from"node:path";
import{graphStringInternerV1,internJsonStringsV1}from"../../🧠️memory/🟨️.mjs";
import{canonicalJsonV1,compareCodePointsV1,decodeFactIndexV1,encodeFactIndexV1,openFactIndexV1}from"./🧮️fact-codec/🟨️.mjs";
import{coverageOfV1,mergeVerifiedStampV1,readVerifiedStampV1,uncoveredOfV1}from"./🔭️coverage/🟨️.mjs";
import{INDEX_CHUNK_POLICY_V1,addOwnerRunsV1,bulkUpsertClassesV1,classesOfChunksV1,createChunkInternerV1}from"./🧱️chunk-classes/🟨️.mjs";
import{createFrontierV1,diffFrontiersV1,frontierFactHoldsV1,frontierHoldsV1,stableDirectoriesV1}from"./🧭️frontier/🟨️.mjs";
import{mergeIntoHeadV1}from"./🔀️merge/🟨️.mjs";
import{createClosureSettlerV1,sharedClosureSettlerV1}from"../🧮️derived-nodes/🧩️closure-kinds/🟨️.mjs";
import{createCasStoreV1}from"../../🗄️cas/🗃️store/🟦️.ts";
import{createNodeStoreV1}from"../🧮️derived-nodes/💾️store/🟨️.mjs";
import{adoptRecordedRootsV1}from"../../👁️observation/🏚️relocation/🟨️.mjs";
import{profilePhaseV1}from"../../⏱️profile/🟨️.mjs";
import{derivedOwnerNameV1,derivedUpsertsV1,isDerivedOwnerV1,orphanedDerivedOwnersV1,unsupportedDerivedV1}from"./✂️cutoff/🟨️.mjs";
import{GENERATION_POLICY_V1,P,HEX,sha256,isObject,unpack,aborted,fail,requireKeys,requireHex,requireCount,requireText}from"./🧱️guards/🟨️.mjs";
import{ROW_KEY_TEXT_LIMIT_V1,createFactIndexV1 as createFactIndexRowsV1,factConsumersV1,patchFactIndexV1,refineFactIndexV1,rowKeyV1,selectFactClassesV1,staleConsumersV1,staleFactRowsV1}from"./🧮️fact-rows/🟨️.mjs";
export{GENERATION_POLICY_V1,ROW_KEY_TEXT_LIMIT_V1,factConsumersV1,patchFactIndexV1,refineFactIndexV1,rowKeyV1,selectFactClassesV1,staleConsumersV1,staleFactRowsV1};
import{emptyFactIndexViewV1,patchFactIndexBytesV1}from"./🌊️stream-patch/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{BUILD_JOURNAL_V1,buildJournalKeyV1,createBuildJournalV1}from"./🔖️journal/🟨️.mjs";
import{retireClosureRepairReceiptsV1}from"./🩹️closure-repair/🟨️.mjs";
import{readGenerationPointerV1,readGenerationManifestV1,readGenerationFragmentV1}from"./📖️documents/🟨️.mjs";
import{createNodeFilePortV1,createFileLeasePortV1,defaultSleep}from"./🗃️file-ports/🟨️.mjs";
export{readGenerationPointerV1,readGenerationManifestV1,readGenerationFragmentV1,createNodeFilePortV1,createFileLeasePortV1};

export{canonicalJsonV1,compareCodePointsV1,createFrontierV1,diffFrontiersV1,frontierFactHoldsV1,frontierHoldsV1,stableDirectoriesV1};
let processStartedV1=false;
/** ⏱️ The milliseconds stages outside the build took since the last build (`start.reader` and `start.prefetch` of the first load, `yield` of the pause between two waves): their owners fill it, the next build reports it and empties it. */
export const carriedStagesV1={};


/** 🌊 The owners one wave re-infers: the policy wave, or its share when the load was granted `basisPoints` of the memory it asked for (the host could not give more: `SEMIO_SLICE_WAVE_BASIS_POINTS`, set by the slice pool), never fewer than one. The wave is what bounds the memory of a build, so a narrower grant means smaller waves: slower, never a wait.
 * @see ../../🏡️resident/🏗️composition/🌐️host/👷️slice-pool/🧱️limit/🟨️.mjs */
export function scaledOwnerWaveV1(ownerWave,basisPoints){
 const share=Number(basisPoints);
 return Number.isFinite(share)&&share>0&&share<10000?Math.max(1,Math.floor(ownerWave*share/10000)):Math.max(1,ownerWave);
}

/** 📈️ The owners of the wave after one that took `elapsedMilliseconds` for `size` owners and inferred `inferred` of them: a wave that inferred nothing costs a fixed time of the whole graph, so the next one grows toward the time budget at the speed of the last, at most by the growth factor and never beyond the maximum; a wave that inferred a quarter at most holds its size and shrinks only when it was over the budget; a wave that said nothing about its inference or inferred more than a quarter returns to the base wave, whose publication is the progress the fleet waits on. Both bounds follow the share the host could grant (`basisPoints`).
 * @see ./🧬️schema/🔣️.json */
export function nextOwnerWaveV1({size,elapsedMilliseconds,inferred,basisPoints}){
 const base=scaledOwnerWaveV1(P.ownerWave,basisPoints);
 if(!Number.isSafeInteger(inferred)||inferred<0||inferred*4>size)return base;
 const maximum=Math.max(base,scaledOwnerWaveV1(P.ownerWaveMaximum,basisPoints)),fitting=Math.floor(size*P.ownerWaveBudgetMilliseconds/Math.max(1,elapsedMilliseconds));
 return Math.max(base,Math.min(maximum,inferred===0?size*P.ownerWaveGrowth:size,fitting));
}

/** 🔌️ Admits bounded consumer external roots without changing their sealed package identity. */
export function readGenerationExternalInputsV1(value){const rule=schema.definitions.ExternalInputs;if(!Array.isArray(value))throw fail("Bounded external input references required: received "+(value===null?"null":typeof value),"GENERATION_EXTERNAL_INPUTS");if(value.length>rule.maxItems)throw fail("Bounded external input references required: "+value.length+" items exceed "+rule.maxItems,"GENERATION_EXTERNAL_INPUTS");const bad=value.findIndex(name=>{if(typeof name!=="string"||!name.length)return true;let length=0;for(const point of name)if(++length>rule.items.maxLength)return true;return false;});if(bad>=0)throw fail("Bounded external input references required: item "+bad+" is "+(typeof value[bad]==="string"?(value[bad].length?"longer than "+rule.items.maxLength:"empty"):typeof value[bad]),"GENERATION_EXTERNAL_INPUTS");return Object.freeze([...new Set(value)].sort(compareCodePointsV1));}
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
 return packFactIndexBytesV1(encodeFactIndexV1(index,{maximumBytes}),maximumBytes);
}
/** 🧊️ Brotli packs the binary document of a fact index (the bytes `encodeFactIndexV1` or the streaming patch wrote) and answers its expanded size. */
export function packFactIndexBytesV1(raw,maximumBytes){
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<1||maximumBytes>P.maximumIndexBytes)throw fail("Finite generation document byte limit required");
 const bytes=brotliCompressSync(raw,{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:P.brotliQuality,[zlibConstants.BROTLI_PARAM_LGWIN]:P.brotliWindow,[zlibConstants.BROTLI_PARAM_SIZE_HINT]:raw.length}});
 if(bytes.length>maximumBytes)throw fail("Stored document exceeds its finite limit ("+bytes.length+" bytes packed, limit "+maximumBytes+")");return{bytes,expandedBytes:raw.length};
}
/** 📖️ Reads a packed fact index within the finite reader ceiling straight from the binary document, without any intermediate text. */
export function unpackFactIndexV1(bytes,maximumBytes){return readFactIndexV1(decodeFactIndexV1(brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),{maximumBytes,share:text=>graphStringInternerV1.share(text)}));}
/** 🪶️ Opens a packed fact index for verification only: strings, facts and rows stay in the document until they are read (`🧮️fact-codec` `openFactIndexV1`). */
export function openPackedFactIndexV1(bytes,maximumBytes,name){return openRawFactIndexV1(brotliDecompressSync(bytes,{maxOutputLength:maximumBytes}),maximumBytes,name);}
/** 🪟️ Opens the binary document of a fact index (not brotli packed) as a view. */
export function openRawFactIndexV1(raw,maximumBytes,name){return openFactIndexV1(raw,{maximumBytes,name,share:text=>graphStringInternerV1.share(text)});}

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

/** 🏛️ Builds the inverted fact index of whole owners in memory (`🧮️fact-rows`): the model the streaming patch (`./🌊️stream-patch`) is tested against; a build patches the empty index instead. */
export function createFactIndexV1(ownerRows){return createFactIndexRowsV1(ownerRows);}
async function mapLimit(items,limit,visit,signal){
 const out=new Array(items.length);let next=0;
 await Promise.all(Array.from({length:Math.min(limit,items.length)},async()=>{while(next<items.length){aborted(signal);const at=next++;out[at]=await visit(items[at],at);}}));
 return out;
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
export function createGenerationStoreV1({root,fs=createNodeFilePortV1(),lease,holds,doubt,derive,answers=createAnswersV1({root,fs}),resources,now=()=>Date.now(),sleep=defaultSleep,stamp=P.stampVerification,workspaceRoot,say,stepped}){
 if(typeof root!=="string"||!root||typeof holds!=="function"||!resources?.facts)throw fail("Generation store root, fact check and facts resource required");
 const{nodes,settler}=answers;
 const stats={holdCalls:0,rowsChecked:0,computed:0,published:0,casConflicts:0,merged:0,mergeConflicts:0,publishHeldMs:[],indexLoads:0,fragmentReads:0,indexViews:0,plansReused:0};
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
  const index=unpackFactIndexV1(bytes,P.maximumIndexBytes);stats.indexLoads++;adoptRecordedRootsV1(index,workspaceRoot);
  indexCache.set(digest,index);return index;
 }
 async function readIndexView(digest,{fresh=false}={}){
  const key="view:"+digest,cached=fresh?undefined:indexCache.get(key)??indexCache.get(digest);if(cached)return cached;
  const bytes=await fs.read(indexPath(digest));if(!bytes)return undefined;
  if(sha256(bytes)!==digest)throw fail("Fact index content differs from its name","GENERATION_CORRUPT");
  const index=openPackedFactIndexV1(bytes,P.maximumIndexBytes,"fact-index:"+digest.slice(0,12));stats.indexViews++;adoptRecordedRootsV1(index,workspaceRoot);
  indexCache.set(key,index);return index;
 }
 /** 🏄️ Patches an opened index without decoding it (`./🌊️stream-patch`) and opens the result: the unit of every index change of a build, a merge and a refresh, so none of them holds the decoded index or the rows of all upserted owners. */
 async function patchView(base,patch,signal){return openRawFactIndexV1(await patchFactIndexBytesV1(base,patch,{maximumBytes:P.maximumIndexBytes,signal}),P.maximumIndexBytes,"fact-index:patched");}
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

 const ledgerPath=join(root,"withheld-revalidation.json"),ledger={read:async()=>{const bytes=await fs.read(ledgerPath);return bytes?JSON.parse(Buffer.from(bytes).toString("utf8")):undefined;},write:book=>fs.write(ledgerPath,Buffer.from(JSON.stringify(book)))};
 const plan=createGenerationPlanV1({readIndexView,fragmentNames:()=>fs.names(join(root,P.fragments)),now,doubt,settler,derive,hold,workspaceRoot,ledger,say:text=>process.stderr.write("[graph-owner-revalidate] "+JSON.stringify({version:1,text})+"\n")});

 /** 🤝️ Swaps the pointer for the next generation only if it still names the generation the builder started from; the exclusive hold covers one small read and one rename and nothing else. */
 async function publish({manifestDigest,expectedGeneration,expectedManifest,expectedRepairParent,reason,verified,signal,claim}){
  const body=async()=>{
   const started=now(),pointer=await readPointer();
   aborted(signal);
   if((pointer?.generation??-1)!==expectedGeneration||expectedManifest!==undefined&&(pointer?.manifest??null)!==expectedManifest||expectedRepairParent!==undefined&&(pointer?.parent!==expectedRepairParent||pointer.reason!=="closure-missing")){stats.casConflicts++;stats.publishHeldMs.push(now()-started);return{published:false,pointer};}
   const next={version:1,generation:expectedGeneration+1,manifest:manifestDigest,parent:pointer?.manifest??null,...(reason?{reason}:{}),...(verified?{verified}:{})};
   await fs.write(pointerPath,Buffer.from(canonicalJsonV1(next)));stats.published++;stats.publishHeldMs.push(now()-started);return{published:true,pointer:next};
  };
  const fenced=()=>claim?.fence?claim.fence(body,{signal}):body();
  return lease?lease.exclusive(resources.facts,fenced,{signal,timeoutMs:P.publishWaitMilliseconds}):fenced();
 }

 const journal=createBuildJournalV1({fs,root,manifestPath,publish,settler,say,stepped});
 async function disposeSources(disposers){for(const dispose of disposers.splice(0))try{await dispose();}catch{}}
 /** 🩹️ Opens the immutable repair source while the real pointer remains a tombstone. */
 async function openRepairBase({repair,identity,implementation,signal,continuing}){
  let manifest,frontier;
  try{
   manifest=await readManifestAt(repair.parent,signal);
   if(!manifest||manifest.generation+1!==repair.generation||manifest.identity!==identity||!repair.owners.every(name=>Object.hasOwn(manifest.owners,name)))return;
   frontier=await readFrontier(manifest.frontier);
   if(frontier?.version!==1||!Array.isArray(frontier.directories)||!Array.isArray(frontier.members)||!Number.isFinite(frontier.capturedAt))return;
   const indexAvailable=continuing?(await fs.names(join(root,P.indexes))).includes(manifest.factIndex+".facts.br"):await readIndexView(manifest.factIndex,{fresh:true});
   if(!indexAvailable)return;
  }catch{aborted(signal);return;}
  return{manifest,frontier};
 }
 /** 🔏️ Reopens and authenticates the original index immediately before composing a repaired candidate. */
 async function repairPatchIndex(digest,signal){
  try{
   const index=await readIndexView(digest,{fresh:true});
   if(!index)throw fail("Archived generation fact index is missing","GENERATION_INCOMPLETE");
   return index;
  }catch(cause){
   aborted(signal);
   throw Object.assign(fail("Archived generation fact index is unavailable for repair","GENERATION_CLOSURE_REPAIR_UNQUALIFIED"),{cause});
  }
 }
 const unfundedRepair=(current,plan)=>({current,built:false,published:false,computed:0,plan,needsBuild:true,repairUnavailable:true,pending:{generation:current?.pointer.generation??-1,dirty:[],roots:{},structural:true}});
 const supersededRepair=(current,identity,computed)=>({current,built:computed>0,published:false,computed,...current?.manifest?.identity===identity?{}:{needsBuild:true,pending:{generation:current?.pointer.generation??-1,dirty:[],roots:{},structural:true}}});
 /** 🏗️ Builds and publishes the next generation from the newest one: the plan names the dirty owners, `compute` refreshes exactly those and unchanged fragments are carried over by digest. The verification view of the fact index (the rows the plan read stay materialized in it) is released before `compute` runs, so the inference never carries it; a later plan in this process opens it again. A cold build releases its row sources after encoding them. A delta retains its sources across waves and a possible publication merge. */
 async function build(request){
  let wave,total=0,last;
  const carry={owners:undefined,env:undefined,envParts:undefined,upserts:new Map(),append:new Map(),dropped:new Set(),returned:new Set(),derived:new Set(),disposers:[],journalKey:undefined,tried:false,done:0,waves:0,files:[],names:undefined,resumedFrom:false};
  try{
   for(;;){
    last=await buildOnce(wave?{...request,hint:wave}:request,carry);
    total+=last.computed??0;
    if(last.more){try{await request.wave?.({generation:(last.current?.pointer.generation??-1)+1,done:total,total:total+(last.deferred?.plan?.dirty?.size??0),...(last.phases?{phases:last.phases}:{})});}catch{}
     const stop=await request.claim?.handover?.().catch(()=>undefined);
     if(stop)throw request.claim.handoverError(stop,{done:total,total:total+(last.deferred?.plan?.dirty?.size??0)});}
    else if(last.published&&last.current){try{await request.published?.(last.current.manifest.generation,{done:total,total:total+(last.deferred?.plan?.dirty?.size??0),...(last.phases?{phases:last.phases}:{})});}catch{}}
    if(!last.more)return{...last,computed:total};
    wave=last.deferred;
   }
  }finally{await disposeSources(carry.disposers);}
 }
  async function buildOnce({identity,implementation,membership,compute,prime,signal,progress,hint,only,observedAtMs=now(),coverage,claim,repair,qualify},carry){
  const marks={};if(!processStartedV1){processStartedV1=true;marks.start=Math.round(process.uptime()*1000);}Object.assign(marks,carriedStagesV1);for(const key of Object.keys(carriedStagesV1))delete carriedStagesV1[key];let markedAt=now();const mark=name=>{const at=now();marks[name]=(marks[name]??0)+at-markedAt;markedAt=at;};
  await profilePhaseV1("graph.build.prime",async()=>prime?.(signal));mark("prime");
  const current=await read(signal);
  if(repair&&(current?.pointer.manifest!==null||current.pointer.generation!==repair.generation||current.pointer.parent!==repair.parent||current.pointer.reason!=="closure-missing"))return supersededRepair(current,identity,0);
  const resumeBinding=repair?{parent:repair.parent,receipt:repair.receipt}:undefined;
  if(repair){
   requireHex(repair.receipt,"repair receipt");requireHex(repair.parent,"repair parent");
   if(typeof qualify!=="function"||!Array.isArray(repair.owners)||!repair.owners.length||repair.owners.length>P.maximumOwners||repair.owners.some(name=>typeof name!=="string"||!name)||new Set(repair.owners).size!==repair.owners.length||repair.maximumOwners!==undefined&&(!Number.isSafeInteger(repair.maximumOwners)||repair.maximumOwners<1))throw fail("Verified owners and candidate qualification required for closure repair");
  }
  const continuing=hint?.wave!==undefined&&carry.waves>0;
  const retained=repair?await openRepairBase({repair,identity,implementation,signal,continuing}):undefined;
  const usable=repair?retained?.manifest:current?.manifest?.identity===identity?current.manifest:undefined;
  const reuse=hint&&usable&&current.pointer.generation===hint.generation&&(hint.wave!==undefined||(hint.holds?await hint.holds().catch(()=>false):now()-hint.atMs<=P.hintMaximumAgeMilliseconds));
  let previous=retained??(usable&&!reuse?{manifest:usable,frontier:await readFrontier(usable.frontier)}:undefined),live=reuse?hint.live:await profilePhaseV1("graph.build.membership",async()=>{const began=now();try{return await membership(previous?.frontier?previous:undefined,signal);}finally{marks.membership=now()-began;}});
  if(repair){only=undefined;coverage=undefined;}
  const found=reuse?hint.plan:await profilePhaseV1("graph.build.plan",()=>plan({current,identity,repairBase:retained?.manifest,forcedOwners:retained?repair.owners:undefined,verifyAll:!!repair,changes:previous?.frontier?live.changes:undefined,...previous?.frontier&&live.dirty?{journalDirty:live.dirty}:{},only,skipEnvironment:coverage!==undefined,signal,progress}));
  const priority=readPriorityV1();mark("plan");for(const[name,ms]of Object.entries(found.times??{}))marks["plan."+name]=ms;
  if(found.kind==="clean"){
   writePendingV1(process.env,current?.pointer.generation??-1,[],priority);
   if(usable.membership===live.digest&&!found.refresh?.size)return{current,built:false,published:false,plan:found,computed:0};
   return{...await refreshMetadata({current,live,signal,held:found.refresh,claim}),built:false,plan:found,computed:0};
  }
  const cold=found.kind==="cold",ordered=cold?[]:orderOwnersV1([...found.dirty].sort(compareCodePointsV1),priority);
  if(repair?.maximumOwners!==undefined&&(cold||found.envStale||found.added.length||found.removed.length||ordered.length>repair.maximumOwners))return unfundedRepair(current,found);
  if(repair&&claim&&claim.structural!==true)return unfundedRepair(current,found);
  if(repair&&hint?.wave&&!reuse&&carry.owners)throw fail("Closure repair base lost between waves","GENERATION_CLOSURE_REPAIR_UNQUALIFIED");
  carry.membership=live.digest;
  const journalKey=carry.journalKey??=buildJournalKeyV1(repair?{parent:current?.pointer.manifest??null,identity,membership:live.digest,cold,names:[...ordered].sort(compareCodePointsV1),removed:[...found.removed].sort(compareCodePointsV1),added:[...found.added].sort(compareCodePointsV1),repair:resumeBinding,generation:repair.generation,implementation}:{parent:current?.pointer.manifest??null,identity,cold});
  const verifyResume=repair?async manifest=>{
   const frontier=await readFrontier(manifest.frontier);if(!frontier)return false;
   const refreshed=await membership({manifest,frontier},signal);
   if(refreshed.digest!==manifest.membership)return false;
   const verdict=await plan({repairBase:manifest,identity,changes:refreshed.changes,verifyAll:true,signal,progress});
   return verdict.kind==="clean"&&!verdict.refresh?.size;
  }:undefined;
  const resumed=!hint?.wave&&!carry.tried?await journal.resume({key:journalKey,current,repair:resumeBinding,identity,implementation,found,signal,claim,qualify,verify:verifyResume,ordered,membership:live.digest,hold}):undefined;carry.tried=true;
  if(resumed?.superseded)return supersededRepair(await read(signal),identity,0);
  if(resumed?.result)return resumed.result;
  if(resumed?.carry)Object.assign(carry,resumed.carry);
  const owners0=cold?{}:carry.owners??usable.owners,names=resumed?.names??ordered,offset=hint?.wave?.done??carry.done,wave=names.slice(0,hint?.wave?.size??scaledOwnerWaveV1(P.ownerWave,process.env.SEMIO_SLICE_WAVE_BASIS_POINTS)),rest=names.slice(wave.length),waveStartedMs=now(),steps=[];
  const markStep=async(step,inputs,output,persist)=>{steps.push({step,inputs,output});await persist?.();await stepped?.(step);};
  if(!cold)writePendingV1(process.env,current?.pointer.generation??-1,names,priority);
  const want=claim&&await claim.extend({owners:[...names,...found.removed],structural:!!repair||cold||found.envStale||found.added.length>0,signal});
  if(want&&!want.held)return{current,built:false,published:false,computed:0,plan:found,needsBuild:true,pending:{generation:current?.pointer.generation??-1,roots:{},...want.request}};
  aborted(signal);
  const repairOwnerSet=repair?new Set(repair.owners):undefined,forcedOwners=repair?wave.filter(name=>repairOwnerSet.has(name)):undefined;
  const spec={cold,owners:wave,...repair?{forcedOwners}:{},more:rest.length>0,total:offset+names.length,done:offset,discover:found.added,removed:found.removed,changed:cold?[]:(live.changes??[]).filter(change=>!change.directory).map(change=>change.path),previousEnv:cold?null:carry.env??usable.env,previousParts:cold?undefined:carry.envParts??usable.envParts,envStale:found.envStale,generation:(current?.pointer.generation??-1)+1,coldReason:cold?found.reason:undefined,reasons:found.reasons??[],staleRows:found.stale??0,staleKinds:found.staleKinds,
   stubs:Object.fromEntries(Object.entries(owners0).map(([name,row])=>[name,{root:row.root,kind:row.kind,targets:row.targets,...(row.nativeRoot?{nativeRoot:row.nativeRoot}:{}),...(row.config?{config:row.config}:{}),...(row.tag?{tag:row.tag}:{}),...(row.reads?{reads:row.reads}:{})}]))};
  previous=undefined;indexCache.clear();
  let result=await profilePhaseV1("graph.build.compute",()=>compute(spec,signal));
  {const dispose=result.dispose;if(dispose)carry.disposers.push(dispose);}
  mark("compute");
  const computed=result.owners.length;stats.computed+=computed;aborted(signal);
  if(forcedOwners?.length){
   const results=new Map(result.owners.map(owner=>[owner.name,owner])),renames=new Set(result.owners.filter(owner=>owner.partial===undefined&&owner.kind==="project"&&!owner.tag&&!Object.hasOwn(owners0,owner.name)).map(owner=>owner.root));
   for(const name of forcedOwners){
    const owner=results.get(name),previous=owners0[name];
    if(owner?owner.partial!==undefined:previous.kind!=="project"||previous.tag||!renames.has(previous.root))throw fail("Closure repair requires a fresh computation of "+name,"GENERATION_CLOSURE_REPAIR_UNQUALIFIED");
   }
  }
  const {env:{digest:envDigest,parts:computedParts},replaceTags,carried,inferred,phases:computePhases,global:globalPayload}=result;
  let {env:{rows:envRows},derived}=result;
  const owners={...owners0},dropped=new Set(found.removed),upserts=new Map(),append=new Map(),returned=new Set(),stored=[];
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
    const noisy=returned.size>P.ownerWave;if(noisy)say?.("storing",{count:returned.size});
  if(cold)result=undefined;
  let fragmentsDone=0;if(stored.length)await progress?.({phase:"fragments",done:0,total:stored.length});
  await profilePhaseV1("graph.build.fragments",()=>mapLimit(stored,P.readConcurrency,async(job,at)=>{
   await job();
   if(cold){stored[at]=undefined;job=undefined;}
   await progress?.({phase:"fragments",done:++fragmentsDone,total:stored.length});
  },signal));mark("fragments");
  upserts.set(P.envOwner,envRows??[]);
  for(const[name,rows]of derivedUpsertsV1(derived??[],found.refresh))upserts.set(name,rows);
  for(const fact of derived??[])carry.derived.add(derivedOwnerNameV1(fact.row));
  result=envRows=derived=undefined;stored.length=0;
  await markStep("storing",journalKey,sha256(Buffer.from(canonicalJsonV1([...returned].sort(compareCodePointsV1).map(name=>owners[name]?.fragment??null)))));aborted(signal);
  for(const name of wave)if(!returned.has(name)&&!dropped.has(name)){delete owners[name];dropped.add(name);}
  for(const[name,row]of Object.entries(owners))if(row.tag&&replaceTags?.includes(row.tag)&&!returned.has(name)){delete owners[name];dropped.add(name);}
  for(const name of returned)dropped.delete(name);
  const renamed=new Map();
  for(const name of returned){const row=owners[name];if(row&&!row.tag&&row.kind==="project")renamed.set(row.root,name);}
  for(const[name,row]of Object.entries(owners))if(!returned.has(name)&&!row.tag&&row.kind==="project"&&renamed.has(row.root)){delete owners[name];dropped.add(name);}
  for(const name of carried??[])if(owners[name]&&!returned.has(name))owners[name]={...owners[name],edgesEnv:envDigest};
  for(const[name,row]of Object.entries(owners))if(row.edgesEnv!==envDigest)throw fail("Every owner needs edges of the published environment: "+name+" ("+JSON.stringify({kind:row.kind,tag:row.tag??null,edges:String(row.edgesEnv??"none").slice(0,12),environment:String(envDigest).slice(0,12),root:row.root,dirty:wave.includes(name),returned:returned.has(name),dropped:dropped.has(name),carried:(carried??[]).includes(name),sameRootReturned:[...returned].filter(other=>owners[other]?.root===row.root)})+")","GENERATION_INCOMPLETE");
  if(rest.length){
   const{owners:_table,...envParts}=computedParts??{};
   carry.owners=owners;carry.env=envDigest;carry.envParts=computedParts?envParts:undefined;carry.names??=ordered;
   for(const[name,rows]of upserts)if(name!==P.envOwner)carry.upserts.set(name,rows);
   for(const[name,rows]of append)carry.append.set(name,rows);
   for(const name of dropped)carry.dropped.add(name);
   for(const name of returned)carry.returned.add(name);
   carry.done=offset+wave.length;
   await journal.wave({key:journalKey,current,repair:resumeBinding,identity,implementation,carry,steps,upserts,append,owners,env:{digest:envDigest,...computedParts?{parts:envParts}:{}}});carry.waves++;
   if(claim&&typeof claim.settle==="function"){await claim.progress?.({phase:"owners",done:carry.done,total:offset+names.length}).catch(()=>{});await claim.settle(wave).catch(()=>false);}
   writePendingV1(process.env,current?.pointer.generation??-1,rest,priority);
   return{current,built:true,published:false,more:true,plan:found,computed:computed,phases:{...marks,...Object.fromEntries(Object.entries(computePhases??{}).map(([name,ms])=>["compute."+name,ms]))},carried:carried?.length??0,deferred:{generation:current?.pointer.generation??-1,live,atMs:now(),wave:{done:carry.done,size:nextOwnerWaveV1({size:wave.length,elapsedMilliseconds:now()-waveStartedMs,inferred:inferred,basisPoints:process.env.SEMIO_SLICE_WAVE_BASIS_POINTS})},plan:{...found,times:{},kind:"delta",dirty:new Set(rest),added:[],removed:[],pending:[],envStale:false,reasons:[],refresh:new Map(),stale:0,staleKinds:{}}}};
  }
  for(const[name,rows]of carry.upserts)if(!upserts.has(name))upserts.set(name,rows);
  for(const[name,rows]of carry.append)if(!append.has(name))append.set(name,rows);
  for(const name of carry.dropped)dropped.add(name);
  for(const name of carry.returned)returned.add(name);
  for(const name of returned)dropped.delete(name);
    if(noisy)say?.("indexing",{count:Object.keys(owners).length});
    await progress?.({phase:"patching",done:1,total:1});
    let index=await profilePhaseV1("graph.build.patch",async()=>{
     const base=cold?emptyFactIndexViewV1():repair?await repairPatchIndex(usable.factIndex,signal):await readIndexView(usable.factIndex);
     return patchView(base,{drop:[...dropped],upserts,append},signal);
    });
  
  if(cold){
   upserts.clear();append.clear();carry.upserts.clear();carry.append.clear();
   await disposeSources(carry.disposers);aborted(signal);
  }
  const orphans=orphanedDerivedOwnersV1(index);if(orphans.length)index=await patchView(index,{drop:orphans},signal);
  const unsupported=unsupportedDerivedV1(index);if(unsupported.length)throw fail("Derived facts need a supporting computation: "+unsupported.slice(0,3).map(row=>row.path.slice(0,96)).join(", "),"GENERATION_INCOMPLETE");

  mark("patch");
  if(noisy)say?.("sealing",{generation:(current?.pointer.generation??-1)+1});
  await progress?.({phase:"packing",done:1,total:1});
  const packedIndex=await profilePhaseV1("graph.build.pack",()=>packFactIndexBytesV1(index.document.buffer,P.maximumIndexBytes)),indexStored=await profilePhaseV1("graph.build.keep",()=>keep(P.indexes,packedIndex.bytes,".facts.br"));
  
  mark("pack");
  const globalDigest=repair&&!cold&&globalPayload===undefined?usable.global:(await store(P.fragments,{version:1,name:"@global",value:globalPayload??null},".json.br",P.maximumFragmentBytes)).digest;
  const frontierStored=await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes);
  const expectedGeneration=current?.pointer.generation??-1;
  const{owners:_table,...envParts}=computedParts??{},next={version:1,generation:expectedGeneration+1,parent:current?.pointer.manifest??null,identity,implementation,env:envDigest,membership:live.digest,frontier:frontierStored.digest,factIndex:indexStored.digest,global:globalDigest,...(computedParts?{envParts}:{}),owners,built:{kind:cold?"cold":"delta",base:cold?null:usable.generation,dirty:[...returned].sort(compareCodePointsV1)}};
  const manifestDigest=await writeManifest(next);aborted(signal);
  await qualify?.(next,signal);aborted(signal);
  await markStep("indexing",steps.at(-1).output,indexStored.digest,()=>journal.sealed({key:journalKey,current,repair:resumeBinding,identity,implementation,steps,manifest:manifestDigest,derived:[...carry.derived]}));
  const verified=stamp&&!rest.length?mergeVerifiedStampV1({owners:Object.keys(owners).sort(compareCodePointsV1),scope:cold?undefined:only,observedAtMs,carried:cold?undefined:coverage}):undefined;
  await progress?.({phase:"publishing",done:1,total:1});
  let outcome=await profilePhaseV1("graph.build.publish",()=>publish({manifestDigest,expectedGeneration,...repair?{expectedManifest:null,expectedRepairParent:repair.parent}:{},verified,signal,claim})),done=outcome.published?{next,index,packed:packedIndex,kept:indexStored}:undefined;
  mark("publish");
  if(!done&&repair){await journal.clear(journalKey);return supersededRepair(await read(signal),identity,computed);}
  if(!done&&!cold){index=undefined;const merged=await mergeIntoHeadV1({read,readIndex:readIndexView,writeManifest,publish:input=>publish({...input,claim}),patch:(base,patch)=>patchView(base,patch,signal),pack:value=>packFactIndexBytesV1(value.document.buffer,P.maximumIndexBytes),keep:bytes=>keep(P.indexes,bytes,".facts.br"),key:rowKeyV1,stats},{base:current,mine:next,patch:{drop:[...dropped],upserts,append},verified,signal});if(merged.outcome){outcome=merged.outcome;done=merged;}}
  
  if(!done)return{current:await read(signal),built:true,published:false,plan:found,computed:computed};
  indexCache.set("view:"+done.kept.digest,done.index);
  await journal.clear(journalKey);await markStep("sealing",indexStored.digest,manifestDigest);
  await profilePhaseV1("graph.build.indexed",()=>settler.markIndexed({held:found.refresh,dropped:[...carry.derived]}).catch(()=>undefined));
  if(!done.theirs)writePendingV1(process.env,outcome.pointer.generation,rest,priority);
  if(!rest.length)await profilePhaseV1("graph.build.stamps",async()=>doubt?.published?.({rows:done.index.rows,index:done.index,observedAt:observedAtMs,signal}).catch(()=>false));
  if(!rest.length&&await(claim?claim.alone({signal}).catch(()=>false):true))await profilePhaseV1("graph.build.retire",()=>retire({signal,claim}).catch(error=>{if(signal?.aborted)throw error;}));
  return{current:{pointer:outcome.pointer,manifest:done.next},built:true,published:true,plan:found,computed:computed,phases:{...marks,...Object.fromEntries(Object.entries(computePhases??{}).map(([name,ms])=>["compute."+name,ms]))},carried:carried?.length??0,...(done.theirs?{merged:done.theirs.length}:{}),...(carry.resumedFrom?{resumed:true}:{}),index:{rows:done.index.rows.length,owners:done.index.owners.filter(name=>!isDerivedOwnerV1(name)).length,expandedBytes:done.packed.expandedBytes,packedBytes:done.kept.bytes}};
 }

 /** 🧷️ Publishes the same owners under a refreshed membership frontier, so the next verification is a stat pass again; losing the swap is harmless. */
 async function refreshMetadata({current,live,signal,held,claim}){
  const manifest=current.manifest,frontier=live?await store(P.frontiers,live.frontier,".json.br",P.maximumIndexBytes):undefined;
  let factIndex=manifest.factIndex,patched;
  if(held?.size){const old=await readIndexView(manifest.factIndex);if(old){patched=await patchView(old,{upserts:derivedUpsertsV1([],held)},signal);indexCache.delete("view:"+manifest.factIndex);factIndex=(await keep(P.indexes,packFactIndexBytesV1(patched.document.buffer,P.maximumIndexBytes).bytes,".facts.br")).digest;}}
  const next={...manifest,generation:manifest.generation+1,parent:current.pointer.manifest,membership:live?live.digest:manifest.membership,frontier:frontier?frontier.digest:manifest.frontier,factIndex,built:{kind:"refresh",base:manifest.generation,dirty:[]}};
  const digest=await writeManifest(next);
  try{const outcome=await publish({manifestDigest:digest,expectedGeneration:current.pointer.generation,verified:current.pointer.verified,signal,claim});if(outcome.published){if(patched){indexCache.set("view:"+factIndex,patched);await settler.markIndexed({held}).catch(()=>undefined);}return{current:{pointer:outcome.pointer,manifest:next},published:true};}}
  catch(error){if(signal?.aborted||error?.code==="SEMIO_STALE_IMPLEMENTATION")throw error;}
  return{current:await read(signal)??current,published:false};
 }

 /** 🔁️ Brings the caller to a generation that holds. `current` serves the newest generation of this identity without verification or waiting; `fresh` verifies (only the closure `only` when given), serves a verified generation without any lease and builds otherwise, inside `builder` (which hands the build its slice claim; see `./🔐️custody`) or the builder lease, where a holder that finds the work done by a peer returns that generation untouched and a build whose swap a peer won merges onto the peer's generation (see `./🔀️merge`). */
 async function refresh({identity,implementation,policy="fresh",only,membership,compute,prime,signal,progress,builder,requestedAt,published,wave,planned,unchanged,repair,qualify}){
  if(!P.policies.includes(policy))throw fail("Closed graph policy required");
  const started=now(),first=await read(signal),usable=first?.manifest&&first.manifest.identity===identity;
  if(policy==="current"&&usable)return{current:first,served:"current",built:false,published:false,computed:0};
  if(repair){
   const body=claim=>build({identity,implementation,membership,compute,prime,signal,progress,published,wave,observedAtMs:started,claim,repair,qualify});
   const pending={generation:repair.generation,dirty:[],roots:{},structural:true};
   const result=await(builder?builder(body,pending):lease?lease.exclusive(resources.builder,body,{signal}):body());
   return{...result,served:"built"};
  }
  let hint,scope=only,coverage,pending,carried;
  if(usable&&requestedAt!==undefined&&first.pointer.verified!==undefined){
   const owners=Object.keys(first.manifest.owners).sort(compareCodePointsV1);
   coverage=coverageOfV1({stamp:first.pointer.verified,owners,implementation,manifestImplementation:first.manifest.implementation,requestedAtMs:requestedAt});
   if(coverage){const open=uncoveredOfV1({needed:only,owners,coverage});if(!open.size)return{current:first,served:"covered",built:false,published:false,computed:0};scope=open;}
  }
  if(usable){
   const previous={manifest:first.manifest,frontier:await readFrontier(first.manifest.frontier)},key=JSON.stringify([first.pointer.generation,scope?[...scope].sort(compareCodePointsV1):null,coverage!==undefined]),reuse=planned?.key===key&&planned.plan.kind==="delta"&&previous.frontier!==undefined&&await planned.holds().catch(()=>false);
   const live=reuse?planned.live:await profilePhaseV1("graph.refresh.membership",()=>membership(previous.frontier?previous:undefined,signal));
   const verdict=reuse?planned.plan:await profilePhaseV1("graph.refresh.plan",()=>plan({current:first,identity,changes:previous.frontier?live.changes:undefined,...previous.frontier&&live.dirty?{journalDirty:live.dirty}:{},only:scope,skipEnvironment:coverage!==undefined,signal,progress}));
   if(reuse)stats.plansReused++;
   if(verdict.kind==="clean"){
    if(!scope&&live.digest!==first.manifest.membership||verdict.refresh?.size){const refreshed=await refreshMetadata({current:first,live:!scope&&live.digest!==first.manifest.membership?live:undefined,signal,held:verdict.refresh});return{...refreshed,served:"verified",built:false,computed:0,plan:verdict};}
    return{current:first,served:"verified",built:false,published:false,computed:0,plan:verdict};
   }
   if(verdict.kind==="delta")pending={structural:verdict.envStale||verdict.added.length>0||verdict.removed.length>0,generation:first.pointer.generation,dirty:[...verdict.dirty].sort(compareCodePointsV1),roots:Object.fromEntries([...verdict.dirty].filter(name=>first.manifest.owners[name]).map(name=>[name,first.manifest.owners[name].root]))};
   if(verdict.kind==="delta"&&previous.frontier){hint={generation:first.pointer.generation,plan:verdict,live,atMs:now(),...unchanged?{holds:()=>unchanged(live)}:{}};carried={key,plan:verdict,live};}
  }
  const body=claim=>build({identity,implementation,membership,compute,prime,signal,progress,published,wave,hint,only:scope,observedAtMs:started,coverage,claim,qualify});
  let resumedComputed;
  for(let attempt=0;attempt<8;attempt++){
   const result=await(builder?builder(body,pending):lease?lease.exclusive(resources.builder,body,{signal}):body());
   if(result.resumed){resumedComputed=(resumedComputed??0)+(result.computed??0);continue;}
   if(result.published||!result.built)return{...result,...resumedComputed===undefined?{}:{published:true,computed:(result.computed??0)+resumedComputed},served:"built",...result.needsBuild&&carried?{planned:carried}:{}};
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
 async function refreshClosure({roots,neighbors,closure,...request}){
  const {identity,policy="fresh",signal}=request;
  const closeOf=current=>closure?closure(current,{signal}):closureOf(current,roots,{neighbors,signal});
  const first=await read(signal);let names,unborn=false;
  if(first?.manifest&&first.manifest.identity===identity){try{names=await closeOf(first);}catch(error){if(error?.code==="GENERATION_UNKNOWN_OWNER"){names=new Set();unborn=true;}else if(error?.code!=="GENERATION_INCOMPLETE")throw error;}}
  let verified=names,result=await refresh({...request,only:names});
  if(result.needsBuild)return result;
  let built=result.built,published=result.published,computed=result.computed;
  for(let round=0;;round++){
   const closure=verified&&!unborn&&result.current.pointer.generation===first.pointer.generation?verified:await closeOf(result.current);
   if(policy==="current"||!verified||round>=8||[...closure].every(name=>verified.has(name)))return{...result,built,published,computed,...(built?{served:"built"}:{}),names:closure};
   verified=new Set([...verified,...closure]);result=await refresh({...request,only:verified,repair:undefined});
   if(result.needsBuild)return result;built=built||result.built;published=published||result.published;computed+=result.computed;
  }
 }

 /** 🧹️ Removes documents that no retained generation names once they are older than the grace window (every file of the index directory is an index document, whatever format its generation used); a failure only costs disk. */
 async function retire({signal,claim}={}){
  aborted(signal);const pointer=await readPointer();if(!pointer)return;
  const files=await fs.list(join(root,P.manifests)),recent=[],live=new Set(),fragments=new Set(),indexes=new Set(),frontiers=new Set();
  const repairParent=pointer.manifest===null&&pointer.reason==="closure-missing"?pointer.parent:null;
  const keep=({digest,factIndex,frontier,global,ownerFragments})=>{
   live.add(digest);indexes.add(factIndex);fragments.add(global);
   if(frontier)frontiers.add(frontier);
   for(const fragment of ownerFragments)fragments.add(fragment);
  };
  for(const file of files){
   aborted(signal);if(!/^[a-f0-9]{64}\.json$/u.test(file.name))continue;
   const digest=file.name.slice(0,64),bytes=await fs.read(manifestPath(digest));
   if(!bytes)throw fail("Generation manifest disappeared before retirement","GENERATION_INCOMPLETE");
   if(bytes.length>P.maximumManifestBytes||sha256(bytes)!==digest)throw fail("Generation manifest content differs from its name","GENERATION_CORRUPT");
   const {generation,factIndex,frontier,global,owners}=readGenerationManifestV1(bytes);
   const at=recent.findIndex(row=>row.generation<generation),keepRecent=at>=0||recent.length<P.retainGenerations,required=digest===pointer.manifest||digest===repairParent;
   if(!keepRecent&&!required)continue;
   const row={digest,generation,factIndex,frontier,global,ownerFragments:Object.values(owners).map(owner=>owner.fragment)};
   if(required)keep(row);
   if(keepRecent){recent.splice(at<0?recent.length:at,0,row);if(recent.length>P.retainGenerations)recent.pop();}
  }
  if(pointer.manifest!==null&&!live.has(pointer.manifest))throw fail("Current generation manifest missing before retirement","GENERATION_INCOMPLETE");
  for(const row of recent)if(!live.has(row.digest))keep(row);
  const cutoff=now()-P.graceMilliseconds;
  const sweep=async(directory,keep,extension)=>{for(const file of await fs.list(join(root,directory))){aborted(signal);if(file.name.endsWith(extension)&&!keep.has(file.name.slice(0,64))&&file.modifiedMs<cutoff)await fs.remove(join(root,directory,file.name));}};
  await sweep(P.manifests,live,".json");await sweep(P.fragments,fragments,".json.br");await sweep(P.indexes,indexes,"");await sweep(P.frontiers,frontiers,".json.br");await sweep(BUILD_JOURNAL_V1.directory,new Set(),"");await nodes?.prune({now:now()});
  await retireClosureRepairReceiptsV1({root,fs,pointer,cutoff,signal,fence:claim?.fence?body=>claim.fence(body,{signal}):undefined});
 }

 /** ♻️ Marks a generation invalid without deleting anything, optionally fenced to the observed manifest. */
 async function invalidate({reason="invalidated",expected,signal,claim}={}){
  const generation=expected?.generation??(await readPointer())?.generation??-1;
  const outcome=await publish({manifestDigest:null,expectedGeneration:generation,expectedManifest:expected?.manifest,reason,signal,claim});
  return outcome.published;
 }

 return Object.freeze({dropIndexes:()=>indexCache.clear(),read,readManifestAt,readIndex,readIndexView,readFrontier,readFragment,plan,build,refresh,refreshClosure,closureOf,assemble,assembleStream,snapshot,retire,invalidate,publish,stats,readGlobal,catalog});
}

