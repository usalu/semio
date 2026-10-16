/** ⚠️ Completion fault projection law: the report keeps the exact original diagnostic and matches an independent JSON wire built with Ajv-validated fixture limits. */
import {expect,test} from "bun:test";
import Ajv from "ajv";
import completionFaultCases from "./🧫️fixtures/🔣️.json";
import completionFaultSchema from "../🧬️schema/🔣️.json";
import {createCompletionFault} from "../🟦️.ts";
test("completion fault projection retains original diagnostic while matching independent JSON wire",()=>{
 expect(new Ajv({strict:false}).compile(completionFaultSchema)(completionFaultCases)).toBe(true);
 for(const row of completionFaultCases.cases){const original={...row,origin:"app" as const,severity:"error" as const,scope:{module:"draw"},causes:[{message:"original retained cause"}]};const actual=createCompletionFault(original);expect(actual.original).toBe(original);expect(actual.original.scope).toBe(original.scope);const wire=JSON.parse(actual.report) as {message:string;code:string};let expected="";for(const scalar of original.message){const candidate=expected+scalar;if(Buffer.byteLength(JSON.stringify(candidate))-2>completionFaultCases.messageEscapeBytes)break;expected=candidate;}expect(wire.message).toBe(expected);const code=Buffer.byteLength(JSON.stringify(row.code))-2<=completionFaultCases.codeEscapeBytes?row.code:"interactive-job.fault-capacity";expect(wire.code).toBe(code);expect(actual.report).toBe(JSON.stringify({origin:"app",code,severity:"error",message:expected,scope:{},retryable:row.retryable}));expect(Buffer.byteLength(actual.report)).toBeLessThanOrEqual(completionFaultCases.frameBytes);}
});
