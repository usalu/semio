/** 🎟️ Independent raw-call receipt precedes every headless semantic failure or continuation. */
import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import Ajv from "ajv";
import law from "./🧫️fixtures/🔣️.json";

test("headless Run receives original call spending before semantic failure or continuation",()=>{
 const extent={type:"array",items:{type:"integer",minimum:0},minItems:5,maxItems:5},spent={...extent,minItems:4,maxItems:4};
 expect(new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:["input","spent","remaining","order","failure","semanticCompletion"],properties:{input:extent,spent,remaining:extent,order:{const:["receiveOriginalCall","inspectTypedCall","continueOrReport"]},failure:{const:"wholeKernelErrorAndOriginalFixedCall"},semanticCompletion:{const:"separateOriginalInput"}}})(law)).toBe(true);
 const db=new Database(":memory:");try{const remaining=db.query("select ?-? as items, ?-? as copy, ?-? as capacity, ?-? as release, ? as depth").get(law.input[0],law.spent[0],law.input[1],law.spent[1],law.input[2],law.spent[2],law.input[3],law.spent[3],law.input[4]);expect(Object.values(remaining as object)).toEqual(law.remaining);}finally{db.close();}
 const source=readModuleTreeSource(new URL("../../🦀️.rs",import.meta.url)),start=source.indexOf("ShardOutcome::OriginalCall {"),arm=source.slice(start,source.indexOf("ShardOutcome::Turn",start));
 expect(start).toBeGreaterThanOrEqual(0);expect(arm).toContain("receive_original_call");expect(arm.indexOf("receive_original_call")).toBeLessThan(arm.indexOf("original.kind"));
 expect(arm).toContain("RunError::Kernel");expect(arm).toContain("RunError::ActorCall");expect(arm).not.toContain("TurnResult {");expect(arm).not.toContain("Event::Wake");
 expect(source).toContain("Kernel(semio_framework_actor::KernelError)");expect(source).toContain("ActorCall(semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall)");
});
