import{validateJsonSchemaSubset}from"@semio-tech/schema-validator-ts";
import schemaData from"./🧬️schema/🔣️.json"with{type:"json"};
const schema:typeof schemaData=structuredClone(schemaData);
/** 📋️ Captures a finite evidence filename and size without granting a destination. */
export function printPipelineEvidenceV1(value:unknown):Readonly<{version:1;name:string;bytes:number}>{if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed finite Print evidence required");const row=value as{version:1;name:string;bytes:number};if(row.name.split("/").some(part=>!part||part==="."||part===".."||part.endsWith(".")||/^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/iu.test(part)))throw Error("Portable relative Print evidence required");return Object.freeze({version:1,name:row.name,bytes:row.bytes});}
/** 📐️ Copies immutable producer ceilings; these DATA facts grant no storage or work authority. */
export function printPipelineEvidencePolicyV1():Readonly<typeof schemaData.definitions.Policy.const>{return Object.freeze({...schema.definitions.Policy.const});}
