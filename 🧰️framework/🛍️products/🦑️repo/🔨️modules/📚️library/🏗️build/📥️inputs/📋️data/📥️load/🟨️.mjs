import{captureBoundedSourceRevisionV1,implementationPinnedV1,physicalSourceOriginV1}from"../../🔏️revision/🟨️.mjs";
import{createHash}from"node:crypto";
import{fileURLToPath}from"node:url";
const sourceURL=new URL("../🟨️.mjs",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),policyURL=new URL("../../🧮️digest/📥️read/🧬️schema/🔣️.json",physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL)),ownerURL=new URL(physicalSourceOriginV1(import.meta.url,import.meta.semioPhysicalSourceURL));
function revision(url,maximumBytes=8388608){return captureBoundedSourceRevisionV1(fileURLToPath(url),maximumBytes).sha256;}
async function loadSourceInventoryImplementationV1(sourceURL,policyURL,ownerURL){
const ownerRevision=revision(ownerURL),sourceCapture=captureBoundedSourceRevisionV1(fileURLToPath(sourceURL),65536,true),policyCapture=captureBoundedSourceRevisionV1(fileURLToPath(policyURL),65536,true),sourceRevision=sourceCapture.sha256,policyRevision=policyCapture.sha256,sourceBytes=Buffer.from(sourceCapture.text??"","utf8"),policyBytes=Buffer.from(policyCapture.text??"","utf8");
const blob="Bun"in globalThis,target=blob?URL.createObjectURL(new Blob([sourceBytes],{type:"text/javascript"})):"data:text/javascript;base64,"+sourceBytes.toString("base64");
let module;try{module=await import(target);}catch{throw Error("Captured inventory Source module load failed");}finally{if(blob)URL.revokeObjectURL(target);}
const loadedURL=module.loadedSourceInventoryURLV1,loaded=module.createSourceInventoryDataV1(JSON.parse(policyBytes.toString("utf8")).definitions.Policy.const);
const loadedExact=loadedURL===target||loadedURL==="file:///"+target;
function requireCurrentSourceInventoryV1(){if(implementationPinnedV1())return;if(!loadedExact||createHash("sha256").update(sourceBytes).digest("hex")!==sourceRevision||createHash("sha256").update(policyBytes).digest("hex")!==policyRevision||revision(sourceURL,65536)!==sourceRevision||revision(policyURL,65536)!==policyRevision||revision(ownerURL)!==ownerRevision)throw Error("Stale loaded inventory implementation; restart required");}
requireCurrentSourceInventoryV1();
return Object.freeze({data:loaded,requireCurrentSourceInventoryV1});
}
const implementation=await loadSourceInventoryImplementationV1(sourceURL,policyURL,ownerURL),loaded=implementation.data;
/** 🧷️ Requires the captured closed helper/policy epoch before inference or publication. */
export function requireCurrentSourceInventoryV1(){implementation.requireCurrentSourceInventoryV1();}
/** 🗂️ Captures Source root DATA through the current closed implementation epoch. */
export function captureRepositorySourceRootV1(root){return loaded.captureRepositorySourceRootV1(root);}
/** 📏️ Captures Source path DATA through the current closed implementation epoch. */
export function captureRepositorySourcePathV1(value){return loaded.captureRepositorySourcePathV1(value);}
/** 🧭️ Captures resolved Source path DATA through the current closed implementation epoch. */
export function captureExactSourcePathV1(root,value){return loaded.captureExactSourcePathV1(root,value);}
/** 📋️ Captures Source inventory DATA before a producer can request a publication directory. */
export function captureSourceInputInventoryV1(value,root){requireCurrentSourceInventoryV1();return loaded.captureSourceInputInventoryV1(value,root);}
/** 🔒️ Captures lock inventory DATA before a producer can request a publication directory. */
export function captureCargoLockInventoryV1(value,root){requireCurrentSourceInventoryV1();return loaded.captureCargoLockInventoryV1(value,root);}
/** 🧮️ Captures exact extent DATA through the current closed implementation epoch. */
export function admitExactSourceExtentV1(input){return loaded.admitExactSourceExtentV1(input);}
