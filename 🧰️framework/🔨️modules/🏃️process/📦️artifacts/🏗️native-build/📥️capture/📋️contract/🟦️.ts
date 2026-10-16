import {validateJsonSchemaSubset} from "@semio-tech/schema-validator-ts";
import {readScriptProcessJsonV1} from "../../../../🧭️routing/📥️invocation/🏃️process/📥️json/🟦️.ts";
import {CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1} from "../../🧾️receipt/🟦️.ts";
import sourceSchema from "../../../📤️publication/📥️sources/🧬️schema/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
export interface CompletedCargoCaptureMetadataV1{readonly version:1;readonly owner:string;readonly files:readonly Readonly<{name:string;bytes:number;sha256:string}>[];readonly observation:Readonly<{bytes:number;sha256:string}>}
/** 🧮️ Copies finite completed-capture metadata as DATA while enforcing actual shared file/byte policy. */
export function readCompletedCargoCaptureMetadataV1(value:unknown,maximumObservationBytes:number):CompletedCargoCaptureMetadataV1{
 if(!Number.isSafeInteger(maximumObservationBytes)||maximumObservationBytes<1||maximumObservationBytes>CARGO_PROVENANCE_RECEIPT_MAX_BYTES_V1)throw Error("Finite observation extent required");const copied=readScriptProcessJsonV1({metadata:value}).metadata;if(validateJsonSchemaSubset(schema,copied).length)throw Error("Closed completed Cargo metadata required");const result=copied as unknown as CompletedCargoCaptureMetadataV1,names=new Set<string>();let total=0;for(const file of result.files){if(validateJsonSchemaSubset(sourceSchema.definitions.SourceName,file.name).length||names.has(file.name)||file.bytes>sourceSchema.properties.maximumBytes.const-total)throw Error("Finite distinct completed Cargo files required");names.add(file.name);total+=file.bytes;}if(result.observation.bytes>maximumObservationBytes)throw Error("Observation exceeds actual declared extent");return result;
}
