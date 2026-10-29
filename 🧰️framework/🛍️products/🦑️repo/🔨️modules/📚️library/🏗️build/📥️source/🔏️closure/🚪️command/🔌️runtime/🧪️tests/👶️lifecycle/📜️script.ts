import{existsSync,mkdirSync}from"node:fs";
import{join}from"node:path";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1,requireIssuedResourceControlV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{openNativeFixedPlatformV1,withFixedNativeDisposableChildStorageV1,retireNativeFixedPlatformV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts";
import{withNativeStorageSelectionV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";
import{observeNativeDirectoryStateV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import{withRepositoryRuntimeEntriesV1}from"../../🟦️.ts";
import fixture from"./🔣️.json"with{type:"json"};
if(import.meta.main)await receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"test:full-fixed-disposable-lifecycle",90000),area=process.argv[2]!,leaseDirectory=join(area,"l");mkdirSync(leaseDirectory,{recursive:true});
 await withRepositoryRuntimeEntriesV1(process.cwd(),original,control,()=>withNativeStorageSelectionV1({backend:"directory-partition"},async()=>{
  const options={directory:join(area,"p"),leaseDirectory,...fixture.parent,control},parent=await openNativeFixedPlatformV1(options);let childAttached=false,childDirectory="",bodyControl:typeof control|undefined;
  await withFixedNativeDisposableChildStorageV1(parent,{directory:join(parent.root,"c"),leaseDirectory,...fixture.child,control},async(child,body,owner)=>{
   bodyControl=body;childDirectory=join(parent.root,"c");await child.recheck();await child.port.withOperation(body,async actual=>{requireIssuedResourceControlV1(actual);});childAttached=observeNativeDirectoryStateV1(childDirectory)==="attached";if(!childAttached||child.root===parent.root||!owner)throw Error("Actual distinct disposable image required");return undefined;
  });
  let bodyClosed=false;try{requireIssuedResourceControlV1(bodyControl!);}catch{bodyClosed=true;}
  const childRemoved=!existsSync(childDirectory);await parent.recheck();await parent.close();const retired=await retireNativeFixedPlatformV1(options,{version:1,wait:true}),parentRetired=retired&&!existsSync(options.directory);
  if(!childRemoved||!parentRetired)throw Error("Actual complete child and parent retirement required");console.log(JSON.stringify({version:1,platform:process.platform,childAttached,childRemoved,parentRetired,bodyClosed,moduleURL:import.meta.url}));
 }));
});
