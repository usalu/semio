import {test,expect} from "bun:test";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import jsonPatch from "fast-json-patch";
import fixture from "../🧫️fixtures/🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import {PublicationClaimReturn} from "../🟦️.ts";
test("original publication permit moves to its pre-admitted slot before the claim bit clears",()=>{
 expect(new Ajv({strict:false}).compile(schema)(fixture)).toBe(true);
 for(const row of fixture.cases){const cursor=new PublicationClaimReturn(row.aliases,32);let oracle:{source:boolean;permit?:string;returned?:string}={source:true};
  for(let index=0;index<row.repetitions;index++){if(index>0)oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/returned"}]).newDocument;expect(cursor.claim()).toBe(true);oracle=jsonPatch.applyPatch(oracle,[{op:"add",path:"/permit",value:"original-alias"}]).newDocument;expect(cursor.close(1,512,512,1)).toEqual({ready:false,released:0});if(row.cancelled)cursor.cancelled=true;cursor.returnPermit();oracle=jsonPatch.applyPatch(oracle,[{op:"move",from:"/permit",path:"/returned"}]).newDocument;expect(oracle.returned).toBe("original-alias");expect(cursor.claimed).toBe(false);if(index+1<row.repetitions){expect(cursor.demand()).toEqual({copy:8,release:0});}}
  expect(cursor.close(0,8,0,1)).toEqual({ready:false,released:0});expect(cursor.close(1,7,0,1)).toEqual({ready:false,released:0});expect(cursor.close(1,8,0,0)).toEqual({ready:false,released:0});expect(cursor.close(1,8,0,1)).toEqual({ready:true,released:0});oracle=jsonPatch.applyPatch(oracle,[{op:"remove",path:"/returned"}]).newDocument;expect(oracle).toEqual({source:true});expect(cursor.close(1,16,31,1)).toEqual({ready:false,released:0});expect(cursor.close(1,16,32,1)).toEqual({ready:true,released:row.aliases===1?32:0});
 }
});
test("publication permit owns no ungranted Arc drop and final source refuses a live return slot",()=>{
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),host=readFileSync(join(import.meta.dir,"../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs"),"utf8");
 expect(source.indexOf("returned.compare_exchange")).toBeLessThan(source.indexOf("state.fetch_and(!1"));expect(source).toContain("Arc::into_raw(self.inner)");expect(source).toContain("drop(Arc::from_raw(pointer))");expect(source).not.toContain("Weak<");expect(source).not.toContain("Arc::downgrade");expect(source).toContain("source.drain_returned_alias()");expect(source).toContain("grant.maximum_release_bytes<release");expect(host).not.toContain("publish_mounted_claimed_operation_unit");expect(host).toContain("let _permit = mounted.cancellation_lease.as_ref()");expect(host).toContain("claim: PublicationClaimHandle");
});
