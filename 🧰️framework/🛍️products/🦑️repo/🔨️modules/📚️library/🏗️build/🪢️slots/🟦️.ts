import {createHash} from "node:crypto";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🪢️ The request header that carries the slot of a stored entry.
 * @see ./🧬️schema/🔣️.json */
export const SLOT_HEADER_V1="x-semio-slot";

/** 🪢️ The kinds of unit a slot identifies. */
export type SlotDomainV1="rust-unit"|"nx-task"|"stage-output";

/** ⏱️ The default grace in milliseconds before a superseded entry is reclaimed: machines a few seconds behind and builds in flight read it meanwhile. */
export const SLOT_GRACE_MILLISECONDS_V1=600000;

/** 🪪️ Names the slot of one unit: the SHA-256 of the canonical JSON array of the domain and the identity parts (no content inputs). Parts keep their order; callers sort set-like parts (features, cfg) first. */
export function slotIdV1(domain:SlotDomainV1,parts:readonly string[]):string{
 for(const part of parts)if(typeof part!=="string")throw Error("Slot parts must be strings / Slot-Teile müssen Zeichenketten sein");
 return createHash("sha256").update(JSON.stringify(["semio-slot-v1",domain,...parts])).digest("hex");
}

/** 🔎️ Tells whether a value is a well-formed slot id. */
export function isSlotIdV1(value:unknown):value is string{return typeof value==="string"&&/^[0-9a-f]{64}$/u.test(value);}

/** 🛡️ Validates a document against a named definition of the slot schema and names the first problem. */
export function requireSlotShapeV1(definition:"Retention"|"Meta"|"Claim"|"AuditReport",value:unknown,message:string):void{
 const errors=validateJsonSchemaSubset({$ref:`#/definitions/${definition}`},value,schema);
 if(errors.length)throw Error(`${message}: ${errors[0]}`);
}

/** 🗣️ The English and German texts of audits, keyed by message. */
export const SLOT_MESSAGES_V1=Object.freeze(schema.definitions.Messages.const);

/** 💬️ Fills one text of the catalogue in one language. */
export function slotTextV1(key:keyof typeof SLOT_MESSAGES_V1,language:"en"|"de",variables:Readonly<Record<string,string|number>>={}):string{
 return SLOT_MESSAGES_V1[key][language].replace(/\{(\w+)\}/gu,(_,name:string)=>String(variables[name]??""));
}

/** 📇️ One entry as retention sees it: its key, its slot (`null` without one), when it was stored or last claimed and when it was last read. */
export type SlotRowV1=Readonly<{key:string;slot:string|null;storedAt:number;accessed:number;size:number}>;

/** 🚦️ Where an entry stands: `current` is the newest key of its slot, `retained` is superseded (or was stored) within the grace and still readable, `expired` is superseded beyond the grace and may go, `unslotted` follows the age and least-recently-used rules of its cache. */
export type SlotStateV1="current"|"retained"|"expired"|"unslotted";

/** 🧾️ The verdict on one entry: its state, when it was superseded and when it may be reclaimed. */
export type SlotVerdictV1=Readonly<{state:SlotStateV1;supersededAt:number|null;reclaimableAt:number|null}>;

const order=(left:string,right:string):number=>left<right?-1:left>right?1:0;

/** 🧮️ Classifies entries by slot, as a pure function. Within a slot the newest `storedAt` (ties by key, the smaller key is newer) is current; every older key is superseded at the `storedAt` of the next newer key and reclaimable once that is older than the grace and it was not stored or claimed within the grace itself. Reads do not matter: a read never resurrects an entry, a store or claim sets `storedAt` and so makes the key current. */
export function classifySlotsV1(rows:Iterable<SlotRowV1>,input:Readonly<{now:number;graceMilliseconds:number}>):ReadonlyMap<string,SlotVerdictV1>{
 const verdicts=new Map<string,SlotVerdictV1>(),groups=new Map<string,SlotRowV1[]>();
 for(const row of rows){
  if(row.slot===null){verdicts.set(row.key,{state:"unslotted",supersededAt:null,reclaimableAt:null});continue;}
  const group=groups.get(row.slot);if(group)group.push(row);else groups.set(row.slot,[row]);
 }
 for(const group of groups.values()){
  group.sort((left,right)=>right.storedAt-left.storedAt||order(left.key,right.key));
  group.forEach((row,index)=>{
   if(index===0){verdicts.set(row.key,{state:"current",supersededAt:null,reclaimableAt:null});return;}
   const supersededAt=group[index-1]!.storedAt,reclaimableAt=Math.max(supersededAt,row.storedAt)+input.graceMilliseconds;
   verdicts.set(row.key,{state:input.now>=reclaimableAt?"expired":"retained",supersededAt,reclaimableAt});
  });
 }
 return verdicts;
}

/** 🥇️ The order in which a size budget may take entries, least valuable tier first: retained before unslotted before current; `expired` entries are not part of it (they go first, whatever the budget says). */
export function slotBudgetTierV1(state:SlotStateV1):number{return state==="expired"?-1:state==="retained"?0:state==="unslotted"?1:2;}
