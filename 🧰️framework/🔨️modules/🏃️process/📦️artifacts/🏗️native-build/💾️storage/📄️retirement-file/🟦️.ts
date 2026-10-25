/** 📖️ Reads one bounded native image retirement file through a pinned descriptor and returns its bytes with its identity stamp; the storage core and both retirement backends read receipts and claim sidecars through it, so it sits below all of them. */

import { lstatSync, readFileSync, openSync, closeSync, constants, fstatSync } from "node:fs";
import { resolve, dirname } from "node:path";

/** 📄️ Reads a bounded single-owner retirement record through the same no-follow descriptor and stable identity. */
export function readNativeImageRetirementFileV1(path:string,maximumBytes:number):Readonly<{bytes:Buffer;stamp:string}>{
 let ancestor=dirname(resolve(path));for(;;){const row=lstatSync(ancestor);if(!row.isDirectory()||row.isSymbolicLink())throw Error("Original image retirement ancestry required");const parent=dirname(ancestor);if(parent===ancestor)break;ancestor=parent;}const stamp=(row:NonNullable<ReturnType<typeof lstatSync>>)=>[row.dev,row.ino,row.nlink,row.size,row.mtimeMs,row.ctimeMs].join(":"),before=lstatSync(path);if(!before.isFile()||before.isSymbolicLink()||before.nlink!==1||before.size>maximumBytes)throw Error("Bounded original image retirement record required");const fd=openSync(path,constants.O_RDONLY|(constants.O_NOFOLLOW??0));try{if(stamp(fstatSync(fd))!==stamp(before))throw Error("Original image retirement record changed");const bytes=readFileSync(fd),after=lstatSync(path);if(bytes.length!==before.size||stamp(after)!==stamp(before)||stamp(fstatSync(fd))!==stamp(before))throw Error("Original image retirement record changed");return{bytes,stamp:stamp(before)};}finally{closeSync(fd);}
}
