import {concurrentEditOfV1,STALE_IMPLEMENTATION_CODES_V1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {OBSERVATION_LIMITS_V1} from "../../../👁️observation/🟨️.mjs";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import graphSchema from "../../🧬️schema/🔣️.json" with {type:"json"};

/** 📜️ The declared retry rounds, pauses and final failure codes of an owner observation. */
export const ownerIsolationPolicyV1=schema.$defs.Policy.const;
const texts=schema.$defs.Texts.const,fill=(template:string,values:Readonly<Record<string,string|number>>)=>template.replace(/\{(\w+)\}/gu,(_,name:string)=>String(values[name]));
const both=(text:{readonly en:string;readonly de:string},values:Readonly<Record<string,string|number>>)=>({en:fill(text.en,values),de:fill(text.de,values)});
const causeOf=(error:unknown)=>String((error as {message?:unknown}|null)?.message??error).split("\n")[0]!.slice(0,ownerIsolationPolicyV1.causeCharacters);

/** 🧾️ One owner of a named failure: its own failure code, the first line of its reason, the first observed path it rests on, and the root-cause owner it cascades from (only when the failures name one). */
export interface OwnerEntryV1 {readonly name:string;readonly code:string;readonly reason:string;readonly path?:string;readonly causedBy?:string}

/** 🛟️ The named, bilingual failure of one or several owners whose inference reproduces on stable inputs (or kept racing a peer's edit through every round). */
export interface OwnerFailureV1 extends Error {readonly code:"BUILD_OWNER_FAILED";readonly owners:readonly OwnerEntryV1[];readonly rounds:number;readonly stable:boolean;readonly text:{readonly en:string;readonly de:string};readonly observed?:readonly unknown[]}

const pathOfRowV1=(row:unknown):string|undefined=>{const path=(row as {path?:unknown}|null)?.path;return typeof path==="string"&&path.length>0?path:undefined;};
const pathOfV1=(observed?:readonly unknown[])=>(observed??[]).map(pathOfRowV1).find(path=>path!==undefined);
const entryOfV1=(name:string,code:string,reason:string,observed?:readonly unknown[]):OwnerEntryV1=>{const path=pathOfV1(observed);return {name,code,reason,...path===undefined?{}:{path}};};
const codeOfV1=(error:unknown)=>String((error as {code?:unknown;name?:unknown}|null)?.code??(error as {name?:unknown}|null)?.name??"Error");

/** 🔎️ Answers whether a failure is the named failure of an owner. */
export function isOwnerFailureV1(error:unknown):error is OwnerFailureV1{return (error as {code?:unknown}|null)?.code===ownerIsolationPolicyV1.code&&Array.isArray((error as {owners?:unknown}).owners);}

const registryFaultV1=(code:unknown,message:unknown)=>graphSchema.definitions.OwnerRegistryWorkerFailures.const.some(fault=>fault.code===code&&typeof message==="string"&&message.startsWith(fault.prefix));

/** 🧭️ Whether a failure is a pending or vanished native owner root of the registry (raw, or as the named failure of an owner whose own code and reason are that fault): the registry follow reconciles the named roots and retries it, so it is never an owner's own failure.
 * @see ../🧭️registry-follow/🟦️.ts */
export function registryPendingFailureV1(error:unknown):boolean{
 const value=error as {code?:unknown;message?:unknown;owners?:readonly OwnerEntryV1[]}|null;
 return registryFaultV1(value?.code,value?.message)||isOwnerFailureV1(error)&&error.owners.some(owner=>registryFaultV1(owner.code,owner.reason));
}

/** 🛑️ Whether a failure ends the repair walk at once: the named failure of an owner that reproduced on stable inputs, whose consumed rows still hold, whatever the workspace tree digest does meanwhile; a registry-pending fault never does. */
export function stableOwnerFailureHoldsV1(error:unknown,holds:boolean):boolean{return holds&&isOwnerFailureV1(error)&&error.stable&&!registryPendingFailureV1(error);}

/** 🏷️ Names the failure of one owner in English and German; the message carries both languages. */
export function ownerFailureV1(owner:string,error:unknown,rounds:number,stable:boolean,observed?:readonly unknown[]):OwnerFailureV1{
 const text=both(stable?texts.stable:texts.unsettled,{owner,rounds,cause:causeOf(error)});
 return Object.assign(new Error(text.en+" / "+text.de,{cause:error}),{code:ownerIsolationPolicyV1.code,owners:[entryOfV1(owner,codeOfV1(error),causeOf(error),observed)],rounds,stable,text,...observed===undefined?{}:{observed}}) as OwnerFailureV1;
}

/** 🧩️ Joins the failures of several owners into one named failure (a single failure stays as it is): every owner keeps its own code, reason and observed path; an owner whose observed rows lie in the sources of another failed owner (`ownerOfPath`, as the admission resolves them) cascades from the root-cause owner it rests on, and the text groups it under that root; owners without such a dependency are root causes with a failure of their own. */
export function joinOwnerFailuresV1(failures:readonly OwnerFailureV1[],ownerOfPath?:(path:string)=>string|undefined):OwnerFailureV1{
 if(failures.length===1)return failures[0]!;
 const owners=failures.flatMap(failure=>failure.owners).sort((a,b)=>a.name.localeCompare(b.name)),failed=new Set(owners.map(owner=>owner.name)),next=new Map<string,string>();
 if(ownerOfPath!==undefined)for(const failure of failures){const own=new Set(failure.owners.map(owner=>owner.name));for(const owner of failure.owners){const hit=(failure.observed??[]).map(pathOfRowV1).filter(path=>path!==undefined).map(path=>ownerOfPath(path)).filter(name=>name!==undefined&&failed.has(name)&&!own.has(name)).sort()[0];if(hit!==undefined)next.set(owner.name,hit);}}
 const rootOf=(name:string)=>{const seen=new Set([name]);let at=name;for(let step=next.get(at);step!==undefined;step=next.get(at)){if(seen.has(step))return undefined;seen.add(step);at=step;}return at===name?undefined:at;};
 const entries=owners.map(owner=>{const causedBy=rootOf(owner.name);return causedBy===undefined?owner:{...owner,causedBy};});
 const roots=entries.filter(owner=>owner.causedBy===undefined);
 const lineOf=(owner:OwnerEntryV1)=>({en:fill(texts.owner.en,{owner:owner.name,code:owner.code,reason:owner.reason,path:owner.path??texts.unobserved.en}),de:fill(texts.owner.de,{owner:owner.name,code:owner.code,reason:owner.reason,path:owner.path??texts.unobserved.de})});
 const groupOf=(root:OwnerEntryV1)=>{const cascades=entries.filter(owner=>owner.causedBy===root.name).map(lineOf),own=lineOf(root);return {en:own.en+(cascades.length?" ["+fill(texts.cascaded.en,{owners:cascades.map(item=>item.en).join("; ")})+"]":""),de:own.de+(cascades.length?" ["+fill(texts.cascaded.de,{owners:cascades.map(item=>item.de).join("; ")})+"]":"")};};
 const groups=roots.map(groupOf),values={count:entries.length,roots:roots.length};
 const text={en:fill(texts.several.en,{...values,owners:groups.map(group=>group.en).join("; ")}),de:fill(texts.several.de,{...values,owners:groups.map(group=>group.de).join("; ")})};
 return Object.assign(new Error(text.en+" / "+text.de,{cause:failures[0]}),{code:ownerIsolationPolicyV1.code,owners:entries,rounds:Math.max(...failures.map(failure=>failure.rounds)),stable:failures.every(failure=>failure.stable),text}) as OwnerFailureV1;
}

/** 🗣️ The bilingual line saying an owner is left out of a read that does not need it. */
export function ownerWithheldTextV1(owner:string,failure:OwnerFailureV1):{readonly en:string;readonly de:string}{return both(texts.withheld,{owner,cause:failure.text.en.replace(/\s+$/u,"")});}

/** 🚦️ Whether a failure belongs to the follow-up of another layer (restart, withheld source, membership, an edit the inventory follows, abort) and is never an owner's own. */
export function isForeignFailureV1(error:unknown,aborted:boolean=false):boolean{
 const value=error as {code?:unknown;name?:unknown;replicaUnsettled?:unknown}|null;
 return aborted||value?.name==="AbortError"||value?.replicaUnsettled===true||(ownerIsolationPolicyV1.finalCodes as readonly string[]).includes(String(value?.code))||STALE_IMPLEMENTATION_CODES_V1.has(String(value?.code))||isOwnerFailureV1(error)||registryPendingFailureV1(error)||concurrentEditOfV1(error)!==undefined;
}

/** ⏱️ The newest write time (modification or change) of the source files the rows name, or zero. */
export function youngestRowMillisecondsV1(rows:readonly unknown[]):number{
 let youngest=0;
 for(const row of rows){const {kind,fact}=row as {kind?:string;fact?:{mtimeMs?:number;ctimeMs?:number;source?:{mtimeMs?:number;ctimeMs?:number}}};const source=kind==="file"?fact:kind==="analysis"?fact?.source:undefined;if(source)youngest=Math.max(youngest,source.mtimeMs??0,source.ctimeMs??0);}
 return youngest;
}

/** 🧮️ The decision table of one failed observation of an owner, shared by every path that infers owners (closure walk, farm child, late references): a foreign failure is another layer's, a failure on rows that held and are not young is stable at once, otherwise the owner is observed again until the declared rounds are used up, and then the failure is `unsettled` when the rows kept moving and `stable` when they only were young.
 * @see ./🧪️tests/🐍️oracle/🐍️.py */
export type OwnerDecisionV1="foreign"|"retry"|"stable"|"unsettled";
export function ownerFailureDecisionV1(input:{readonly foreign:boolean;readonly moved:boolean;readonly young:boolean;readonly round:number}):OwnerDecisionV1{
 if(input.foreign)return "foreign";
 if(!input.moved&&!input.young)return "stable";
 if(input.round>=ownerIsolationPolicyV1.retryRounds)return input.moved?"unsettled":"stable";
 return "retry";
}

/** ⏱️ The pause before the next observation: the round times the declared pause (never past the declared rounds), long enough to leave the racy window of the youngest row, never past the ceiling. */
export function ownerPauseMillisecondsV1(round:number,youngest:number,racyMilliseconds:number,now:number):number{
 const policy=ownerIsolationPolicyV1;
 return Math.min(Math.max(Math.min(round,policy.maximumPauseRounds)*policy.settlePauseMilliseconds,youngest+racyMilliseconds-now),policy.maximumPauseRounds*policy.settlePauseMilliseconds);
}

/** 🧭️ Judges one failed observation of an owner from the rows it consumed up to the failure: whether they moved (`hold`), whether one was written within the racy window before the attempt began, and the decision of the table with the pause that goes with a retry. */
export async function ownerRetryPlanV1(input:{readonly rows:readonly unknown[];readonly hold:(rows:readonly unknown[])=>Promise<boolean>;readonly foreign:boolean;readonly started:number;readonly round:number;readonly racyMilliseconds?:number;readonly now?:()=>number}):Promise<{readonly decision:OwnerDecisionV1;readonly milliseconds:number}>{
 if(input.foreign)return {decision:"foreign",milliseconds:0};
 const racy=input.racyMilliseconds??OBSERVATION_LIMITS_V1.racyMilliseconds,now=(input.now??Date.now)(),moved=!await input.hold(input.rows),youngest=youngestRowMillisecondsV1(input.rows),young=youngest>=input.started-racy,decision=ownerFailureDecisionV1({foreign:false,moved,young,round:input.round});
 return {decision,milliseconds:decision==="retry"?ownerPauseMillisecondsV1(input.round,youngest,racy,now):0};
}

/** 📨️ What a farm child sends instead of a reference when an owner failed on inputs that settled (or kept moving): the named failure and the rows it consumed, bounded by the declared reply rows. */
export interface OwnerFailureReplyV1{readonly message:string;readonly rounds:number;readonly stable:boolean;readonly code:string;readonly reason:string;readonly text:{readonly en:string;readonly de:string};readonly observed:readonly unknown[]}
export function ownerFailureReplyV1(failure:OwnerFailureV1):OwnerFailureReplyV1{const [owner]=failure.owners;return {message:failure.message,rounds:failure.rounds,stable:failure.stable,code:owner!.code,reason:owner!.reason,text:failure.text,observed:(failure.observed??[]).slice(0,ownerIsolationPolicyV1.replyRows)};}

/** 📩️ The named failure of `owner` rebuilt from the reply of a farm child; an answer of another shape is not a failure of the owner (`undefined`). */
export function ownerFailureOfReplyV1(owner:string,reply:unknown):OwnerFailureV1|undefined{
 const value=reply as Partial<OwnerFailureReplyV1>|null;
 if(value===null||typeof value!=="object"||typeof value.message!=="string"||typeof value.stable!=="boolean"||typeof value.code!=="string"||typeof value.reason!=="string"||!Number.isInteger(value.rounds)||typeof value.text?.en!=="string"||typeof value.text?.de!=="string"||!Array.isArray(value.observed))return undefined;
 const observed=value.observed.slice(0,ownerIsolationPolicyV1.replyRows);
 return Object.assign(new Error(value.message),{code:ownerIsolationPolicyV1.code,owners:[entryOfV1(owner,value.code,value.reason,observed)],rounds:value.rounds as number,stable:value.stable,text:value.text,observed}) as OwnerFailureV1;
}

/** 🔁️ Observes one owner with retries: a failure whose consumed rows changed after they were read (or were written within the racy window) observes the owner again after a settle pause, up to the declared rounds; only a failure that reproduces on rows that held becomes the named failure of that owner, and every foreign failure is thrown unchanged.
 * @see ./🧪️tests/🟦️.ts */
export async function observeOwnerWithRetryV1<T>(input:{
 readonly owner:string;
 readonly attempt:(onFailure:(rows:readonly unknown[])=>void)=>Promise<T>;
 readonly hold:(rows:readonly unknown[])=>Promise<boolean>;
 readonly racyMilliseconds?:number;
 readonly aborted?:()=>boolean;
 readonly foreign?:(error:unknown)=>boolean;
 readonly pause?:(milliseconds:number)=>Promise<void>;
 readonly now?:()=>number;
 readonly onRetry?:(event:{readonly owner:string;readonly round:number;readonly milliseconds:number;readonly text:{readonly en:string;readonly de:string}})=>void;
}):Promise<T>{
 const racy=input.racyMilliseconds??OBSERVATION_LIMITS_V1.racyMilliseconds,now=input.now??Date.now,pause=input.pause??(milliseconds=>new Promise<void>(accept=>setTimeout(accept,milliseconds)));
 for(let round=1;;round++){
  const started=now();let rows:readonly unknown[]=[];
  try{return await input.attempt(observed=>{rows=observed;});}
  catch(error){
   if(isForeignFailureV1(error,input.aborted?.()===true)||input.foreign?.(error)===true)throw error;
   const plan=await ownerRetryPlanV1({rows,hold:input.hold,foreign:false,started,round,racyMilliseconds:racy,now});
   if(plan.decision==="stable")throw ownerFailureV1(input.owner,error,round,true,rows);
   if(plan.decision==="unsettled")throw ownerFailureV1(input.owner,error,round,false,rows);
   const milliseconds=plan.milliseconds;
   input.onRetry?.({owner:input.owner,round,milliseconds,text:both(texts.retry,{owner:input.owner,round,milliseconds,cause:causeOf(error)})});
   await pause(milliseconds);
  }
 }
}
