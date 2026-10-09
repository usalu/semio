import assert from "node:assert/strict";
import Ajv from "ajv";
import {existsSync,readFileSync} from "node:fs";
import {MockFlowBridge} from "../🎭️mock-flow-bridge/🟦️.ts";
import {createFlowHost} from "../../🖥️host/🏃️runtime/🟨️.js";
import {createFlowBrowserRuntime} from "../../🌐️browser/🏃️runtime/🟨️.js";

/** 🧾️ Actual Flow receiving transport preserves pending, failed and terminal child receipts. */
export async function testFlowRetainedReceipt():Promise<void>{
 assert.equal(existsSync(new URL("../../🧬️schema/🎟️retained-receipt/🔣️.json",import.meta.url)),false,"Examples cannot own a receipt corpus schema");
 const fixture=JSON.parse(readFileSync(new URL("../../🧫️fixtures/🎟️retained-receipt/🔣️.json",import.meta.url),"utf8"));
 const schema=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json",import.meta.url),"utf8"));
 const validate=new Ajv({strict:true,allErrors:true}).compile(schema);assert.deepEqual(fixture.axes,schema.required);let assertions=1;
 for(const row of fixture.cases){
  assert.equal(validate(row.progress),true);assertions++;
  for(const axis of fixture.axes){const invalid={...row.progress};delete invalid[axis];assert.equal(validate(invalid),false);assertions++;}
  assert.equal(validate({...row.progress,depth:1}),false);assertions++;
  const reference=new BigUint64Array(Object.values(row.progress).map((value)=>BigInt(value as number)));const memory=new WebAssembly.Memory({initial:2});const bridge=new MockFlowBridge(memory);const scheduled:(()=>void)[]=[];let terminal=false;let queries=0;
  const exports={...bridge.exports,flow_bridge_poll(){terminal=row.terminal;return row.length;},flow_bridge_step_progress(axis:number){queries++;return reference[axis];},flow_bridge_terminal_is_empty(){return Number(terminal);}};
  const host=createFlowHost({exports,memory,schedule:(step:()=>void)=>scheduled.push(step)});const close=host.close();const outcome=close.then(()=>({failed:false,error:undefined}),error=>({failed:true,error}));assert.equal(scheduled.length,1);scheduled.shift()!();
  assert.deepEqual(host.stepProgress(),row.progress);assert.equal(queries,4);assertions+=2;
  if(row.length===0){terminal=true;scheduled.shift()!();}
  const result=await outcome;assert.equal(result.failed,row.length<0&&!row.terminal);assertions++;
  if(result.failed){assert.deepEqual(result.error.retainedProgress,row.progress);assertions++;}
  const runtimeMemory=new WebAssembly.Memory({initial:2});const runtimeBridge=new MockFlowBridge(runtimeMemory);const runtime=await createFlowBrowserRuntime({source:runtimeBridge.exports});assert.deepEqual(runtime.stepProgress(),{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0});await runtime.close();assertions++;
 }
 console.log("[DEBUG] original Flow actual retained receipt cases="+fixture.cases.length+" assertions="+assertions+" canonicalProgressAjv=true BigUint64ArrayReference=true");
}
