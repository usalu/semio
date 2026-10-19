import{createHash}from"node:crypto";
import{isAbsolute,join,resolve}from"node:path";
import{readScriptProcessJsonV1}from"@semio-tech/process/routing/invocation/process/json";

/** 📦️ Names one stable renderer product without issuing physical custody. */
export function rendererWasmPublicationV1(productsRoot:string,value:unknown):Readonly<{directory:string;owner:string}>{
 const request=readScriptProcessJsonV1(value);if(!request||typeof request!=="object"||Array.isArray(request))throw Error("Closed renderer publication request required");const row=request as Record<string,unknown>,owner=row.owner,mode=row.mode;
 if(!isAbsolute(productsRoot)||resolve(productsRoot)!==productsRoot||Object.keys(row).sort().join(",")!=="mode,owner,version"||row.version!==1||typeof owner!=="string"||!owner||owner.length>192||/[\\:\u0000-\u001f]/.test(owner)||owner.split("/").some(part=>!part||part==="."||part==="..")||Buffer.from(owner).toString("utf8")!==owner||(mode!=="prototype"&&mode!=="release"))throw Error("Canonical renderer owner and mode required");
 const directory=join(productsRoot,"renderer",createHash("sha256").update(owner).digest("hex").slice(0,20),mode,"wasm");if(directory.length>224)throw Error("Renderer publication path capacity exceeded");return Object.freeze({directory,owner:"wgpu-renderer:"+mode});
}
