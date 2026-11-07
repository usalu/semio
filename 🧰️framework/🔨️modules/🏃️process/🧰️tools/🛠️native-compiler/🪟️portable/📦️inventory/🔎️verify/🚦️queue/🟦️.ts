import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import readerSchema from "../../../../../../📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🧬️schema/🔣️.json" with {type:"json"};

const policy=schema.definitions.Protocol.const;
if(policy.replayChunkBytes!==readerSchema.definitions.Protocol.const.maximumChunkBytes)throw Error("Compiler inventory allocation must match the physical reader");
/** 📣️ Reports only completed work at the authored cadence or the final item. */
export const inventoryProgressDueV1=(completed:number,total:number):boolean=>completed>0&&(completed%policy.progressEntries===0||completed===total);
/** 📐️ Accounts for the reader's file buffer, replay buffer and EOF probe. */
export function inventoryAllocationV1(bytes:number):number{
 if(!Number.isSafeInteger(bytes)||bytes<0||bytes>policy.maximumFileBytes)throw Error("Bounded compiler file extent required");
 return bytes+Math.min(bytes,policy.replayChunkBytes)+policy.eofBytes;
}

/** 🚦️ Bounds concurrent physical reads and drains every started operation before reporting failure. */
export function inventoryQueueV1(signal?:AbortSignal):Readonly<{submit:(bytes:number,work:()=>Promise<void>)=>Promise<void>;fail:(error:unknown)=>void;drain:()=>Promise<void>}>{
 const running=new Set<Promise<void>>();let used=0,failed=false,closed=false,failure:unknown;
 const check=()=>{if(!failed&&signal?.aborted){failed=true;failure=signal.reason;}if(failed)throw failure;};
 const admit=()=>{check();if(closed)throw Error("Compiler inventory queue is closed");};
 const wait=async()=>{let abort:(()=>void)|undefined;const cancelled=signal?new Promise<void>(resolve=>{abort=()=>resolve();signal.addEventListener("abort",abort,{once:true});if(signal.aborted)resolve();}):undefined;try{await Promise.race(cancelled?[...running,cancelled]:[...running]);}finally{if(abort)signal!.removeEventListener("abort",abort);}};
 return Object.freeze({
  fail(error:unknown){if(!failed){failed=true;failure=error;}},
  async submit(bytes:number,work:()=>Promise<void>){const weight=inventoryAllocationV1(bytes);admit();while(running.size>=policy.workers||used+weight>policy.maximumAllocationBytes){await wait();admit();}used+=weight;let task:Promise<void>;task=Promise.resolve().then(()=>{check();return work();}).catch(error=>{if(!failed){failed=true;failure=error;}}).finally(()=>{used-=weight;running.delete(task);});running.add(task);},
  async drain(){closed=true;await Promise.all([...running]);check();}
 });
}
