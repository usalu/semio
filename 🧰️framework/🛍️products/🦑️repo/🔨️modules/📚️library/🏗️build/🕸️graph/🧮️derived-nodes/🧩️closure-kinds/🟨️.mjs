import{createHash}from"node:crypto";
import{isAbsolute,relative as relativePath,resolve,sep}from"node:path";
import{createDerivedGraphV1}from"../🟨️.mjs";
import{KIND_RULES_V1}from"../🧩️kinds/🟨️.mjs";
import{createWorkspaceGraphV1}from"../🏭️workspace/🟨️.mjs";
import{canonicalJsonV1}from"../../🕰️generations/🧮️fact-codec/🟨️.mjs";
import{rowKeyV1}from"../../🕰️generations/🧮️fact-rows/🟨️.mjs";
import{affectedPartsV1,createManifestV1,indexPartV1,intersectsV1,partIdsOfV1,readManifestV1,recordEntryV1,writeManifestV1}from"./📒️manifest/🟨️.mjs";
import{dirtySummaryV1,rowPathsV1,rowTouchedV1}from"./🧭️row-paths/🟨️.mjs";
import{CUTOFF_POLICY_V1}from"../../🕰️generations/✂️cutoff/🟨️.mjs";
import{refuseBuildV1}from"../../../📖️texts/🟦️.ts";
import{runtimeCollectGarbageV1}from"../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🧩️ The node kinds of the command closures: the answers of the questions put to the pure walkers of inference (a bounded script-input module walk, a TypeScript program, a derived walk recipe). */
export const CLOSURE_KINDS_V1=Object.freeze(["script-walk","ts-program","derived-walk"]);

/** 🧩️ The finite limits of settling derived questions through the node graph. */
export const CLOSURE_SETTLE_V1=Object.freeze({deriveConcurrency:CUTOFF_POLICY_V1.deriveConcurrency,collectEveryClosures:CUTOFF_POLICY_V1.collectEveryClosures,settleChunkClosures:CUTOFF_POLICY_V1.settleChunkClosures,maximumSuspects:CUTOFF_POLICY_V1.maximumSuspects,environmentKey:"closure-kinds:1"});

/** 🏷️ The node kind of one question: `program` questions are TypeScript programs, `derive` questions derived walks, every other question a script walk. */
export function closureKindOfV1(path){
 let question;try{question=JSON.parse(path);}catch{return"script-walk";}
 return question?.program===1?"ts-program":question?.derive===1?"derived-walk":"script-walk";
}

const digestOfIds=(fact,ids)=>createHash("sha256").update(canonicalJsonV1([fact,ids])).digest("hex");
const PATH_ROWS=new Set(["file","exists","stat","lstat","fileExists","directoryExists","readdir","readdirNames","typescriptText","analysis","realpath","realpathNative","typescriptRealpath"]);
const CLOSURE_PATH="\0closure:";
const VOLATILE="\0volatile";

const partIds=new WeakMap();
/** 🪪️ The identity of one part of a closure's support (the rows one module node observed, or the rows of one shared computation): the digest of its row keys, kept per array, so a part shared by thousands of closures is identified and held once. */
const partIdOf=rows=>{let id=partIds.get(rows);if(id===undefined){const hash=createHash("sha256");for(const row of rows)hash.update(rowKeyV1(row)).update("\n");id=hash.digest("hex").slice(0,20);partIds.set(rows,id);}return id;};

const closureRule=async ctx=>{
 const answer=await ctx.environment.derive({kind:"derived",path:ctx.self.subject},ctx.signal);
 ctx.environment.computed(ctx.self.subject);
 const ids=[...new Set((answer.parts??[answer.rows]).filter(rows=>rows.length>0).map(rows=>ctx.environment.parts.put(rows)))].sort();
 const digest=digestOfIds(answer.fact,ids);
 ctx.observed({paths:[CLOSURE_PATH+digest],digest,rows:ids});
 return{fact:answer.fact};
};

/** 📚️ The support of one held closure as the row of the question followed by the rows of its parts, read part by part: a held closure costs its list of parts, and the rows are produced where they are consumed. */
export const partRowsV1=(row,parts,digest,ids,fact)=>Object.freeze({digest,parts,ids,fact,length:1+parts.reduce((sum,rows)=>sum+rows.length,0),*[Symbol.iterator](){yield row;for(const rows of parts)yield*rows;}});

