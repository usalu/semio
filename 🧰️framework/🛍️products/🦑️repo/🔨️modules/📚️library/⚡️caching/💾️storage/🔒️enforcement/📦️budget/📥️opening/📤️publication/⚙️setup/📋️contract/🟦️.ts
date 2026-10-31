import{types}from"node:util";
import schema from"../🧬️schema.json"with{type:"json"};
import nativeSchema from"@semio-tech/process/artifacts/native-build/storage/schema.json";
import{readNativeProtectedMaintenanceRequestV1,type NativeProtectedMaintenanceRequestV1}from"@semio-tech/process/artifacts/native-build/storage/platform/contract";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
/** 🧾️ Captures a distinct genuine bootstrap preparation purpose without admitting caller-selected funding. */
export function readRepositoryDependencyBootstrapPreparationV1(value:unknown):Readonly<{version:1;purpose:"prepare-current-dependency-bootstrap";parent:NativeProtectedMaintenanceRequestV1}>{
 if(!value||typeof value!=="object"||types.isProxy(value)||![Object.prototype,null].includes(Object.getPrototypeOf(value)))throw Error("Own closed bootstrap preparation DATA required");const captured:Record<string,unknown>=Object.create(null);for(const key of Reflect.ownKeys(value)){const field=Object.getOwnPropertyDescriptor(value,key);if(typeof key!=="string"||!field?.enumerable||!("value"in field))throw Error("Own primitive bootstrap preparation fields required");captured[key]=field.value;}if(validateJsonSchemaSubset(schema.definitions.PreparationRequest,captured,schema,[nativeSchema]).length)throw Error("Closed distinct bootstrap preparation request required");return Object.freeze({version:1,purpose:"prepare-current-dependency-bootstrap",parent:readNativeProtectedMaintenanceRequestV1(captured.parent)});
}

/** 📄️ Reads a bounded exact allocation marker without granting funding authority. */
export function readRepositoryDependencyAllocationMarkerV1(text:string):Readonly<{version:1;allocation:string;owner:"repository:dependencies-store";bytes:number}>{if(typeof text!=="string"||Buffer.byteLength(text)>2048)throw Error("Bounded Dependencies allocation marker required");const value=JSON.parse(text);if(validateJsonSchemaSubset(schema.definitions.AllocationMarker,value,schema).length)throw Error("Closed Dependencies allocation marker required");return Object.freeze({version:1,allocation:value.allocation,owner:value.owner,bytes:value.bytes});}
