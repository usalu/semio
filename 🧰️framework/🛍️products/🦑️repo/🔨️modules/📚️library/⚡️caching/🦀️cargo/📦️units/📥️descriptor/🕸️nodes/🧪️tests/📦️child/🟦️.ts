import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withRustUnitDescriptorGraphV2}from"../../🟦️.ts";
import { readRustUnitV1 } from "../../../../🧬️definition/🟦️.ts";
import{captureRustUnitInputV1}from"../../../../📋️plan/🟦️.ts";
import fixture from"../../../../🧪️tests/🧫️fixtures/🔣️.json";

/** 🏃️ Program of the sibling router: the former top-level body of its `📜️script.ts`, kept verbatim. */
export async function runScriptProgram(): Promise<void> {
  const kind=process.argv[2];if(!["cancel","cycle","graph-limit","input-copy"].includes(kind??""))throw new Error("Closed actual graph failure required");const signal=new AbortController(),primary=Error("Exact scoped graph cancellation");let failed=false,message="",callbacks=0,exact=false;
  try{await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"rust:graph:failure",0,signal.signal),input=Array.from({length:32},(_,index)=>{const unit=readRustUnitV1(structuredClone(fixture.units[0]!));unit.id="u"+index;unit.dependencies=kind==="cycle"&&index<2?[{unit:"u"+(1-index),externName:"previous",public:false,noprelude:false,artifactDigest:null}]:[];return captureRustUnitInputV1(unit);});if(kind==="input-copy")input[0]={...input[0]!};const pending=withRustUnitDescriptorGraphV2(input,"out",control,async()=>{callbacks++;},kind==="graph-limit"?1:undefined);if(kind==="cancel")queueMicrotask(()=>signal.abort(primary));await pending;});}catch(error){failed=true;message=error instanceof Error?error.message:String(error);exact=Object.is(error,primary);}console.log(JSON.stringify({failed,message,callbacks,cancelled:signal.signal.aborted,exact}));process.exitCode=failed?1:0;
}
