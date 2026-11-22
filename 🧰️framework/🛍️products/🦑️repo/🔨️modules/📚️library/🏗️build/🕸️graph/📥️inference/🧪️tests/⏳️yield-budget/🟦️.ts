import {expect,spyOn,test} from "bun:test";
import {resolve} from "node:path";
import {withCommandSourceControlV1} from "../../../../🧪️tests/🕸️graph/🧰️support/🟦️.ts";
import {yieldInferenceV1} from "../../🧭️script-inputs/🟨️.mjs";
import {libraryBootstrap} from "../../🟨️.mjs";

const workspaceRoot=resolve(import.meta.dir,"../../../../../../../../../..");
/** ⏳️ Progress advances follow a time budget shared by every owner of one invocation: the model keeps the time of the last advance per invocation and advances when a forced call or the throttle window allows. */
const model=(calls:{at:number;force:boolean}[],window:number)=>{let last=-Infinity,advances=0;for(const call of calls)if(call.force||call.at-last>=window){last=call.at;advances++;}return advances;};
test("progress advances once per time window across the per-owner contexts of one invocation, forced calls always advance, equal to an independent model",async()=>{
 await libraryBootstrap;
 const window=(await import("../../🧬️schema/🟨️.mjs")).INFERENCE_LIMITS_V1.advanceThrottleMilliseconds;
 const calls=[0,10,20,30,250,260,270,520,530,531].map((at,index)=>({at,force:index===5}));
 await withCommandSourceControlV1(workspaceRoot,"yield-budget",600000,Error("stop"),async control=>{
  let now=0;const clock=spyOn(performance,"now").mockImplementation(()=>now);
  try{for(const call of calls){now=call.at;await yieldInferenceV1({invocation:control.invocation,phaseControl:control.phaseControl},"build:selected-yield-budget",0,call.force);}}finally{clock.mockRestore();}
  expect(control.events.filter(event=>event==="build:selected-yield-budget").length).toBe(model(calls,window));
  expect(model(calls,window)).toBeLessThan(calls.length);
 });
},60000);
