/** 🧪️ Shared publication identities agree with an independent SHA oracle and atomic JSON Patch output. */
import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import Ajv2020 from "ajv/dist/2020";
import {applyPatch} from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {imagePublicationIdentities} from "../🟦️.ts";
test("original image publication identities and atomic asset/layer/selection match independent oracles",()=>{
 expect(new Ajv2020({strict:true}).compile(schema)(fixture)).toBe(true);const before=structuredClone(fixture.identity);const {assetId,layerId}=imagePublicationIdentities(before);const seed=Buffer.from(before.authoringSeed,"utf8"),length=Buffer.alloc(8);length.writeBigUInt64BE(BigInt(seed.length));const digest=createHash("sha256").update(before.prefix).update(Buffer.from(before.revisionHex+before.appInstanceHex+before.operationHex+before.generationHex,"hex")).update(length).update(seed).digest("hex");expect(digest).toBe(before.expectedDigest);expect(assetId).toBe("image-asset-"+digest);expect(layerId).toBe("layer-"+digest);expect(fixture.identity).toEqual(before);
 const original:{assets:Record<string,unknown>;layers:{kind:string;id:string;imageKey:string}[];selection:{domain:string;granularity:string;id:string}[]}={assets:{},layers:[],selection:[]};const published=applyPatch(original,[{op:"add",path:"/assets/"+assetId,value:{width:1,height:1,samples:[[128,128,128,255]]}},{op:"add",path:"/layers/-",value:{kind:"image",id:layerId,imageKey:assetId}},{op:"add",path:"/selection/-",value:{domain:fixture.selection.domain,granularity:fixture.selection.granularity,id:layerId}}],true,false).newDocument;expect(Object.keys(published.assets)).toEqual([assetId]);expect(published.layers[0]).toEqual({kind:"image",id:layerId,imageKey:assetId});expect(published.selection).toEqual([{domain:fixture.selection.domain,granularity:fixture.selection.granularity,id:layerId}]);expect(original).toEqual({assets:{},layers:[],selection:[]});
 expect(imagePublicationIdentities({...before,authoringSeed:"another writer"}).assetId).not.toBe(assetId);console.log("[DEBUG] First-party image identities match independent SHA-256; original authoring seed separates writers; atomic asset/layer/selection agrees with RFC6902");
});