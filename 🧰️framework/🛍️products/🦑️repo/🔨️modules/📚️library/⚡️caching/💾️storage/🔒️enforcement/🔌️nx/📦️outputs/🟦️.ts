import {isAbsolute,resolve,relative,sep} from "node:path";
import {requireMaterializedPathV1} from "../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

/** 📦️ Resolves an output to one exact physical store and its actual publisher's resource identity. */
export function classifyNxOutputV1(roots:Readonly<{rust:string;products:string}>,path:string):Readonly<{version:1;partition:"rust"|"products";resource:string}>{
 if(!isAbsolute(path)||resolve(path)!==path)throw Error("Canonical absolute Nx output required");requireMaterializedPathV1(path);for(const partition of ["rust","products"] as const){const root=roots[partition];if(!isAbsolute(root)||resolve(root)!==root)throw Error("Canonical Nx output partition required");const child=relative(root,path);if(child&&child!==".."&&!child.startsWith(".."+sep)&&!isAbsolute(child)){const result={version:1 as const,partition,resource:(partition==="rust"?"rust-unit:":"artifact:")+path};if(validateJsonSchemaSubset(schema,result).length)throw Error("Closed Nx output custody required");return Object.freeze(result);}}throw Error("Nx output escapes declared cache partitions");
}
