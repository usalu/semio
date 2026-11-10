/** 🧊 Reference-counted, size-bounded pool of fact indexes keyed by their content digest: every distinct index is resident once however many workspaces use it, and unreferenced ones leave least recently used first. */

/** 🧊 Creates the pool; `acquire` single-flights `load` per key and returns a handle whose `release` is idempotent. */
export function createIndexPoolV1({maximumBytes,maximumEntries,now=()=>Date.now(),onEvict=()=>{}}){
 const entries=new Map();
 let total=0,evictions=0;
 const trim=()=>{
  while(total>maximumBytes||entries.size>maximumEntries){
   let victim;
   for(const entry of entries.values())if(entry.refs===0&&entry.ready&&(victim===undefined||entry.usedAt<victim.usedAt))victim=entry;
   if(!victim)return;
   entries.delete(victim.key);total-=victim.bytes;evictions++;
   try{onEvict(victim.key,victim.value);}catch{}
  }
 };
 const handle=entry=>{
  let released=false;
  return{key:entry.key,value:entry.value,bytes:entry.bytes,release(){if(released)return;released=true;entry.refs--;entry.usedAt=now();trim();}};
 };
 return{
  async acquire(key,load,signal){
   signal?.throwIfAborted();
   let entry=entries.get(key),hit=true;
   if(!entry){
    hit=false;
    const controller=new AbortController();
    entry={key,refs:0,waiters:0,controller,usedAt:now(),ready:false,bytes:0,value:undefined,loading:undefined};
    entries.set(key,entry);
    const mine=entry;
    mine.loading=Promise.resolve().then(()=>load(controller.signal)).then(loaded=>{if(controller.signal.aborted)throw controller.signal.reason??new Error("aborted");mine.value=loaded.value;mine.bytes=loaded.bytes;mine.ready=true;total+=loaded.bytes;},error=>{if(entries.get(key)===mine)entries.delete(key);throw error;});
    mine.loading.catch(()=>{});
   }
   const mine=entry;
   mine.refs++;mine.waiters++;mine.usedAt=now();
   const leave=()=>{mine.waiters--;};
   let onAbort;
   try{
    await new Promise((resolve,reject)=>{
     if(signal){onAbort=()=>reject(signal.reason??new Error("aborted"));signal.addEventListener("abort",onAbort,{once:true});}
     mine.loading.then(resolve,reject);
    });
   }catch(error){
    mine.refs--;leave();
    if(!mine.ready&&mine.waiters===0){mine.controller.abort(error);if(entries.get(key)===mine)entries.delete(key);}
    throw error;
   }finally{if(onAbort)signal.removeEventListener("abort",onAbort);}
   leave();trim();
   return{...handle(mine),hit};
  },
  size(key){return entries.get(key)?.bytes??0;},
 stats(){return{indexes:entries.size,bytes:total,evictions,referenced:[...entries.values()].filter(entry=>entry.refs>0).length};},
  keys:()=>[...entries.keys()],
  clear(){for(const entry of[...entries.values()])if(entry.refs===0&&entry.ready){entries.delete(entry.key);total-=entry.bytes;}},
 };
}
