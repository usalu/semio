import{Writable}from"node:stream";
import fixture from"../../🧫️fixtures/🔣️.json";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{writeOwnedCommandOutputV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import{runScriptProgram as runPreparationProgram}from"../../../../🧪️tests/🔬️script-commands/🟦️.ts";

/** 🌊️ Observes genuine producer backpressure beneath the received nonrenewable Original. */
async function runScriptProgram():Promise<void>{
 await receiveScriptProcessInvocation(process.env,async original=>{const control=createIssuedResourceControlV1(original,"custody:output-fixture",0),write=process.stdout.write.bind(process.stdout);let backpressure=false,completed=false;const output=new Writable({highWaterMark:fixture.backpressureHighWaterMark,write(chunk,encoding,complete){write(chunk,encoding,error=>setTimeout(()=>{completed=true;complete(error);},fixture.backpressureDelayMilliseconds));}});process.stdout.write=(chunk:Uint8Array|string,encoding?:BufferEncoding|((error?:Error|null)=>void),callback?:(error?:Error|null)=>void)=>{const accepted=typeof encoding==="function"?output.write(chunk,encoding):output.write(chunk,encoding,callback);if(accepted===false)backpressure=true;return accepted;};try{await runPreparationProgram(control);}finally{process.stdout.write=write;}await writeOwnedCommandOutputV1(process.stderr,"[DEBUG] custody stdout backpressure="+backpressure+" completed="+completed+"\n",control);});
}
if(import.meta.main)await runScriptProgram();
