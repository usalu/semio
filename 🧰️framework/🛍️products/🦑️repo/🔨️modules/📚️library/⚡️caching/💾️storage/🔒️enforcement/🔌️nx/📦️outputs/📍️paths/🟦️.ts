import {posix,win32} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
/** 🛤️ Admits only canonical native system descendants, preserving other artifact custody. */
export function isNxSystemPathV1(request:Readonly<{version:1;platform:"posix"|"windows";root:string;path:string}>):boolean{if(validateJsonSchemaSubset(schema,request).length)throw Error("Closed native Nx path request required");const flavor=request.platform==="windows"?win32:posix;if(!flavor.isAbsolute(request.root)||!flavor.isAbsolute(request.path)||flavor.resolve(request.root)!==request.root||flavor.resolve(request.path)!==request.path)throw Error("Canonical native Nx paths required");const distance=flavor.relative(request.root,request.path);return distance===""||!flavor.isAbsolute(distance)&&distance.split(flavor.sep)[0]!=="..";}
