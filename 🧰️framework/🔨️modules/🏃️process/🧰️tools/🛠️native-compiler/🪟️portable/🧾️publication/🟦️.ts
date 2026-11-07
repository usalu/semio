import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json" with{type:"json"};

/** 🚦️ Decides only whether provisioning must take the exclusive path; no observation grants readiness. */
export function portableMsvcProvisionDecisionV1(input:unknown):"reader"|"exclusive"{if(validateJsonSchemaSubset(schema.definitions.ProvisionObservation,input,schema).length)throw Error("Closed compiler provision observation required");const row=input as{receipt:"absent"|"ordinary"|"other";pending:boolean};return row.receipt==="ordinary"&&!row.pending?"reader":"exclusive";}

/** 🧭️ Classifies publication observations without granting filesystem custody. */
export function portableMsvcRecoveryV1(value:Readonly<{intent:boolean;receipt:boolean;stage:boolean;published:boolean}>):"create"|"resume"|"finish"|"ready"|"refuse"{
 if(value.intent){if(value.stage&&!value.published&&!value.receipt)return "resume";if(!value.stage&&value.published)return "finish";return "refuse";}
 if(!value.stage&&!value.published&&!value.receipt)return "create";
 if(!value.stage&&value.published&&value.receipt)return "ready";
 return "refuse";
}

/** 🧾️ Compares receipt DATA only; neither a match nor its caller-supplied facts grants store custody. */
export function portableMsvcReceiptMatchesV1(input:unknown,expected:Readonly<{allocation:string;recipeSha256:string;inventorySha256:string;directory:Readonly<{device:string;inode:string}>}>):boolean{
 if(validateJsonSchemaSubset(schema.definitions.PublicationReceipt,input,schema).length)return false;
 const value=input as typeof expected&{version:1;owner:string;catalogSha256:string};
 return value.version===1&&value.owner===schema.definitions.Protocol.const.owner&&value.catalogSha256===schema.definitions.Protocol.const.manifestSha256&&value.allocation===expected.allocation&&value.recipeSha256===expected.recipeSha256&&value.inventorySha256===expected.inventorySha256&&value.directory.device===expected.directory.device&&value.directory.inode===expected.directory.inode;
}
