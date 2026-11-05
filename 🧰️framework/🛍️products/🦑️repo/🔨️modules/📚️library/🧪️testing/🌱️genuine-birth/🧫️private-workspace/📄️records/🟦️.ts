import {join} from "node:path";
import {readNativeImageRetirementFileV1} from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📄️retirement-file/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
/** 📄️ Pins bounded regular single-owner birth records before receiver effects. */
export function readPrivateNativeRecordsV1(workspace:string):Readonly<{metadata:Uint8Array;owner:Uint8Array}>{const protocol=schema.definitions.Protocol.const;return Object.freeze({metadata:readNativeImageRetirementFileV1(join(workspace,protocol.metadataFile),protocol.maximumMetadataBytes).bytes,owner:readNativeImageRetirementFileV1(join(workspace,protocol.ownerFile),protocol.maximumOwnerBytes).bytes});}
