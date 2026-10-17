/** 🧹️ Independent wire backing grant and source conservation oracle. */
import assert from "node:assert/strict";
import {readFileSync} from "node:fs";
import {createHash} from "node:crypto";
import {applyPatch} from "fast-json-patch";

export function testWireRetirementFixture():void{
 const fixture=JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json",import.meta.url),"utf8"));
 assert.equal(new Set(fixture.cases.map((row:{id:string})=>row.id)).size,5);
 const wire=Buffer.alloc(8);wire.writeBigUInt64LE(42n);assert.equal(wire.toString("hex"),fixture.shortClose.wireHex);
 const original=Buffer.from(fixture.completedInput.wireHex,"hex"),capacity=fixture.completedInput.capacityBytes;
 const pending={phase:"copy",original:original.toString("hex"),capacity};
 const completed=applyPatch(structuredClone(pending),[{op:"replace",path:"/phase",value:"complete"}],true).newDocument;
 assert.equal(completed.original,pending.original);assert.equal(completed.capacity,capacity);assert.equal(fixture.completedInput.retainedAfterCompletion,true);
 const native=readFileSync(new URL("../../../🧪️tests/🔬️unit/🦀️.rs",import.meta.url),"utf8");
 const stepGrant=native.slice(native.indexOf("fn fixture_step_grant()"),native.indexOf("fn fixture_scope_grant("));
 for(const[field,value]of Object.entries(fixture.stepGrant))assert.ok(stepGrant.includes(`${field}:${value}`),`original fixture authority ${field}`);
 const scope=native.slice(native.indexOf("fn fixture_scope_grant()"),native.indexOf("struct ImmediateJob"));
 for(const[field,value]of Object.entries(fixture.scopeGrant))assert.ok(scope.includes(`${field}:${value}`),`literal original whole scope ${field}`);
 assert.equal(fixture.scopeFuel,64);assert.equal(fixture.scopeDeadlineUs,660000000);
 assert.ok(!native.includes("fixture_close_grant(")&&!native.includes("exact_close_grant(")&&!native.includes("original_close_context("));
 assert.ok(!native.includes("StepBudget::new(1, u64::MAX"));
 assert.equal((native.match(/StepContext::new[^\n]+&mut retained_progress\)/g)??[]).length,4);
 const owner=readFileSync(new URL("../../../🦀️.rs",import.meta.url),"utf8");
 const erased=owner.slice(owner.indexOf("impl InteractiveJob for ErasedToolJob"),owner.indexOf("pub trait ToolJobFactory"));
 assert.ok(erased.includes("?.step(cx)")&&erased.includes("cx.with_retained_loan(child, |same| inner.close_step(same))"));
 assert.ok(erased.includes("drop(self.inner.take())")&&erased.includes("cx.consume_retained(progress)")&&!erased.includes("StepContext::new"));
 assert.ok(erased.includes("Result<Option<semio_framework_job::JobOutcomeBorrow<\'a>>, semio_framework_job::OriginalTurnFailure>")&&erased.includes("Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure>"));
 assert.ok(!erased.includes("StepContext::new")&&!erased.includes("Refused"));
 assert.equal((erased.match(/cx.consume_retained\(progress\)/g)??[]).length,1);
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
}
