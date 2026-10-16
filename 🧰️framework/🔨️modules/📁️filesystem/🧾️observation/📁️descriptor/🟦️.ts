import {constants} from "node:fs";
import exportedSchema from"./🧬️schema/🔣️.json";
const protocol=Object.freeze({...exportedSchema.definitions.Protocol.const});
type Scope={descriptor?:unknown;unresolved:boolean;failure?:unknown};
const scopes=new Set<Scope>();
/** 🧬️ Reports finite private descriptor policy without issuing or adopting a handle. */
export function artifactDescriptorProtocolV1():typeof protocol{return protocol;}
/** 📊️ Observes counts only; no path, numeric descriptor or release operation escapes. */
export function artifactDescriptorCustodyV1():Readonly<{active:number;unresolved:number}>{let unresolved=0;for(const scope of scopes)if(scope.unresolved)unresolved++;return Object.freeze({active:scopes.size-unresolved,unresolved});}
/** 🛑️ Ambiguous close only denies destructive work; absence never proves Native ownership or kernel closure. */
export function requireArtifactDescriptorClosureV1():void{for(const scope of scopes)if(scope.unresolved)throw Error("Unresolved original descriptor close retained",{cause:scope.failure});}
/** 📁️ Opens a final component without following links where the platform exposes that flag. */
export function artifactReadFlagsV1():number{return constants.O_RDONLY|(constants.O_NOFOLLOW??0);}
/** 🔒️ Reserves before open and retains the exact object after one ambiguous close, preserving the body's primary. */
export async function withArtifactDescriptorV1<D extends{close():Promise<void>},T>(open:()=>Promise<D>,body:(descriptor:D)=>Promise<T>):Promise<T>{if(typeof open!=="function"||typeof body!=="function"||scopes.size>=protocol.maximumOwners)throw Error("Finite descriptor custody unavailable before open");const scope:Scope={unresolved:false};scopes.add(scope);let descriptor:D;try{descriptor=await open();scope.descriptor=descriptor;}catch(error){scopes.delete(scope);throw error;}let close:D["close"];try{close=descriptor.close;if(typeof close!=="function")throw Error("Actual opened descriptor close required");}catch(error){scope.unresolved=true;scope.failure=error;throw error;}let outcome:{value:T}|{error:unknown};try{outcome={value:await body(descriptor)};}catch(error){outcome={error};}try{await close.call(descriptor);scopes.delete(scope);}catch(error){scope.unresolved=true;scope.failure=error;if("value"in outcome)throw error;}if("error"in outcome)throw outcome.error;return outcome.value;}
