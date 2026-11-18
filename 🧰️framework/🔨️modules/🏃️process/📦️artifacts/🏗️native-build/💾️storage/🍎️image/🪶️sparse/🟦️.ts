import imageSchema from"../../🧬️schema/🔣️.json" with { type: "json" };
import schema from"./🧬️schema/🔣️.json" with { type: "json" };
import {refuseProcessV1} from "../../../../../📖️texts/🟦️.ts";
const protocol=schema.definitions.Protocol.const,image=imageSchema.definitions.ImageProtocol.const;
const size=(bytes:number)=>{if(!Number.isSafeInteger(bytes)||bytes<image.minimumFilesystemBytes||bytes%image.unitBytes!==0)throw refuseProcessV1("invalid","nativeBuild.image-size-invalid",{bytes,minimum:image.minimumFilesystemBytes,unit:image.unitBytes});return bytes;};
/** 🪶️ Names the bytes a fresh image occupies on the host before it grows: the seeded creation size, at most the initial extent. See https://ss64.com/mac/hdiutil.html */
export function nativeImageSparseInitialBytesV1(imageBytes:number):number{return Math.min(size(imageBytes),protocol.initialBytes);}
/** 🌱️ Builds the closed `hdiutil resize` arguments that stretch a seeded image to its declared size without writing a byte, or null when the creation size already is the declared size. */
export function nativeImageSparseGrowArgumentsV1(imageBytes:number,backing:string):readonly string[]|null{
 return nativeImageSparseInitialBytesV1(imageBytes)===imageBytes?null:Object.freeze([protocol.growVerb,protocol.growSizeFlag,String(imageBytes/image.unitBytes)+protocol.growUnit,backing]);
}
/** 🔁️ Runs the grow command again (0.5 s, 1 s, 2 s, 4 s apart) while `hdiutil` answers status 35 (`EAGAIN`: another image operation holds the backing for a moment) and fails with the last error otherwise. See https://ss64.com/mac/hdiutil.html */
export async function nativeImageGrowRetryV1(run:()=>Promise<void>,pause:(milliseconds:number)=>Promise<void>=milliseconds=>new Promise(done=>setTimeout(done,milliseconds))):Promise<void>{
 for(let attempt=0;;attempt++){try{return await run();}catch(error){if(attempt>=4||!/ exited with status 35 signal null/u.test(error instanceof Error?error.message:String(error)))throw error;await pause(500*2**attempt);}}
}
/** 🚪️ Bounds the free host bytes a creation needs: `hdiutil resize` refuses (status 22, "Invalid argument") any size beyond the free bytes of the host volume although the backing stays sparse, so the whole envelope plus the initial extent must be free. See https://ss64.com/mac/hdiutil.html */
export function nativeImageSparseAdmissionBytesV1(maximumPhysicalBytes:number):number{return maximumPhysicalBytes+protocol.initialBytes;}
