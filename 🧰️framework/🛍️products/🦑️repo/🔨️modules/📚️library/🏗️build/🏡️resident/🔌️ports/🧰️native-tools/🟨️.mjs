import {posix,win32} from "node:path";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import {validateJsonSchemaSubset} from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

/** 🧪 Closed fixture capabilities; qualified paths are data and confer no storage or process authority. */
export function readNativeFixtureV1(value,platform=process.platform){
 if(validateJsonSchemaSubset(schema,value).length)throw Error("Closed native fixture profile required");
 const qualified=path=>typeof path==="string"&&!(path.includes("\0"))&&(platform==="win32"?win32.isAbsolute(path)&&win32.parse(path).root.length>1:posix.isAbsolute(path));
 if(!qualified(value.watcher)||(value.executionTool!==null&&!qualified(value.executionTool)))throw Error("Qualified native fixture tools required");
 return Object.freeze({...value});
}

