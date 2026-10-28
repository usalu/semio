/** 🔐️ Original IO observers cannot receive or replace captured cancellation and finite grants. */
import {beforeAll,expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Box3} from "three";
import {GLTFLoader} from "three/examples/jsm/loaders/GLTFLoader.js";
import cases from "./control-authority-cases.json";
import schema from "./control-authority.schema.json";
import {captureIfcIoControl,captureIfcIoControlScope,runIfcIoSteps,type IfcIoControl} from "../control.ts";
import {parseIfc4x3Step,writeIfc4x3Step,writeIfc4x3StepChunks} from "../step.ts";
import {readIfcGltfGlb,writeIfcGltfGlb} from "../gltf.ts";
import {validateIfcBodySweptSolid} from "../../🧬️schema/👁️mvd/🧩️concepts/🌊️swept-solid/🟦️.ts";
let text:string,bytes:Uint8Array,graph:Awaited<ReturnType<typeof parseIfc4x3Step>>,gltf:Awaited<ReturnType<typeof readIfcGltfGlb>>;
beforeAll(async()=>{text=await Bun.file(new URL("../../🧪️fixtures/official/extruded-solid.ifc",import.meta.url)).text();bytes=new Uint8Array(await Bun.file(new URL("box.glb",import.meta.url)).arrayBuffer());graph=await parseIfc4x3Step(text);gltf=await readIfcGltfGlb(bytes);});
async function run(route:typeof cases.routes[number],control:IfcIoControl):Promise<unknown>{
 if(route==="step-parse")return parseIfc4x3Step(text,control);
 if(route==="step-write")return writeIfc4x3Step(graph,control);
 if(route==="step-stream"){let result="";for await(const chunk of writeIfc4x3StepChunks(graph,control))result+=chunk;return result;}
 if(route==="glb-read")return readIfcGltfGlb(bytes,control);
 if(route==="glb-write")return writeIfcGltfGlb(gltf,control);
 if(route==="body-concept")return validateIfcBodySweptSolid(graph,{targets:[{representation:"1020",applicable:true}]},control);
 return runIfcIoSteps((function*(){yield;yield;yield;return 3;})(),control,"authority");
}
test("closed language-neutral original-control contract",()=>{const validate=new Ajv({strict:true}).compile(schema);expect(validate(cases)).toBe(true);expect(validate({...cases,nativeProof:true})).toBe(false);});
for(const scoped of [false,true])test("all original fields are read once before observer work "+scoped,()=>{
 const first=new AbortController(),later=new AbortController();let firstCalls=0,laterCalls=0;const observer=()=>{firstCalls++;},replacement=()=>{laterCalls++;};
 const original:IfcIoControl={signal:first.signal,progress:observer,...cases.limits},other:IfcIoControl={signal:later.signal,progress:replacement,...Object.fromEntries(Object.entries(cases.limits).map(([key,value])=>[key,value+1]))};
 const reads:Record<string,number>={},input={}as IfcIoControl;for(const key of cases.properties)Object.defineProperty(input,key,{get(){reads[key]=(reads[key]??0)+1;return(reads[key]===1?original:other)[key as keyof typeof original];}});
 const scope=scoped?captureIfcIoControlScope(input):undefined,control=scope?.control??captureIfcIoControl(input);
 expect(reads).toEqual(Object.fromEntries(cases.properties.map(key=>[key,cases.expected.propertyReads])));expect(control.signal).toBe(first.signal);for(const [key,value]of Object.entries(cases.limits))expect(control[key as keyof IfcIoControl]).toBe(value);
 control.progress?.({phase:"authority",completed:0,total:0});expect(firstCalls).toBe(1);expect(laterCalls).toBe(0);expect(reads).toEqual(Object.fromEntries(cases.properties.map(key=>[key,1])));
});
for(const scoped of [false,true])test("captured grants and scoped control cannot be changed "+scoped,()=>{
 const source={...cases.limits,signal:new AbortController().signal,progress:()=>{}},scope=scoped?captureIfcIoControlScope(source):undefined,control=scope?.control??captureIfcIoControl(source);
 expect(Object.isFrozen(control)).toBe(true);for(const key of cases.properties){const value=control[key as keyof IfcIoControl];expect(Reflect.set(control,key,undefined)).toBe(cases.expected.capturedMutable);expect(Reflect.deleteProperty(control,key)).toBe(false);expect(control[key as keyof IfcIoControl]).toBe(value);}
 if(scope){expect(Object.isFrozen(scope)).toBe(true);expect(Reflect.set(scope,"control",{})).toBe(false);}
 expect(Object.isFrozen(source)).toBe(false);
});
for(const route of cases.routes)test("actual "+route+" refuses receiver replacement under the original abort",async()=>{
 const original=new AbortController(),replacement=new AbortController();let originalCalls=0,redirected=0;const receivers:unknown[]=[];
 const control={signal:original.signal,batchSize:1,progress:function(this:unknown){originalCalls++;receivers.push(this);if(this&&typeof this==="object"){Reflect.set(this,"signal",replacement.signal);Reflect.set(this,"progress",()=>redirected++);Reflect.set(this,"maxValues",Number.MAX_SAFE_INTEGER);}original.abort();}};
 await expect(run(route,control)).rejects.toHaveProperty("code",cases.expected.cancelCode);expect(originalCalls).toBe(cases.expected.originalCalls);expect(redirected).toBe(cases.expected.replacementCalls);expect(receivers).toEqual([undefined]);
});
for(const route of cases.routes)test("actual "+route+" retains the first getter observer",async()=>{
 const original=new AbortController();let reads=0,first=0,later=0;
 const control=Object.defineProperty({signal:original.signal,batchSize:1},"progress",{get(){reads++;return reads===1?()=>{first++;original.abort();}:()=>later++;}});
 await expect(run(route,control)).rejects.toHaveProperty("code",cases.expected.cancelCode);expect(reads).toBe(1);expect(first).toBe(1);expect(later).toBe(0);
});
for(const boundary of cases.boundaries)test("owned generator closes on original "+boundary+" cancellation",async()=>{
 const original=new AbortController(),replacement=new AbortController();let closed=false,reached=false,redirected=0;
 function* work(){try{yield;yield;yield;return 3;}finally{closed=true;}}
 const control={signal:original.signal,batchSize:1,progress:function(this:unknown,row:{completed:number,total:number}){if(!((boundary==="completion"&&row.total===3&&row.completed===3)||(boundary==="interior"&&row.completed===1&&row.total===0)))return;reached=true;if(this&&typeof this==="object"){Reflect.set(this,"signal",replacement.signal);Reflect.set(this,"progress",()=>redirected++);}original.abort();}};
 await expect(runIfcIoSteps(work(),control,"authority")).rejects.toHaveProperty("code",cases.expected.cancelCode);expect(reached).toBe(true);expect(closed).toBe(true);expect(redirected).toBe(0);
});
for(const route of cases.routes)for(const kind of cases.failureKinds)test("actual "+route+" retains original "+kind+" observer failure",async()=>{
 const failure=kind==="undefined"?undefined:kind==="number"?17:Object.freeze({original:true});let caught=false,actual:unknown;
 try{await run(route,{progress(){throw failure;}});}catch(error){caught=true;actual=error;}expect(caught).toBe(true);expect(actual).toBe(failure);
});
for(const failure of [undefined,17,Object.freeze({original:true})])test("scoped swallowed failure still refuses with exact identity "+String(failure),async()=>{
 const scope=captureIfcIoControlScope({progress(){throw failure;}});let caught=false,actual:unknown;
 try{await scope.run(async()=>{try{scope.control.progress?.({phase:"authority",completed:0,total:0});}catch{}return true;});}catch(error){caught=true;actual=error;}
 expect(caught).toBe(true);expect(actual).toBe(failure);
});
test("immutable control preserves original STEP and independently loaded GLB output",async()=>{
 expect(await parseIfc4x3Step(await writeIfc4x3Step(graph,{progress(){}}))).toEqual(graph);const rebuilt=await writeIfcGltfGlb(gltf,{progress(){}});expect((await readIfcGltfGlb(rebuilt)).document).toEqual(gltf.document);
 const load=(value:Uint8Array)=>new Promise<{scene:import("three").Group}>((resolve,reject)=>new GLTFLoader().parse(value.buffer.slice(value.byteOffset,value.byteOffset+value.byteLength)as ArrayBuffer,"",resolve,reject));
 const original=await load(bytes),after=await load(rebuilt);original.scene.updateMatrixWorld(true);after.scene.updateMatrixWorld(true);const left=new Box3().setFromObject(original.scene,true),right=new Box3().setFromObject(after.scene,true);expect(right.min.toArray()).toEqual(left.min.toArray());expect(right.max.toArray()).toEqual(left.max.toArray());
});
