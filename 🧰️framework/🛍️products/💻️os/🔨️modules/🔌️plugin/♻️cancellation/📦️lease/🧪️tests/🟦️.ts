import {test,expect} from "bun:test";
import Ajv from "ajv";
import jsonPatch,{type Operation} from "fast-json-patch";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import corpus from "../🧫️fixtures/🔣️.json";
import {LeaseDetachCursor,type LeaseScopeSource} from "../🟦️.ts";
test("original cancellation scope detach preserves every sibling and retains detached owners",()=>{
 expect(new Ajv({strict:false}).compile(schema)(corpus)).toBe(true);
 for(const row of corpus.cases){
  const source:LeaseScopeSource={keyed:row.keyed,active:row.operations,documentActive:row.keyed?row.operations:0,operation:true,document:row.keyed};const before=structuredClone(source);const cursor=new LeaseDetachCursor(source,128);
  for(const denied of [{items:0,copy:128,depth:1},{items:1,copy:127,depth:1},{items:1,copy:128,depth:0}]){expect(cursor.step({...denied,capacity:0,release:0})).toBe(false);expect(source).toEqual(before);}
  expect(cursor.step({items:1,copy:128,depth:1,capacity:0,release:0})).toBe(true);
  const operations:Operation[]=[{op:"replace",path:"/operation",value:false},{op:"replace",path:"/active",value:row.expectedActive}];if(row.keyed){operations.push({op:"replace",path:"/documentActive",value:row.expectedDocumentActive});if(row.expectedDocumentActive===0)operations.push({op:"replace",path:"/document",value:false});}
  expect(source).toEqual(jsonPatch.applyPatch(before,operations).newDocument);expect(cursor.operation).toBe(true);expect(cursor.document).toBe(row.keyed&&row.expectedDocumentActive===0);expect(cursor.step({items:1,copy:128,depth:1,capacity:0,release:0})).toBe(false);
 }
});
test("mounted original lease uses canonical alias custody and independent physical cursor birth/release",()=>{
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),host=readFileSync(join(import.meta.dir,"../../../🦀️.rs"),"utf8"),frontier=readFileSync(join(import.meta.dir,"../../../🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs"),"utf8");
 expect(source).toContain("PublicationClaimHandle::same_original");expect(source).toContain("canonical.state");expect(source).toContain("canonical.app_generation");expect(source).toContain("canonical.active");expect(source).not.toContain("strong_count");expect(source).not.toContain("weak_count");expect(source).not.toContain("original.finish()");
 expect(source).toContain("capacity_bytes:std::mem::size_of::<ToolCancellationLeaseRetirement>()");expect(source).toContain("release_bytes:std::mem::size_of::<ToolCancellationLeaseRetirement>()");expect(source.indexOf("std::alloc::alloc(layout)")).toBeLessThan(source.indexOf("original.take().unwrap()"));
 expect(host).toContain("self.cancellation_retirement.is_none()");expect(host).toContain("operation.lease_retirement_demands(&self.tool_cancellations,body)");expect(frontier).toContain("lease_retirement_step(&self.tool_cancellations,grant)");expect(host).toContain("if scope.claim.is_finished()");
});
test("yielded window refresh keeps the original mutation and receipt before ACK",()=>{
 const host=readFileSync(join(import.meta.dir,"../../../🦀️.rs"),"utf8");
 expect(host).not.toContain("self.window_config_store.refresh(authority)?");expect(host).not.toContain("self.window_transient_store.refresh(authority)");expect(host).toContain("mounted.pending_window_config_receipt=Some(receipt)");expect(host).toContain("if let Some(receipt)=mounted.pending_window_config_receipt");expect(host).toContain("Pending(_))=>{emit.window_config_mutations.push(mutation);return Ok(())");expect(host).toContain("Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(())");expect(host).toContain("copy_bytes:std::mem::size_of::<store::LaneItemReceipt>()");
});

test("mounted cancellation flags original gesture custody before both retirement frontiers",()=>{
 const host=readFileSync(join(import.meta.dir,"../../../🦀️.rs"),"utf8"),frontier=readFileSync(join(import.meta.dir,"../../../🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs"),"utf8"),gesture=readFileSync(join(import.meta.dir,"../../../🛠️tool-machine/🦀️.rs"),"utf8");
 const normal=host.slice(host.indexOf("fn retire_typed_operation_unit("),host.indexOf("/// ♻️ Drives ONE retiring typed operation"));
 expect(normal.includes("self.request_gesture_cancellation(operation_id)")).toBe(true);
 const granted=frontier.slice(frontier.indexOf("fn granted_mounted_retirement_step("));
 expect(granted.includes("self.request_gesture_cancellation(id)")).toBe(true);
 expect(granted.indexOf("self.request_gesture_cancellation(id)")).toBeLessThan(granted.indexOf("let owner=self.tool_operations.get_mut(id).unwrap()"));
 const request=gesture.slice(gesture.indexOf("fn request_gesture_cancellation("),gesture.indexOf("fn begin_gesture_retirement("));
 expect(request).toContain("original.request_cancellation()");expect(request).not.toContain(".take()");expect(request).not.toContain("drop(");
});