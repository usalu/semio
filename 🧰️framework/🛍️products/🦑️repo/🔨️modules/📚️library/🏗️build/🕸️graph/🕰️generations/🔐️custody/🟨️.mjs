import{randomBytes}from"node:crypto";
import{join}from"node:path";
import{canonicalJsonV1,compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import{waitAnyLeaseV1}from"../🧩️claims/🟨️.mjs";
import{advanceProgressV1,askReasonV1,idleMillisecondsV1,isDeposedV1,holderSuffixV1,readAskV1,readHolderV1,readProgressV1,rememberDoorV1}from"../🪧️door-board/🟨️.mjs";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 🔐️ The declared constants of the slice custody; the schema file is the single source and a law keeps both equal. */
export const CUSTODY_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));
/** 🗨️ The bilingual texts of the slice custody; the schema file declares the same table. */
export const CUSTODY_MESSAGES_V1=Object.freeze(structuredClone(schema.definitions.Messages.const));
const P=CUSTODY_POLICY_V1,TOKEN=/^[a-f0-9]{24}$/u,STATES=["waiting","held"],fail=(message,code)=>Object.assign(Error(message),code?{code}:{});
const aborted=signal=>{if(signal?.aborted)throw signal.reason??fail("Aborted","ABORT_ERR");};
const ownersOf=names=>[...new Set(names)].sort(compareCodePointsV1);

/** 📢️ Renders one custody message in the language the caller resolved with the shared resolver. */
export function renderCustodyMessageV1(key,fields,language){
 const template=CUSTODY_MESSAGES_V1[language]?.[key];
 if(template===undefined)throw fail("Closed custody message required: "+key);
 return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]??""));
}

/** 🛂️ Admits a closed custody claim: its ticket, the owners it writes in strict code point order, whether it is structural and whether it holds the custody. */
export function readCustodyClaimV1(value){
 const required=schema.definitions.Claim.required,allowed=Object.keys(schema.definitions.Claim.properties);
 if(value===null||typeof value!=="object"||Array.isArray(value))throw fail("Closed custody claim required");
 for(const key of Object.keys(value))if(!allowed.includes(key))throw fail("Closed custody claim required: unknown "+key);
 for(const key of required)if(!Object.hasOwn(value,key))throw fail("Closed custody claim required: missing "+key);

 if(value.holder!==undefined)try{readHolderV1(value.holder);}catch{throw fail("Closed custody holder required");}
 if(value.version!==1||typeof value.token!=="string"||!TOKEN.test(value.token))throw fail("Custody token of 24 hex digits required");
 if(!Number.isSafeInteger(value.ticket)||value.ticket<1)throw fail("Positive custody ticket required");
 if(!Number.isSafeInteger(value.queuedMs)||value.queuedMs<0)throw fail("Epoch milliseconds required in a custody claim");
 if(typeof value.structural!=="boolean"||!STATES.includes(value.state))throw fail("Closed custody state required");
 if(!Array.isArray(value.owners)||value.owners.length>P.maximumOwners)throw fail("Bounded owner list required in a custody claim");
 value.owners.forEach((name,at)=>{if(typeof name!=="string"||name===""||at>0&&compareCodePointsV1(value.owners[at-1],name)>=0)throw fail("Owners in strict code point order required in a custody claim");});
 return Object.freeze({...value,owners:Object.freeze([...value.owners])});
}

/** ⚔️ Whether two claims may not build at the same time: either is structural or their sorted owner lists share a name. */
export function claimsConflictV1(a,b){
 if(a.structural||b.structural)return true;
 for(let i=0,j=0;i<a.owners.length&&j<b.owners.length;){const order=compareCodePointsV1(a.owners[i],b.owners[j]);if(order===0)return true;if(order<0)i++;else j++;}
 return false;
}

/** 🚦️ Decides one admission: `mine` may hold the custody when no other live claim that is held or carries a smaller ticket conflicts with it; `blockers` are those claims in ticket order. */
export function admissionOfV1(mine,others){
 const blockers=others.filter(claim=>claim.token!==mine.token&&(claim.state==="held"||claim.ticket<mine.ticket)&&claimsConflictV1(mine,claim)).sort((a,b)=>a.ticket-b.ticket);
 return{admit:blockers.length===0,blockers};
}

