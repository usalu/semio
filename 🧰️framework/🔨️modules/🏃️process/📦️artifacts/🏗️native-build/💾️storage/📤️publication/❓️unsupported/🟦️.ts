import {createHash} from "node:crypto";
import {validateJsonSchemaSubset} from "../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
/** 📄️ Observes unsupported publication DATA without granting cleanup or accepting its receipt. */
export type NativeUnsupportedPublicationV1=Readonly<{version:1;kind:"unsupported-receipt-shape";directory:string;receiptStamp:string;receiptSha256:string}>;
/** 🔎️ Classifies a bounded physical receipt rejected by the current schema as DATA before Native open enrollment. */
export function observeUnsupportedNativeImagePublicationV1(directory:string,record:Readonly<{bytes:Uint8Array;stamp:string}>):NativeUnsupportedPublicationV1{
 if(record.bytes.byteLength>4096)throw Error("Bounded unsupported publication receipt required");const data={version:1 as const,kind:"unsupported-receipt-shape" as const,directory,receiptStamp:record.stamp,receiptSha256:createHash("sha256").update(record.bytes).digest("hex")};if(validateJsonSchemaSubset(schema,data).length)throw Error("Closed unsupported publication observation required");return Object.freeze(data);
}
