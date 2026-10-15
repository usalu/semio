import assert from"node:assert/strict";
import{join,dirname,basename}from"node:path";
import{realpathSync,readFileSync}from"node:fs";
import{pathToFileURL}from"node:url";
import{withResourceLeases,type ResourceLeaseControl}from"../../../../../../🔒️leases/🟦️.ts";
import{scriptInvocationEnvironment,requireIssuedResourceControlV1,requireIssuedScriptInvocationV1,type IssuedScriptInvocationV1}from"../../../🟦️.ts";
import{withRepositoryToolingStoreV1}from"../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{withNxToolingUseV1,nxToolingUseStorageV1}from"../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/📥️use/🟦️.ts";
import{nxAuthorizerReceiptHoldsV1}from"../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/📤️receipt/🟦️.ts";

/** 📦️ Checks actual Source-to-frozen callback identity while the exact SDK and tools image remain held. */
export async function proveFrozenOriginalCallbackV1(root:string,original:IssuedScriptInvocationV1):Promise<void>{
 await withRepositoryToolingStoreV1(root,original,store=>withResourceLeases({directory:join(root,".cache/b/l/m/data"),control:store.control,signal:store.control.signal,resources:[{resource:"nx-tooling-provision",mode:"shared"}]},async()=>{
  const directory=realpathSync(join(root,".nx/installation")),identity=basename(directory);assert.equal(dirname(directory),join(store.root,"nx"));assert.match(identity,/^[a-f0-9]{64}$/u);const marker=JSON.parse(readFileSync(join(directory,".semio-nx-tooling.json"),"utf8"));assert.equal(marker.digest,identity);assert.equal(nxAuthorizerReceiptHoldsV1(directory,marker.authorizers),true);
  await withNxToolingUseV1(nxToolingUseStorageV1(root),{identity,toolkitRoot:directory,invocation:original},async()=>{
   const frozen=await import(pathToFileURL(join(directory,"semio-authorizers.mjs")).href)as{withGraphPublisherControlV1:(environment:Readonly<Record<string,string|undefined>>,body:(control:ResourceLeaseControl)=>Promise<void>)=>Promise<void>},environment=scriptInvocationEnvironment(original,process.env),prior=process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=environment.SEMIO_SCRIPT_PROCESS_INVOCATION;
   try{await frozen.withGraphPublisherControlV1(environment,async control=>{console.log("[DEBUG] Actual frozen callback observation "+JSON.stringify({identity,sourceSha256:marker.authorizers.sourceSha256,sameSignal:control.signal===original.control.signal}));requireIssuedResourceControlV1(control);assert.equal(control.signal,original.control.signal);requireIssuedScriptInvocationV1(original);});console.log("[DEBUG] Actual Source-to-frozen SDK callback retains one private Original registry "+JSON.stringify({identity,sourceSha256:marker.authorizers.sourceSha256}));}finally{if(prior===undefined)delete process.env.SEMIO_SCRIPT_PROCESS_INVOCATION;else process.env.SEMIO_SCRIPT_PROCESS_INVOCATION=prior;}
  });
 }));
}
