import{machinePathV1}from"../../../../../../../🔨️modules/🏃️process/🏗️build/🏠️machine-state/🟦️.ts";
import{countEventV1}from"../../../🏗️build/📈️counter-ledger/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};

/** 📐️ The markers of Nx's cache answers and the names of the counter files.
 * @see ./🧬️schema/🔣️.json */
export const NX_CACHE_POLICY_V1=schema.definitions.Policy.const;
const P=NX_CACHE_POLICY_V1,TASK=new RegExp(P.taskPattern,"u"),LOCAL=new RegExp(P.localPattern,"u"),REMOTE=new RegExp(P.remotePattern,"u"),COLOUR=/\u001b\[[0-9;]*[A-Za-z]/gu;

/** 🏷️ How one Nx task ended. */
export type NxCacheOutcomeV1="local"|"remote"|"executed";

/** 🔎️ The outcome a line of Nx output reports: a task line with a local marker is a local hit, with the remote marker a remote hit, without a marker an execution; `undefined` for every other line (colours are ignored). */
export function nxCacheOutcomeOfLineV1(line:string):NxCacheOutcomeV1|undefined{
 const text=line.replace(COLOUR,"").trim();
 if(!TASK.test(text))return undefined;
 return LOCAL.test(text)?"local":REMOTE.test(text)?"remote":"executed";
}

/** 🔢️ Counts the cache outcomes of the Nx output pushed into it, line by line (a line cut between two chunks is completed by the next), into the counter files of a worktree's machine state; `flush` counts the unfinished last line. Never throws. `directory` replaces the machine's directory in a test.
 * @see ./🧪️tests/🟦️.ts */
export function createNxCacheCounterV1(workspace:string,directory:string=machinePathV1(workspace,".cache",...P.directory)):Readonly<{push(chunk:string|Uint8Array):void;flush():void}>{
 const decoder=new TextDecoder("utf-8");let pending="";
 const count=(line:string):void=>{const outcome=nxCacheOutcomeOfLineV1(line);if(outcome!==undefined)countEventV1(directory,P.events[outcome]);};
 return Object.freeze({
  push(chunk:string|Uint8Array):void{
   const parts=(pending+(typeof chunk==="string"?chunk:decoder.decode(chunk,{stream:true}))).split("\n");pending=parts.pop()!;
   for(const line of parts)count(line);
   if(pending.length>P.maximumLineCharacters){count(pending);pending="";}
  },
  flush():void{if(pending!==""){count(pending);pending="";}},
 });
}
