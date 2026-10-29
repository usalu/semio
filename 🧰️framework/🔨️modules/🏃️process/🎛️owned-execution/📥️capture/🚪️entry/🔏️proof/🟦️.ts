import{isAbsolute}from"node:path";
import{validateJsonSchemaSubset}from"../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
/** 🧬️ Authenticates a captured entry against the identities injected by the actual loader. */
export function requireCapturedRuntimeEntryProofV1(proof:Readonly<{version:1;snapshot:null;sha256:string;loaderSha256:string;root:string;recheck:()=>void}>,identity:unknown,loader:unknown):void{if(!proof||typeof proof.recheck!=="function"||validateJsonSchemaSubset(schema.definitions.Proof,{version:proof.version,snapshot:proof.snapshot,sha256:proof.sha256,loaderSha256:proof.loaderSha256,root:proof.root},schema).length||!isAbsolute(proof.root)||identity!==proof.sha256||loader!==proof.loaderSha256)throw Error("Authenticated captured runtime entry required");proof.recheck();}
