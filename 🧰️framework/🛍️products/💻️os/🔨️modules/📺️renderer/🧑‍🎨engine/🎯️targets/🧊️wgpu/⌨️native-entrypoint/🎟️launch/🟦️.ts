import { readIssuedScriptProcessEnvelopeV1, type ScriptInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json";
import admissionSchema from "./📨️admission/🧬️schema/🔣️.json";
/** 🎟️ The native launcher owns independent currencies before any startup or host I/O job exists. */
export type NativeLaunchGrantV1=Readonly<{maximumItems:number;maximumCopyBytes:number;maximumCapacityBytes:number;maximumReleaseBytes:number;maximumDepth:number}>;
export type NativeLaunchOperationPolicyV1=Readonly<{fuel:number;maximumElapsedMicroseconds:number;grant:NativeLaunchGrantV1;retirementReserve:Readonly<{fuel:number;grant:NativeLaunchGrantV1}>}>;
export type NativeLaunchDeclarationV1=Readonly<{version:1;owner:string;startup:NativeLaunchOperationPolicyV1;hostIo:NativeLaunchOperationPolicyV1;compilerTask:NativeLaunchOperationPolicyV1}>;
export type NativeLaunchAdmissionV1=Readonly<{version:1;declaration:NativeLaunchDeclarationV1;scriptOwner:string;scriptDeadlineEpochMilliseconds:number|null}>;
export const NATIVE_LAUNCH_ADMISSION_ENV="SEMIO_NATIVE_LAUNCH_ADMISSION";
/** 📨️ The required original Script owner and nullable absolute deadline cross the native process boundary explicitly. */
export function readNativeLaunchAdmissionV1(value:unknown):NativeLaunchAdmissionV1{
 if(validateJsonSchemaSubset(admissionSchema,value,admissionSchema,[schema]).length)throw Error("Complete native launch admission required");
 const admission=value as NativeLaunchAdmissionV1;readNativeLaunchDeclarationV1(admission.declaration);return admission;
}
/** 🧮️ The declared cleanup reserve is part of the original stock, never a later grant. */
export function readNativeLaunchDeclarationV1(value:unknown):NativeLaunchDeclarationV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Complete native launch declaration required");
 const declaration=value as NativeLaunchDeclarationV1;
 for(const policy of [declaration.startup,declaration.hostIo,declaration.compilerTask]){
  if(policy.retirementReserve.fuel>policy.fuel)throw Error("Native launch cleanup fuel exceeds original stock");
  for(const axis of Object.keys(policy.grant) as (keyof NativeLaunchGrantV1)[])if(policy.retirementReserve.grant[axis]>policy.grant[axis])throw Error("Native launch cleanup currency exceeds original stock");
 }
 return declaration;
}
/** ⏱️ Only elapsed time intersects the Script clock; retained currencies and fuel keep their own units. */
export function nativeLaunchDurationV1(policy:NativeLaunchOperationPolicyV1,deadlineEpochMilliseconds:number|null,nowEpochMilliseconds:number):number{
 if(!Number.isSafeInteger(nowEpochMilliseconds)||nowEpochMilliseconds<0)throw Error("Native launch original wall clock refused");
 if(deadlineEpochMilliseconds===null)return policy.maximumElapsedMicroseconds;
 if(!Number.isSafeInteger(deadlineEpochMilliseconds)||deadlineEpochMilliseconds<=nowEpochMilliseconds)throw Error("Native launch original Script deadline exhausted");
 const remaining=(deadlineEpochMilliseconds-nowEpochMilliseconds)*1000;
 if(!Number.isSafeInteger(remaining))throw Error("Native launch Script duration range refused");
 return Math.min(policy.maximumElapsedMicroseconds,remaining);
}

/** 📨️ Transfers the actual launch owner and original Script deadline without rebasing independent stock. */
export function nativeLaunchEnvironmentV1(invocation:ScriptInvocation,environment:NodeJS.ProcessEnv,declaration:NativeLaunchDeclarationV1,nowEpochMilliseconds:number):NodeJS.ProcessEnv{
 const envelope=readIssuedScriptProcessEnvelopeV1(invocation),original=readNativeLaunchDeclarationV1(declaration);
 nativeLaunchDurationV1(original.startup,envelope.deadlineEpochMilliseconds,nowEpochMilliseconds);
 nativeLaunchDurationV1(original.hostIo,envelope.deadlineEpochMilliseconds,nowEpochMilliseconds);
 nativeLaunchDurationV1(original.compilerTask,envelope.deadlineEpochMilliseconds,nowEpochMilliseconds);
 const admission:NativeLaunchAdmissionV1={version:1,declaration:original,scriptOwner:envelope.policy.owner,scriptDeadlineEpochMilliseconds:envelope.deadlineEpochMilliseconds};
 return{...environment,[NATIVE_LAUNCH_ADMISSION_ENV]:JSON.stringify(readNativeLaunchAdmissionV1(admission))};
}
