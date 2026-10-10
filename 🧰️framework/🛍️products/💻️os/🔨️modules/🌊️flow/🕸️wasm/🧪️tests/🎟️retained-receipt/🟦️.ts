import assert from "node:assert/strict";
import Ajv from "ajv";
import {readFileSync,existsSync} from "node:fs";
import {applyPatch} from "fast-json-patch";
import stableStringify from "fast-json-stable-stringify";
import {MockFlowBridge} from "../🎭️mock-flow-bridge/🟦️.ts";
import {createFlowHost} from "../../🖥️host/🏃️runtime/🟨️.js";
import {createFlowBrowserRuntime} from "../../🌐️browser/🏃️runtime/🟨️.js";

/** 🧾️ Actual Flow receiving transport preserves pending, failed and terminal child receipts. */
export async function testFlowRetainedReceipt():Promise<void>{
 assert.equal(existsSync(new URL("../../🧬️schema/🎟️retained-receipt/🔣️.json",import.meta.url)),false,"original Flow receipt examples cannot own trial schema authority");
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️retained-receipt/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json",import.meta.url),"utf8"));
 const ajv=new Ajv({strict:true,allErrors:true}).addSchema(schema,"flow.original-retained-progress"),validate=ajv.getSchema("flow.original-retained-progress")!;
 assert.deepEqual(fixture.axes,schema.required);assert.equal(validate(fixture.normalFailure.progress),true,JSON.stringify(validate.errors));let assertions=2;
 for(const row of fixture.cases){
  assert.equal(validate(row.progress),true);assertions++;
  for(const axis of fixture.axes){const invalid={...row.progress};delete invalid[axis];assert.equal(validate(invalid),false);assertions++;}
  assert.equal(validate({...row.progress,depth:1}),false);assertions++;
  const reference=new BigUint64Array(fixture.axes.map((axis:string)=>BigInt(row.progress[axis])));const memory=new WebAssembly.Memory({initial:2});const bridge=new MockFlowBridge(memory);const scheduled:(()=>void)[]=[];let terminal=false;let queries=0;
  const exports={...bridge.exports,flow_bridge_poll(){terminal=row.terminal;return row.length;},flow_bridge_step_progress(axis:number){queries++;return reference[axis];},flow_bridge_terminal_is_empty(){return Number(terminal);}};
  const host=createFlowHost({exports,memory,schedule:(step:()=>void)=>scheduled.push(step)});const close=host.close();const outcome=close.then(()=>({failed:false,error:undefined}),error=>({failed:true,error}));assert.equal(scheduled.length,1);scheduled.shift()!();
  assert.deepEqual(host.stepProgress(),row.progress);assert.equal(queries,4);assertions+=2;
  if(row.length===0){terminal=true;scheduled.shift()!();}
  const result=await outcome;assert.equal(result.failed,row.length<0&&!row.terminal);assertions++;
  if(result.failed){assert.deepEqual(result.error.retainedProgress,row.progress);assertions++;}
  const runtimeMemory=new WebAssembly.Memory({initial:2});const runtimeBridge=new MockFlowBridge(runtimeMemory);const runtime=await createFlowBrowserRuntime({source:runtimeBridge.exports});assert.deepEqual(runtime.stepProgress(),{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});await runtime.close();assertions++;
 }
 const protocol=readFileSync(new URL("../../📡️protocol/🦀️.rs",import.meta.url),"utf8");
 assert.ok(/FlowFeatureStep::Failed\(failure\)\s*=>\s*\{\s*self\.retained_progress\s*=\s*failure\.retained_progress\s*;/.test(protocol),"actual normal failed child receipt must survive before reply projection");assertions++;
 const causes=fixture.failureCauses,text=causes.text.repeat(causes.repeat);assert.equal(Buffer.byteLength(text),causes.utf8Bytes);assertions++;
 for(const kind of causes.kinds){const original={kind,message:text,retainedProgress:fixture.normalFailure.progress};assert.equal(stableStringify(applyPatch(original,[],true,false).newDocument),stableStringify(original));assertions++;}
 const vcs=readFileSync(new URL("../../../🌿️vcs/♻️retirement/🦀️.rs",import.meta.url),"utf8"),domain=readFileSync(new URL("../../🦀️.rs",import.meta.url),"utf8");
 assert.ok(protocol.includes("pub enum FlowFailureCause")&&protocol.includes("Value(semio_framework_value::ValueError)"),"the original Flow failure must own the complete typed refusal");assertions++;
 assert.ok(vcs.includes("pub cause:Option<ValueError>")&&vcs.includes("cause:Some(error)"));assertions++;
 assert.ok(domain.includes("FlowFailure::from_value")&&domain.includes("failure.cause"));assertions++;
 assert.equal(/#\[derive\(Clone[^\n]*\)\]\s*pub (?:struct FlowFailure|enum FlowFeatureStep|struct FlowVcsCloseFailure)/.test(protocol+vcs),false);assertions++;
 console.log("[DEBUG] original Flow receiving retained receipt cases="+fixture.cases.length+" assertions="+assertions+" plainExamples=true canonicalProgressAjv=true BigUint64ArrayReference=true normalFailureSource=true");
}
