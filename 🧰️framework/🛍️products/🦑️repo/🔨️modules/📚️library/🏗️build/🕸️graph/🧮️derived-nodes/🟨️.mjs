import{createHash}from"node:crypto";
import{availableParallelism,freemem}from"node:os";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import{canonicalJsonV1}from"../🕰️generations/🧮️fact-codec/🟨️.mjs";
import{entryHashV1,globSpecV1,hexOfV1,rosterHashV1}from"./🧩️kinds/🟨️.mjs";

/** 🧮️ The declared constants of the derived-node DAG; the schema file is the single source. */
export const DERIVED_NODES_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
const P=DERIVED_NODES_POLICY_V1,ABSENT="absent";

/** #️⃣ The sha256 of a text or byte string. */
export const sha256V1=data=>createHash("sha256").update(data).digest("hex");

/** #️⃣ The digest of a JSON value, by its canonical text. */
export const digestOfV1=value=>sha256V1(canonicalJsonV1(value));

/**
 * 🌡️ The `capacity` port from the host capacity grant: `source()` answers the grant the scheduler admitted (`handle` is the `HostCapacityHandleV1` with `cpu`, `jobs`, `reservedMegabytes`; `limit` the `JobLimitPolicyV1` with `memoryMegabytes`, `cpuShares`) and `undefined` when the process runs ungranted, then the host (`os`) answers. The memory this process may still use is the granted memory minus its resident size, a quarter of the grant stays in reserve.
 * @see ../../../🌡️host-capacity/🟦️.ts
 */
export function capacityOfGrantV1(source,{rss=()=>process.memoryUsage.rss(),host=()=>({cpus:availableParallelism(),availableBytes:freemem()})}={}){
 return()=>{
  const{handle,limit}=source()??{},megabytes=limit?.memoryMegabytes??handle?.reservedMegabytes;
  if(megabytes===undefined)return host();
  const granted=megabytes*1048576;
  return{cpus:Math.max(1,handle?.cpu??handle?.jobs??Math.ceil((limit?.cpuShares??1024)/1024)),availableBytes:Math.max(0,granted-rss()),reserveBytes:Math.floor(granted/4)};
 };
}

/** 🔑 The key of one node: its kind, its subject and the digest of its declared params. */
export const nodeKeyV1=(kind,subject,params={})=>({kind,subject,params:digestOfV1(params)});

/** 🪪 The identity of a key as one string. */
export const keyIdV1=key=>key.kind+"\0"+key.subject+"\0"+key.params;

const unknown=(id)=>Object.assign(new Error("Derived node params unknown: "+id.replaceAll("\0","|")),{code:"DERIVED_NODE_UNKNOWN"});
const isAbort=error=>error?.name==="AbortError"||error?.code==="ABORT_ERR";

function limiterOf(count){
 let active=0;const queue=[],limit=typeof count==="function"?count:()=>count;
 const next=()=>{while(active<limit()&&queue.length){active++;queue.shift()();}};
 return work=>new Promise((resolve,reject)=>{queue.push(()=>work().then(resolve,reject).finally(()=>{active--;next();}));next();});
}

/**
 * 🧮️ The derived-node graph: pull-recompute with push-invalidate and early cutoff (red-green marking). `get` answers a node, revalidating or deriving only what is unverified; `invalidate` takes the paths a journal reports, rechecks only the raw reads those paths touch and walks the changed values upward by depth, stopping at every node whose value digest is unchanged. A glob read (`ctx.glob`) keeps its members as one file read each plus two additive multiset hashes, so `ctx.globDelta` patches the previous answer with the dirty entries only: the cost of an edit does not depend on the size of the glob. Single-flight per key, bounded node count (least recently used nodes are evicted together with every node above them), world reads limited by `concurrency`, verification streamed in slices, every wait cancellable. `rules[kind](ctx)` is a pure function of `ctx.file`, `ctx.glob` and `ctx.node`.
 * @see ./🧬️schema/🔣️.json
 */
