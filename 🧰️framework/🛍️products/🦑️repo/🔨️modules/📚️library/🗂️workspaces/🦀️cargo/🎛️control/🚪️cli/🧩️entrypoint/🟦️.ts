import limits from "./🎛️policy/🔣️.json";
import {CargoCliOperationOwner,parseCargoCliPolicy,type CargoCliPolicy} from "../🟦️.ts";
import {parseCargoCommandScope,type CargoCommandScope} from "../../../📤️arguments/🟦️.ts";

export type CargoCommandEntryPolicyV1=Readonly<{version:1;scope:CargoCommandScope;control:CargoCliPolicy;maximumElapsedMilliseconds:number}>;
/** 🧬️ Admits authored command scope and positive finite time without enlarging original work or ownership grants. */
export function parseCargoCommandEntryPolicyV1(value:unknown):CargoCommandEntryPolicyV1 {
 if(!value||typeof value!=="object"||Array.isArray(value)||Object.keys(value).sort().join(",")!=="control,maximumElapsedMilliseconds,scope,version")throw Error("Complete Cargo command entry policy required");const row=value as Record<string,unknown>,maximumElapsedMilliseconds=row.maximumElapsedMilliseconds;if(row.version!==1||typeof maximumElapsedMilliseconds!=="number"||!Number.isSafeInteger(maximumElapsedMilliseconds)||maximumElapsedMilliseconds<=0||maximumElapsedMilliseconds>86400000)throw Error("Finite Cargo command deadline required");const control=parseCargoCliPolicy(row.control),scope=parseCargoCommandScope(row.scope);for(const phase of ["discovery","retirement"] as const)for(const key of ["maximumUnits","maximumOwnedBytes","maximumDepth"] as const)if(control[phase][key]>limits[phase][key])throw Error("Cargo command grant exceeds its original owner policy");return{version:1,scope,control,maximumElapsedMilliseconds};
}
/** 🚪️ Owns actual command time, observable progress, cooperative yielding and separately funded system-owner retirement. */
export class CargoCommandEntryOwnerV1 {
 readonly policy:CargoCommandEntryPolicyV1;readonly cli:CargoCliOperationOwner;private readonly started:number;
 constructor(policy:unknown,diagnostic:(line:string)=>void){
  this.policy=parseCargoCommandEntryPolicyV1(policy);if(typeof diagnostic!=="function")throw Error("Cargo command requires its actual diagnostic port");this.started=performance.now();this.cli=new CargoCliOperationOwner(this.policy.control,{remainingMilliseconds:()=>this.remainingMilliseconds(),publish:progress=>diagnostic("[cargo-command] "+JSON.stringify(progress)),yieldContinuation:()=>new Promise<void>(accept=>setImmediate(accept))});
 }
 remainingMilliseconds():number{return Math.max(0,this.policy.maximumElapsedMilliseconds-(performance.now()-this.started));}
}
