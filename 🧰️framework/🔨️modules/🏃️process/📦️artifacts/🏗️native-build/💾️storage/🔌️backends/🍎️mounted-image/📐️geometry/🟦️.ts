import {lstatSync,statfsSync} from "node:fs";
import {dirname,join} from "node:path";
import {captureOwnedCommandV1} from "../../../../../../🎛️owned-execution/🟦️.ts";
import {nativeImageBackingFilenameV1,nativeImageRootV1} from "../../../🌐️platform/📋️contract/🟦️.ts";
import {nativeImageInformationV1} from "../../../🍎️image/📥️information/🟦️.ts";
import {nativeImageHdiutilProgramV1} from "../../../🍎️image/🛠️hdiutil/🟦️.ts";
import {invalidateNativeImageRosterV1} from "../../../🍎️image/📡️roster/🟦️.ts";
import storageSchema from "../../../🧬️schema/🔣️.json" with {type:"json"};
import {bindNativeStorageGeometryV1,type NativeStorageGeometryV1} from "../../📋️contract/🟦️.ts";

/** 📐️ Measures a kernel-mounted image: the volume answers for itself, so every figure comes from the mounted filesystem. */
export const mountedImageGeometryV1:NativeStorageGeometryV1={
 attached:(image,root)=>lstatSync(root).dev!==lstatSync(image).dev,
 filesystemBytes:root=>{const filesystem=statfsSync(root);return filesystem.blocks*filesystem.bsize;},
 usedBytes:root=>{const filesystem=statfsSync(root,{bigint:true});if(filesystem.bsize<1n||filesystem.blocks<filesystem.bfree||filesystem.bfree<filesystem.bavail||filesystem.bavail<0n)throw Error("Coherent filesystem statistics required");return Number((filesystem.blocks-filesystem.bfree)*filesystem.bsize);},
 space:root=>{const filesystem=statfsSync(root);return Object.freeze({bsize:filesystem.bsize,bavail:filesystem.bavail});},
 invalidate:invalidateNativeImageRosterV1,
 mapped:async(directories,control)=>{
  const protocol=storageSchema.definitions.ImageProtocol.const,images=directories.map(directory=>join(directory,nativeImageBackingFilenameV1())),roots=directories.map(directory=>nativeImageRootV1(directory)),text=await captureOwnedCommandV1(nativeImageHdiutilProgramV1,["info","-plist"],dirname(directories[0]!),{control,output:{maximumBytes:protocol.informationMaximumBytes,maximumLines:protocol.informationMaximumLines}}),information=nativeImageInformationV1(text);
  if(!Array.isArray(information.images))return true;
  return (information.images as Record<string,unknown>[]).some(row=>images.includes(row["image-path"] as string)||Array.isArray(row["system-entities"])&&(row["system-entities"] as Record<string,unknown>[]).some(entity=>roots.includes(entity["mount-point"] as string)));
 }
};
bindNativeStorageGeometryV1("mounted-image",mountedImageGeometryV1);
