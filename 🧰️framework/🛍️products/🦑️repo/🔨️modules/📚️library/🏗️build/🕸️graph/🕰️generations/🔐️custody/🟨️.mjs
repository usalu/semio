import{randomBytes}from"node:crypto";
import{join}from"node:path";
import{canonicalJsonV1,compareCodePointsV1}from"../🧮️fact-codec/🟨️.mjs";
import{waitAnyLeaseV1}from"../🧩️claims/🟨️.mjs";
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
 return template.replace(/\{([a-z]+)\}/gu,(_token,name)=>String(fields[name]));
}

/** 🛂️ Admits a closed custody claim: its ticket, the owners it writes in strict code point order, whether it is structural and whether it holds the custody. */
export function readCustodyClaimV1(value){
 const required=schema.definitions.Claim.required;
 if(value===null||typeof value!=="object"||Array.isArray(value))throw fail("Closed custody claim required");
 for(const key of Object.keys(value))if(!required.includes(key))throw fail("Closed custody claim required: unknown "+key);
 for(const key of required)if(!Object.hasOwn(value,key))throw fail("Closed custody claim required: missing "+key);
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
 const directory=join(root,P.directory),pathOf=name=>join(directory,name+".json"),own=new Map();
 const resource={claim:name=>"graph-custody:"+root+":"+name,admission:"graph-custody-admission:"+root};
 const write=async claim=>{await fs.write(pathOf(claim.token),Buffer.from(canonicalJsonV1(readCustodyClaimV1(claim))));own.set(claim.token,claim);return claim;};
 const readOne=async name=>{const bytes=await fs.read(pathOf(name));if(!bytes)return undefined;try{return readCustodyClaimV1(JSON.parse(Buffer.from(bytes).toString("utf8")));}catch{return undefined;}};

 /** 💓️ The live claims: a claim of this process is live while it is not released, a claim of another command while its lease is held; the document of a dead claim and an unreadable document older than ten polls are removed. */
 async function live(signal){
  const rows=[];
  for(const file of await fs.list(directory)){
   aborted(signal);
   if(!/^[a-f0-9]{24}\.json$/u.test(file.name))continue;
   if(rows.length>=P.maximumClaims)throw fail("Bounded number of custody claims required","CUSTODY_FULL");
   const name=file.name.slice(0,24);
   if(own.has(name)){rows.push(own.get(name));continue;}
   const claim=await readOne(name);
   if(!claim){if(now()-file.modifiedMs>10*P.pollMilliseconds)await fs.remove(join(directory,file.name));continue;}
   const probe=await lease.tryHold(resource.claim(name));
   if(!probe){rows.push(claim);continue;}
   try{await fs.remove(pathOf(name));}finally{probe.release();}
  }
  return rows;
 }

 const admitted=async(signal,body)=>{const admission=await lease.hold(resource.admission,{signal});try{return await body();}finally{admission.release();}};

 /** 🎫️ The handle of a held claim: `owners`/`structural` say what it may write, `extend` widens it at once when no other held claim conflicts (answering the request to queue again otherwise), `alone` makes it structural when no other live claim exists, `release` gives it back. */
 function handle(name,lock){
  const current=()=>{const claim=own.get(name);if(!claim)throw fail("Released custody claim","CUSTODY_RELEASED");return claim;};
  return Object.freeze({
   token:name,
   get owners(){return current().owners;},
   get structural(){return current().structural;},
   async extend({owners=[],structural=false,signal}={}){
    const held=current(),wanted=ownersOf([...held.owners,...owners]),wide=held.structural||structural===true,request={dirty:wanted,structural:wide};
    if(held.structural||!wide&&wanted.length===held.owners.length)return{held:true,request};
    const next=readCustodyClaimV1({...held,owners:wanted,structural:wide});
    return admitted(signal,async()=>{
     if((await live(signal)).some(claim=>claim.token!==name&&claim.state==="held"&&claimsConflictV1(next,claim)))return{held:false,request};
     await write(next);return{held:true,request};
    });
   },
   async alone({signal}={}){
    const held=current();if(held.structural)return true;
    return admitted(signal,async()=>{if((await live(signal)).some(claim=>claim.token!==name))return false;await write({...held,structural:true});return true;});
   },
   async release(){if(!own.has(name))return;own.delete(name);try{await fs.remove(pathOf(name));}finally{lock.release();}},
  });
 }

 /** 🚪️ Queues a claim on `owners` (every owner when `structural`) and resolves with its handle once no held or earlier live claim conflicts; `say` tells at once and then every notice interval which claims go first. */
 async function hold({owners=[],structural=false,signal,say=()=>{}}={}){
  const name=token(),lock=await lease.tryHold(resource.claim(name),{fresh:true});
  if(!lock)throw fail("A fresh custody lease is free","CUSTODY_LEASE_TAKEN");
  const mine={owners:ownersOf(owners),structural:structural===true},started=now();let claim,noticed=-Infinity;
  try{
   for(;;){
    aborted(signal);
    const decision=await admitted(signal,async()=>{
     const others=(await live(signal)).filter(row=>row.token!==name);
     claim??={version:1,token:name,ticket:others.reduce((top,row)=>Math.max(top,row.ticket),0)+1,...mine,state:"waiting",queuedMs:now()};
     const found=admissionOfV1(claim,others);
     claim=await write({...claim,state:found.admit?"held":"waiting"});
     return found;
    });
    if(decision.admit)return handle(name,lock);
    if(now()-noticed>=P.noticeMilliseconds){
     noticed=now();const first=decision.blockers[0],sample=ownersOf(first.owners.filter(owner=>claim.owners.includes(owner))).slice(0,P.sampleOwners);
     say(claim.structural||first.structural||!sample.length?"blockedAll":"blocked",{peers:decision.blockers.length,seconds:Math.round((now()-started)/1000),sample:sample.join(", ")});
    }
    await waitAnyLeaseV1(lease,decision.blockers.map(row=>resource.claim(row.token)),{signal,timeoutMs:P.pollMilliseconds});
   }
  }catch(error){own.delete(name);await fs.remove(pathOf(name)).catch(()=>{});lock.release();throw error;}
 }

 /** 💤️ Whether a claim on `owners` would hold the custody at once: no live claim conflicts with it. */
 async function idle({owners=[],structural=false,signal}={}){
  const mine={owners:ownersOf(owners),structural:structural===true};
  return!(await live(signal)).some(claim=>claimsConflictV1(mine,claim));
 }

 return Object.freeze({hold,idle,list:live,resource});
}
