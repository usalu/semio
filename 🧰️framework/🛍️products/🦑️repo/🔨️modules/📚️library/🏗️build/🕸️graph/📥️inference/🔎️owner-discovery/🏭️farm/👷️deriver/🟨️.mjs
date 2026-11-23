/** 👷️ The deriver of an owner discovery child: answers `owner-facts` requests with the admission facts of one owner and the rows its discovery observed, exactly what the parent would have observed itself; any refusal (another registry, another admission format, an owner failure) is the child's failure and the parent discovers the owner itself.
 * @see ../🟨️.mjs */
export async function openDeriverV1(workspaceRoot){
 const producer=await import("../../../🟨️.mjs"),{captureObservedSourceInputsV1}=await import("../../../../../👁️observation/🟨️.mjs"),{readBuildOwnerRegistryV1}=await import("../../../../🚪️admission/🟦️.ts"),{ownerRegistryDigestV1}=await import("../../../../🧾️registry-reads/🟨️.mjs"),{loadedGraphAdmissionFormatV1}=await import("../../../../🔐️authority/🟦️.ts");
 await producer.libraryBootstrap;producer.prepareSourceAnalysesV1();
 const refuse=(code,message)=>Object.assign(new Error(message),{code});
 let held,epoch,caches;
 const registryOf=digest=>{
  const source=producer.readBuildOwnerRegistrySourceV1(workspaceRoot,"current");
  if(held?.source!==source){const registry=readBuildOwnerRegistryV1(JSON.parse(source));held={source,registry,digest:ownerRegistryDigestV1(registry),owners:new Map(registry.owners.map(owner=>[owner.name,owner]))};}
  if(held.digest!==digest)throw refuse("OWNER_FARM_REGISTRY","The owner registry of the child differs from the registry of the parent");
  return held;
 };
 return async row=>{
  const request=JSON.parse(row.path);
  if(request.format!==loadedGraphAdmissionFormatV1)throw refuse("OWNER_FARM_FORMAT","The admission format of the child differs from the format of the parent");
  if(request.epoch!==epoch){epoch=request.epoch;caches={native:new Map(),closures:new Map()};}
  const {registry,owners}=registryOf(request.registryDigest),owner=owners.get(request.owner);
  if(owner===undefined)throw refuse("OWNER_FARM_OWNER","The owner is unknown to the child: "+request.owner);
  const {value,observations}=await captureObservedSourceInputsV1(()=>producer.discoverBuildOwnerAdmissionFactsV1(owner,request.targets,registry,{workspaceRoot,selection:request.selection,nativeCache:caches.native,nativeClosures:caches.closures,preparationControl:{advance:async()=>{}}},request.inputs),undefined,false);
  return {fact:{facts:value,rows:observations},parts:[]};
 };
}
