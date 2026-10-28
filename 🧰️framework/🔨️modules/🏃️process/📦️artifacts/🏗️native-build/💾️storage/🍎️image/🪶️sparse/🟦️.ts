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
/** 🚪️ Bounds the free host bytes a creation needs: the envelope when small, else the admission floor, because the backing only grows with its content. */
export function nativeImageSparseAdmissionBytesV1(maximumPhysicalBytes:number):number{return Math.min(maximumPhysicalBytes,protocol.admissionFreeBytes)+67108864;}
