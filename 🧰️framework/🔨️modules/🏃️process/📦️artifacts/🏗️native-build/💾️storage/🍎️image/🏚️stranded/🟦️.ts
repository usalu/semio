import{refuseProcessV1}from"../../../../../📖️texts/🟦️.ts";
/** 🏚️ An image of the repository that the kernel lists as attached somewhere other than its canonical mount point: the whole-disk device to detach and the mount points it holds (none when it is attached without a mount). */
export type NativeImageStrandedV1=Readonly<{device:string;mountPoints:readonly string[]}>;
const record=(value:unknown):value is Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value);
/** 🔎️ Finds the image in one kernel inventory (`hdiutil info -plist`) by its exact path and tells whether it is attached at a foreign mount point: the image path ties the row to the image, its entities tie it to a mount. `undefined` when the image is not attached, is mounted at `root` or names no whole-disk device. Pure and language-agnostic.
 * @see ./🧪️tests/🟦️.ts */
export function nativeImageStrandedMountV1(information:Record<string,unknown>,image:string,root:string):NativeImageStrandedV1|undefined{
 const images=Array.isArray(information.images)?information.images:[],rows=images.filter(row=>record(row)&&row["image-path"]===image) as Record<string,unknown>[];
 if(rows.length!==1)return undefined;
 const entities=(Array.isArray(rows[0]!["system-entities"])?rows[0]!["system-entities"] as unknown[]:[]).filter(record),mountPoints=entities.map(entity=>entity["mount-point"]).filter((value):value is string=>typeof value==="string");
 const device=entities.map(entity=>entity["dev-entry"]).find((value):value is string=>typeof value==="string"&&/^\/dev\/disk[0-9]+$/u.test(value));
 if(device===undefined||mountPoints.includes(root))return undefined;
 return Object.freeze({device,mountPoints:Object.freeze(mountPoints)});
}
/** 🔌️ The effects of one recovery: the whole kernel inventory and the detach of one whole-disk device without force (a busy device refuses). */
export type NativeImageStrandedPortV1=Readonly<{inventory():Promise<Record<string,unknown>>;detach(device:string):Promise<void>}>;
/** ♻️ Recovers an image that a crashed or cancelled flow left attached at a foreign mount point (as the private scratch mount of a reclaim cycle): it detaches the exact device without force so the next opener attaches the image at its canonical mount point; a busy device refuses with a bilingual text that names the image, the mount points and the command. Returns what it detached, `undefined` when nothing was stranded. */
export async function recoverNativeImageStrandedV1(port:NativeImageStrandedPortV1,image:string,root:string):Promise<NativeImageStrandedV1|undefined>{
 const stranded=nativeImageStrandedMountV1(await port.inventory(),image,root);
 if(stranded===undefined)return undefined;
 try{await port.detach(stranded.device);}
 catch(error){throw refuseProcessV1("refused","nativeBuild.image-stranded",{image,device:stranded.device,mountPoints:stranded.mountPoints.join(", ")||"-",canonical:root},{cause:error});}
 return stranded;
}
