import schema from "../🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.$defs.Policy.const,template=schema.$defs.Texts.const.revalidate,fill=(text,values)=>text.replace(/\{(\w+)\}/gu,(_,name)=>String(values[name]));

/** 🔁️ Decides whether a read infers a withheld owner once more: the first sighting of a withholding only starts its cool-down; after the declared cool-down it is inferred again, at most the declared attempts, then it stays until one of its rows changes (the owner is then no longer withheld and the generation drops it). `fragment` is the digest of the withholding, so a different one is a new withholding.
 * @see ../🧪️tests/🐍️oracle/🐍️.py
 * @param {{entry?:{fragment:string,attempts:number,at:number},fragment:string,now:number}} input */
export function withheldRevalidationV1({entry,fragment,now}){
 if(entry===undefined||entry.fragment!==fragment)return{revalidate:false,entry:{fragment,attempts:0,at:now}};
 if(entry.attempts>=policy.revalidateAttempts||now-entry.at<policy.revalidateCooldownMilliseconds)return{revalidate:false,entry};
 return{revalidate:true,entry:{fragment,attempts:entry.attempts+1,at:now}};
}

/** 🗣️ The bilingual line saying a withheld owner is inferred again after its cool-down. */
export function withheldRevalidationTextV1(owner,entry,previousAt){
 const values={owner,minutes:Math.max(0,Math.round((entry.at-previousAt)/60000)),attempt:entry.attempts,attempts:policy.revalidateAttempts};
 return{en:fill(template.en,values),de:fill(template.de,values)};
}

/** 🔁️ The withheld owners of a generation (untagged `unavailable` rows that name their declaration) that this read infers again, judged by {@link withheldRevalidationV1} against a small ledger kept beside the generations (`ledger.read()` answers the object of the last read or undefined, `ledger.write(object)` stores it); owners the read does not concern (`include`) are not judged. A ledger that cannot be read or written never fails a read: nothing is inferred again then. `say` receives the bilingual line of each re-validation.
 * @param {Record<string,{kind:string,tag?:string,config?:string,fragment:string}>} owners */
export async function revalidatedWithheldOwnersV1(owners,{ledger,now,include=()=>true,say}){
 let book;try{book=await ledger.read();}catch{return[];}
 book=book!==null&&typeof book==="object"?book:{};
 const next={},names=[],at=now();
 for(const[name,row]of Object.entries(owners)){
  if(row.kind!=="unavailable"||row.tag||typeof row.config!=="string"||!include(name))continue;
  const decision=withheldRevalidationV1({entry:book[name],fragment:row.fragment,now:at});
  next[name]=decision.entry;
  if(decision.revalidate){names.push(name);try{say?.(withheldRevalidationTextV1(name,decision.entry,book[name].at));}catch{}}
 }
 const kept=Object.fromEntries(Object.entries(book).filter(([name])=>!(name in next)&&owners[name]?.kind==="unavailable"));
 const merged={...kept,...next};
 if(JSON.stringify(merged)!==JSON.stringify(book))try{await ledger.write(merged);}catch{return[];}
 return names;
}