export function createDerivedGraphV1({rules,world,store,statIndex,environment,environmentKey,verifyObserved,capacity=()=>({cpus:availableParallelism(),availableBytes:freemem()}),release=globalThis.Bun?.gc?()=>globalThis.Bun.gc(true):undefined,revision="0",concurrency=P.deriveConcurrency,maximumNodes=P.maximumNodes,trace=false}){
 const nodes=new Map(),registry=new Map(),rdeps=new Map(),byPath=new Map(),byRoot=new Map(),inflight=new Map(),pending=new Set(),gate=limiterOf(concurrency),heavy=limiterOf(()=>{const grant=capacity();return Math.max(1,Math.min(P.heavyMaximum,Math.floor(grant.cpus/2),Math.floor((grant.availableBytes-(grant.reserveBytes??P.heavyReserveBytes))/P.heavyBytes)));}),log=[],specs=new WeakMap(),verifiers=new WeakMap(),frozen=environment===undefined?undefined:Object.freeze(environment);
 const counters={computed:0,cutoffHit:0,revalidated:0,rawReads:0,statReused:0,stampHeld:0,evicted:0,patched:0,released:0};
 let rev=revision,operations=0,retained=0,held=new Map(),dirtyNow={all:false,paths:new Set(),directories:[],parents:new Set()},reads=new Map(),changedLog=[];
 const add=(map,key,id)=>{let set=map.get(key);if(!set)map.set(key,set=new Set());set.add(id);};
 const drop=(map,key,id)=>{const set=map.get(key);if(set){set.delete(id);if(!set.size)map.delete(key);}};
 const specOf=ref=>{let spec=specs.get(ref);if(!spec)specs.set(ref,spec=globSpecV1(ref.path,ref.patterns));return spec;};
 const linkDeps=rec=>{for(const dep of rec.deps)add(rdeps,dep.id,rec.id);};
 const unlinkDeps=rec=>{for(const dep of rec.deps)drop(rdeps,dep.id,rec.id);};
 const linkReads=rec=>{for(const ref of rec.observed.values())for(const path of ref.paths)add(byPath,path,rec.id);for(const path of rec.files.keys())add(byPath,path,rec.id);for(const ref of rec.globs.values())for(const base of ref.bases)add(byRoot,base,rec.id);};
 const unlinkReads=rec=>{for(const ref of rec.observed.values())for(const path of ref.paths)drop(byPath,path,rec.id);for(const path of rec.files.keys())drop(byPath,path,rec.id);for(const ref of rec.globs.values())for(const base of ref.bases)drop(byRoot,base,rec.id);};
 const locate=(ref,path)=>{
  let low=0,high=ref.paths.length-1;
  while(low<=high){const mid=(low+high)>>1,at=ref.paths[mid];if(at===path)return mid;if(at<path)low=mid+1;else high=mid-1;}
  return -(low+1);
 };
 const note=(kind,id)=>{if(trace){log.push({kind,id});if(log.length>P.maximumTrace)log.shift();}};
 const registerKey=(kind,subject,params={})=>{const key=nodeKeyV1(kind,subject,environmentKey===undefined?params:{params,environment:environmentKey}),id=keyIdV1(key);registry.delete(id);registry.set(id,params);while(registry.size>maximumNodes*2)registry.delete(registry.keys().next().value);return key;};
 const rawReadsOf=rec=>[...rec.files.values(),...rec.globs.values(),...rec.observed.values()];
 const isDirtyPath=path=>dirtyNow.all||dirtyNow.paths.has(path)||dirtyNow.directories.some(dir=>dir===""||path===dir||path.startsWith(dir+"/"));
 const ancestorsOf=path=>{const out=[];for(let at=path;at!=="";){const cut=at.lastIndexOf("/");at=cut<0?"":at.slice(0,cut);out.push(at);}return out;};
 const toPublic=rec=>({key:rec.key,valueDigest:rec.valueDigest,value:rec.value,deps:rec.deps.map(dep=>({key:dep.key,digest:dep.digest})),get rawReads(){return rawReadsOf(rec);},computedAtRev:rec.computedAtRev});
 const fromPublic=(id,node,params)=>{
  const files=new Map(),globs=new Map(),observed=new Map();
  for(const ref of node.rawReads){if(ref.kind==="file")files.set(ref.path,ref);else if(ref.kind==="observed")observed.set(ref.digest,ref);else globs.set(ref.path+"\0"+ref.patterns.join("\0"),ref);}
  return{id,key:node.key,params,value:node.value,valueDigest:node.valueDigest,deps:node.deps.map(dep=>({id:keyIdV1(dep.key),key:dep.key,digest:dep.digest,depth:1})),files,globs,observed,computedAtRev:node.computedAtRev,valid:false,full:true,depth:1};
 };
 const invalidateAbove=(id,seen=new Set())=>{
  for(const above of rdeps.get(id)??[]){if(seen.has(above))continue;seen.add(above);const rec=nodes.get(above);if(rec){rec.valid=false;invalidateAbove(above,seen);}}
 };
 const evict=()=>{
  for(const[id,rec]of nodes){
   if(nodes.size<=maximumNodes)return;
   if(inflight.has(id))continue;
   invalidateAbove(id);unlinkDeps(rec);unlinkReads(rec);nodes.delete(id);pending.delete(id);rdeps.delete(id);counters.evicted++;
  }
 };
 const fileOf=(path,signal)=>{
  let known=reads.get(path);
  if(known===undefined){
   known=gate(async()=>{signal?.throwIfAborted();let bytes,digest;if(statIndex){const found=await statIndex.digest(path,world,signal);digest=found.digest;bytes=found.bytes;if(found.read)counters.rawReads++;else if(!found.absent)counters.statReused++;}else{counters.rawReads++;bytes=await world.read(path);digest=bytes===undefined?ABSENT:sha256V1(bytes);}if(bytes!==undefined&&bytes.length<=P.retainedFileBytes&&retained+bytes.length<=P.retainedBytesPerOperation){retained+=bytes.length;held.set(path,bytes);}reads.set(path,digest);return digest;});
   reads.set(path,known);
  }
  return known;
 };
 const owners=new Map(),views=new Map();
 let listed=0,memberCache=new Map();
 const lower=(list,key)=>{let low=0,high=list.length;while(low<high){const mid=(low+high)>>1;if(list[mid]<key)low=mid+1;else high=mid;}return low;};
 const touch=owner=>{owners.delete(owner.root);owners.set(owner.root,owner);};
 const rangeOf=(owner,root)=>root===""?[0,owner.arr.length]:[lower(owner.arr,root+"/"),lower(owner.arr,root+"0")];
 const viewOf=(owner,root)=>{const[lo,hi]=rangeOf(owner,root);return{arr:owner.arr,lo,hi};};
 const listingOf=async(root,signal)=>{
  let owner=views.get(root);
  if(!owner)for(let at=root;at!==""&&!owner;){const cut=at.lastIndexOf("/");at=cut<0?"":at.slice(0,cut);owner=owners.get(at);}
  if(owner){views.set(root,owner);touch(owner);return viewOf(owner,root);}
  const key="\0files\0"+root;let walking=reads.get(key);
  if(!walking)reads.set(key,walking=Promise.resolve(world.files(root,signal)).then(paths=>{
   const fresh={root,arr:[...paths].sort(),stamp:world.stamp?.(root)};
   owners.set(root,fresh);views.set(root,fresh);listed+=fresh.arr.length;
   for(const[other,entry]of owners){
    if(listed<=P.maximumListedPaths)break;
    if(other!==root){owners.delete(other);listed-=entry.arr.length;for(const[view,held]of views)if(held===entry)views.delete(view);}
   }
   return fresh;
  }));
  const walked=await walking;
  return viewOf(walked,root);
 };
 const refreshListings=async(dirty,signal)=>{
  if(dirty.overflow||dirty.directories?.length){
   if(dirty.overflow&&world.verify&&!dirty.directories?.length){
    for(const[root,owner]of[...owners]){
     const fresh=await world.verify(root,signal);
     if(fresh===undefined||fresh!==owner.stamp){owners.delete(root);listed-=owner.arr.length;for(const[view,held]of views)if(held===owner)views.delete(view);}
    }
    return;
   }
   owners.clear();views.clear();listed=0;
   if(dirty.overflow){
    const bases=[...byRoot.keys()].sort((a,b)=>a.length-b.length),walked=[];
    for(const base of bases)if(!walked.some(top=>top===""||base===top||base.startsWith(top+"/"))){walked.push(base);await listingOf(base,signal);}
   }
   return;
  }
  for(const path of dirty.paths){
   let present;
   for(const owner of owners.values()){
    if(owner.root!==""&&!path.startsWith(owner.root+"/"))continue;
    present??=await fileOf(path,signal)!==ABSENT;
    const at=lower(owner.arr,path),has=owner.arr[at]===path;
    if(present&&!has){owner.arr.splice(at,0,path);listed++;}else if(!present&&has){owner.arr.splice(at,1);listed--;}
   }
  }
 };
 const membersOf=(spec,signal)=>{
  let known=memberCache.get(spec.key);
  if(!known)memberCache.set(spec.key,known=(async()=>{
   const out=[];
   for(const base of spec.bases){const{arr,lo,hi}=await listingOf(base,signal);for(let at=lo;at<hi;at++)if(spec.has(arr[at]))out.push(arr[at]);}
   return spec.bases.length>1?[...new Set(out)].sort():out;
  })());
  return known;
 };
 const stampsOf=bases=>{const out={};if(world.stamp)for(const base of bases){const stamp=world.stamp(base);if(stamp!==undefined)out[base]=stamp;}return out;};
 const stampsHold=ref=>ref.stamps!==undefined&&world.stamp!==undefined&&ref.bases.every(base=>ref.stamps[base]!==undefined&&world.stamp(base)===ref.stamps[base]);
 const rosterNow=async(ref,signal)=>{
  let roster=0n;for(const path of await membersOf(specOf(ref),signal))roster+=rosterHashV1(path);
  return hexOfV1(roster);
 };
 const writes=new Map();
 let timer,writing=Promise.resolve();
 const drainWrites=()=>{
  clearTimeout(timer);timer=undefined;
  writing=writing.then(async()=>{
   while(writes.size){
    const batch=[...writes].slice(0,P.storeParallelWrites);
    for(const[id]of batch)writes.delete(id);
    await Promise.all(batch.map(([id,rec])=>nodes.get(id)===rec?store.put(toPublic(rec),rec.params).catch(()=>{}):undefined));
   }
  });
  return writing;
 };
 const schedule=()=>{if(timer===undefined){timer=setTimeout(drainWrites,P.storeDelayMilliseconds);timer.unref?.();}};
 const SLICE=256,relief={at:0,misses:0,pausedUntil:0};
 const relieve=()=>{
  if(!release)return;
  const now=Date.now(),before=process.memoryUsage.rss();
  if(before<P.releaseRssBytes||now<relief.pausedUntil||now-relief.at<P.releaseMinimumMilliseconds)return;
  release();relief.at=Date.now();
  const gain=(before-process.memoryUsage.rss())/before;
  counters.released++;
  if(gain<P.releaseMinimumGain){if(++relief.misses>=3){relief.pausedUntil=Date.now()+P.releasePauseMilliseconds;relief.misses=0;}}else relief.misses=0;
 };
 async function derive(key,id,params,old,signal,chain){
  const rule=rules[key.kind];if(!rule)throw Error("Unknown derived node kind: "+key.kind);
  signal?.throwIfAborted();
  const deps=new Map(),files=new Map(),globs=new Map(),observed=new Map(),previous=old&&!old.full&&!dirtyNow.all?old:undefined;
  let patch;
  const sameDeps=()=>previous!==undefined&&previous.deps.length===deps.size&&previous.deps.every(dep=>deps.get(dep.id)?.digest===dep.digest);
  const ctx={
   self:key,key:registerKey,params,revision:rev,signal,environment:frozen,
   observed({paths,digest,rows,verify}){const ref={kind:"observed",path:digest,paths:[...paths],digest,rows};observed.set(digest,ref);if(verify)verifiers.set(ref,verify);},
   async file(path){
    await fileOf(path,signal);
    const read=held.get(path)??await gate(async()=>{signal?.throwIfAborted();counters.rawReads++;return world.read(path);});
    files.set(path,{kind:"file",path,digest:read===undefined?ABSENT:sha256V1(read)});return read;
   },
   glob:(root,patterns)=>heavy(async()=>{
    const spec=globSpecV1(root,patterns),paths=[...await membersOf(spec,signal)],key=root+"\0"+patterns.join("\0"),prior=previous?.globs.get(key);
    const known=prior?new Map(prior.paths.map((path,index)=>[path,prior.digests[index]])):undefined,digests=new Array(paths.length);
    let entry=0n,roster=0n;
    for(let at=0;at<paths.length;at+=SLICE){
     const slice=paths.slice(at,at+SLICE),found=await Promise.all(slice.map(path=>{const before=known?.get(path);return before!==undefined&&!isDirtyPath(path)?before:fileOf(path,signal);}));
     found.forEach((digest,index)=>{digests[at+index]=digest;entry+=entryHashV1(slice[index],digest);roster+=rosterHashV1(slice[index]);});
    }
    const ref={kind:"glob",path:root,patterns,bases:spec.bases,digest:hexOfV1(roster),acc:hexOfV1(entry),count:paths.length,paths,digests,stamps:stampsOf(spec.bases)};
    globs.set(key,ref);specs.set(ref,spec);
    return{files:paths.length,digest:ref.acc};
   }),
   async globDelta(root,patterns){
    const key=root+"\0"+patterns.join("\0"),before=previous?.globs.get(key);
    if(!before||!sameDeps()||patch||files.size||globs.size||observed.size||previous.files.size||previous.observed.size||dirtyNow.directories.length)return undefined;
    const spec=specOf(before);let entry=BigInt("0x"+before.acc),roster=BigInt("0x"+before.digest),count=before.count;const edits=[];
    for(const path of dirtyNow.paths){
     if(!spec.has(path))continue;
     const at=locate(before,path),digest=await fileOf(path,signal),is=digest!==ABSENT;
     if(at>=0&&is){if(before.digests[at]!==digest){entry+=entryHashV1(path,digest)-entryHashV1(path,before.digests[at]);edits.push([path,digest]);}}
     else if(at>=0){entry-=entryHashV1(path,before.digests[at]);roster-=rosterHashV1(path);count--;edits.push([path,null]);}
     else if(is){entry+=entryHashV1(path,digest);roster+=rosterHashV1(path);count++;edits.push([path,digest]);}
    }
    const ref={kind:"glob",path:root,patterns,bases:before.bases,digest:hexOfV1(roster),acc:hexOfV1(entry),count,paths:before.paths,digests:before.digests,stamps:{}};
    specs.set(ref,spec);patch={edits,key,ref};
    return{files:count,digest:ref.acc};
   },
   async nodes(list){
    const found=await Promise.all(list.map(item=>resolve(item,signal,chain.concat(id))));
    for(const rec of found)deps.set(rec.id,{id:rec.id,key:rec.key,digest:rec.valueDigest,depth:rec.depth});
    return found;
   },
   async node(item){return(await ctx.nodes([item]))[0].value;},
   async digests(list){return(await ctx.nodes(list)).map(rec=>rec.valueDigest);},
  };
  const value=await rule(ctx),valueDigest=digestOfV1(value);
  counters.computed++;note("computed",id);
  const depList=[...deps.values()],rec={id,key,params,value,valueDigest,deps:depList,files,globs,observed,computedAtRev:rev,valid:true,full:false,depth:1+Math.max(0,...depList.map(dep=>dep.depth))};
  if(patch){
   if(files.size||observed.size)throw Error("A patched glob node reads nothing else: "+id.replaceAll("\0","|"));
   counters.patched++;
   for(const[path,digest]of patch.edits){
    const ref=patch.ref,at=locate(ref,path);
    if(digest===null)at>=0&&(ref.paths.splice(at,1),ref.digests.splice(at,1));
    else if(at>=0)ref.digests[at]=digest;
    else{ref.paths.splice(-at-1,0,path);ref.digests.splice(-at-1,0,digest);}
   }
   rec.files=old.files;rec.observed=old.observed;rec.globs=new Map(old.globs);rec.globs.set(patch.key,patch.ref);
   unlinkDeps(old);
  }else if(old){unlinkDeps(old);unlinkReads(old);}
  if(old){
   if(old.valueDigest===valueDigest)counters.cutoffHit++;
   else{changedLog.push(key);for(const above of rdeps.get(id)??[]){const upper=nodes.get(above);if(upper){upper.valid=false;pending.add(above);}}}
  }
  nodes.delete(id);nodes.set(id,rec);linkDeps(rec);if(!patch)linkReads(rec);pending.delete(id);evict();
  if(store){writes.set(id,rec);schedule();}
  return rec;
 }
 async function verifyRef(ref,signal){
  const verify=verifiers.get(ref)??verifyObserved;
  if(!verify)return false;
  try{return Boolean(await verify(ref,{signal,environment:frozen}));}catch(error){if(isAbort(error))throw error;return false;}
 }
 async function globChanged(ref,signal){
  const spec=specOf(ref);
  if(dirtyNow.directories.some(dir=>ref.bases.some(base=>dir===""||base===dir||base.startsWith(dir+"/")||dir.startsWith(base+"/")))){
   if(await rosterNow(ref,signal)!==ref.digest)return true;
   for(let at=0;at<ref.paths.length;at++)if(isDirtyPath(ref.paths[at])&&await fileOf(ref.paths[at],signal)!==ref.digests[at])return true;
  }
  for(const path of dirtyNow.paths){
   if(!spec.has(path))continue;
   const at=locate(ref,path),digest=await fileOf(path,signal);
   if((at>=0)!==(digest!==ABSENT)||(at>=0&&ref.digests[at]!==digest))return true;
  }
  return false;
 }
 async function holds(rec,signal,chain){
  for(const dep of rec.deps){
   let found;try{found=await resolve(dep.key,signal,chain.concat(rec.id));}catch(error){if(error?.code==="DERIVED_NODE_UNKNOWN")return false;throw error;}
   if(found.valueDigest!==dep.digest)return false;
  }
  if(rec.full||dirtyNow.all){
   const list=[...rec.files.values()];
   for(let at=0;at<list.length;at+=SLICE){
    const checked=await Promise.all(list.slice(at,at+SLICE).map(async ref=>await fileOf(ref.path,signal)===ref.digest));
    if(!checked.every(Boolean))return false;
   }
   relieve();
   for(const ref of rec.observed.values())if(!await verifyRef(ref,signal))return false;
   for(const ref of rec.globs.values()){
    if(stampsHold(ref))counters.stampHeld++;
    else{if(await rosterNow(ref,signal)!==ref.digest)return false;ref.stamps=stampsOf(ref.bases);}
    for(let at=0;at<ref.paths.length;at+=SLICE){
     const checked=await Promise.all(ref.paths.slice(at,at+SLICE).map(async(path,index)=>await fileOf(path,signal)===ref.digests[at+index]));
     if(!checked.every(Boolean))return false;
     if(at%(SLICE*16)===0)relieve();
    }
   }
  }else{
   for(const ref of rec.files.values())if(isDirtyPath(ref.path)&&await fileOf(ref.path,signal)!==ref.digest)return false;
   for(const ref of rec.globs.values())if(await globChanged(ref,signal))return false;
   for(const ref of rec.observed.values())if(ref.paths.some(path=>isDirtyPath(path)||dirtyNow.parents.has(path))&&!await verifyRef(ref,signal))return false;
  }
  rec.full=false;return true;
 }
 async function settle(key,id,signal,chain){
  let rec=nodes.get(id);
  if(!rec&&store){
   const stored=await store.get(id);
   if(stored&&keyIdV1(stored.key)===id){rec=fromPublic(id,stored,stored.params);nodes.set(id,rec);linkDeps(rec);linkReads(rec);}
  }
  const params=registry.get(id)??rec?.params;
  if(rec&&await holds(rec,signal,chain)){rec.valid=true;pending.delete(id);counters.revalidated++;note("revalidated",id);return rec;}
  if(params===undefined)throw unknown(id);
  return derive(key,id,params,nodes.get(id),signal,chain);
 }
 function resolve(key,signal,chain=[]){
  const id=keyIdV1(key),rec=nodes.get(id);
  if(rec?.valid){nodes.delete(id);nodes.set(id,rec);return Promise.resolve(rec);}
  if(chain.includes(id))return Promise.reject(Error("Derived node cycle: "+chain.concat(id).map(item=>item.replaceAll("\0","|")).join(" -> ")));
  let running=inflight.get(id);
  if(!running){running=settle(key,id,signal,chain).finally(()=>inflight.delete(id));inflight.set(id,running);}
  return running.catch(error=>{if(isAbort(error)&&!signal?.aborted)return resolve(key,signal,chain);throw error;});
 }
 async function drain(signal,progress){
  let done=0;
  while(pending.size){
   signal?.throwIfAborted();
   const level=[...pending].map(id=>nodes.get(id)).filter(Boolean);
   if(!level.length){pending.clear();break;}
   const depth=Math.min(...level.map(rec=>rec.depth)),batch=level.filter(rec=>rec.depth===depth);
   for(const rec of batch)pending.delete(rec.id);
   for(let at=0;at<batch.length;at+=P.verifyParallelNodes)await Promise.all(batch.slice(at,at+P.verifyParallelNodes).map(rec=>resolve(rec.key,signal).catch(error=>{if(error?.code==="DERIVED_NODE_UNKNOWN")return undefined;throw error;})));
   done+=batch.length;relieve();await progress?.({phase:"derive",done,total:done+pending.size});
  }
 }
 const around=async work=>{if(operations===0){reads=new Map();held=new Map();memberCache=new Map();retained=0;}operations++;try{return await work();}finally{operations--;}};
 const seedsOf=dirty=>{
  const seeds=new Set();
  if(dirty.overflow){for(const id of nodes.keys())seeds.add(id);return seeds;}
  for(const dir of dirty.directories??[]){
   for(const[path,ids]of byPath)if(dir===""||path===dir||path.startsWith(dir+"/"))for(const id of ids)seeds.add(id);
   for(const[base,ids]of byRoot)if(dir===""||base===dir||base.startsWith(dir+"/")||dir.startsWith(base+"/"))for(const id of ids)seeds.add(id);
  }
  for(const path of dirty.paths){
   for(const id of byPath.get(path)??[])seeds.add(id);
   for(const ancestor of ancestorsOf(path))for(const id of byPath.get(ancestor)??[])seeds.add(id);
   let at=path;for(;;){for(const id of byRoot.get(at)??[])seeds.add(id);if(at==="")break;const cut=at.lastIndexOf("/");at=cut<0?"":at.slice(0,cut);}
  }
  return seeds;
 };
 return Object.freeze({
  key:registerKey,
  get:(key,{signal,progress}={})=>around(async()=>{await resolve(key,signal);await drain(signal,progress);return toPublic(await resolve(key,signal));}),
  getMany:(keys,{signal,progress,slice=256}={})=>around(async()=>{
   const out=[];
   for(let at=0;at<keys.length;at+=slice){
    signal?.throwIfAborted();
    await Promise.all(keys.slice(at,at+slice).map(key=>resolve(key,signal)));
    await drain(signal);relieve();await progress?.({phase:"derive",done:Math.min(keys.length,at+slice),total:keys.length});
   }
   for(const key of keys)out.push(toPublic(await resolve(key,signal)));
   return out;
  }),
  put:(node,params)=>{const id=keyIdV1(node.key),rec=fromPublic(id,node,params),old=nodes.get(id);if(old){unlinkDeps(old);unlinkReads(old);}nodes.set(id,rec);linkDeps(rec);linkReads(rec);if(params!==undefined)registry.set(id,params);evict();},
  invalidate:(dirty,{signal,progress}={})=>around(async()=>{
   rev=dirty.revision??rev;
   await refreshListings(dirty,signal);
   const before={computed:counters.computed,cutoffHit:counters.cutoffHit,revalidated:counters.revalidated};
   changedLog=[];const from=log.length;
   for(const id of seedsOf(dirty)){const rec=nodes.get(id);if(rec){rec.valid=false;pending.add(id);}}
   dirtyNow.all=Boolean(dirty.overflow);dirtyNow.paths=new Set(dirty.paths);dirtyNow.directories=[...(dirty.directories??[])];dirtyNow.parents=new Set(dirty.paths.flatMap(ancestorsOf));
   try{await drain(signal,progress);}finally{dirtyNow={all:false,paths:new Set(),directories:[],parents:new Set()};}
   return{changed:changedLog.slice(),computed:counters.computed-before.computed,cutoffHit:counters.cutoffHit-before.cutoffHit,revalidated:counters.revalidated-before.revalidated,trace:log.slice(from)};
  }),
  /** 🫧️ Drops every settled node from memory once the durable store holds them (answers the number of nodes left, or undefined while no store can answer them again): the store answers them lazily, so a settled graph costs no heap between plans. */
  async release(){if(!store)return undefined;await drainWrites();await store.flush?.();for(const[id,rec]of[...nodes]){if(inflight.has(id))continue;unlinkDeps(rec);unlinkReads(rec);nodes.delete(id);pending.delete(id);rdeps.delete(id);counters.evicted++;}if(!inflight.size){reads.clear();byPath.clear();byRoot.clear();}return nodes.size;},
  stats:()=>({nodes:nodes.size,registry:registry.size,rdeps:rdeps.size,pending:pending.size,...counters}),
  drainTrace:()=>log.splice(0),
  async flush(){if(store){await drainWrites();await store.flush?.();}await statIndex?.flush();},
 });
}