/**
 * 🏛️ Creates the slice custody of the generation documents under `root`. `fs` is the generation file port, `lease` the lease port `{tryHold(resource), hold(resource, {signal}), wait(resource, {signal})}` of the claim registry. `hold` queues one claim and resolves once it holds the custody; the handle extends the claim, makes it structural when it is alone, and gives it back.
 */
export function createSliceCustodyV1({root,fs,lease,now=()=>Date.now(),token=()=>randomBytes(12).toString("hex")}){
 if(typeof root!=="string"||!root||!fs||!lease)throw fail("Custody root, file port and lease port required");
 const directory=join(root,P.directory),pathOf=name=>join(directory,name+".json"),sidePath=(name,kind)=>join(directory,name+"."+kind+".json"),own=new Map(),SIDE=/^([a-f0-9]{24})\.(progress|ask)\.json$/u;
 const resource={claim:name=>"graph-custody:"+root+":"+name,admission:"graph-custody-admission:"+root};
 const write=async claim=>{await fs.write(pathOf(claim.token),Buffer.from(canonicalJsonV1(readCustodyClaimV1(claim))));own.set(claim.token,claim);return claim;};
 const decodeSide=(bytes,read)=>{if(!bytes)return undefined;try{return read(JSON.parse(Buffer.from(bytes).toString("utf8")));}catch{return undefined;}};
 const readSide=async(name,kind,read)=>decodeSide(await fs.read(sidePath(name,kind)),read);
 const writeSide=(name,kind,value,options)=>fs.write(sidePath(name,kind),Buffer.from(canonicalJsonV1(value)),options);
 const removeSides=name=>Promise.all(["progress","ask"].map(kind=>fs.remove(sidePath(name,kind)).catch(()=>{})));
 const readOne=async name=>{const bytes=await fs.read(pathOf(name));if(!bytes)return undefined;try{return readCustodyClaimV1(JSON.parse(Buffer.from(bytes).toString("utf8")));}catch{return undefined;}};

 /** 💓️ The live claims: a claim of this process is live while it is not released, a claim of another command while its lease is held; the document of a dead claim and an unreadable document older than ten polls are removed. */
 async function live(signal){
  const rows=[],files=[...await fs.list(directory)],sides=new Map(),named=new Set();
  for(const file of files){const found=SIDE.exec(file.name);if(found)sides.set(file.name,found[1]);else if(/^[a-f0-9]{24}\.json$/u.test(file.name))named.add(file.name.slice(0,24));}
  for(const file of files){
   aborted(signal);
   if(!/^[a-f0-9]{24}\.json$/u.test(file.name))continue;
   if(rows.length>=P.maximumClaims)throw fail("Bounded number of custody claims required","CUSTODY_FULL");
   const name=file.name.slice(0,24);
   const attach=async claim=>{const extra={};if(sides.has(name+".progress.json"))extra.progress=await readSide(name,"progress",readProgressV1);if(sides.has(name+".ask.json"))extra.ask=await readSide(name,"ask",readAskV1);return{...claim,...extra};};
   if(own.has(name)){rows.push(await attach(own.get(name)));continue;}
   const claim=await readOne(name);
   if(!claim){if(now()-file.modifiedMs>10*P.pollMilliseconds){await fs.remove(join(directory,file.name));await removeSides(name);}continue;}
   const probe=await lease.tryHold(resource.claim(name));
   if(!probe){rows.push(await attach(claim));continue;}
   try{await fs.remove(pathOf(name));await removeSides(name);}finally{probe.release();}
  }
  for(const[sideName,owner]of sides)if(!named.has(owner)&&now()-(files.find(file=>file.name===sideName)?.modifiedMs??now())>10*P.pollMilliseconds)await fs.remove(join(directory,sideName)).catch(()=>{});
  return rows;
 }

 const active=async signal=>{const at=now();return(await live(signal)).filter(row=>!isDeposedV1(row,at));};

 const admitted=async(signal,body)=>{const admission=await lease.hold(resource.admission,{signal});try{return await body();}finally{admission.release();}};

 /** 🛡️ Verifies that the named blocker is still held and conflicts with the waiter. */
 async function heldBlocker(name,waiter){
  const held=await readOne(name);
  if(held?.state!=="held"||!claimsConflictV1(waiter,held))return undefined;
  if(own.has(name))return held;
  const probe=await lease.tryHold(resource.claim(name));
  if(!probe)return held;
  probe.release();
 }

 /** 🎫️ The handle of a held claim: `owners`/`structural` say what it may write, `extend` widens it at once when no other held claim conflicts (answering the request to queue again otherwise), `alone` makes it structural when no other live claim exists, `release` gives it back. */
 function handle(name,lock,settings){
  const current=()=>{const claim=own.get(name);if(!claim)throw fail("Released custody claim","CUSTODY_RELEASED");return claim;};
  let side={phase:"start",done:0,total:0,advancedMs:now()},wrote=side.advancedMs;
  const digestNow=()=>settings.digest();
  const handoverError=(reason,{done=side.done,total=side.total}={})=>{
   const fields={seconds:Math.round((idleMillisecondsV1({progress:side},now())??0)/1000),done,total,digest:(own.get(name)?.holder?.digest??"").slice(0,12),current:(reason.current??"").slice(0,12)||"?"},key=reason.reason==="stale-code"?"handoverStale":"handoverStalled";
   return Object.assign(Error(renderCustodyMessageV1(key,fields,"en")+"\n"+renderCustodyMessageV1(key,fields,"de")),{code:"SEMIO_STALE_IMPLEMENTATION",handover:{reason:reason.reason,done,total}});
  };
  return Object.freeze({
   token:name,
   get owners(){return current().owners;},
   get structural(){return current().structural;},
   async extend({owners=[],structural=false,signal}={}){
    const held=current(),wanted=ownersOf([...held.owners,...owners]),wide=held.structural||structural===true,request={dirty:wanted,structural:wide},ask=await readSide(name,"ask",readAskV1);
    if(ask&&isDeposedV1({ask},now()))return{held:false,request};
    if(held.structural||!wide&&wanted.length===held.owners.length)return{held:true,request};
    const next=readCustodyClaimV1({...held,owners:wanted,structural:wide});
    return admitted(signal,async()=>{
     if((await active(signal)).some(claim=>claim.token!==name&&claim.state==="held"&&claimsConflictV1(next,claim)))return{held:false,request};
     await write(next);return{held:true,request};
    });
   },
   async alone({signal}={}){
    const held=current();if(held.structural)return true;
    return admitted(signal,async()=>{if((await active(signal)).some(claim=>claim.token!==name))return false;await write({...held,structural:true});return true;});
   },
   async settle(finished){
    const held=current();if(held.structural)return false;
    const done=new Set(finished),rest=held.owners.filter(owner=>!done.has(owner));
    if(rest.length===held.owners.length)return true;
    await write(readCustodyClaimV1({...held,owners:rest}));return true;
   },
   async progress(event){
    if(!own.has(name))return;
    const at=now(),next=advanceProgressV1(side,event,at),phaseChanged=next.phase!==side.phase,moved=phaseChanged||next.done>side.done;
    side=next;
    if(!moved||!phaseChanged&&at-wrote<P.progressWriteMilliseconds)return;
    wrote=at;await writeSide(name,"progress",side).catch(()=>{});
   },
   async handover({requestedOnly=false}={}){
    const ask=await readSide(name,"ask",readAskV1);
    if(ask)return{reason:ask.reason,ask};
    if(requestedOnly)return undefined;
    const mine=own.get(name)?.holder?.digest,found=await digestNow();
    if(!mine||!found||mine===found)return undefined;
    const claim=current();
    return(await live()).some(row=>row.token!==name&&row.state==="waiting"&&claimsConflictV1(claim,row))?{reason:"stale-code",current:found}:undefined;
   },
   handoverError,
   async fence(body,{signal}={}){
    return admitted(signal,async()=>{
     current();
     const ask=await readSide(name,"ask",readAskV1);
     if(isDeposedV1({ask},now()))throw handoverError(ask,{done:side.done,total:side.total});
     return body();
    });
   },
   async release(){if(!own.has(name))return;own.delete(name);try{await fs.remove(pathOf(name));await removeSides(name);}finally{lock.release();}},
  });
 }

 /** 🚪️ Queues a claim on `owners` (every owner when `structural`) and resolves with its handle once no held or earlier live claim conflicts; `say` tells at once and then every notice interval which claims go first. */
 async function hold({owners=[],structural=false,signal,say=()=>{},language="en",holder,currentDigest,peerWait}={}){
  const name=token(),lock=await lease.tryHold(resource.claim(name),{fresh:true});
  if(!lock)throw fail("A fresh custody lease is free","CUSTODY_LEASE_TAKEN");
  rememberDoorV1(directory);
  const mine={owners:ownersOf(owners),structural:structural===true,...holder?{holder:readHolderV1(holder)}:{}},started=now(),told=new Set();let claim,noticed=-Infinity,seen,seenAt=-Infinity;
  const digestNow=async()=>{if(!currentDigest)return undefined;if(now()-seenAt<P.noticeMilliseconds&&seen!==undefined)return seen;try{seen=await currentDigest();}catch{}seenAt=now();return seen;};
  const asking=async blockers=>{
   const current=await digestNow(),seenAt=now(),candidates=blockers.filter(row=>askReasonV1({claim:row,now:seenAt,current}));
   aborted(signal);
   if(!candidates.length)return;
   const written=await admitted(signal,async()=>{
    const asked=[];
    for(const {token} of candidates){
     aborted(signal);
     const held=await heldBlocker(token,claim);
     if(!held)continue;
     const path=sidePath(token,"ask"),[progress,bytes]=await Promise.all([readSide(token,"progress",readProgressV1),fs.read(path)]);
     const row={...held,progress,ask:decodeSide(bytes,readAskV1)},at=now(),reason=askReasonV1({claim:row,now:at,current});
     if(!reason)continue;
     aborted(signal);
     if(bytes)await fs.remove(path).catch(()=>{});
     aborted(signal);
     const saved=await writeSide(token,"ask",{version:1,reason,atMs:at,by:{pid:process.pid,digest:mine.holder?.digest??""}},{ifAbsent:true}).catch(()=>false);
     if(!saved)continue;
     if(!await heldBlocker(token,claim)){await fs.remove(path).catch(()=>{});continue;}
     asked.push({row,reason,at});
    }
    return asked;
   });
   for(const {row,reason,at} of written){
    if(reason==="stalled")say("askedStalled",{pid:row.holder?.pid??0,seconds:Math.round((idleMillisecondsV1(row,at)??0)/1000)});else say("askedStale",{pid:row.holder?.pid??0,digest:(row.holder?.digest??"").slice(0,12),current:(current??"").slice(0,12)});
   }
  };
  try{
   for(;;){
    aborted(signal);
    const decision=await admitted(signal,async()=>{
     const at=now(),all=(await live(signal)).filter(row=>row.token!==name),others=all.filter(row=>!isDeposedV1(row,at));
     for(const row of all)if(isDeposedV1(row,at)&&!told.has(row.token)){told.add(row.token);say("deposed",{pid:row.holder?.pid??0,seconds:Math.round((at-row.ask.atMs)/1000)});}
     claim??={version:1,token:name,ticket:others.reduce((top,row)=>Math.max(top,row.ticket),0)+1,...mine,state:"waiting",queuedMs:now()};
     const found=admissionOfV1(claim,others);
     claim=await write({...claim,state:found.admit?"held":"waiting"});
     return found;
    });
    peerWait?.(!decision.admit);
    if(decision.admit){await writeSide(name,"progress",{phase:"start",done:0,total:0,advancedMs:now()}).catch(()=>{});return handle(name,lock,{digest:digestNow});}
    await asking(decision.blockers);
    if(now()-noticed>=P.noticeMilliseconds){
     noticed=now();const first=decision.blockers[0],sample=ownersOf(first.owners.filter(owner=>claim.owners.includes(owner))).slice(0,P.sampleOwners);
     say(claim.structural||first.structural||!sample.length?"blockedAll":"blocked",{peers:decision.blockers.length,seconds:Math.round((now()-started)/1000),sample:sample.join(", "),holder:holderSuffixV1(first,now(),language)});
    }
    await waitAnyLeaseV1(lease,decision.blockers.map(row=>resource.claim(row.token)),{signal,timeoutMs:P.pollMilliseconds});
   }
  }catch(error){own.delete(name);await fs.remove(pathOf(name)).catch(()=>{});await removeSides(name);lock.release();throw error;}
 }

 /** 💤️ Whether a claim on `owners` would hold the custody at once: no live claim conflicts with it. */
 async function idle({owners=[],structural=false,signal}={}){
  const mine={owners:ownersOf(owners),structural:structural===true};
  return!(await active(signal)).some(claim=>claimsConflictV1(mine,claim));
 }

 return Object.freeze({hold,idle,list:live,resource});
}

