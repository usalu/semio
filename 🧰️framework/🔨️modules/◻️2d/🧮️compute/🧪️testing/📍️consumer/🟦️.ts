/** 🔗️ Defines the compute family owned by a consumer witness. */
export type ComputeFamilyName="EngineKey"|"EngineHandle"|"EngineFault"|"Engine"|"EngineHandles"|"EngineCache"|"EngineRep";
/** 📍️ A caller-owned source and direct manifest binding. */
export interface ComputeConsumerDescriptor{version:1;id:string;source:string;manifest:string;bindings:readonly ComputeFamilyName[];}
/** 📖️ Reads only the caller-declared source coordinates. */
export interface ComputeConsumerAccess{read(path:string):string;dependencies(manifest:string):readonly string[];}
import schema from "../../🧬️schema/📍️consumer/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/🟦️.ts";
/** ✅️ Verifies direct canonical bindings without discovering any product owner. */
export function verifyComputeConsumer(descriptor:ComputeConsumerDescriptor,access:ComputeConsumerAccess):{directDependency:true;bindings:readonly ComputeFamilyName[]}{
 if(validateJsonSchemaSubset(schema,descriptor).length)throw Error("Invalid compute consumer descriptor");
 const source=access.read(descriptor.source);
 for(const name of descriptor.bindings)if(!source.includes("semio_framework_2d::compute::"+name))throw Error("Missing compute binding "+name);
 if(/\b(?:store|semio_framework_os_kernel)::Engine(?:Handles|Cache|Key|Handle|Fault|Rep)?\b|\bKernelEngineHandle\b/.test(source))throw Error("Foreign compute facade");
 if(!access.dependencies(access.read(descriptor.manifest)).includes("semio-framework-2d"))throw Error("Missing direct compute dependency");
 return {directDependency:true,bindings:[...descriptor.bindings]};
}
