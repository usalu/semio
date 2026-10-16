import {createHash} from "node:crypto";
import {isAbsolute,join,resolve} from "node:path";
import schemaDocument from "./🧬️schema/🔣️.json";
import producerDocument from "../🧬️schema/🔣️.json";
import {requireRepositoryCompilerStoreV1,type RepositoryCompilerStoreV1} from "../../../🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {readScriptProcessJsonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/📥️json/🟦️.ts";
import {withRequestedIssuedResourceControlV1,withIssuedResourceOperationV1,type ScriptInvocation} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {withResourceLeases,checkResourceLeaseControl,type ResourceLeaseControl} from "../../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {readPhysicalCargoSourceV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
const schema=structuredClone(schemaDocument),protocol=Object.freeze({...producerDocument.definitions.Protocol.const});
export interface ObservedCargoConfigurationFileV1{readonly path:string;readonly sha256:string;readonly bytes:number}
export interface ObservedCargoConfigurationPlanV1 extends ObservedCargoConfigurationFileV1{readonly resource:string}

/** 🗺️ Projects exact existing configuration-file read shards as DATA, without configuration or compiler issuance. */
export function readObservedCargoConfigurationPlanV1(root:string,value:unknown):readonly ObservedCargoConfigurationPlanV1[]{
 const captured=readScriptProcessJsonV1({files:value}).files;if(!isAbsolute(root)||resolve(root)!==root||validateJsonSchemaSubset(schema,captured).length||!Array.isArray(captured))throw Error("Closed bounded observed Cargo configuration files required");const seen=new Set<string>();return Object.freeze(captured.map(row=>{if(row===null||typeof row!=="object"||Array.isArray(row)||typeof row.path!=="string"||typeof row.sha256!=="string"||typeof row.bytes!=="number")throw Error("Exact observed Cargo configuration claim required");const path=join(root,protocol.namespace,row.sha256,protocol.entry);if(row.path!==path||seen.has(path))throw Error("Exact unique immutable configuration path required");seen.add(path);return Object.freeze({path,sha256:row.sha256,bytes:row.bytes,resource:"cargo:configuration-view:"+(Number.parseInt(row.sha256.slice(0,8),16)%protocol.leaseShards)});}));
}

/** 👓️ Holds genuine existing registry/configuration readers through current observations and accepted drainage, without compiler ingress. */
export function withRepositoryObservedCargoConfigurationV1<T>(workspace:string,invocation:ScriptInvocation,store:RepositoryCompilerStoreV1,control:ResourceLeaseControl,files:readonly ObservedCargoConfigurationFileV1[],body:(control:ResourceLeaseControl,recheck:()=>Promise<void>)=>Promise<T>,settle:(control:ResourceLeaseControl)=>Promise<void>):Promise<T>{
 requireRepositoryCompilerStoreV1(store,workspace,invocation,control);const plans=readObservedCargoConfigurationPlanV1(store.root,files),home=store.environment.CARGO_HOME;if(!home)throw Error("Actual compiler registry home required");const verify=async(phase:ResourceLeaseControl)=>{requireRepositoryCompilerStoreV1(store,workspace,invocation,phase);await store.storage.recheck();for(const row of plans){checkResourceLeaseControl(phase);const bytes=await readPhysicalCargoSourceV1(row.path,{version:1,root:store.root,expectedBytes:row.bytes,maximumBytes:protocol.maximumViewBytes,remainingFiles:1},phase);if(!bytes||createHash("sha256").update(bytes).digest("hex")!==row.sha256)throw Error("Original observed configuration bytes changed");}checkResourceLeaseControl(phase);};
 return withResourceLeases({directory:join(workspace,".cache/b/l/m/data"),resources:["cargo:registry:"+home,...new Set(plans.map(row=>row.resource))].map(resource=>({resource,mode:"shared" as const})),control,signal:control.signal},async()=>{await verify(control);const value=await withRequestedIssuedResourceControlV1(control,0,async actual=>{await verify(actual);return body(actual,()=>withIssuedResourceOperationV1(actual,()=>verify(actual)));});await verify(control);await settle(control);await verify(control);return value;});
}
