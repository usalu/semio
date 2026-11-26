import descriptorSchema from "../🧬️schema/🧷️descriptor.json" with {type:"json"};
import {concurrentEditV1,type ConcurrentEditErrorV1} from "../🟦️.ts";

const protocol=Object.freeze(JSON.parse(JSON.stringify(descriptorSchema.definitions.Protocol.const)) as Readonly<{version:1;kinds:readonly DescriptorKindV1[];pausesMilliseconds:readonly number[];stampFields:readonly string[];texts:Readonly<Record<DescriptorKindV1,Readonly<{en:string;de:string}>>>}>);
const sleeper=new Int32Array(new SharedArrayBuffer(4));

/** 🧷️ The three classes of an observed-then-opened file: `append-only` (siblings append, times are volatile), `immutable` (nobody may touch it while open), `renamed` (replaced atomically by rename, so the name is reopened). */
export type DescriptorKindV1="append-only"|"immutable"|"renamed";
/** 🪪️ The numbers of one stat, always bigint, so a descriptor and a name compare field by field. */
export type DescriptorFactsV1=Readonly<{dev:bigint;ino:bigint;mode:bigint;nlink:bigint;size:bigint;mtimeNs:bigint;ctimeNs:bigint}>;
/** 🧾️ What a verification found: `same` all fields equal, `grew` an append-only file got longer, `touched` only the times moved, `renamed` the name denotes another file than the observation and the descriptor was reopened. `facts` are the facts of the descriptor that now holds. */
export type DescriptorVerdictV1=Readonly<{state:"same"|"grew"|"touched"|"renamed";facts:DescriptorFactsV1}>;
/** 🏷️ What the file is, in both languages, for the refusal text. */
export type DescriptorLabelV1=Readonly<{en:string;de:string}>;
/** 🎛️ The ports of one verification: the observed facts, the descriptor's current facts, for `renamed` the name's current facts and the reopening of the name, the pauses and the clock-free sleeper. */
export type DescriptorSettleV1=Readonly<{kind:DescriptorKindV1;path:string;label:DescriptorLabelV1;expected:DescriptorFactsV1;descriptor:()=>DescriptorFactsV1;named?:()=>DescriptorFactsV1;reopen?:()=>void;pausesMilliseconds?:readonly number[];sleepSync?:(milliseconds:number)=>void}>;

type StatsV1=Readonly<{dev:bigint|number;ino:bigint|number;mode:bigint|number;nlink:bigint|number;size:bigint|number;mtimeNs?:bigint;ctimeNs?:bigint;mtimeMs?:number|bigint;ctimeMs?:number|bigint}>;
const big=(value:bigint|number):bigint=>typeof value==="bigint"?value:BigInt(Math.trunc(value));
const nanoseconds=(exact:bigint|undefined,milliseconds:number|bigint|undefined):bigint=>exact??(typeof milliseconds==="bigint"?milliseconds*1000000n:BigInt(Math.round((milliseconds??0)*1000))*1000n);

/** 📖️ The closed numbers and EN/DE texts exactly as the schema declares them. */
export function descriptorProtocolV1():typeof protocol{return protocol;}
/** 🪪️ The facts of one stat (bigint or number stats). */
export function descriptorFactsV1(stats:StatsV1):DescriptorFactsV1{return Object.freeze({dev:big(stats.dev),ino:big(stats.ino),mode:big(stats.mode),nlink:big(stats.nlink),size:big(stats.size),mtimeNs:nanoseconds(stats.mtimeNs,stats.mtimeMs),ctimeNs:nanoseconds(stats.ctimeNs,stats.ctimeMs)});}
/** 🔖️ The stamp string of the facts, fields in the order of the schema (`dev:ino:mode:nlink:size:mtimeNs:ctimeNs`). */
export function descriptorStampV1(facts:DescriptorFactsV1):string{return protocol.stampFields.map(field=>String(facts[field as keyof DescriptorFactsV1])).join(":");}
/** 🔎️ The facts a stamp string names. */
export function descriptorFactsOfStampV1(stamp:string):DescriptorFactsV1{
 const parts=stamp.split(":");if(parts.length!==protocol.stampFields.length||parts.some(part=>!/^\d+$/u.test(part)))throw Error("Declared descriptor stamp required: "+stamp);
 const row=Object.fromEntries(protocol.stampFields.map((field,at)=>[field,BigInt(parts[at]!)]));return Object.freeze(row as unknown as DescriptorFactsV1);
}
const identity=(facts:DescriptorFactsV1):string=>facts.dev+":"+facts.ino;
const judge=(kind:DescriptorKindV1,expected:DescriptorFactsV1,actual:DescriptorFactsV1):DescriptorVerdictV1["state"]|undefined=>{
 const stamp=descriptorStampV1(actual)===descriptorStampV1(expected);
 if(kind==="immutable")return stamp?"same":undefined;
 if(actual.dev!==expected.dev||actual.ino!==expected.ino||actual.nlink!==expected.nlink||actual.size<expected.size)return undefined;
 return stamp?"same":actual.size>expected.size?"grew":"touched";
};
/** 🛑️ The refusal of a file that kept differing: a concurrent edit of the `read` layer, whose text names what the file is in both languages, so the retry layers of the churn policy run the read again and callers classify it as a transient source change. */
export function descriptorRefusalV1(kind:DescriptorKindV1,path:string,label:DescriptorLabelV1):ConcurrentEditErrorV1{
 const text=protocol.texts[kind];return concurrentEditV1("descriptor",path,label.en+" "+text.en+" / "+label.de+" "+text.de);
}
/** ⚖️ Verifies one open descriptor against its earlier observation by its class and returns the verdict; a mismatch is observed again after each declared pause and only a mismatch that survives all of them is refused with {@link descriptorRefusalV1}. For `renamed` a descriptor that is not the file the name denotes now is replaced through `reopen` before the next observation. The append-only identity ignores the modification and change times, so a sibling's append or a metadata-only touch is not a refusal. */
export function settleDescriptorV1(input:DescriptorSettleV1):DescriptorVerdictV1{
 const{kind,path,label,expected,descriptor,named,reopen}=input,pauses=input.pausesMilliseconds??protocol.pausesMilliseconds,sleep=input.sleepSync??((milliseconds:number)=>{Atomics.wait(sleeper,0,0,milliseconds);});
 if(!protocol.kinds.includes(kind))throw Error("Declared descriptor kind required: "+kind);
 for(let attempt=0;;attempt++){
  const actual=descriptor();
  if(kind==="renamed"){
   let now:DescriptorFactsV1|undefined;try{now=named?.();}catch(error){if((error as NodeJS.ErrnoException)?.code!=="ENOENT")throw error;}
   if(now&&identity(now)===identity(actual)&&now.nlink===1n&&now.size===actual.size)return Object.freeze({state:descriptorStampV1(actual)===descriptorStampV1(expected)?"same" as const:"renamed" as const,facts:actual});
   if(attempt<pauses.length){sleep(pauses[attempt]!);reopen?.();continue;}
   throw descriptorRefusalV1(kind,path,label);
  }
  const state=judge(kind,expected,actual);
  if(state)return Object.freeze({state,facts:actual});
  if(attempt>=pauses.length)throw descriptorRefusalV1(kind,path,label);
  sleep(pauses[attempt]!);
 }
}
