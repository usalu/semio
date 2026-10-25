import{dirname,join}from"node:path";
import type{NativeStorageStateV1}from"../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json" with{type:"json"};
/** 🧷️ Names how a mounted descendant of a parent image is accounted for by the parent's journal. */
export type NativeStorageDescendantKindV1="funded"|"persistent-state"|"unfunded";

/** 🧲️ Returns the reserved namespace directory below a parent root in which the persistent state owner mounts its unfunded children. */
export function nativeStoragePersistentNamespaceV1(parentRoot:string):string{return join(parentRoot,schema.definitions.ImageProtocol.const.persistentNamespace);}

/** 🧭️ Classifies one mounted descendant directory from the replayed parent journal alone: a funded child owns a `child` request, a persistent-state child lives directly in the reserved namespace while a live persistent-state charge exists, anything else is unfunded and poisons the parent. */
export function classifyNativeStorageDescendantV1(state:NativeStorageStateV1,parentRoot:string,directory:string):NativeStorageDescendantKindV1{
 if(state.allocations.some(row=>row.child?.directory===directory))return"funded";
 return dirname(directory)===nativeStoragePersistentNamespaceV1(parentRoot)&&state.allocations.some(row=>row.kind==="persistent-state"&&!row.child&&row.phase!=="retiring")?"persistent-state":"unfunded";
}