/** 🧩️ The rules of the closure kinds: the existing walkers run through `environment.derive` and every row they read is recorded as one observed read, verified by `environment.hold`. */
export const CLOSURE_RULES_V1=Object.freeze(Object.fromEntries(CLOSURE_KINDS_V1.map(kind=>[kind,closureRule])));

/** 🔎 Re-checks the observed rows of a closure: a closure that read nothing never holds, every other holds while all rows still hold on the disk (or the pinned view). */
export const verifyClosureObservedV1=async(ref,{signal,environment})=>Array.isArray(ref.rows)&&ref.rows.length>0&&await environment.holdParts(ref.rows,signal);

const idleWorld=Object.freeze({read:async()=>undefined,files:async()=>[]});

let grantOfProcess;
/** 🎟️ Binds the capacity grant the scheduler admitted to this process: the workspace graphs size their concurrency and memory from it. */
export function useCapacityGrantV1(grant){grantOfProcess=grant;}
const currentGrantV1=()=>grantOfProcess?.();

const WORKSPACE_SETTLERS_V1=16;
const workspaceSettlers=new Map();
/** 🏭️ The settler of one workspace, shared by every plan of the host service: one node graph over the disk world, the shared stat index and the node store below `cacheRoot` (at most 16 workspaces, the least recently used one is dropped). */
export function sharedClosureSettlerV1(options){
 const known=workspaceSettlers.get(options.workspaceRoot);
 if(known!==undefined){workspaceSettlers.delete(options.workspaceRoot);workspaceSettlers.set(options.workspaceRoot,known);return known;}
 const settler=createClosureSettlerV1({workspace:options});
 workspaceSettlers.set(options.workspaceRoot,settler);
 while(workspaceSettlers.size>WORKSPACE_SETTLERS_V1)workspaceSettlers.delete(workspaceSettlers.keys().next().value);
 return settler;
}

/**
 * ❓️ The settler of derived questions: one node graph per store (in memory, bounded, and `store` makes the answers durable and shared between plans). `settle` asks the suspect questions again: a recorded answer whose rows still hold is taken (the cutoff: an equal answer keeps every holder valid), otherwise the walker runs, and a different answer or a failing walk changes the fact. `hold(rows, signal)` says whether rows still hold; a shared row is checked once per call and a failing row is found by bisection. Calls on one settler run one after the other.
 * @see ../🟨️.mjs
 */
