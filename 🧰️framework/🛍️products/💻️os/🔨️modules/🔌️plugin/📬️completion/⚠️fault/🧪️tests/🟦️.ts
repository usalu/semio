/** ⚠️ Completion fault projection law: the report keeps the exact original diagnostic and matches an independent JSON wire built with Ajv-validated fixture limits. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import completionFaultCases from "./🧫️fixtures/🔣️.json";
import completionFaultSchema from "../🧬️schema/🔣️.json";
import {createCompletionFault} from "../🟦️.ts";
test("completion fault projection retains original diagnostic while matching independent JSON wire",()=>{
 expect(new Ajv({strict:false}).compile(completionFaultSchema)(completionFaultCases)).toBe(true);
 for(const row of completionFaultCases.cases){
  const original={...row,origin:"app" as const,severity:"error" as const,scope:{module:"draw"},causes:[{message:"original retained cause"}]};
  const actual=createCompletionFault(original);expect(actual.original).toBe(original);expect(actual.original.scope).toBe(original.scope);
  const escaped=(text:string)=>Buffer.byteLength(JSON.stringify(text))-2;
  const code=escaped(row.code)<=completionFaultCases.codeEscapeBytes?row.code:"interactive-job.fault-capacity";
  const expected={origin:"app",code,severity:"error",message:"",scope:{},retainedProgress:{copiedItems:0,copiedBytes:0,retainedCapacityBytes:0,releasedBytes:0},retryable:row.retryable};
  const limit=Math.min(completionFaultCases.messageEscapeBytes,completionFaultCases.frameBytes-Buffer.byteLength(JSON.stringify(expected)));
  for(const scalar of row.message){const candidate=expected.message+scalar;if(escaped(candidate)>limit)break;expected.message=candidate;}
  expect(actual.report).toBe(JSON.stringify(expected));
  expect(Buffer.byteLength(actual.report)).toBeLessThanOrEqual(completionFaultCases.frameBytes);
 }
});
