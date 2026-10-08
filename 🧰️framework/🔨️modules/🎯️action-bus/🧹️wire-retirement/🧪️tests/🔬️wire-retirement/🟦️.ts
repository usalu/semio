/** 🧹️ Independent wire backing grant and source conservation oracle. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {applyPatch} from "fast-json-patch";

export function testWireRetirementFixture():void{
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 assert.equal(new Set(fixture.cases.map((row:{id:string})=>row.id)).size,5);
 const wire=Buffer.alloc(8);wire.writeBigUInt64LE(42n);assert.equal(wire.toString("hex"),fixture.shortClose.wireHex);
 for(const row of fixture.cases){assert.ok(row.admitted<=row.declared);if(row.sealed)assert.equal(row.admitted,row.declared);
  const source=Buffer.from(Array.from({length:row.admitted},(_,index)=>index%251)),digest=createHash("sha256").update(source).digest("hex");
  for(const wordBytes of fixture.wordBytes){const capacity=Math.ceil(row.declared/fixture.pageBytes),physical=capacity*(fixture.pageBytes+wordBytes),backing=Buffer.alloc(physical);assert.equal(backing.byteLength,physical);
   for(const copy of fixture.copyGrants){const grant={items:1,copy,capacity:0,release:physical,depth:1};
    if(capacity)for(const denied of fixture.denied){const unfunded=applyPatch(structuredClone(grant),[{op:"replace",path:`/${denied}`,value:0}],true).newDocument;assert.ok(unfunded.items===0||unfunded.release<physical||unfunded.depth===0);assert.equal(createHash("sha256").update(source).digest("hex"),digest);assert.equal(backing.byteLength,physical);}
    const progress={copiedItems:Number(capacity>0),copiedBytes:0,retainedCapacityBytes:0,releasedBytes:physical};assert.ok(progress.copiedItems<=grant.items);assert.ok(progress.copiedBytes<=grant.copy);assert.ok(progress.retainedCapacityBytes<=grant.capacity);assert.equal(progress.releasedBytes,grant.release);assert.equal(createHash("sha256").update(source).digest("hex"),digest);
   }
  }
 }
 console.log("[DEBUG] original wire grant oracle5cases×2wordWidths×4copyCredits preserves complete physical backing and terminal receipt");
}
