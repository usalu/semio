import {statfsSync} from "node:fs";
import {dirname} from "node:path";
import {nativeDirectoryProtocolV1,nativeDirectoryUsageV1,observeNativeDirectoryStateV1,readNativeDirectoryAnchorFileV1} from "../../../📁️directory/🟦️.ts";
import {nativeDirectoryUsedBytesSyncV1} from "../../../🌐️platform/📋️contract/📐️statfs/🟦️.ts";
import {bindNativeStorageGeometryV1,type NativeStorageGeometryV1} from "../../📋️contract/🟦️.ts";

const space=(root:string,declared:number,used:number)=>{
 const bsize=process.platform==="win32"?nativeDirectoryProtocolV1().exchange.windows.blockBytes:statfsSync(root).bsize;
 return Object.freeze({bsize,bavail:Math.max(0,Math.floor((declared-used)/bsize))});
};

/** 📐️ Measures a directory partition: the declared envelope is the capacity and the used bytes come from the bounded shared tree measurement. */
export const directoryPartitionGeometryV1:NativeStorageGeometryV1={
 attached:image=>observeNativeDirectoryStateV1(dirname(image))==="attached",
 filesystemBytes:(_root,declared)=>declared,
 usedBytes:root=>nativeDirectoryUsedBytesSyncV1(root,readNativeDirectoryAnchorFileV1(dirname(root)).maximumPhysicalBytes),
 space:(root,declared)=>space(root,declared,nativeDirectoryUsedBytesSyncV1(root,declared)),
 freshSpace:async(root,declared,signal)=>space(root,declared,await nativeDirectoryUsageV1(root,declared).fresh(signal)),
 invalidate:()=>{},
 mapped:async()=>false
};
bindNativeStorageGeometryV1("directory-partition",directoryPartitionGeometryV1);
