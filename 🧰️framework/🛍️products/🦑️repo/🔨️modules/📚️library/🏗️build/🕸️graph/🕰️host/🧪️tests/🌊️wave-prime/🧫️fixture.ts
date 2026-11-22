import {mock} from "bun:test";
import {generationHostV1} from "../../🟦️.ts";

const trace:string[]=[];
const heavy={...await import("../../../📥️inference/🧭️script-inputs/🪆️heavy-questions/🟨️.mjs")};
mock.module("../../../📥️inference/🧭️script-inputs/🪆️heavy-questions/🟨️.mjs",()=>({
 ...heavy,
 preflightToolClosuresInChildV1:async({signal}:{signal?:AbortSignal})=>{
  trace.push("preflight");
  signal?.throwIfAborted();
 },
 primeSharedClosuresInChildV1:async()=>{trace.push("prime");},
}));

const control={signal:new AbortController().signal};
const host=generationHostV1(process.cwd(),{control} as never,control as never,{catalogProviders:[],owners:[]} as never);
const waves=JSON.parse(process.argv[2]!);
const outcomes=[];
for(const wave of waves){
 const controller=new AbortController(),reason=Error("cancelled");
 if(wave==="cancelled")controller.abort(reason);
 try{
  await host.prime(controller.signal);
  outcomes.push("prepared");
 }catch(error){
  if(error!==reason)throw error;
  outcomes.push("cancelled");
 }
}
process.stdout.write("[DEBUG] "+JSON.stringify({outcomes,trace})+"\n");
