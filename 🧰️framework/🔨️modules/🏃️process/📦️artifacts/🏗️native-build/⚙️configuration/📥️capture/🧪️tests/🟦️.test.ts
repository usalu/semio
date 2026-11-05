import {expect,test,spyOn} from "bun:test";
import {parse,type ParseError} from "jsonc-parser";
import AjvSchema from "ajv";
import {readCargoObjectCaptureV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};

/** 🧪️ Independent strict JSON syntax and object shape; no production parser is reused. */
function independent(text:string):Record<string,unknown>|undefined {
 const errors:ParseError[]=[],value=parse(text,errors,{disallowComments:true,allowTrailingComma:false,allowEmptyContent:false});
 return errors.length===0&&value!==null&&typeof value==="object"&&!Array.isArray(value)?value:undefined;
}
test("portable Cargo capture cases satisfy their schema",()=>{const validate=new AjvSchema().compile(schema);expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);});
for(const row of fixture.cases)test("one complete Cargo object: "+row.id,()=>{
 const expected=row.accepted?(row as {value:Record<string,unknown>}).value:undefined;
 const oracle=independent(row.text);
 expect(oracle).toEqual(expected);
 expect(readCargoObjectCaptureV1(row.text)).toEqual(oracle);
});

import responses from "./📬️responses.json" with {type:"json"};
import responseSchema from "../🧬️schema/📬️responses.json" with {type:"json"};
import {readCargoResponseObjectV1} from "../🟦️.ts";
test("portable Cargo response cases satisfy their schema",()=>{const validate=new AjvSchema().compile(responseSchema);expect(validate(responses),JSON.stringify(validate.errors)).toBe(true);});
for(const row of responses.cases)test("separated Cargo response: "+row.id,()=>{
 const expected=row.accepted?(row as {value:Record<string,unknown>}).value:undefined;
 const oracle=row.capture.status===0&&row.capture.signal===null?independent(row.capture.stdout):undefined;
 expect(oracle).toEqual(expected);
 expect(readCargoResponseObjectV1(row.capture)).toEqual(oracle);
});

import {readCargoObjectStreamV1} from "../🟦️.ts";
for(const row of responses.cases)test("streamed Cargo response: "+row.id,async()=>{
 const failure=Error("fixture owned terminal refused");
 const run=async(accept:(bytes:Uint8Array,stream:"stdout"|"stderr")=>void)=>{
  const stdout=new TextEncoder().encode(row.capture.stdout),stderr=new TextEncoder().encode(row.capture.stderr);
  for(let i=0;i<Math.max(stdout.length,stderr.length);i++){if(i<stdout.length)accept(stdout.subarray(i,i+1),"stdout");if(i<stderr.length)accept(stderr.subarray(i,i+1),"stderr");}
  if(row.capture.status!==0||row.capture.signal!==null)throw failure;
 };
 if(row.capture.status!==0||row.capture.signal!==null){await expect(readCargoObjectStreamV1(run)).rejects.toBe(failure);return;}
 expect(await readCargoObjectStreamV1(run)).toEqual(row.accepted?(row as {value:Record<string,unknown>}).value:undefined);
});

import streamCases from "./🧵️streams.json" with {type:"json"};
import streamSchema from "../🧬️schema/🧵️streams.json" with {type:"json"};
test("portable Cargo stream cases satisfy their schema",()=>{const validate=new AjvSchema().compile(streamSchema);expect(validate(streamCases),JSON.stringify(validate.errors)).toBe(true);});
for(const row of streamCases.cases)test("streamed Cargo refuses "+row.id,async()=>{
 const failure=Error("fixture owned "+row.terminal),run=async(accept:(bytes:Uint8Array,stream:"stdout"|"stderr")=>void)=>{
  for(const chunk of row.chunks)accept(new Uint8Array(chunk.bytes),chunk.stream as "stdout"|"stderr");
  if(row.terminal!=="success")throw failure;
 };
 if(row.terminal==="success")await expect(readCargoObjectStreamV1(run)).rejects.toThrow();
 else await expect(readCargoObjectStreamV1(run)).rejects.toBe(failure);
});

