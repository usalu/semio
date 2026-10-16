import {test,expect} from "bun:test";
import Ajv from "ajv";
import jsonPatch,{type Operation} from "fast-json-patch";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import corpus from "./🧫️fixtures/🔣️.json";
import {LeaseDetachCursor,type LeaseScopeSource} from "../🟦️.ts";
import { familyText } from "../../../🧷️companions/🧯️test-support/🧵️instance-family/🟦️.ts";
/** 📏️ A `field: size_of::<T>()` demand, whatever path prefix or spacing spells the intrinsic. */
const sizeOfDemand=(field:string,type:string)=>new RegExp(`\\b${field}\\s*:\\s*(?:(?:std|core)::mem::)?size_of::<\\s*${type}\\s*>\\(\\)`);
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
 const source=readFileSync(join(import.meta.dir,"../🦀️.rs"),"utf8"),host=familyText(),frontier=["🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs","🧵️instance/🔄️full-operation/🦀️.rs"].map(file=>readFileSync(join(import.meta.dir,"../../..",file),"utf8")).join("\n");
 expect(source).toContain("PublicationClaimHandle::same_original");expect(source).toContain("canonical.state");expect(source).toContain("canonical.app_generation");expect(source).toContain("canonical.active");expect(source).not.toContain("strong_count");expect(source).not.toContain("weak_count");expect(source).not.toContain("original.finish()");
 expect(host).toMatch(sizeOfDemand("capacity_bytes","ToolCancellationLeaseRetirement"));expect(host).toMatch(sizeOfDemand("release_bytes","ToolCancellationLeaseRetirement"));expect(source.indexOf("std::alloc::alloc(layout)")).toBeLessThan(source.indexOf("original.take().unwrap()"));
 expect(host).toContain("self.cancellation_retirement.is_none()");expect(host).toContain("operation.lease_retirement_demands(&self.tool_cancellations,body)");expect(frontier).toContain("lease_retirement_step(&self.tool_cancellations,grant)");expect(readFileSync(join(import.meta.dir,"../../../🎮️command/📥️admission/🦀️.rs"),"utf8")).toMatch(/\bif\s+scope\.claim\.is_finished\(\s*\)/);
});
test("yielded window refresh keeps the original mutation and receipt before ACK",()=>{
 const host=familyText();
 expect(host).not.toContain("self.window_config_store.refresh(authority)?");expect(host).not.toContain("self.window_transient_store.refresh(authority)");expect(host).toContain("mounted.pending_window_config_receipt = Some(receipt);");expect(host).toContain("if mounted.pending_window_config_receipt.is_some() { return self.advance_mounted_window_config_refresh(mounted, retained); }");expect(host).toContain("let receipt = mounted.pending_window_config_receipt.ok_or_else(");expect(host).toContain("mounted.queue_page(TypedOperationResultPage::try_serialize(token, TypedOperationResultLane::WindowConfig, &receipt)?)");expect(host).toContain("self.window_config_store.admit_begin(mounted.operation.operation,&mut mounted.actor_capture,&mut emit.window_config_mutations,authority,retained)");expect(host).toContain("Pending(_))=>{ephemeral.window_transient.push(mutation);return Ok(())");expect(host).toMatch(sizeOfDemand("copy_bytes","store::os_store::presence::LaneItemReceipt"));
});

test("mounted cancellation flags original gesture custody before both retirement frontiers",()=>{
 const host=familyText(),frontier=["🧵️retained-command/🪟️mounted/♻️frontier/🦀️.rs","🧵️instance/🔄️full-operation/🦀️.rs"].map(file=>readFileSync(join(import.meta.dir,"../../..",file),"utf8")).join("\n"),gesture=["🛠️tool-machine/🦀️.rs","🧵️instance/🛠️tool-machine/🦀️.rs"].map(file=>readFileSync(join(import.meta.dir,"../../..",file),"utf8")).join("\n");
 const normal=host.slice(host.indexOf("fn retire_typed_operation_unit("),host.indexOf("/// ♻️ Drives ONE retiring typed operation"));
 expect(normal.includes("self.request_gesture_cancellation(operation_id)")).toBe(true);
 const granted=frontier.slice(frontier.indexOf("fn granted_mounted_retirement_step("));
 expect(granted.includes("self.request_gesture_cancellation(id)")).toBe(true);
 expect(granted.indexOf("self.request_gesture_cancellation(id)")).toBeLessThan(granted.indexOf("let owner=self.tool_operations.get_mut(id).unwrap()"));
 const request=gesture.slice(gesture.indexOf("fn request_gesture_cancellation("),gesture.indexOf("fn begin_gesture_retirement("));
 expect(request).toContain("original.request_cancellation()");expect(request).not.toContain(".take()");expect(request).not.toContain("drop(");
});