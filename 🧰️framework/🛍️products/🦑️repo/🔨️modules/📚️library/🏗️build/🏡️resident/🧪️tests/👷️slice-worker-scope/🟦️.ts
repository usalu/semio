import {expect,test} from "bun:test";
import {readFileSync} from "node:fs";
const worker=readFileSync(new URL("../../🏗️composition/🌐️host/👷️slice-pool/👷️worker.ts",import.meta.url),"utf8");
test("the slice worker loads the graph inside the Native runtime entry scope, because an unmounted fixed storage captures its helper through that provider",()=>{
 const scope=worker.indexOf("withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control"),load=worker.indexOf("loadBuildInvocationGraphV1(workerData.root");
 expect(scope).toBeGreaterThan(-1);
 expect(load).toBeGreaterThan(scope);
});

test("the slice worker is committed to the code it loaded: it opts into the process-wide pin before it imports anything of the build and runs its whole load inside an implementation pin, so an edit of a loaded Source by a peer never ends a running load with the restart code",()=>{
 const pin=worker.indexOf("useProcessWideImplementationPinV1();"),firstImport=worker.indexOf('await import("../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts")'),scope=worker.indexOf("withImplementationPinV1(()=>routing.receiveScriptProcessInvocation("),load=worker.indexOf("loadBuildInvocationGraphV1(workerData.root");
 expect(pin).toBeGreaterThan(-1);expect(firstImport).toBeGreaterThan(pin);
 expect(scope).toBeGreaterThan(pin);expect(load).toBeGreaterThan(scope);
 expect(worker).toContain('📥️inputs/🔏️revision/🟨️.mjs');
});

import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import corpus from "./🔣️.json" with { type: "json" };
import { withResidentSliceNativeRecheckV1 } from "../../🏗️composition/🌐️host/👷️slice-pool/🪟️recheck/🟦️.ts";
import { createNativePhysicalGateV1, nativeRecheckWindowV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/📡️roster/🟦️.ts";
import { createIssuedResourceControlV1, createScriptProcessEnvelope, withScriptProcessEnvelope } from "../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import { checkResourceLeaseControl } from "../../../../../../../../🔨️modules/🏃️process/🔒️leases/🎛️control/🟦️.ts";

test("the resident worker restores the Native window only inside the issued graph load",()=>{
 const scope=worker.indexOf("withRepositoryCompilerRuntimeEntriesV1(workerData.root,original,control"),window=worker.indexOf("withResidentSliceNativeRecheckV1(original,control"),load=worker.indexOf("loadBuildInvocationGraphV1(workerData.root");
 expect(scope).toBeGreaterThan(-1);
 expect(window).toBeGreaterThan(scope);
 expect(load).toBeGreaterThan(window);
});

test("interleaved issued slices each recheck Native identity, reuse duplicate probes, and settle cancellation separately",async()=>{
 const width=corpus.windowMilliseconds,observations:string[]=[],turns=Array.from({length:corpus.steps.length+1},()=>Promise.withResolvers<void>()),ids=["first","second"],clock={now:0};
 const outside=ids.map(()=>createNativePhysicalGateV1(()=>clock.now));
 for(const row of corpus.steps)if(row.action==="verify"){
  clock.now=row.at;
  await outside[ids.indexOf(row.request)]!.run(nativeRecheckWindowV1(),()=>undefined,async()=>{});
 }
 const gates=ids.map(()=>createNativePhysicalGateV1(()=>clock.now)),cheap=[0,0];
 const issuances=ids.map((id,index)=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:slice-window",maximumElapsedMilliseconds:0},{request:id},Date.now()),async original=>{
  const abort=new AbortController(),stop=Error("fixture withdrawal"),control=createIssuedResourceControlV1(original,"fixture:slice-window",0,abort.signal);
  try{
   await withResidentSliceNativeRecheckV1(original,control,async()=>{
    for(const [at,row] of corpus.steps.entries())if(row.request===id){
     try{
      await turns[at]!.promise;
      expect(nativeRecheckWindowV1()).toBe(width);
      clock.now=row.at;
      if(row.action==="cancel")abort.abort(stop);
      try{
       checkResourceLeaseControl(control);
       await gates[index]!.run(nativeRecheckWindowV1(),()=>{cheap[index]++;if(row.action==="replace-identity")throw Error("identity");},async()=>{if(row.action==="change-mapping")throw Error("mapping");});
       observations[at]="verified";
      }catch(error){observations[at]=error===stop?"cancelled":(error as Error).message;if(error===stop)throw error;}
     }finally{turns[at+1]!.resolve();}
    }
   });
  }catch(error){expect(error).toBe(stop);throw error;}
  expect(nativeRecheckWindowV1()).toBe(0);
 }));
 turns[0]!.resolve();
 const settled=await Promise.allSettled(issuances);
 expect(settled.map(row=>row.status)).toEqual(["rejected","fulfilled"]);
 expect(nativeRecheckWindowV1()).toBe(0);
 const found={baseline:outside.map(gate=>gate.fulls()),observations,probes:gates.map(gate=>gate.fulls()),cheap};
 const oracle=spawnSync(process.platform==="win32"?"python":"python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url))],{input:JSON.stringify(corpus),encoding:"utf8"});
 expect(oracle.status).toBe(0);
 expect(found).toEqual(corpus.expected);
 expect(found).toEqual(JSON.parse(oracle.stdout));
});
