/** 🔐️ Descriptive coordinates resolve only against a live native issuer and its original value custody. */
import {sameStagedFormToken,stagedFormToken,type StagedFormToken} from "./🟦️.ts";
export type StagedValuePurpose="draft"|"seed"|"variant"|"commit";
export interface StagedValueOrigin {
 readonly activation:string;
 readonly instance:string;
 readonly guestLifetime:string;
 readonly operation:string;
 readonly sourceGeneration:string;
 readonly serial:string;
 readonly controller:string;
 readonly token:StagedFormToken;
}
export interface StagedValueReference {readonly source:StagedValueOrigin;readonly slot:string;readonly purpose:StagedValuePurpose}
const originKeys=["activation","instance","guestLifetime","operation","sourceGeneration","serial","controller","token"] as const;
function record(value:unknown,keys:readonly string[]):Record<string,unknown>|undefined{
 if(!value||typeof value!=="object"||![Object.prototype,null].includes(Object.getPrototypeOf(value))||Reflect.ownKeys(value).length!==keys.length)return undefined;
 const result:Record<string,unknown>=Object.create(null);
 for(const key of keys){const field=Object.getOwnPropertyDescriptor(value,key);if(!field?.enumerable||!("value" in field))return undefined;result[key]=field.value;}
 return result;
}
function u64(value:unknown):value is string{return typeof value==="string"&&/^(0|[1-9][0-9]{0,19})$/u.test(value)&&(value.length<20||value<="18446744073709551615");}
function read(value:unknown):StagedValueReference|undefined{
 try{
  const reference=record(value,["source","slot","purpose"]);if(!reference||!u64(reference.slot)||!["draft","seed","variant","commit"].includes(reference.purpose as string))return undefined;
  const source=record(reference.source,originKeys);if(!source||typeof source.controller!=="string"||!source.controller)return undefined;
  for(const key of ["activation","instance","guestLifetime","operation","sourceGeneration","serial"])if(!u64(source[key]))return undefined;
  for(const key of ["activation","guestLifetime","serial"])if(source[key]==="0")return undefined;
  if(BigInt(source.instance as string)>4294967295n)return undefined;
  const token=record(source.token,["action","generation","revision","window"]);if(!stagedFormToken(token))return undefined;
  source.token=token;
  return {source:source as unknown as StagedValueOrigin,slot:reference.slot,purpose:reference.purpose as StagedValuePurpose};
 }catch{return undefined;}
}
/** 🧾️ Shape admission does not issue a native source or permit a command. */
export function stagedValueReference(value:unknown):value is StagedValueReference{return read(value)!==undefined;}
/** 🪪️ Exact lookup comparison preserves every original scope, counter, slot and purpose. */
export function sameStagedValueReference(left:unknown,right:unknown):boolean{
 const a=read(left),b=read(right);return !!a&&!!b&&a.slot===b.slot&&a.purpose===b.purpose&&sameStagedFormToken(a.source.token,b.source.token)&&originKeys.every(key=>key==="token"||a.source[key]===b.source[key]);
}
