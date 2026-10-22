import {lstatSync,statfsSync} from "node:fs";
import {bindNativeStorageGeometryV1,type NativeStorageGeometryV1} from "../../📋️contract/🟦️.ts";

/** 📐️ Measures a kernel-mounted image: the volume answers for itself, so every figure comes from the mounted filesystem. */
export const mountedImageGeometryV1:NativeStorageGeometryV1={
 attached:(image,root)=>lstatSync(root).dev!==lstatSync(image).dev,
 filesystemBytes:root=>{const filesystem=statfsSync(root);return filesystem.blocks*filesystem.bsize;},
 usedBytes:root=>{const filesystem=statfsSync(root,{bigint:true});if(filesystem.bsize<1n||filesystem.blocks<filesystem.bfree||filesystem.bfree<filesystem.bavail||filesystem.bavail<0n)throw Error("Coherent filesystem statistics required");return Number((filesystem.blocks-filesystem.bfree)*filesystem.bsize);},
 space:root=>{const filesystem=statfsSync(root);return Object.freeze({bsize:filesystem.bsize,bavail:filesystem.bavail});}
};
bindNativeStorageGeometryV1("mounted-image",mountedImageGeometryV1);
