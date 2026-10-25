import {acquireResourceLease} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {concurrentLeaseResourceV1} from "../../🟦️.ts";
import {withConcurrentPinsV1} from "../../💾️storage/🟦️.ts";
import {existsSync} from "node:fs";

/** 📖️ Is a separate process that reads like a dependency consumer: shared bindings gate plus a shared pin, probed without waiting, reporting every transition between served and refused. */
const [directory,workspace,generation,stop,deadline]=process.argv.slice(2) as [string,string,string,string,string],end=Date.now()+Number(deadline),signal=new AbortController().signal,timeline:[number,"served"|"refused"][]=[];
let served=0,refused=0,slowest=0,state:string|undefined;
while(Date.now()<end&&!existsSync(stop)){
 const started=performance.now();
 let outcome:"served"|"refused"="served";
 try{
  const gate=await acquireResourceLease({directory,resource:concurrentLeaseResourceV1("bindings",workspace),mode:"shared",signal,timeoutMs:0});
  try{await withConcurrentPinsV1(directory,workspace,[generation],undefined,signal,async()=>{});}finally{gate.release();}
 }catch(error){if(error instanceof Error&&error.message.startsWith("Resource lease timed out:"))outcome="refused";else throw error;}
 const elapsed=performance.now()-started;
 if(outcome==="served"){served++;slowest=Math.max(slowest,elapsed);}else refused++;
 if(state!==outcome){state=outcome;timeline.push([Date.now(),outcome]);}
 await new Promise<void>(accept=>setTimeout(accept,4));
}
process.stdout.write(JSON.stringify({served,refused,slowestServedMs:Math.round(slowest),timeline})+"\n");
