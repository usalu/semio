import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
/** 📐️ Captures exact dependency installation inputs separately from first-party SDK code. */
export function nxDependencyInstallFilesV1(recipe:unknown,lock:unknown):Map<string,Buffer>{
 if(validateJsonSchemaSubset(schema.definitions.Recipe,recipe,schema).length||!lock||typeof lock!=="object"||Array.isArray(lock))throw Error("Closed pinned dependency recipe required");
 const value=recipe as {name:string;private:true;packageManager:string;dependencies:Record<string,string>},captured={...lock as Record<string,unknown>};delete captured.patchedDependencies;
 const files=new Map([["package.json",Buffer.from(JSON.stringify({name:value.name,private:value.private,packageManager:value.packageManager,dependencies:value.dependencies})+"\n")],["bun.lock",Buffer.from(JSON.stringify(captured)+"\n")]]);
 if([...files.values()].reduce((total,bytes)=>total+bytes.byteLength,0)>schema.definitions.Protocol.const.maximumInputBytes)throw Error("Pinned dependency input byte ceiling exceeded");return files;
}