const pause=(milliseconds,signal)=>new Promise((resolve,reject)=>{
 if(signal?.aborted){reject(signal.reason??fail("Aborted","ABORT_ERR"));return;}
 const onAbort=()=>{clearTimeout(timer);reject(signal.reason??fail("Aborted","ABORT_ERR"));},timer=setTimeout(()=>{signal?.removeEventListener("abort",onAbort);resolve();},milliseconds);
 signal?.addEventListener("abort",onAbort,{once:true});
});

/**
 * 🧲️ Attaches a reader to the live builder of its stale projects: the second reader of a machine neither builds nor prepares the projects beside the builder, it looks every `attachPollMilliseconds` (never more often than `attachSpendRatio` times the duration of the last look, at most every `attachMaximumPollMilliseconds`: a look opens the store and is slow on a busy host, so the waiting reader cannot keep the machine busy) (`look()` answers whether a live claim holds the custody against the reader and the newest generation), tells at once and then every notice interval that it is attached, tells each wave the builder publishes and at most every `attachRecheckMilliseconds` lets `recheck()` serve it from the generation published so far (any answer but undefined ends the attachment and is returned). It answers undefined as soon as nothing holds the custody against the reader any more, and when the custody cannot be looked at, so the caller builds or prepares as before: an attachment never fails a command, a cancellation is rethrown.
 */
