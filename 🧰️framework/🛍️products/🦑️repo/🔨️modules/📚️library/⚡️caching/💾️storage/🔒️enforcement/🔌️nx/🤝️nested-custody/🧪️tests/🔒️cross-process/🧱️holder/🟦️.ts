/** 🧱️ A second process that holds one output lease until its stdin closes: the other process of the concurrent put and restore law. */
import {createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
const [directory,resource,mode]=process.argv.slice(2) as [string,string,"shared"|"exclusive"];
await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"nx-nested-custody-holder",maximumElapsedMilliseconds:60000},{},Date.now()),async original=>{
 const control=createIssuedResourceControlV1(original,"nx:nested-custody-holder",0),lease=await acquireResourceLease({directory,resource,mode,signal:control.signal,control,timeoutMs:20000});
 process.stdout.write("held\n");await new Promise<void>(done=>{process.stdin.on("end",()=>done());process.stdin.resume();});lease.release();
});
