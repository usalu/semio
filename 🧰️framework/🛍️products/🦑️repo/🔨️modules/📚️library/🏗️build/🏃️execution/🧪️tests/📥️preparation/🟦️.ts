import {createHash} from "node:crypto";
import {readFileSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../📦️packages/🟦️typescript/🧬️schema/🟦️.ts";
import {advanceScriptInvocation,checkScriptInvocation,type ScriptInvocation} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureSourcePreparationV1,receiveSourcePreparationV1,observedFs} from "../../../👁️observation/🟨️.mjs";
import schema from "../../📥️preparation/🧬️schema/🔣️.json";

/** 🔬️ Loads the actual test-only inference implementation and checks every consumed DATA boundary. */
async function targetPreparationOracleV1(){
 const location=new URL("../../../🕸️graph/📥️inference/🟨️.mjs",import.meta.url),loaded:unknown=await import(location.href);
 if(!loaded||typeof loaded!=="object")throw Error("Actual inference test module required");const internals:unknown=Reflect.get(loaded,"cacheInternals");if(!internals||typeof internals!=="object")throw Error("Actual inference test API required");
 const call=(name:string,args:readonly unknown[]):unknown=>{const method:unknown=Reflect.get(internals,name);if(typeof method!=="function")throw Error("Actual inference method required: "+name);return Reflect.apply(method,internals,args);};
 const strings=(value:unknown):string[]=>{if(!Array.isArray(value)||value.length>131072||value.some(row=>typeof row!=="string"))throw Error("Finite actual manifest paths required");return value;};
 const rows=(value:unknown):Record<string,unknown>=>{if(!value||typeof value!=="object"||Array.isArray(value))throw Error("Actual inferred target DATA record required");const result:Record<string,unknown>={};for(const[key,row]of Object.entries(value))result[key]=row;return result;};
 return{revision:()=>{const value=call("implementationRevision",[]);if(typeof value!=="string"||!value||value.length>4096)throw Error("Actual inference revision required");return value;},call,strings,rows};
}

/** 📥️ Restricts preparation to the same original operation and explicitly bounded receiving phase. */
export function targetExecutionPreparationControlV1(invocation:ScriptInvocation,policy:unknown,now:()=>number){
 if(validateJsonSchemaSubset(schema,policy).length)throw Error("Complete target execution preparation policy required");
 const maximum=(policy as {maximumElapsedMilliseconds:number}).maximumElapsedMilliseconds,started=now();
 const remaining=()=>{checkScriptInvocation(invocation);const available=maximum-(now()-started),parent=invocation.control.remainingMilliseconds(),budget=parent===null?available:Math.min(parent,available);if(!Number.isFinite(budget)||budget<=0)throw Error("Original target execution preparation deadline exhausted");return budget;};
 return {signal:invocation.control.signal,remaining,check:()=>{remaining();},advance:async(stage:string)=>{
  const budget=remaining();let timer:ReturnType<typeof setTimeout>|undefined,remove!:()=>void;
  const interrupted=new Promise<never>((_,reject)=>{const abort=()=>reject(invocation.control.signal.reason);invocation.control.signal.addEventListener("abort",abort,{once:true});if(invocation.control.signal.aborted)abort();remove=()=>invocation.control.signal.removeEventListener("abort",abort);timer=setTimeout(()=>reject(Error("Original target execution preparation deadline exhausted")),budget);});
  try{await Promise.race([advanceScriptInvocation(invocation,`target-execution:${stage}`,"running"),interrupted]);remaining();}finally{clearTimeout(timer);remove();}
 }};
}

/** 🧾️ Prepares all actual emitted targets once and captures their original producer and source facts. */
export async function prepareTargetExecutionInventoryV1(root:string,output:string,control:ReturnType<typeof targetExecutionPreparationControlV1>){
 const capture=await captureSourcePreparationV1(async()=>{
  for(const path of ["./🟦️.ts","../🟦️.ts","../🧫️fixtures/🔣️.json","../../📥️preparation/🧬️schema/🔣️.json","../../../../📦️packages/🟦️typescript/📜️script.ts"])observedFs.readFileSync(new URL(path,import.meta.url));
  const oracle=await targetPreparationOracleV1(),producer=oracle.revision(),manifests=oracle.strings(await oracle.call("walkCargoTomlControlledV1",[root,control])),contributions=await oracle.call("workspaceCatalogBuildContributionsV1",[root]),catalog=oracle.call("collectPlaygroundCatalog",[manifests,root,contributions]);
  await control.advance("manifest-catalog");
  const developmentRoot="🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript",printRoot="🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript",printProject=JSON.parse(observedFs.readFileSync(join(root,printRoot,"📋️project.json"),"utf8"));
  const groups:Record<string,Record<string,unknown>>={root:oracle.rows(oracle.call("rootCommandTargets",[join(root,"📜️script.ts")])),sessions:oracle.rows(await oracle.call("playgroundSessionTargets",[manifests,root,contributions]))};
  await control.advance("sessions");groups.playgrounds=oracle.rows(await oracle.call("playgroundPreparationTargets",[manifests,root,developmentRoot,{},contributions,{preparationControl:control}]));await control.advance("playgrounds");groups.documents=oracle.rows(await oracle.call("printDocumentTargets",[printProject,printRoot,root]));
  for(const component of oracle.strings(oracle.call("componentCrateRoots",[manifests,root]))){await control.advance("component-sites");groups[`component:${component}`]=oracle.rows(await oracle.call("componentBuildTargets",[component,root]));const sites=oracle.rows(oracle.call("pluginSiteTargetsForCrate",[component,catalog,developmentRoot,root]));if(Object.keys(sites).length)groups[`sites:${component}`]=sites;}
  if(oracle.revision()!==producer)throw Error("Original target execution producer changed");
  return {producer,groups};
 },control);
 control.check();const bytes=JSON.stringify(capture);writeFileSync(output,bytes);return {capture,digest:createHash("sha256").update(bytes).digest("hex")};
}

/** 🔐️ Accepts the final receiver only while every actually consumed source fact and producer still holds. */
export async function verifyTargetExecutionInventoryV1(output:string,digest:string,control:ReturnType<typeof targetExecutionPreparationControlV1>){
 control.check();const bytes=readFileSync(output),capture=JSON.parse(bytes.toString());if(createHash("sha256").update(bytes).digest("hex")!==digest)throw Error("Original target execution inventory changed");
 const received=await receiveSourcePreparationV1(capture.original,control);
 const oracle=await targetPreparationOracleV1();if(oracle.revision()!==capture.value.producer)throw Error("Original target execution receiver producer changed");
 await control.advance("receiver-fingerprints");
 writeFileSync(output.replace(/\.json$/u,"-final.json"),JSON.stringify({producer:capture.value.producer,original:capture.original,received}));
}
