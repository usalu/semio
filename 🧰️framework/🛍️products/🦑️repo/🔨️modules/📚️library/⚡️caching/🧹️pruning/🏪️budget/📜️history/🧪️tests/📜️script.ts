import {join,resolve} from "node:path";
import {receiveScriptProcessInvocation} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "@semio-tech/process/compiler-capacity";
import {withCompilerWriterCustodyV1} from "../../../../💾️storage/🔒️enforcement/🏗️compiler/📏️capacity/🟦️.ts";
import {withHistoricalCargoImageMaintenanceV1} from "../🟦️.ts";
import {retireInactiveCargoStoreV1} from "../../⚙️execution/🟦️.ts";
if(process.argv.length!==2)throw new Error("Exact detached historical failure fixture required");
await receiveScriptProcessInvocation(process.env,async invocation=>{
 const root=resolve(import.meta.dir,"../../../../../../../../../.."),mode=process.env.SEMIO_BUILD_MODE,control=scriptCompilerAdmissionControlV1(invocation,"test:accepted-historical-failure",120000),leaseDirectory=join(root,".cache/b/l/m/data");if(mode!=="prototype"&&mode!=="release")throw new Error("Exact inherited historical test mode required");
 await withHistoricalCargoImageMaintenanceV1(root,invocation,control,{version:1,mode,preview:true},async(image,buildDirectory,targetDirectory,heldControl)=>{
  let entered!:()=>void;const ready=new Promise<void>(accept=>entered=accept);
  void withCompilerWriterCustodyV1(image,leaseDirectory,heldControl,{version:1,wait:false},async()=>{
   void retireInactiveCargoStoreV1({workspace:root,invocation,control:heldControl,buildDirectory,relatedDirectories:[targetDirectory],leaseDirectory,artifactDirectory:join(root,".unissued-historical-custody"),preview:true,wait:false,policy:{version:1,retainedBytes:0,targetAvailableBytes:0,maximumDeletionBytes:0}}).catch(()=>{});
   entered();
  }).catch(()=>{});
  await ready;
 });
 throw new Error("Detached historical retirement failure was silently lost");
});
