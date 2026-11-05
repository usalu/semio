import{dirname,isAbsolute,join,resolve}from"node:path";
import type{CurrentPhysicalPortV1}from"@semio-tech/framework-filesystem/observation/current";
import type{RuntimeWasmObservationV1,RuntimeWasmRostersV1}from"../../../🧬️schema/🟦️.ts";
import{runtimeWasmProtocolV1,runtimeWasmPublicationMatchesV1}from"../🟦️.ts";
/** 📂️ Admits complete physical binding and mounted inventories under one cancellable observation owner. */
export async function runtimeWasmPhysicalRostersV1(receipt:RuntimeWasmObservationV1,physical:Pick<CurrentPhysicalPortV1,"checkpoint"|"read"|"directoryEntries">):Promise<RuntimeWasmRostersV1|undefined>{
 const binding=receipt.transformations[0]?.arguments.at(-1);if(!binding||!isAbsolute(binding)||!isAbsolute(receipt.outputDirectory))return;
 const marker=join(receipt.outputDirectory,runtimeWasmProtocolV1.publicationMarker),bytes=await physical.read(marker);if(bytes===undefined)return;let value:unknown;try{value=JSON.parse(typeof bytes==="string"?bytes:new TextDecoder("utf-8",{fatal:true}).decode(bytes));}catch{return;}
 if(!runtimeWasmPublicationMatchesV1(value,receipt.profile,receipt.outputDirectory,receipt.outputs))return;
 const files=async(root:string,metadata?:string):Promise<readonly string[]|undefined>=>{
  const pending=[root],result:string[]=[],seen=new Set<string>();let directories=0,entries=0;
  while(pending.length){await physical.checkpoint();const directory=pending.pop()!;if(seen.has(resolve(directory))||++directories>runtimeWasmProtocolV1.maximumDirectories)return;seen.add(resolve(directory));const rows=await physical.directoryEntries(directory);if(!rows)return;
   for(const row of rows){await physical.checkpoint();if(++entries>runtimeWasmProtocolV1.maximumEntries||!isAbsolute(row.path)||row.path.length>256||resolve(dirname(row.path))!==resolve(directory))return;if(row.kind==="directory")pending.push(row.path);else if(row.kind==="file"){if(row.path!==metadata)result.push(row.path);if(result.length>runtimeWasmProtocolV1.maximumFiles)return;}else return;}
  }
  return result.sort();
 };
 const bound=await files(binding);if(!bound)return;const published=await files(receipt.outputDirectory,marker);return published?{binding:bound,published}:undefined;
}