export function createClosureSettlerV1({store,workspace,limits=CLOSURE_SETTLE_V1}={}){
 let scope,chain=Promise.resolve();
 const root=workspace?.workspaceRoot===undefined?undefined:resolve(workspace.workspaceRoot);
 const relative=path=>{if(root===undefined||typeof path!=="string"||!isAbsolute(path))return path;const inner=relativePath(root,path);return inner===""?"":inner.startsWith("..")||isAbsolute(inner)?VOLATILE:inner.split(sep).join("/");};
 let shared;
 const table=new Map(),metas=new Map(),persisted=new Set(),uses=new Map();
 const partIdentity=id=>"closure-part\0"+id+"\0",partStore=()=>shared?.store??store;
 const parts=Object.freeze({put(rows){const id=partIdOf(rows);uses.set(id,(uses.get(id)??0)+1);if(!table.has(id)){rows=rows.map(internRow);table.set(id,rows);if(partStore()===undefined||!persisted.has(id)){persisted.add(id);partStore()?.put({key:{kind:"closure-part",subject:id,params:""},value:{rows},deps:[],rawReads:[],valueDigest:id,computedAtRev:"0"},"");buildMeta(id);}}return id;},rowsOf:id=>table.get(id)});
 const loadPart=async id=>{let rows=table.get(id);if(rows!==undefined)return rows;const record=await partStore()?.get(partIdentity(id));rows=record?.value?.rows;if(Array.isArray(rows)){rows=rows.map(internRow);table.set(id,rows);}return Array.isArray(rows)?rows:undefined;};
 const manifestIdentity="closure-manifest\0all\0";
 let manifest,manifestDirty=false;
 const manifestOf=async()=>{if(manifest!==undefined)return manifest;const value=(await partStore()?.get(manifestIdentity))?.value;manifest=value===undefined?createManifestV1():readManifestV1(value);return manifest;};
 const saveManifest=async()=>{if(!manifestDirty||manifest===undefined)return;manifestDirty=false;partStore()?.put({key:{kind:"closure-manifest",subject:"all",params:""},value:writeManifestV1(manifest),deps:[],rawReads:[],valueDigest:"manifest",computedAtRev:"0"},"");await partStore()?.flush?.();};
 let activeSummary;const slimOf=meta=>({size:meta.size,outside:meta.outside,touched:activeSummary===undefined||rowTouchedV1(meta,activeSummary)});
 const metaIdentity=id=>"closure-meta\0"+id+"\0",outsideRows=new Map();
 const internOutside=row=>{const key=rowKeyV1(row);let known=outsideRows.get(key);if(known===undefined)outsideRows.set(key,known=row);return known;};
 const rowPool=new Map();
 const internRow=row=>{const key=rowKeyV1(row),known=rowPool.get(key)?.deref();if(known!==undefined)return known;rowPool.set(key,new WeakRef(row));return row;};
 const sweepRows=()=>{for(const[key,ref]of rowPool)if(ref.deref()===undefined)rowPool.delete(key);};
 const buildMeta=id=>{const paths=new Set(),dirs=new Set(),bare=new Set(),rows=table.get(id)??[],meta={paths:[],dirs:[],bare:[],config:false,volatile:false,outside:[],size:rows.length};for(const row of rows){const one=rowPathsV1(row,relative,VOLATILE);if(one.outside)meta.outside.push(internOutside(row));for(const path of one.paths)paths.add(path);for(const dir of one.dirs)dirs.add(dir);for(const dir of one.bare)bare.add(dir);meta.config||=one.config;meta.volatile||=one.volatile;}meta.paths=[...paths];meta.dirs=[...dirs];meta.bare=[...bare];if(manifest!==undefined)indexPartV1(manifest,id,meta);partStore()?.put({key:{kind:"closure-meta",subject:id,params:""},value:meta,deps:[],rawReads:[],valueDigest:id,computedAtRev:"0"},"");const slim=slimOf(meta);metas.set(id,slim);return slim;};
 const metaOf=id=>metas.get(id)??(table.has(id)?buildMeta(id):undefined);
 const metaLoad=async id=>{let meta=metas.get(id);if(meta!==undefined)return meta;const value=(await partStore()?.get(metaIdentity(id)))?.value;if(value!==undefined&&Array.isArray(value.paths)&&Number.isSafeInteger(value.size)){const full={...value,outside:value.outside.map(internOutside)};if(manifest!==undefined)indexPartV1(manifest,id,full);meta=slimOf(full);metas.set(id,meta);return meta;}return await loadPart(id)===undefined?undefined:buildMeta(id);};
 const environment={parts,derive:(row,signal)=>scope.derive(row,signal),hold:(rows,signal)=>scope.holds(rows,signal),holdParts:(ids,signal)=>scope.holdParts(ids,signal),computed:subject=>scope.computed.add(subject),relative,listProjectFiles:async signal=>(await shared.world.files("",signal)).filter(path=>/(^|\/)(📋️project\.json|Cargo\.toml)$/u.test(path)).sort()};
 shared=workspace===undefined?undefined:createWorkspaceGraphV1({...workspace,grant:workspace.grant??currentGrantV1,rules:{...KIND_RULES_V1,...CLOSURE_RULES_V1},environment,environmentKey:limits.environmentKey,verifyObserved:verifyClosureObservedV1});
 const graph=shared?.graph??createDerivedGraphV1({rules:CLOSURE_RULES_V1,world:idleWorld,store,environment,environmentKey:limits.environmentKey,verifyObserved:verifyClosureObservedV1,concurrency:limits.deriveConcurrency});
 const run=async options=>{try{return await runOnce(options);}finally{scope=undefined;}};
 const runOnce=async({pseudoRows,derive,hold=async()=>true,dirty,signal,progress,answers:keepAnswers=false})=>{
  const names=[...pseudoRows.keys()].sort(),held=new Map(),changed=new Map(),answers=new Map(),verified=new Set(),broken=new Set(),fresh=new Set();let failed=0,done=0;
  const kept=new Set();
  if(typeof derive!=="function"){for(const name of names)changed.set(name,pseudoRows.get(name));return{held,kept,changed,answers,failed,reused:0};}
  if(names.length>limits.maximumSuspects)throw refuseBuildV1("refused","graph.cutoff-suspects-unbounded",{count:names.length,maximum:limits.maximumSuspects});
  const classify=async(list,abort)=>{
   if(await hold(list,abort)){for(const row of list)verified.add(row);return true;}
   if(list.length===1){broken.add(list[0]);return false;}
   const middle=list.length>>1,first=await classify(list.slice(0,middle),abort),second=await classify(list.slice(middle),abort);
   return first&&second;
  };
  const summary=dirty===undefined||dirty.overflow===true?undefined:dirtySummaryV1(dirty);activeSummary=summary;
  const pass={transient:true,...summary===undefined?{}:{dirty:summary}},partOk=new Map();
  const partHolds=async(id,abort)=>{
   if(summary!==undefined){const meta=await metaLoad(id);if(meta!==undefined&&!meta.touched)return meta.outside.length===0||await scope.holds(meta.outside,abort);}
   const rows=await loadPart(id);return rows!==undefined&&await scope.holds(rows,abort);
  };
  let derivations=0;
  scope={derive:async(row,abort)=>{const answer=await derive(row,abort,pass);if(++derivations%limits.collectEveryClosures===0)runtimeCollectGarbageV1(true);return answer;},computed:fresh,holdParts:async(ids,abort)=>{for(const id of ids){let ok=partOk.get(id);if(ok===undefined){ok=await partHolds(id,abort);partOk.set(id,ok);}if(!ok)return false;}return true;},holds:async(rows,abort)=>{
   const unchecked=[];
   for(const row of rows){if(broken.has(row))return false;if(!verified.has(row))unchecked.push(row);}
   return unchecked.length===0||classify(unchecked,abort);
  }};
  const keys=new Map(names.map(name=>[name,graph.key(closureKindOfV1(pseudoRows.get(name).path),pseudoRows.get(name).path)])),nodes=new Map(),broke=new Set();
  const pool=async(list,work)=>{let next=0;await Promise.all(Array.from({length:Math.min(limits.deriveConcurrency,list.length)},async()=>{while(next<list.length){signal?.throwIfAborted();await work(list[next++]);}}));};
  const stage=async(phase,total,work)=>{if(total===0)return work(()=>undefined);let at=0;await progress?.({phase,done:0,total});const result=await work(()=>progress?.({phase,done:Math.min(++at,total),total}));if(at<total)await progress?.({phase,done:total,total});return result;};
  const ask=async name=>{try{nodes.set(name,await graph.get(keys.get(name),{signal}));}catch{signal?.throwIfAborted();broke.add(name);}};
  const forgetParts=(final=false)=>{if(!final&&partStore()===undefined)return;const retained=new Set([...held.values()].flatMap(rows=>rows.parts));for(const[id,rows]of[...table])if(!retained.has(rows)&&(final||uses.get(id)===1))table.delete(id);if(final){metas.clear();outsideRows.clear();rowPool.clear();persisted.clear();uses.clear();}};
  const settleChunk=async chunk=>{
   await pool(chunk,async name=>{await ask(name);await progress?.({phase:"verification",done:Math.min(++asked,names.length),total:names.length});});
   const again=chunk.filter(name=>nodes.has(name)&&!fresh.has(pseudoRows.get(name).path)).map(name=>[name,nodes.get(name)]);
   if(again.length){
    const partIds=[...new Set(again.flatMap(([,node])=>node.rawReads.filter(ref=>ref.kind==="observed").flatMap(ref=>ref.rows)))];
    await stage("parts",partIds.length,async tick=>{await Promise.all(partIds.map(async id=>{await metaLoad(id);await tick();}));});
    const dirtyParts=new Map();
    const touched=id=>{let hit=dirtyParts.get(id);if(hit===undefined){const meta=metas.get(id);hit=summary===undefined||meta===undefined||meta.touched;dirtyParts.set(id,hit);}return hit;};
    const forced=[];
    for(const[,node]of again)for(const ref of node.rawReads)if(ref.kind==="observed"&&ref.rows.some(touched))forced.push(...ref.paths);
    await stage("invalidation",1,async tick=>{try{await graph.invalidate({paths:forced,...dirty?.directories?{directories:dirty.directories}:{}},{signal});}catch{signal?.throwIfAborted();}await tick();});
    const redo=again.map(([name])=>name);
    await stage("rederivation",redo.length,tick=>pool(redo,async name=>{await ask(name);await tick();}));
   }
   for(const name of chunk){
    const row=pseudoRows.get(name),node=nodes.get(name);
    if(broke.has(name)||node===undefined){failed++;changed.set(name,row);if(book.entries.delete(name))manifestDirty=true;}
    else{
     const ids=node.rawReads.find(ref=>ref.kind==="observed")?.rows??[],digest=node.rawReads.find(ref=>ref.kind==="observed")?.digest,equal=canonicalJsonV1(node.value.fact)===canonicalJsonV1(row.fact);
     let size=1;for(const id of ids)size+=(await metaLoad(id))?.size??0;
     const mark=equal&&digest!==undefined?book.marks.get(name):undefined;
     if(digest!==undefined){recordEntryV1(book,name,{digest,fact:canonicalJsonV1(node.value.fact),length:size,ids});manifestDirty=true;}
     if(mark!==undefined&&mark.digest===digest&&mark.length===size)kept.add(name);
     else{
      const parts=(await Promise.all(ids.map(loadPart))).filter(rows=>rows!==undefined),support=partRowsV1(row,parts,digest,ids,canonicalJsonV1(node.value.fact));
      if(keepAnswers)answers.set(name,{fact:node.value.fact,digest,length:size,ids,canonical:canonicalJsonV1(node.value.fact),get rows(){return[...partRowsV1(row,parts)].slice(1);}});
      if(equal)held.set(name,support);else changed.set(name,row);
     }
    }
    nodes.delete(name);done++;
   }
   await stage("flush",1,async tick=>{await graph.flush();await tick();});
   forgetParts();
   runtimeCollectGarbageV1(true);sweepRows();
  };
  let asked=0;
  if(names.length)await progress?.({phase:"verification",done:0,total:names.length});
  const book=await manifestOf(),slow=[];
  const answerFromManifest=async(name,entry)=>{
   const row=pseudoRows.get(name),equal=entry.fact===canonicalJsonV1(row.fact),mark=book.marks.get(name);
   if(equal&&mark!==undefined&&mark.digest===entry.digest&&mark.length===entry.length){kept.add(name);return;}
   if(!equal){changed.set(name,row);return;}
   const ids=partIdsOfV1(book,entry),loaded=(await Promise.all(ids.map(loadPart))).filter(rows=>rows!==undefined);
   held.set(name,partRowsV1(row,loaded,entry.digest,ids,entry.fact));
  };
  if(summary!==undefined&&!keepAnswers&&book.entries.size){
   const affected=await affectedPartsV1(book,summary,async rows=>{if(await classify(rows,signal))return[];return rows.filter(row=>broken.has(row));});
   for(const name of names){const entry=book.entries.get(name);if(entry===undefined||intersectsV1(entry.bits,affected)){slow.push(name);continue;}await answerFromManifest(name,entry);done++;asked++;}
  }else slow.push(...names);
  if(asked>0)await progress?.({phase:"verification",done:asked,total:names.length});
  for(let at=0;at<slow.length;at+=limits.settleChunkClosures)await settleChunk(slow.slice(at,at+limits.settleChunkClosures));
  forgetParts(true);await saveManifest();runtimeCollectGarbageV1(true);
  return{held,kept,changed,answers,failed,reused:Math.max(0,names.length-failed-fresh.size)};
 };
 return Object.freeze({graph,store:shared?.store??store,settle:options=>{const next=chain.then(()=>run(options),()=>run(options));chain=next.catch(()=>undefined);return next;},invalidate:(dirty,options)=>graph.invalidate(dirty,options),release:async()=>{if(await graph.release?.()===undefined)return;await saveManifest();manifest=undefined;table.clear();metas.clear();outsideRows.clear();rowPool.clear();},markIndexed:async({held=new Map(),dropped=[]})=>{const book=await manifestOf();for(const[name,rows]of held)if(rows.digest!==undefined&&!dropped.includes(name)){book.marks.set(name,{digest:rows.digest,length:rows.length});manifestDirty=true;}for(const name of dropped)if(book.marks.delete(name))manifestDirty=true;await saveManifest();},stats:()=>graph.stats()});
}
