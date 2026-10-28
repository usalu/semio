/** 📜️ One promoter process of the multi-process law: it resolves the sealed implementation of a fixture workspace, counts its captures in a shared file, may hold the lock for a while (`HOLD_MS`) and prints its verdict as one JSON line. */
import{appendFileSync}from"node:fs";
import{sealedEntryOfV1}from"../../🟦️.ts";
import{resolveSealedImplementationV1}from"../../📤️promotion/🟦️.ts";
import{fixtureCaptureV1,fixtureEntryV1,fixtureLoaderV1,labelOfTextV1,testPolicyV1}from"../🧫️fixtures/🟦️.ts";

const[root,counter]=process.argv.slice(2) as[string,string],hold=Number(process.env.HOLD_MS??0),entry=sealedEntryOfV1(root,fixtureEntryV1,testPolicyV1);
const base=fixtureCaptureV1(root,fixtureEntryV1,()=>appendFileSync(counter,process.pid+"\n"));
const capture=async(signal:AbortSignal)=>{const candidate=await base(signal);if(hold){appendFileSync(counter,"held "+process.pid+"\n");await new Promise(accept=>setTimeout(accept,hold));}return candidate;};
const result=await resolveSealedImplementationV1(root,entry,{capture,loader:process.env.LOADER==="1"?fixtureLoaderV1:undefined,policy:testPolicyV1,environment:{...process.env,SEMIO_LANGUAGE:"en"}});
console.log(JSON.stringify({pid:process.pid,status:result.status,label:labelOfTextV1(result.text),captures:result.captures}));