import planning from "./🗃️planning.json" with {type:"json"};
import planningSchema from "../🧬️schema/🗃️planning.json" with {type:"json"};
import {actualCargoCompilerPlanningPortV1} from "../../../🕸️units/📥️capture/🟦️.ts";
import {cargoLockStaleFailureV1} from "../../../../../🦀️cargo-workspace/🟦️.ts";
import {OwnedCommandFailure} from "../../../../../🎛️owned-execution/🟦️.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,withIssuedResourceControlV1,SCRIPT_PROCESS_INVOCATION_ENV} from "../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";

test("portable large Cargo planning data satisfies its schema",()=>{
 const validate=new AjvSchema().compile(planningSchema);
 expect(validate(planning),JSON.stringify(validate.errors)).toBe(true);
});
test("streamed Cargo planning accepts a complete response above the ordinary capture ceiling",async()=>{
 const text=JSON.stringify({padding:"x".repeat(planning.large.paddingBytes),label:planning.large.label}),bytes=new TextEncoder().encode(text);
 expect(bytes.length).toBeGreaterThan(planning.ordinaryCaptureMaximumBytes);
 expect(bytes.length).toBeLessThan(planning.planningMaximumBytes);
 const value=await readCargoObjectStreamV1(async accept=>{
  for(let offset=0;offset<bytes.length;offset+=planning.large.chunkBytes)accept(bytes.subarray(offset,offset+planning.large.chunkBytes),"stdout");
 });
 const oracle=independent(text);
 expect(value?.label).toBe(oracle?.label);
 expect((value?.padding as string).length).toBe((oracle?.padding as string).length);
 expect(value?.padding).toBe(oracle?.padding);
},planning.maximumElapsedMilliseconds);

const planningInvocation=<T>(run:(control:import("../../../../../🔒️leases/🟦️.ts").ResourceLeaseControl)=>Promise<T>,signal?:AbortSignal):Promise<T>=>withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"cargo-planning-stream-test",maximumElapsedMilliseconds:planning.maximumElapsedMilliseconds},{},Date.now()),invocation=>withIssuedResourceControlV1(invocation,"cargo:planning-capture",planning.maximumElapsedMilliseconds,run,signal));
test("actual Cargo planning streams large data with its same finite Original child envelope",async()=>{
 await planningInvocation(async control=>{
  const source='const key=process.argv[1],wire=process.env[key];process.stdout.write(JSON.stringify({padding:"x".repeat('+planning.large.paddingBytes+'),label:'+JSON.stringify(planning.large.label)+',envelope:wire?JSON.parse(wire):null}));';
  const value=await actualCargoCompilerPlanningPortV1().capture("node",["-e",source,"--",SCRIPT_PROCESS_INVOCATION_ENV],process.cwd(),{...process.env,[SCRIPT_PROCESS_INVOCATION_ENV]:undefined},control,planning.planningMaximumBytes) as Record<string,any>;
  expect(value.padding.length).toBe(planning.large.paddingBytes);
  expect(value.label).toBe(planning.large.label);
  expect(value.envelope?.policy.owner).toBe("cargo-planning-stream-test");
  expect(value.envelope?.deadlineEpochMilliseconds).toBeGreaterThan(Date.now());
  expect(value.envelope?.policy.maximumElapsedMilliseconds).toBeLessThanOrEqual(planning.maximumElapsedMilliseconds);
 });
},planning.maximumElapsedMilliseconds);

for(const row of planning.failures)test("actual Cargo planning preserves original "+row.id+" evidence",async()=>{
 const source='process.stdout.write("{}");process.stderr.write('+JSON.stringify(row.stderr)+');process.exitCode='+row.status+';';
 let failure:unknown;
 try{await planningInvocation(control=>actualCargoCompilerPlanningPortV1().capture("node",["-e",source],process.cwd(),process.env,control,planning.planningMaximumBytes));}catch(error){failure=error;}
 expect(failure).toBeInstanceOf(OwnedCommandFailure);
 expect((failure as OwnedCommandFailure).outcome).toEqual({version:1,status:row.status,signal:null});
 expect(cargoLockStaleFailureV1(failure)).toBe(row.stale);
 expect((failure as Error).cause).toBeInstanceOf(OwnedCommandFailure);
 expect(((failure as Error).cause as OwnedCommandFailure).outcome).toBe((failure as OwnedCommandFailure).outcome);
},planning.maximumElapsedMilliseconds);

