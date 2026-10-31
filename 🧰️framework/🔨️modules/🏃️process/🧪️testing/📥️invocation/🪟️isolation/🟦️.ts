import {validateJsonSchemaSubset} from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 🪟️ Separates one authored body from its bounded prepared-file observation. */
export type IsolatedBuildTestPolicyV1=Readonly<{bodyMilliseconds:number;scope:Readonly<{kind:"body"}>|Readonly<{kind:"file";maximumElapsedMilliseconds:number}>}>;
/** 🧯️ Records an actual receiver failure without conferring execution authority. */
export type IsolatedBuildTestFailureV1=Readonly<{phase:"body"|"callback"|"fixture"|"cleanup"|"receiver";reason:string}>;
/** 🧾️ The receiver publishes its first failure and every independently observed settlement failure. */
export type IsolatedBuildTestPrimaryV1=Readonly<{version:1;test:string;overflow:boolean;failures:readonly IsolatedBuildTestFailureV1[]}>;
export const ISOLATED_BUILD_TEST_PROTOCOL_V1=Object.freeze({...schema.definitions.Protocol.const});

/** 🧭️ Reads the closed phase policy before registering any child. */
export function readIsolatedBuildTestPolicyV1(value:unknown):IsolatedBuildTestPolicyV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed isolated build test phase policy required");const policy=value as IsolatedBuildTestPolicyV1;
 if(policy.scope.kind==="file"&&policy.scope.maximumElapsedMilliseconds<policy.bodyMilliseconds)throw Error("Isolated body must fit its file scope");return Object.freeze({...policy,scope:Object.freeze({...policy.scope})});
}

/** ⏲️ Clamps observation to the original clock and the actual process's remaining file funding. */
export function isolatedBuildTestBudgetV1(value:unknown,processAgeMilliseconds:number,originalRemainingMilliseconds:number|null):number{
 const policy=readIsolatedBuildTestPolicyV1(value);if(!Number.isFinite(processAgeMilliseconds)||processAgeMilliseconds<0||originalRemainingMilliseconds!==null&&(!Number.isFinite(originalRemainingMilliseconds)||originalRemainingMilliseconds<=0))throw Error("Live isolated observation clocks required");
 const maximum=Math.floor(Math.min(policy.scope.kind==="file"?policy.scope.maximumElapsedMilliseconds-processAgeMilliseconds:policy.bodyMilliseconds,originalRemainingMilliseconds??Infinity));
 if(maximum<=0)throw Error("Original isolated file funding exhausted");return maximum;
}

/** 🔎️ Validates failure DATA received from the actual child's settled receiver. */
export function readIsolatedBuildTestPrimaryV1(value:unknown):IsolatedBuildTestPrimaryV1{
 if(validateJsonSchemaSubset({...schema.definitions.IsolatedBuildTestPrimaryV1,definitions:schema.definitions},value).length)throw Error("Closed isolated receiver failure required");return value as IsolatedBuildTestPrimaryV1;
}

/** 🎯️ Requires the settled primary and refuses independently failed cleanup. */
export function isolatedBuildTestPrimaryMatchesV1(value:unknown,name:string,rejection:RegExp):boolean{
 try{const record=readIsolatedBuildTestPrimaryV1(value),expected=new RegExp(rejection.source,rejection.flags.replace(/[gy]/gu,""));return record.test===name&&!record.overflow&&record.failures.every(row=>expected.test(row.reason))&&!record.failures.some((row,index)=>index>0&&row.phase==="cleanup");}catch{return false;}
}

/** ✂️ Retains bounded failure DATA and explicitly refuses lossy observations. */
export function isolatedBuildTestFailureReasonV1(error:unknown):Readonly<{reason:string;overflow:boolean}>{
 let reason:string;try{reason=String(error instanceof Error?error.message:error);}catch{return{reason:"Unprintable isolated failure",overflow:true};}
 return{reason:reason.slice(0,ISOLATED_BUILD_TEST_PROTOCOL_V1.maximumReasonCharacters),overflow:reason.length>ISOLATED_BUILD_TEST_PROTOCOL_V1.maximumReasonCharacters};
}
