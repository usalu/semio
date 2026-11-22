import {test} from "bun:test";
import {toolsProgressPaths} from "../../../🟦️.ts";
import {join} from "node:path";
import {writeFileSync} from "node:fs";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";

declareLibraryBuildTestEnvelopeV1({suite:"build-remote-execution",maximumElapsedMilliseconds:60000});
test.skipIf(!process.env.SEMIO_READY_TEST_PRODUCER)("private tools producer",()=>withBuildTestInvocationV1(async original=>{
 const root=process.env.SEMIO_TEST_ARTIFACT_DIR!, registry=process.env.SEMIO_LEASE_REGISTRY_DIRECTORY!;
 const paths=toolsProgressPaths(root,registry)!;
 const phase=createIssuedResourceControlV1(original,"repository:command",0);
 const acquire=(resource:string)=>acquireResourceLease({control:phase,signal:phase.signal,directory:paths.directory,resource,mode:"exclusive",timeoutMs:0,maximumDatabaseBytes:16384});
 const parent=await acquire(paths.parent),gate=await acquire(paths.source);
 const delay=Number(process.env.PRODUCER_DELAY??"5100");
 try{
  for(let n=1;n<10;n++){
   await phase.publish({resource:"native-artifact-input-read:"+(n*64),mode:"exclusive",elapsedMs:0});
   if(n===1)writeFileSync(join(root,"producer-ready"),String(process.pid));
   await Bun.sleep(delay);
  }
 }finally{gate.release();parent.release();}
}),55000);
