import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.$defs.Policy.const,template=schema.$defs.Texts.const.revalidate,fill=(text,values)=>text.replace(/\{(\w+)\}/gu,(_,name)=>String(values[name]));

/** 🌩️ Whether the reason a withholding recorded names a transient cause (a resource, capacity or memory limit, a timeout, a cancellation, a source that changed during the read; English or German, fragments declared in the policy), so that the failure may not repeat.
 * @param {unknown} reason */
export function transientWithheldReasonV1(reason){
 if(typeof reason!=="string"||reason==="")return false;
 const text=reason.toLowerCase();
 return policy.transientReasons.some(fragment=>text.includes(fragment));
}

/** 🔁️ Decides whether a read infers a withheld owner once more: the first sighting of a withholding only starts its cool-down; after the declared cool-down it is inferred again, at most the declared attempts, then it stays until one of its rows changes (the owner is then no longer withheld and the generation drops it). `fragment` is the digest of the withholding, so a different one is a new withholding. An exhausted withholding is nevertheless inferred again at once when the inference `implementation` digest differs from the one that counted its attempts (the failure may be fixed), and after every cool-down while its recorded reason is `transient`.
 * @see ../🧪️tests/🐍️oracle/🐍️.py
 * @param {{entry?:{fragment:string,attempts:number,at:number,implementation?:string},fragment:string,now:number,implementation?:string,transient?:boolean}} input */
export function withheldRevalidationV1({entry,fragment,now,implementation,transient=false}){
 const stamp=implementation===undefined?{}:{implementation};
 if(entry===undefined||entry.fragment!==fragment)return{revalidate:false,entry:{fragment,attempts:0,at:now,...stamp}};
 if(implementation!==undefined&&entry.implementation!==implementation)return{revalidate:true,entry:{fragment,attempts:1,at:now,...stamp}};
 const exhausted=entry.attempts>=policy.revalidateAttempts,attempts=exhausted&&transient?0:entry.attempts;
 if(attempts>=policy.revalidateAttempts||now-entry.at<policy.revalidateCooldownMilliseconds)return{revalidate:false,entry:attempts===entry.attempts?entry:{...entry,attempts}};
 return{revalidate:true,entry:{fragment,attempts:attempts+1,at:now,...stamp}};
}

/** 🗣️ The bilingual line saying a withheld owner is inferred again after its cool-down. */
export function withheldRevalidationTextV1(owner,entry,previousAt){
 const values={owner,minutes:Math.max(0,Math.round((entry.at-previousAt)/60000)),attempt:entry.attempts,attempts:policy.revalidateAttempts};
 return{en:fill(template.en,values),de:fill(template.de,values)};
}

/** 🔁️ The withheld owners of a generation (untagged `unavailable` rows that name their declaration) that this read infers again, judged by {@link withheldRevalidationV1} against a small ledger kept beside the generations (`ledger.read()` answers the object of the last read or undefined, `ledger.write(object)` stores it); owners the read does not concern (`include`) are not judged. A ledger that cannot be read or written never fails a read: nothing is inferred again then. `say` receives the bilingual line of each re-validation; `implementation` is the digest of the inference implementation, `reasonOf(name,row)` answers the recorded reason of an exhausted withholding (a failure of it never fails the read: the reason is then not transient).
 * @param {Record<string,{kind:string,tag?:string,config?:string,fragment:string}>} owners */
export async function revalidatedWithheldOwnersV1(owners,{ledger,now,include=()=>true,say,implementation,reasonOf}){
 let book;try{book=await ledger.read();}catch{return[];}
 book=book!==null&&typeof book==="object"?book:{};
 const next={},names=[],at=now();
 for(const[name,row]of Object.entries(owners)){
  if(row.kind!=="unavailable"||row.tag||typeof row.config!=="string"||!include(name))continue;
  const entry=book[name];let transient=false;
  if(reasonOf!==undefined&&entry?.fragment===row.fragment&&entry.attempts>=policy.revalidateAttempts)try{transient=transientWithheldReasonV1(await reasonOf(name,row));}catch{}
  const decision=withheldRevalidationV1({entry,fragment:row.fragment,now:at,implementation,transient});
  next[name]=decision.entry;
  if(decision.revalidate){names.push(name);try{say?.(withheldRevalidationTextV1(name,decision.entry,book[name].at));}catch{}}
 }
 const kept=Object.fromEntries(Object.entries(book).filter(([name])=>!(name in next)&&owners[name]?.kind==="unavailable"));
 const merged={...kept,...next};
 if(JSON.stringify(merged)!==JSON.stringify(book))try{await ledger.write(merged);}catch{return[];}
 return names;
}
