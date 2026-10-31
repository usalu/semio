import type{ScriptInvocation}from"@semio-tech/process/routing";
import{checkResourceLeaseControl,type ResourceLeaseControl}from"@semio-tech/process/leases";
import{repositoryCacheImagePublicationOptionsV1,prepareRepositoryDependencyBootstrapV1}from"../🟦️.ts";
import{observeRepositoryDependencyMaintenanceParentV1}from"../📸️observation/🟦️.ts";
import{retireObsoleteRepositoryDependencyGenerationsV1}from"./♻️retirement/🟦️.ts";
import{observeRepositoryDependencyBootstrapFundingV1,reconcileRepositoryDependencyBootstrapFundingV1}from"./💰️funding/🟦️.ts";
import{publishObservedNativePlatformImageV1}from"@semio-tech/process/artifacts/native-build/storage/publication/platform";
import{readNativeImageRetirementFileV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📄️retirement-file/🟦️.ts";
import{join}from"node:path";
import{lstatSync}from"node:fs";
import metadataSchema from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🧬️schema/🔣️.json"with{type:"json"};
/** 🛠️ Reconciles pending publication and existing dependency envelopes before ordinary setup reads. */
export async function setupRepositoryDependencyPublicationV1(invocation:ScriptInvocation,control:ResourceLeaseControl):Promise<void>{
 const options=repositoryCacheImagePublicationOptionsV1(invocation,control,"dependencies");checkResourceLeaseControl(control);
 let record:ReturnType<typeof readNativeImageRetirementFileV1>;try{record=readNativeImageRetirementFileV1(join(options.directory,"image.json"),4096);}catch(error){if((error as NodeJS.ErrnoException).code==="ENOENT")return;throw error;}
 checkResourceLeaseControl(control);const receipt=JSON.parse(record.bytes.toString()),currentEnvelope=receipt.filesystemBytes===options.filesystemBytes&&receipt.maximumPhysicalBytes===options.maximumPhysicalBytes;let pending=false;try{lstatSync(join(options.directory,metadataSchema.definitions.Slot.const.directory));pending=true;}catch(error){if((error as NodeJS.ErrnoException).code!=="ENOENT")throw error;}
 let funding=await observeRepositoryDependencyBootstrapFundingV1(invocation,control);if(funding.presence==="initial-empty"){if(!currentEnvelope||pending)throw Error("Only genuine current empty initialization may reach the existing allocator");checkResourceLeaseControl(control);return;}const currentMarker=funding.marker,transition=funding.state.allocations.find(row=>row.funding&&(row.funding.phase==="reserved"||row.id!==currentMarker?.allocation));
 if(transition||funding.markerIntent){if(!currentEnvelope)throw Error("Interrupted funding requires its exact current published declaration");if(pending){await publishObservedNativePlatformImageV1(options);pending=false;}await reconcileRepositoryDependencyBootstrapFundingV1(invocation,control);funding=await observeRepositoryDependencyBootstrapFundingV1(invocation,control);if(funding.presence!=="present")throw Error("Interrupted funding lost its actual committed association");}
 const row=funding.state.allocations[0];if(funding.state.allocations.length!==1||!row||row.kind!=="artifact-publication"||row.phase!=="committed"||row.child||row.namespace||row.owner!==funding.marker?.owner||row.id!==funding.marker?.allocation||row.bytes!==funding.marker?.bytes||row.bytes>options.filesystemBytes)throw Error("Actual retained Dependencies funding required before producer preparation");
 const parent=await observeRepositoryDependencyMaintenanceParentV1(invocation,control),prepared=await prepareRepositoryDependencyBootstrapV1(invocation,control,{version:1,purpose:"prepare-current-dependency-bootstrap",parent:parent.parent});if(!prepared||typeof prepared!=="object"||!("acquired"in prepared)||prepared.acquired!==true)throw Error("Genuine dependency bootstrap producer is busy");checkResourceLeaseControl(control);
 await retireObsoleteRepositoryDependencyGenerationsV1(invocation,control);checkResourceLeaseControl(control);
 if(!currentEnvelope||pending)await publishObservedNativePlatformImageV1(options);checkResourceLeaseControl(control);
 await reconcileRepositoryDependencyBootstrapFundingV1(invocation,control);checkResourceLeaseControl(control);
}