for(const row of fixture.cases)test("streamed Cargo strict object parity: "+row.id,async()=>{
 const bytes=new TextEncoder().encode(row.text),value=await readCargoObjectStreamV1(async accept=>{
  for(let offset=0;offset<bytes.length;offset++)accept(bytes.subarray(offset,offset+1),"stdout");
 });
 expect(value).toEqual(independent(row.text));
});

test("actual Cargo planning never retries an output refusal as stale-lock failure",async()=>{
 const source='process.stderr.write('+JSON.stringify(planning.failures[0]!.stderr)+');process.stdout.write("x".repeat('+planning.refusal.bytes+'));setInterval(()=>{},1000);';
 let failure:unknown;
 try{await planningInvocation(control=>actualCargoCompilerPlanningPortV1().capture("node",["-e",source],process.cwd(),process.env,control,planning.refusal.maximumBytes));}catch(error){failure=error;}
 expect(String(failure)).toContain("output byte limit");
 expect(failure).not.toBeInstanceOf(OwnedCommandFailure);
 expect(cargoLockStaleFailureV1(failure)).toBe(false);
},planning.maximumElapsedMilliseconds);

test("actual Cargo planning refuses output quotes beyond its authored domain ceiling",async()=>{
 let failure:unknown;
 try{await planningInvocation(control=>actualCargoCompilerPlanningPortV1().capture("node",["-e",'process.stdout.write("{}")'],process.cwd(),process.env,control,planning.invalidMaximumBytes));}catch(error){failure=error;}
 expect(String(failure)).toContain("Cargo planning output bound");
 expect(failure).not.toBeInstanceOf(OwnedCommandFailure);
});

test("actual Cargo stale-lock recovery settles successfully under the same Original",async()=>{
 const stale=planning.failures.find(row=>row.stale)!,port=actualCargoCompilerPlanningPortV1();
 const value=await planningInvocation(async control=>{
  try{await port.capture("node",["-e",'process.stderr.write('+JSON.stringify(stale.stderr)+');process.exitCode='+stale.status+';'],process.cwd(),process.env,control,planning.planningMaximumBytes);}
  catch(error){if(!cargoLockStaleFailureV1(error))throw error;}
  return port.capture("node",["-e",'process.stdout.write('+JSON.stringify(fixture.cases[0]!.text)+');'],process.cwd(),process.env,control,planning.planningMaximumBytes);
 });
 expect(value).toEqual(independent(fixture.cases[0]!.text));
},planning.maximumElapsedMilliseconds);

test("Cargo planning preserves an actual Original abort reason even when it is an OwnedCommandFailure",async()=>{
 const port=actualCargoCompilerPlanningPortV1(),row=planning.failures.find(row=>!row.stale)!;
 let reason:unknown;
 try{await planningInvocation(control=>port.capture("node",["-e",'process.stderr.write('+JSON.stringify(row.stderr)+');process.exitCode='+row.status+';'],process.cwd(),process.env,control,planning.planningMaximumBytes));}catch(error){reason=error;}
 expect(reason).toBeInstanceOf(OwnedCommandFailure);
 const controller=new AbortController(),write=process.stderr.write;
 let publications=0,observed:unknown;
 const spy=spyOn(process.stderr,"write").mockImplementation(((...args:Parameters<typeof write>)=>{
  const text=typeof args[0]==="string"?args[0]:new TextDecoder().decode(args[0]);
  if(text.includes("cargo:planning-capture")&&++publications===planning.controlRefusal.abortAtPublication)controller.abort(reason);
  return write.apply(process.stderr,args);
 }) as typeof write);
 try{
  try{await planningInvocation(async control=>{
   try{return await port.capture("node",["-e",'process.stderr.write("capture fixture");process.stdout.write("{}");'],process.cwd(),process.env,control,planning.planningMaximumBytes);}
   catch(error){observed=error;throw error;}
  },controller.signal);}catch{}
 }finally{spy.mockRestore();}
 expect(controller.signal.aborted).toBe(true);
 expect(observed===reason).toBe(planning.controlRefusal.preserveIdentity);
},planning.maximumElapsedMilliseconds);
