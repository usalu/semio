/** 🧪️ External FEM capture owns source data before semantic callbacks and never executes source accessors. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import cases from "./external-source-cases.json";
import schema from "./external-source.schema.json";
import connectivity from "./fem-isolated-cases.json";
import {femConnectivityFixture} from "./fem-isolated-fixtures.ts";
import {copyIfcOwnedSteps,runIfcEdit,IfcEditError} from "../../🧬️schema/🔺️diff/🟦️.ts";
import {fem3dToIfc4x3} from "../fem.ts";
import type {IfcIoControl} from "../control.ts";

const source=()=>femConnectivityFixture(connectivity.cases.find(row=>row.id==="connected-frame")!);
const comparable=(value:any)=>{const owned=structuredClone(value);if(owned.artifact)owned.artifact.header.fileName[1]={kind:"string",value:"timestamp"};return owned;};
test("external source ownership vectors have a closed schema",()=>expect(new Ajv({strict:true,allErrors:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.policyCases)test("central external copier policy "+row.id,()=>{
 let value:any={a:1,b:2},policy={maxProperties:100,maxContainerDepth:16,plainDataOnly:true};
 if(row.kind==="properties")policy.maxProperties=1;
 if(row.kind==="depth"){value={a:{b:{}}};policy.maxContainerDepth=1;}
 if(row.kind==="function")value={value:()=>{throw Error("function executed");}};
 if(row.kind==="symbol")value={value:Symbol("source")};
 if(row.kind==="proto"){value={safe:true};Object.defineProperty(value,"__proto__",{value:{owned:true},enumerable:true});}
 if(row.kind==="aliases"){const shared={bits:0x8000000000000000n};value={a:shared,b:shared,array:[shared,shared],integer:9007199254740993n,number:-0,text:"𝄞"};}
 const copy=()=>runIfcEdit(copyIfcOwnedSteps(value,{},false,policy));
 if(row.code){let error:unknown;try{copy();}catch(caught){error=caught;}expect(error).toBeInstanceOf(IfcEditError);expect((error as IfcEditError).code).toBe(row.code);}
 else{
  const owned=copy();expect(owned).toEqual(value);expect(Object.getPrototypeOf(owned)).toBe(Object.prototype);
  if(row.kind==="proto"){expect(Object.hasOwn(owned,"__proto__")).toBe(true);expect((owned as any).owned).toBeUndefined();}
  if(row.kind==="aliases"){expect(owned.a).not.toBe(value.a);expect(owned.b).not.toBe(value.b);expect(owned.a).not.toBe(owned.b);expect(owned.array[0]).not.toBe(owned.array[1]);expect(Object.is(owned.number,-0)).toBe(true);}
 }
});
test("external copier admits exact property and container-depth boundaries",()=>{
 expect(runIfcEdit(copyIfcOwnedSteps({a:{b:1}},{},false,{maxProperties:2,maxContainerDepth:1,plainDataOnly:true}))).toEqual({a:{b:1}});
});
for(const phase of [cases.indexPhase,...cases.semanticPhases])test("FEM semantic callbacks cannot mutate captured source at "+phase,async()=>{
 const model=source(),expected=comparable(await fem3dToIfc4x3(source()));let mutated=false;
 const result=await fem3dToIfc4x3(model,{progress:progress=>{
  if(progress.phase!==phase||mutated)return;mutated=true;
  model.nodes=[];model.elements=[];model.materials=[];model.sections=[];model.supports=[];model.loadCases=[];
 }});
 expect(mutated).toBe(true);expect(comparable(result)).toEqual(expected);
});
test("FEM private capture expands mutable aliases before nested source mutation",async()=>{
 const model=source(),shared=model.nodes[0]!.x,fixed=model.supports[0]!.fixed;
 model.nodes[0]!.y=shared;model.nodes[0]!.z=shared;
 model.supports.push({id:"end-fixed",nodeId:"end",fixed});
 const expected=comparable(await fem3dToIfc4x3(structuredClone(model)));let changed=false;
 const actual=await fem3dToIfc4x3(model,{progress:progress=>{
  if(progress.phase!=="fem-export"||changed)return;changed=true;
  Object.defineProperty(shared,"bits",{value:0x3ff0000000000000n,writable:true,enumerable:true,configurable:true});
  fixed.length=0;model.elements[0]!.start="missing";
  Object.defineProperty(model.materials[0]!.e,"bits",{value:0n,enumerable:true});
  model.sections[0]!.name="Changed";model.loadCases[0]!.loads=[];
 }});
 expect(changed).toBe(true);expect(comparable(actual)).toEqual(expected);
});
test("FEM domain records and actual source container depth have exact admission boundaries",async()=>{
 const model=source();
 expect((await fem3dToIfc4x3(model,{maxEntities:8,maxDepth:5})).artifact).toBeDefined();
 for(const control of [{maxEntities:7},{maxDepth:4}]){
  const actual=await fem3dToIfc4x3(model,control);expect(actual.artifact).toBeUndefined();expect(actual.diagnostics[0]?.code).toBe(cases.sourceRefusal);
 }
 let properties=0;runIfcEdit(copyIfcOwnedSteps(model),{onProgress:progress=>{properties=progress.completed;}});
 expect((await fem3dToIfc4x3(model,{maxValues:properties})).artifact).toBeDefined();
 const denied=await fem3dToIfc4x3(model,{maxValues:properties-1});expect(denied.artifact).toBeUndefined();expect(denied.diagnostics[0]?.code).toBe(cases.sourceRefusal);
});
for(const variant of cases.accessors)test("FEM capture never invokes "+variant+" accessors",async()=>{
 const model=source();let reads=0;let owner:object=model,key="nodes";
 if(variant==="node"){owner=model.nodes[0]!;key="x";}
 if(variant==="member"){owner=model.elements[0]!;key="start";}
 if(variant==="material"){owner=model.materials[0]!;key="e";}
 if(variant==="support"){owner=model.supports[0]!;key="fixed";}
 if(variant==="loads"){owner=model.loadCases[0]!;key="loads";}
 if(variant==="word"){owner=model.nodes[0]!.x;key="bits";}
 if(variant==="constructor")key="constructor";
 if(variant==="array-constructor"){owner=model.nodes;key="constructor";}
 const value=Object.getOwnPropertyDescriptor(owner,key)?.value;
 Object.defineProperty(owner,key,{get:()=>{reads++;return value;},enumerable:true,configurable:true});
 let result;try{result=await fem3dToIfc4x3(model);}catch{}
 expect(reads).toBe(0);expect(result?.artifact).toBeUndefined();expect(result?.diagnostics[0]?.code).toBe(cases.sourceRefusal);
});
test("FEM capture rejects prototypes, function and symbol values before publication",async()=>{
 for(const variant of ["prototype","function","symbol","cycle"]){
  const model=source();
  if(variant==="prototype")Object.setPrototypeOf(model.nodes[0]!,{get x(){throw Error("prototype getter executed");}});
  if(variant==="function")Object.defineProperty(model,"extra",{value:()=>{throw Error("function executed");},enumerable:true});
  if(variant==="symbol")Object.defineProperty(model,"extra",{value:Symbol("source"),enumerable:true});
  if(variant==="cycle")Object.defineProperty(model,"extra",{value:model,enumerable:true});
  const result=await fem3dToIfc4x3(model);expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe(cases.sourceRefusal);
 }
});
test("FEM external-source limits refuse before semantic progress",async()=>{
 for(const limit of cases.limits){
  let semantic=false;const result=await fem3dToIfc4x3(source(),{...limit,progress:progress=>{if(cases.semanticPhases.includes(progress.phase))semantic=true;}});
  expect(result.artifact).toBeUndefined();expect(result.diagnostics[0]?.code).toBe(cases.sourceRefusal);expect(semantic).toBe(false);
 }
});
for(const phase of [cases.capturePhase,cases.indexPhase])test("FEM "+phase+" retains original cancellation authority",async()=>{
 const model=source(),before=structuredClone(model),original=new AbortController(),replacement=new AbortController();let copied=false,semantic=false,replacements=0;
 const control:IfcIoControl={signal:original.signal,batchSize:1,progress:progress=>{
  if(cases.semanticPhases.includes(progress.phase))semantic=true;
  if(progress.phase===phase&&progress.completed>=2){copied=true;control.signal=replacement.signal;control.progress=()=>replacements++;original.abort();}
 }};
 await expect(fem3dToIfc4x3(model,control)).rejects.toHaveProperty("code",cases.cancelled);
 expect(copied).toBe(true);expect(semantic).toBe(false);expect(replacements).toBe(0);expect(model).toEqual(before);
});
