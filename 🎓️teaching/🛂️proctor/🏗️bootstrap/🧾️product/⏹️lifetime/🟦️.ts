import type {ProctorProcess} from "../../🟦️.ts";

/** ⏹️ Stops and drains an owned proctor when the original signal is cancelled. */
export async function waitForProctorV1(child:Pick<ProctorProcess,"exited"|"stop">,signal:AbortSignal):Promise<number>{
 let cancel!:()=>void;const aborted=new Promise<Readonly<{kind:"abort"}>>(resolve=>{cancel=()=>resolve({kind:"abort"});signal.addEventListener("abort",cancel,{once:true});if(signal.aborted)cancel();});
 try{
  const result=await Promise.race([aborted,child.exited.then(status=>({kind:"exit" as const,status}))]);
  if(result.kind==="exit")return result.status;
  const reason=signal.reason;
  try{await child.stop();await child.exited;}catch(error){throw new AggregateError([reason,error],"Proctor cancellation cleanup failed");}
  throw reason;
 }finally{signal.removeEventListener("abort",cancel);}
}
