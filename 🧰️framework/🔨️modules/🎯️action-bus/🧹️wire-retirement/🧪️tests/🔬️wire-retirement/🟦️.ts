/** 🧹️ Independent wire backing grant and source conservation oracle. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {applyPatch} from "fast-json-patch";

export function testWireRetirementFixture():void{
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 assert.equal(new Set(fixture.cases.map((row:{id:string})=>row.id)).size,5);
 const wire=Buffer.alloc(8);wire.writeBigUInt64LE(42n);assert.equal(wire.toString("hex"),fixture.shortClose.wireHex);
 const original=Buffer.from(fixture.completedInput.wireHex,"hex"),capacity=fixture.completedInput.capacityBytes;
 const pending={phase:"copy",original:original.toString("hex"),capacity};
 const completed=applyPatch(structuredClone(pending),[{op:"replace",path:"/phase",value:"complete"}],true).newDocument;
 assert.equal(completed.original,pending.original);assert.equal(completed.capacity,capacity);assert.equal(fixture.completedInput.retainedAfterCompletion,true);
 const native=readFileSync(new URL("../../../🧪️tests/🔬️unit/🦀️.rs",import.meta.url),"utf8");
 const stepGrant=native.slice(native.indexOf("fn fixture_step_grant()"),native.indexOf("fn fixture_close_grant("));
 for(const[field,value]of Object.entries(fixture.stepGrant))assert.ok(stepGrant.includes(`${field}:${value}`),`original fixture authority ${field}`);
 assert.equal((native.match(/&mut retained_progress\)/g)??[]).length,4);
 const immediate=native.slice(native.indexOf("impl InteractiveJob for ImmediateJob"),native.indexOf("struct EchoFactory"));
 const step=immediate.slice(0,immediate.indexOf("fn begin_close"));
 assert.ok(!step.includes("self.output = None;"),"completion retains original input until granted close");
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
