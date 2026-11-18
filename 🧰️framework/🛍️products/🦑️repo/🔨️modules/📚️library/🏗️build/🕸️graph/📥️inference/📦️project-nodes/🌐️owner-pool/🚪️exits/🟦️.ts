import schema from "../🧬️schema/🔣️.json"with{type:"json"};

const T=schema.definitions.Texts.const;
export type OwnerPoolExitV1=Readonly<{atMs:number;pid:number;reason:string;code:number|null;ageSeconds:number;owners:number;text:{en:string;de:string}}>;
export type OwnerPoolExitInputV1=Readonly<{pid:number;code:number|null;signal?:string|null;reason?:string;drained:boolean;killed:boolean;watchdog?:string;startedMs:number;now:number;owners:number;target?:number;why?:string|null}>;
const KEYS:Readonly<Record<string,"exitFinished"|"exitRecycled"|"exitDrained"|"exitStopped"|"exitWatchdog"|"exitRevision"|"exitOpen"|"exitUnexpected">>={finished:"exitFinished",recycled:"exitRecycled",drained:"exitDrained",stopped:"exitStopped",watchdog:"exitWatchdog",revision:"exitRevision","open-failed":"exitOpen",unexpected:"exitUnexpected"};
const fill=(text:string,fields:Readonly<Record<string,unknown>>)=>text.replace(/\{(\w+)\}/gu,(_all,name:string)=>String(fields[name]??""));

/** 🚪️ Why a child of the pool ended, as one record with its line in English and German: what the watchdog saw (it writes its verdict before it ends the process), what the pool did to it (drained, stopped), what the child reported itself (`end`: finished, recycled, revision, open) and otherwise the exit code and signal; a child that ends for none of these reasons is `unexpected`, never silent.
 * @see ../🧪️tests/🚪️exits/🟦️.ts */
export function ownerPoolExitRecordV1(input:OwnerPoolExitInputV1):OwnerPoolExitV1{
 const reported=input.reason==="open"?"open-failed":input.reason;
 const reason=input.watchdog!==undefined?"watchdog":input.killed?"stopped":reported==="revision"||reported==="open-failed"||reported==="recycled"?reported:input.drained?"drained":reported==="finished"||input.code===0?"finished":reported==="drained"?"drained":"unexpected";
 const fields={pid:input.pid,age:Math.max(0,Math.round((input.now-input.startedMs)/1000)),owners:input.owners,target:input.target??0,why:input.why??"-",verdict:input.watchdog??"",code:input.code??"-",signal:input.signal??"-"},key=KEYS[reason]!;
 return{atMs:input.now,pid:input.pid,reason,code:input.code,ageSeconds:fields.age,owners:input.owners,text:{en:fill(T.en[key],fields),de:fill(T.de[key],fields)}};
}

/** 🧾️ Adds one exit to the counts by reason and keeps the newest `kept` rows. */
export function ownerPoolExitsAddV1(state:Readonly<{exits:Readonly<Record<string,number>>;lastExits:readonly OwnerPoolExitV1[]}>,exit:OwnerPoolExitV1,kept:number):{exits:Record<string,number>;lastExits:OwnerPoolExitV1[]}{
 return{exits:{...state.exits,[exit.reason]:(state.exits[exit.reason]??0)+1},lastExits:[...state.lastExits,exit].slice(-kept)};
}
