import{validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/🟦️.ts";
import{readBuildMode,type BuildMode}from"../🟦️.ts";
import sourceSchema from"./🧬️schema/🔣️.json"with{type:"json"};
const schema:typeof sourceSchema=JSON.parse(JSON.stringify(sourceSchema)),protocol=Object.freeze({...schema.definitions.Protocol.const});
/** 📥️ Inserts finite mode-matched inline selectors into Cargo's receiver before opaque program arguments; DATA issues no authority. */
export function appendCargoProfileConfigurationArgumentsV1(args:readonly string[],mode:BuildMode,selectors:readonly string[]):readonly string[]{
 mode=readBuildMode(mode);if(validateJsonSchemaSubset(schema.definitions.Arguments,args,schema).length||validateJsonSchemaSubset(schema.definitions.Selectors,selectors,schema).length||selectors.some(value=>!value.startsWith("profile."+mode+".")))throw Error("Exact finite Cargo profile selector DATA required");const end=args.indexOf("--"),result=[...(end<0?args:args.slice(0,end)),...selectors.flatMap(value=>["--config",value]),...(end<0?[]:args.slice(end))];if(validateJsonSchemaSubset(schema.definitions.Arguments,result,schema).length||result.some(value=>value.includes("\0")||Buffer.byteLength(value)>protocol.maximumArgumentBytes)||Buffer.byteLength(JSON.stringify(result))>protocol.maximumSerializedBytes)throw Error("Cargo profile argument bounds exceeded");return Object.freeze(result);
}
