import schema from "../🧬️schema/🔣️.json" with {type:"json"};
const p=schema.definitions.Protocol.const,limits=Object.freeze({manifest:p.maximumManifestBytes,files:p.maximumFiles,entries:p.maximumEntries,bytes:p.maximumBytes});
export type ArtifactGroupInspectionKindV1="manifest"|"files"|"entries"|"bytes";
/** 📏️ Counts exact observed physical work without granting storage or reader authority. */
export interface ArtifactGroupInspectionBudgetV1{charge(kind:ArtifactGroupInspectionKindV1,count:number):void;snapshot():Readonly<Record<ArtifactGroupInspectionKindV1,number>>}
/** 🧮️ Creates fixed authored aggregate ceilings; caller per-artifact grants are never summed. */
export function createArtifactGroupInspectionBudgetV1():ArtifactGroupInspectionBudgetV1{const used={manifest:0,files:0,entries:0,bytes:0};return Object.freeze({charge(kind:ArtifactGroupInspectionKindV1,count:number):void{if(!Object.hasOwn(limits,kind)||!Number.isSafeInteger(count)||count<0||count>limits[kind]-used[kind])throw Error("Artifact group aggregate "+kind+" capacity exceeded");used[kind]+=count;},snapshot:()=>Object.freeze({...used})});}
