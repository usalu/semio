import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {resolve} from "node:path";
import {createRequire} from "node:module";
import {Database} from "bun:sqlite";
import type {IntrinsicValue} from "@semio-tech/value-ts/schema/intrinsic";
import {faultFromOriginalTurnFailure} from "../../../../🧾️retained/🟦️.ts";
import {decodeFaultDomainControlled,encodeFaultDomainControlled} from "../../../../🎛️controlled/🟦️.ts";
import type {ValueRefusalCodecControl} from "@semio-tech/value-ts/refusal/codec";
import {ValueError} from "../../../../../🌱️value/📦️packages/🟦️typescript/⚠️refusal/🟦️.ts";
test("Canonical Fault lends complete domain with exact original intrinsic u64 and octet owners",async()=>{
 const root=resolve(import.meta.dir,".."),law=JSON.parse(readFileSync(resolve(root,"🧪️tests/🧫️fixtures/🔣️.json"),"utf8")),schema=JSON.parse(readFileSync(resolve(root,"🧬️schema/🔣️.json"),"utf8")),require=createRequire(import.meta.url);expect(new(require("ajv").default)({strict:true}).compile(schema)(law)).toBe(true);
 const octets=Uint8Array.from(law.octets),nested:IntrinsicValue={kind:"object",members:[{name:"payload",value:{kind:"bytes",value:octets}}]},domain:Extract<IntrinsicValue,{kind:"object"}>={kind:"object",members:[{name:"operation",value:{kind:"unsigned",value:BigInt(law.unsigned)}},{name:"signed",value:{kind:"signed",value:BigInt(law.signed)}},{name:"nested",value:nested}]},progress={copiedItems:3,copiedBytes:17,retainedCapacityBytes:128,releasedBytes:64};
 const receipt=new ValueError("ownershipLimit","original denied receipt").withRetainedProgress({copiedItems:1,copiedBytes:0,retainedCapacityBytes:64,releasedBytes:0});
 const fault=faultFromOriginalTurnFailure({primary:{kind:"domain",domain:{code:"geometry.tessellate",message:law.text,retainedProgress:progress,original:domain}},receiptRefusal:receipt});expect(fault.domain).toBe(domain);expect(fault.retainedProgress).toBe(progress);expect(fault.causes?.[0]?.retainedProgress).toBe(receipt.retainedProgress);expect(fault.code).toBe("geometry.tessellate");
 let storage=0,events=0;const control:ValueRefusalCodecControl={checkpoint:async()=>{events++;if(events>8)throw Error("finite codec work refused");},charge:async bytes=>{storage+=bytes;if(storage>32)throw Error("finite codec storage refused");},step:async()=>{throw Error("domain alias must not traverse owned body");},advance:async()=>{throw Error("domain alias must not traverse owned body");},beginStage:async()=>{},scopedStage:async operation=>operation()};
 const encoded=await encodeFaultDomainControlled(domain,control),decoded=await decodeFaultDomainControlled(encoded,control);expect(decoded).toBe(domain);expect(decoded.members[2]?.value).toBe(nested);expect(nested.kind==="object"&&nested.members[0]?.value.kind==="bytes"&&nested.members[0].value.value).toBe(octets);expect(storage).toBe(32);expect(events).toBe(2);
 const independent=Buffer.alloc(16);independent.writeBigUInt64LE(BigInt(law.unsigned));independent.writeBigInt64LE(BigInt(law.signed),8);expect(domain.members[0]?.value).toEqual({kind:"unsigned",value:independent.readBigUInt64LE()});expect(domain.members[1]?.value).toEqual({kind:"signed",value:independent.readBigInt64LE(8)});
 const db=new Database(":memory:");try{expect(db.query("SELECT json_extract(?, '$.unsigned') AS operation,json_array_length(json_extract(?, '$.octets')) AS octets").get(JSON.stringify(law),JSON.stringify(law))).toEqual({operation:law.unsigned,octets:law.octets.length});}finally{db.close();}
 const source=readFileSync(resolve(root,"../🦀️.rs"),"utf8");expect(source).toContain("self.scalar.write_node(node)");expect(source).toContain("maximum_depth<demand.depth");expect(source).not.toContain("ScalarBytes::from_node");
 for(const value of [{kind:"null"},{kind:"array",items:[]},{kind:"text",value:"unowned JSON text"}]as IntrinsicValue[])await expect(decodeFaultDomainControlled(value,{...control,checkpoint:async()=>{},charge:async()=>{}})).rejects.toThrow("Fault.domain requires an original object projection");
 console.log("[DEBUG] actual TS Fault.domain borrows original projection/nested/bytes/u64max/i64min and receipt; finite codec alias32/2 plus Ajv/Buffer/SQLite oracle; native qualification remains separate");
});
