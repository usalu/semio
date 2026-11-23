import {clearPinnedAuthorityV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import {reconcileBuildOwnerRegistry} from "../../🧾️registry-consistency/🟦️.ts";
import {GRAPH_LIMITS_V1} from "../../🔐️authority/🟦️.ts";
import {readBuildOwnerRegistryV1} from "../../🚪️admission/🟦️.ts";
import graphSchema from "../../🧬️schema/🔣️.json" with{type:"json"};
import {createReadTakenMemoV1,emitGraphPassNoticeV1} from "../♻️pass-reuse/🟦️.ts";
import type {TakenMemoV1} from "../../📥️inference/📦️project-nodes/♻️taken-memo/🟨️.mjs";
import {isOwnerFailureV1,registryFaultReasonV1} from "../🛟️owner-isolation/🟦️.ts";

/** 🪪️ The native roots a refused inference names because no registry row owns them yet (the two refusals of the native closure), whatever wrapper carried the message across the worker farm. */
export function pendingOwnerRootsV1(message:string):string[]{
  return [...new Set([...message.matchAll(/Native dependency has no build project owner: (.+?) \(required by|Canonical native dependency owner required: (\S+)/gu)].map(match=>(match[1]??match[2])!))];
}

/** 👻️ The native roots whose Cargo manifest vanished while the graph was inferred (a refused project names the owner and the absent manifest), whatever wrapper carried the message across the worker farm. */
export function vanishedOwnerRootsV1(message:string):string[]{
  return [...new Set([...message.matchAll(/Build owner manifest absent: (.+?)\/Cargo\.toml \(required by/gu)].map(match=>match[1]!))];
}

/** 🛠️ Reconciles only the named owner roots under the current physical registry. */
function adoptOwnerRegistryRootsV1(root:string,roots:readonly string[]):boolean{
  if(roots.length===0)return false;
  const result=reconcileBuildOwnerRegistry(root,{scope:roots,write:true});
  return result.added.length>0||result.removed.length>0;
}

/** 🤝️ Reconciles missing or vanished owners named in the inference fault and reports whether a row changed. */
export function adoptPendingBuildOwnersV1(root:string,message:string):boolean{
  return adoptOwnerRegistryRootsV1(root,[...pendingOwnerRootsV1(message),...vanishedOwnerRootsV1(message)]);
}

/** 🧱️ The typed refusal of a read whose dependency still names an owner root without a row at the final check, in English and German; its text is declared by the graph schema.
 * @see ../../🧬️schema/🔣️.json */
export function ownerRegistryRefusalV1(input:{roots:readonly string[];rounds:number;moved:boolean;cause:unknown}):Error{
  const text=graphSchema.definitions.OwnerRegistryRefusal.const,fill=(template:string)=>template.replace("{roots}",input.roots.join(", ")).replace("{rounds}",String(input.rounds)).replace("{moved}",String(input.moved)).replace("{cause}",String((input.cause as Error)?.message??input.cause).slice(0,400));
  return Object.assign(new Error(fill(text.en)+" | "+fill(text.de),{cause:input.cause}),{code:text.code,roots:[...input.roots]});
}

/** 🧾️ The text that names the pending or vanished owner roots of a registry fault: the raw fault itself, or the reasons of a named failure whose owners all failed on a registry fault (a failure that also holds a real owner fault is never a registry fault). */
export function registryFaultTextV1(error:Error):string|undefined{
  if(isOwnerFailureV1(error)){
    const reasons=error.owners.map(owner=>registryFaultReasonV1(owner.code,owner.reason));
    return reasons.every(reason=>reason!==undefined)?reasons.join("\n"):undefined;
  }
  return registryFaultReasonV1("code"in error?error.code:undefined,error.message);
}

type RegistryCustodyOutcomeV1<T>={kind:"result";value:T}|{kind:"invalidated";fault:Error};

class ClosedOwnerRegistryInvalidationV1 extends Error{
  readonly fault:Error;
  constructor(fault:Error){super(fault.message,{cause:fault});this.fault=fault;}
}

/** 🚪️ Recognizes an expected registry fault after the physical operation returned and settled. */
export function isClosedOwnerRegistryInvalidationV1(error:unknown):boolean{
  return error instanceof ClosedOwnerRegistryInvalidationV1;
}

/** 🛟️ Carries only a named owner-registry fault through issued custody as data, then follows it after successful cleanup. */
export async function withOwnerRegistryCustodyV1<T>(signal:AbortSignal,hold:(contain:(run:()=>Promise<T>)=>Promise<RegistryCustodyOutcomeV1<T>>)=>Promise<RegistryCustodyOutcomeV1<T>>):Promise<T>{
  const outcome=await hold(async run=>{
    try{return {kind:"result",value:await run()};}
    catch(error){
      if(signal.aborted||!(error instanceof Error)||registryFaultTextV1(error)===undefined)throw error;
      return {kind:"invalidated",fault:error};
    }
  });
  signal.throwIfAborted();
  if(outcome.kind==="invalidated")throw new ClosedOwnerRegistryInvalidationV1(outcome.fault);
  return outcome.value;
}

/** 🧠️ What the attempts of one registry follow share: the taken memo of the read, so a restart keeps every owner the earlier passes took (each is answered again only while its job and every row it consumed still hold).
 * @see ../♻️pass-reuse/🟦️.ts */
export type RegistryCarryV1=Readonly<{takenMemo:TakenMemoV1}>;

/** 🧭️ Retries a refused native-owner lookup when the registry changed or reconciliation repaired the named roots, carrying the taken memo of the read from one attempt to the next. */
export async function settleOwnerRegistryV1<T>(root:string,attempt:(carried:RegistryCarryV1)=>Promise<T>,options:{aborted?:()=>boolean;say?:(line:string)=>void}={}):Promise<T>{
  const {readBuildOwnerRegistrySourceV1}=await import("../../📥️inference/🟨️.mjs"),carried:RegistryCarryV1=Object.freeze({takenMemo:createReadTakenMemoV1()});
  for(let round=0;;round++){
    const before=readBuildOwnerRegistrySourceV1(root);
    try{return await attempt(carried);}
    catch(error){
      if(!(error instanceof ClosedOwnerRegistryInvalidationV1))throw error;
      const fault=error.fault,message=registryFaultTextV1(fault)??fault.message,pending=pendingOwnerRootsV1(message),vanished=vanishedOwnerRootsV1(message);
      const roots=[...new Set([...pending,...vanished])];
      if(roots.length===0||options.aborted?.())throw fault;
      const current=readBuildOwnerRegistrySourceV1(root,"current");
      let moved=current!==before,adopted=false;
      if(round>=GRAPH_LIMITS_V1.repairRounds)throw ownerRegistryRefusalV1({roots,rounds:round,moved,cause:fault});
      let unresolved:readonly string[]=roots;
      try{
        const {owners}=readBuildOwnerRegistryV1(JSON.parse(current));
        const owned=(path:string)=>owners.some(owner=>owner.root===path||owner.nativeRoots?.includes(path));
        unresolved=[...pending.filter(path=>!owned(path)),...vanished.filter(owned)];
        if(unresolved.length<roots.length)moved=true;
      }catch{moved=true;}
      try{adopted=adoptOwnerRegistryRootsV1(root,moved?unresolved:roots);}catch{}
      if(!moved&&!adopted)moved=readBuildOwnerRegistrySourceV1(root,"current")!==before;
      if(!moved&&!adopted)throw ownerRegistryRefusalV1({roots,rounds:round,moved,cause:fault});
      clearPinnedAuthorityV1();
      emitGraphPassNoticeV1({round:round+1,moved:roots.length,evicted:0,heap:0});
      options.say?.("[graph-repair] "+JSON.stringify({round:round+1,registry:"followed",roots,adopted,moved})+"\n");
    }
  }
}
