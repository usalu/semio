import {createHash} from "node:crypto";
import {isAbsolute,join,resolve} from "node:path";
import {readCompletedCargoPublicationReferenceV1} from "@semio-tech/process/artifacts/native-build/receipt/retention/reference";
/** 📦️ Derives the fixed publication identity without accepting a caller-selected origin or path map. */
export function trustedCargoCompilationPlanV1(root:string,value:unknown){
 const reference=readCompletedCargoPublicationReferenceV1(value),key=createHash("sha256").update(JSON.stringify(reference)).digest("hex"),path=join(root,"hc",reference.mode,key);if(!isAbsolute(root)||resolve(root)!==root||join(path,"blobs","a".repeat(64)).length>256)throw Error("Bounded actual Hub compiler publication root required");return Object.freeze({reference,path,owner:"hub:cargo:"+reference.mode+":"+key});
}
