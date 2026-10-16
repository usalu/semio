import{createHash}from"node:crypto";
import{join,isAbsolute}from"node:path";
import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import{readBuildMode,type BuildMode}from"@semio-tech/process/build-mode";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
export const testBrowserHostStageLimitsV1=Object.freeze({...schema.definitions.Policy.const});
/** 📋️ Names immutable selected Source DATA without granting storage or reader custody. */
export function readTestBrowserHostSelectedSourceV1(value:unknown){
 if(validateJsonSchemaSubset(schema,value).length||value===null||typeof value!=="object"||!("generationId" in value)||typeof value.generationId!=="string"||!("currentSha256" in value)||typeof value.currentSha256!=="string"||!("componentSha256" in value)||typeof value.componentSha256!=="string"||!("descriptorSha256" in value)||typeof value.descriptorSha256!=="string")throw Error("Exact browser host selected Source DATA required");
 return Object.freeze({generationId:value.generationId,currentSha256:value.currentSha256,componentSha256:value.componentSha256,descriptorSha256:value.descriptorSha256});
}
/** 📂️ Projects a fixed paid publication path from selected DATA, never a storage grant. */
export function testBrowserHostPublicationPlanV1(root:string,mode:BuildMode,value:unknown){
 const selected=readTestBrowserHostSelectedSourceV1(value),key=createHash("sha256").update(JSON.stringify([readBuildMode(mode),selected.generationId,selected.currentSha256,selected.componentSha256,selected.descriptorSha256])).digest("hex").slice(0,24),directory=join(root,"browser-host",mode,key,"browser-host");
 if(!isAbsolute(root)||directory.length>224)throw Error("Bounded exact browser host publication DATA required");
 return Object.freeze({key,selected,directory,owner:"test-browser-host:"+mode+":"+key});
}