export async function attachToBuilderV1({look,recheck,say=()=>{},signal,now=()=>Date.now(),sleep=pause,policy=P}){
 const started=now();let previous,waves=0,noticed=-Infinity,checked=started;
 for(;;){
  aborted(signal);
  let seen;
  const lookBegan=now();
  try{seen=await look();}catch(error){if(signal?.aborted)throw error;return undefined;}
  const took=now()-lookBegan;
  if(!seen.held)return undefined;
  const seconds=Math.round((now()-started)/1000);
  if(seen.generation!==undefined&&previous!==undefined&&seen.generation>previous){
   waves+=seen.generation-previous;noticed=now();say("attachedWave",{generation:seen.generation,waves,seconds});
   if(now()-checked>=policy.attachRecheckMilliseconds){
    checked=now();let served;
    try{served=await recheck();}catch(error){if(signal?.aborted)throw error;}
    if(served!==undefined){say("attachedServed",{generation:seen.generation,seconds});return served;}
   }
  }else if(now()-noticed>=policy.noticeMilliseconds){noticed=now();say("attached",{generation:seen.generation??-1,seconds});}
  if(seen.generation!==undefined)previous=seen.generation;
  await sleep(Math.min(policy.attachMaximumPollMilliseconds,Math.max(policy.attachPollMilliseconds,policy.attachSpendRatio*took)),signal);
 }
}
