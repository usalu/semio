import assert from "node:assert/strict";
import Ajv from "ajv";
import {readFileSync} from "node:fs";
import {MockFlowBridge} from "../🎭️mock-flow-bridge/🟦️.ts";
import {createFlowBrowserRuntime} from "../../🌐️browser/🏃️runtime/🟨️.js";

/** 🎟️ Exercises the real browser transport with independently supplied ownership currencies. */
export async function testFlowRetainedGrant():Promise<void>{
 const schema=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌉️abi/🧬️schema/🔣️.json",import.meta.url),"utf8"));
 const fixture=JSON.parse(readFileSync(new URL("../../../../../../../🔨️modules/🌉️abi/🧫️fixtures/🎟️retained-grant/🔣️.json",import.meta.url),"utf8"));
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addKeyword({keyword:"x-semio-binary",valid:true});ajv.addSchema(schema);
 const validate=ajv.compile({$ref:schema.$id+"#/definitions/AbiWorkBudget"});
 let assertions=0;
 for(const row of fixture.cases){
  const budget={byteCredit:fixture.byteCredit,retained:row.retained,nowMs:fixture.nowMs,deadlineMs:fixture.deadlineMs,cancelled:false,interrupted:false};
  assert.equal(validate(budget),true);assertions++;
  for(const field of Object.keys(row.retained)){
   const changed=structuredClone(budget);delete changed.retained[field];assert.equal(validate(changed),false);assertions++;
   const invalid=structuredClone(budget);invalid.retained[field]=-1;assert.equal(validate(invalid),false);assertions++;
  }
  assert.equal(validate({...budget,retained:{...row.retained,maximumBytes:99}}),false);assertions++;
  const reference=new Uint32Array(row.expectedArguments);
  assert.deepEqual(Array.from(reference),Object.values(row.retained));assertions++;
  const memory=new WebAssembly.Memory({initial:2});const bridge=new MockFlowBridge(memory);const captured:{kind:string,args:unknown[]}[]=[];
  const exports={...bridge.exports,flow_bridge_send(...args:any[]){captured.push({kind:"send",args:args.slice(3,8)});return bridge.exports.flow_bridge_send(...args as Parameters<typeof bridge.exports.flow_bridge_send>);},flow_bridge_poll(...args:any[]){captured.push({kind:"poll",args:args.slice(3,8)});return bridge.exports.flow_bridge_poll(...args as Parameters<typeof bridge.exports.flow_bridge_poll>);}};
  const runtime=await createFlowBrowserRuntime({source:exports,retainedGrant:row.retained,now:()=>fixture.nowMs});
  const session=runtime.openSession();await session.catalogueJson().result;await session.close();await runtime.close();
  assert(captured.some(value=>value.kind==="send"));assert(captured.some(value=>value.kind==="poll"));assertions+=2;
  for(const call of captured){assert.deepEqual(call.args,Array.from(reference),row.name+" "+call.kind+" preserves five independent currencies");assertions++;}
  assert.equal(runtime.terminalIsEmpty(),true);assertions++;
 }
 console.log("[DEBUG] original Flow browser independent retained wallet cases="+fixture.cases.length+" assertions="+assertions+" strictAjv=true nativeUint32ArrayReference=true");
}
