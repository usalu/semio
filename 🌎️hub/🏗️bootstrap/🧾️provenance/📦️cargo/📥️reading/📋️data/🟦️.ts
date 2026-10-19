import {createHash} from "node:crypto";
import {readCompletedCargoPublicationReferenceV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/reference";
import {readCompletedCargoRetainedIndexV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/reading";
import {completedCargoOutputClaimsV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/inventory";
import {decodeCargoProvenanceV1} from "@semio-tech/process/native-receipt";
import { runtimeCargoProvenanceV1 } from "@semio-tech/repo-lib/inventory";
/** 🧾️ Validates packet DATA without issuing physical custody or granting workspace reuse. */
export function readTrustedCargoPacketDataV1(indexBytes:Uint8Array,receiptBytes:Uint8Array,workspace:string,value:unknown,inventory:readonly string[]){
 const reference=readCompletedCargoPublicationReferenceV1(value),digest=(bytes:Uint8Array)=>createHash("sha256").update(bytes).digest("hex");
 if(digest(indexBytes)!==reference.indexSha256||digest(receiptBytes)!==reference.receiptSha256)throw Error("Unchanged authenticated compilation packet required");
 const index=readCompletedCargoRetainedIndexV1(indexBytes,receiptBytes,workspace,reference.mode),text=new TextDecoder("utf-8",{fatal:true}).decode(receiptBytes),records=runtimeCargoProvenanceV1(text),record=records[0];if(records.length!==1||!record)throw Error("One exact original completed Cargo record required");
 const expected=["receipt.json","index.json",...new Set(index.files.map(row=>row.path))].sort();if(JSON.stringify([...inventory].sort())!==JSON.stringify(expected))throw Error("Complete exact published compiler packet inventory required");
 const binding=reference.originalArtifact;if(binding){const outputs=completedCargoOutputClaimsV1(decodeCargoProvenanceV1(text),index.origin).filter(row=>row.path===binding.path&&row.sha256===binding.sha256),mapped=index.files.filter(row=>row.source===binding.path&&row.sha256===binding.sha256&&row.bytes===binding.byteLength);if(outputs.length!==1||mapped.length!==1)throw Error("Original output binding has no exact completed producer");}
 return Object.freeze({index,observation:record});
}
