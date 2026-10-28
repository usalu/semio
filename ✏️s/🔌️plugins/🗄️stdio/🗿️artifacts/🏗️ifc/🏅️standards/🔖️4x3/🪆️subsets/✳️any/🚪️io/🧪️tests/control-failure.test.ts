/** 🧨️ Caller observer values remain exact across STEP graph-domain catches and stream lifetime. */
import {beforeAll,expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import cases from "./control-failure-cases.json";
import schema from "./control-failure.schema.json";
import {captureIfcIoControlScope,runIfcIoSteps,IfcStepError,type IfcIoControl} from "../control.ts";
import {parseIfc4x3Step,writeIfc4x3Step,writeIfc4x3StepChunks} from "../step.ts";
import {readIfcGltfGlb,writeIfcGltfGlb} from "../gltf.ts";
import {IfcEditError} from "../../🧬️schema/🔺️diff/🟦️.ts";
let text:string,bytes:Uint8Array,source:Awaited<ReturnType<typeof parseIfc4x3Step>>,gltf:Awaited<ReturnType<typeof readIfcGltfGlb>>;
beforeAll(async()=>{text=await Bun.file(new URL("../../🧪️fixtures/official/extruded-solid.ifc",import.meta.url)).text();source=await parseIfc4x3Step(text,{batchSize:4096});bytes=new Uint8Array(await Bun.file(new URL("box.glb",import.meta.url)).arrayBuffer());gltf=await readIfcGltfGlb(bytes,{batchSize:4096});});
async function run(route:string,control:IfcIoControl):Promise<unknown>{
 if(route==="step-parse")return parseIfc4x3Step(text,control);
 if(route==="step-write"||route==="step-text")return writeIfc4x3Step(source,control);
 if(route==="step-stream"){let output="";for await(const chunk of writeIfc4x3StepChunks(source,control))output+=chunk;return output;}
 return route==="glb-read"?readIfcGltfGlb(bytes,control):writeIfcGltfGlb(gltf,control);
}
function failureValue(row:(typeof cases.failures)[number]):unknown{return row.kind==="edit"?new IfcEditError(row.code!,"original callback"):row.kind==="step"?new IfcStepError(row.code!,"original callback"):row.kind==="undefined"?undefined:row.kind==="number"?17:row.kind==="error"?new Error("original callback"):Object.freeze({original:true});}
test("closed neutral late-refusal matrix",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,nativeProof:true})).toBe(false);});
for(const row of cases.routes)for(const failure of cases.failures)test("late "+row.route+" "+failure.kind+" "+(failure.code??""),async()=>{
 const original=failureValue(failure);
 let reached=false,caught=false,actual:unknown;
 try{await run(row.route,{batchSize:1,progress:event=>{if(event.completed>0&&(row.phase==="capture-"?event.phase.startsWith(row.phase):event.phase===row.phase)){reached=true;throw original;}}});}catch(error){caught=true;actual=error;}
 expect(reached).toBe(true);expect(caught).toBe(true);expect(actual===original).toBe(cases.expected.exactOriginal);
});
for(const failure of cases.failures)test("generator cleanup retains original observer "+failure.kind+" "+(failure.code??""),async()=>{
 const original=failureValue(failure);let reached=false,closed=false,caught=false,actual:unknown;
 function* steps():Generator<void,boolean,void>{try{yield;yield;return true;}finally{closed=true;throw new Error("cleanup failure");}}
 try{await runIfcIoSteps(steps(),{batchSize:1,progress:event=>{if(event.completed>0){reached=true;throw original;}}},cases.cleanup.phase);}catch(error){caught=true;actual=error;}
 expect(reached).toBe(true);expect(caught).toBe(true);expect(closed).toBe(cases.cleanup.closed);expect(actual===original).toBe(cases.cleanup.exactOriginal);
});
test("original signal refusal survives a distinct generator cleanup error",async()=>{
 const abort=new AbortController();let closed=false,calls=0;
 function* steps():Generator<void,boolean,void>{try{yield;yield;return true;}finally{closed=true;throw new Error("cleanup failure");}}
 await expect(runIfcIoSteps(steps(),{signal:abort.signal,batchSize:1,progress:event=>{calls++;if(event.completed>0)abort.abort();}},cases.cleanup.phase)).rejects.toHaveProperty("code",cases.cleanup.cancelCode);
 expect(closed).toBe(cases.cleanup.closed);expect(calls).toBe(2);
});
for(const kind of ["observer","signal"] as const)test("entered finite yielding cleanup completes after "+kind+" refusal",async()=>{
 const abort=new AbortController();let entered=false,closed=false,calls=0,caught=false,actual:unknown;
 function* steps():Generator<void,boolean,void>{try{yield;yield;return true;}finally{entered=true;yield;yield;closed=true;if(kind==="observer")throw new Error("later cleanup failure");}}
 try{await runIfcIoSteps(steps(),{signal:abort.signal,batchSize:1,progress:event=>{calls++;if(event.completed>0){if(kind==="observer")throw undefined;abort.abort();}}},cases.cleanup.phase);}catch(error){caught=true;actual=error;}
 expect(caught).toBe(true);expect(entered).toBe(true);expect(closed).toBe(cases.cleanup.yieldedClose);expect(calls).toBe(2);
 if(kind==="observer")expect(actual).toBeUndefined();else expect(actual).toHaveProperty("code",cases.cleanup.cancelCode);
});
test("scoped swallowed first refusal cannot be overwritten by later observer work",async()=>{
 const first=Object.freeze({first:true}),later=Object.freeze({later:true});let calls=0,caught=false,actual:unknown;
 const scope=captureIfcIoControlScope({progress(){throw ++calls===1?first:later;}});
 try{await scope.run(async()=>{for(let completed=0;completed<2;completed++){try{scope.control.progress?.({phase:"original",completed,total:2});}catch{}}return true;});}catch(error){caught=true;actual=error;}
 expect(caught).toBe(true);expect(actual).toBe(first);expect(calls).toBe(cases.expected.firstCalls);
});
test("genuine STEP source diagnostics retain normalized domain codes",async()=>{
 const schemaSource=text.replace("'IFC4X3_ADD2'","'IFC2X3'");await expect(parseIfc4x3Step(schemaSource)).rejects.toHaveProperty("code","step.schema");
 await expect(parseIfc4x3Step(text.replace("DATA;","DATA;\n#999999=IFCDOOR(#999998);"))).rejects.toHaveProperty("code","step.reference");
 const badSchema=structuredClone(source);Reflect.set(badSchema,"schema","foreign");await expect(writeIfc4x3Step(badSchema)).rejects.toHaveProperty("code","step.graph");
 const badReference=structuredClone(source);badReference.entities[0]!.args.push({kind:"reference",value:18446744073709551615n});await expect(writeIfc4x3Step(badReference)).rejects.toHaveProperty("code","step.graph");
});
for(const row of cases.routes)test("original signal cancellation stays normalized "+row.route,async()=>{const abort=new AbortController();abort.abort();await expect(run(row.route,{signal:abort.signal})).rejects.toHaveProperty("code",cases.expected.cancelCode);});
