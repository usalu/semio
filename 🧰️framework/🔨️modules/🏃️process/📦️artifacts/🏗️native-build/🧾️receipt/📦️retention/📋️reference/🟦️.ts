import {isAbsolute,resolve} from "node:path";
import {readScriptProcessJsonV1} from "../../../../../🧭️routing/📥️invocation/🏃️process/📥️json/🟦️.ts";
import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import schemaDocument from "./🧬️schema/🔣️.json";
export interface CompletedCargoPublicationReferenceV1{readonly version:1;readonly mode:"prototype"|"release";readonly kind:"component"|"emitter";readonly pass:string;readonly receiptSha256:string;readonly indexSha256:string;readonly originalArtifact:Readonly<{path:string;sha256:string;byteLength:number}>|null}
const schema=structuredClone(schemaDocument);
/** 📋️ Reads complete packet reference DATA; only a genuine publication reader can give it custody. */
export function readCompletedCargoPublicationReferenceV1(value:unknown):CompletedCargoPublicationReferenceV1{
 const captured=readScriptProcessJsonV1({reference:value}).reference;
 if(validateJsonSchemaSubset(schema,captured).length)throw Error("Closed completed Cargo publication reference required");const row=captured as unknown as CompletedCargoPublicationReferenceV1;if(row.originalArtifact&&(!isAbsolute(row.originalArtifact.path)||resolve(row.originalArtifact.path)!==row.originalArtifact.path))throw Error("Exact original compiler output path required");if(row.kind==="emitter"&&!row.originalArtifact)throw Error("Actual descriptor emitter binding required");return Object.freeze({version:1,mode:row.mode,kind:row.kind,pass:row.pass,receiptSha256:row.receiptSha256,indexSha256:row.indexSha256,originalArtifact:row.originalArtifact?Object.freeze({path:row.originalArtifact.path,sha256:row.originalArtifact.sha256,byteLength:row.originalArtifact.byteLength}):null});
}
