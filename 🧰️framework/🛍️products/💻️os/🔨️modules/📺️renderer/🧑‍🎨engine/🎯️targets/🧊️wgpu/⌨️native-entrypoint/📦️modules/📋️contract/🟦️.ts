import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import modelSchema from "../../../../../../../🔌️plugin/📦️model/🧬️schema/🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
export type NativeRuntimeManifestV1=Readonly<{version:1;variant:string;profile:"prototype"|"release";modules:readonly Readonly<{pluginId:string;wasmSha256:string;wasmPath:string;descriptorPath:string}>[]}>;
/** 📋️ Copies a closed self-contained runtime inventory; paths and receipts grant no custody. */
export function readNativeRuntimeManifestV1(value:unknown,variant:string,mode:"prototype"|"release"):NativeRuntimeManifestV1{
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed Native runtime manifest required");const row=value as NativeRuntimeManifestV1,seen=new Set<string>(),names=modelSchema.definitions.Protocol.const;
 if(row.variant!==variant||row.profile!==mode)throw Error("Exact Native variant and actual mode required");for(const item of row.modules){if(seen.has(item.pluginId)||item.wasmPath!==item.pluginId+"/"+names.component||item.descriptorPath!==item.pluginId+"/"+names.descriptor)throw Error("Distinct self-contained Native models required");seen.add(item.pluginId);}return Object.freeze({...row,modules:Object.freeze(row.modules.map(item=>Object.freeze({...item})))});
}
