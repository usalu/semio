/** 🧪️ Shared IFC edit vectors and independently applied JSON Patch results. */
import {expect, test} from "bun:test";
import {applyPatch} from "fast-json-patch";
import fixture from "./🔣️.json";
import type {IfcSnapshot, IfcValue} from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {applyIfcDiff, applyIfcDiffAsync, inverseIfcDiff, composeIfcDiff} from "../../🧬️schema/🔺️diff/🟦️.ts";
import {ifcMutationDiff, inverseIfcMutation, applyIfcMutation, IfcEditHistory} from "../../🧬️schema/🧬️mutations/🟦️.ts";

const integer = (value: number): IfcValue => ({kind:"integer", value:BigInt(value)});
function graph(): IfcSnapshot {
  return {schema:fixture.schema, header:{fileDescription:[], fileName:[], fileSchema:[{kind:"aggregate", value:[{kind:"string", value:fixture.fileSchema}]}]}, entities:[
    {id:1n,name:"IFCPROPERTYSINGLEVALUE",args:[{kind:"string",value:"Original"},{kind:"typedValue",value:{name:"IFCLIST",items:[{kind:"aggregate",value:[integer(1),integer(2)]}]}}],complex:[{name:"IFCROOT",args:[integer(9)]}]},
    {id:BigInt(fixture.entityIds[1]!),name:"IFCCARTESIANPOINT",args:fixture.originalArguments.map(integer),complex:[]},
  ]};
}
const wire = (value: unknown) => JSON.parse(JSON.stringify(value, (_, item) => typeof item === "bigint" ? item.toString() : item));

test("a mutation yields the independently verified sparse diff and undo", () => {
  const before=graph(), mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:{kind:"string" as const,value:"Edited"}};
  const diff=ifcMutationDiff(before,mutation), after=applyIfcDiff(before,diff);
  expect(diff).toEqual({entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value:mutation.value}]}}}]}});
  expect(wire(after)).toEqual(applyPatch(wire(before),fixture.jsonPatch as Parameters<typeof applyPatch>[1]).newDocument);
  expect(applyIfcDiff(after,inverseIfcDiff(before,diff))).toEqual(before);
  expect(applyIfcMutation(after,inverseIfcMutation(before,mutation))).toEqual(before);
  expect(before.entities[0]!.args[0]).toEqual({kind:"string",value:"Original"});
});

test("positional removals modifications and insertions use the declared coordinate spaces", () => {
  const before=graph(), id=before.entities[1]!.id;
  const diff={entities:{modified:[{id,diff:{args:{removed:[3,1],modified:[{index:2,value:integer(35)}],added:[{index:2,value:integer(15)},{index:0,value:integer(5)}]}}}]}};
  const after=applyIfcDiff(before,diff);
  expect(after.entities[1]!.args).toEqual(fixture.editedArguments.map(integer));
  expect(applyIfcDiff(after,inverseIfcDiff(before,diff))).toEqual(before);
});

test("nested typed values and complex constituents yield reversible argument and complex diffs", () => {
  const before=graph(), mutation={mutation:"setEntityValue" as const,id:1n,index:1,path:fixture.nestedPath,value:integer(77)};
  const after=applyIfcMutation(before,mutation);
  expect(applyIfcMutation(after,inverseIfcMutation(before,mutation))).toEqual(before);
  expect(after.entities[0]!.args[1]).toEqual({kind:"typedValue",value:{name:"IFCLIST",items:[{kind:"aggregate",value:[integer(1),integer(77)]}]}});
  const complex={mutation:"setEntityComplex" as const,id:1n,complex:[{name:"IFCROOT",args:[integer(12)]}]};
  expect(applyIfcMutation(applyIfcMutation(before,complex),inverseIfcMutation(before,complex))).toEqual(before);
});

test("entity replacement and ordered removal retain exact u64 identities under undo", () => {
  const before=graph(), diff={entities:{removed:[{id:1n,index:0}],added:[{index:1,entity:{id:1n,name:"IFCLABEL",args:[],complex:[]}}]}};
  const after=applyIfcDiff(before,diff);
  expect(after.entities.map(entity=>entity.id)).toEqual([18446744073709551615n,1n]);
  expect(applyIfcDiff(after,inverseIfcDiff(before,diff))).toEqual(before);
});

test("invalid targets values and references are refused atomically", () => {
  const before=graph(), saved=structuredClone(before);
  for(const diff of [
    {entities:{removed:[{id:1n,index:0},{id:1n,index:0}]}},
    {entities:{modified:[{id:99n,diff:{name:"IFCWALL"}}]}},
    {entities:{modified:[{id:1n,diff:{args:{modified:[{index:99,value:integer(1)}]}}}]}},
    {entities:{added:[{index:0,entity:before.entities[0]!}]}},
    {entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value:{kind:"reference" as const,value:99n}}]}}}]}},
    {fileSchema:[{kind:"aggregate" as const,value:[{kind:"string" as const,value:"IFC4"}]}]},
  ]) expect(()=>applyIfcDiff(before,diff)).toThrow();
  expect(before).toEqual(saved);
  expect(()=>ifcMutationDiff(before,{mutation:"setEntityValue",id:1n,index:0,path:[0],value:integer(4)})).toThrow();
  const cycle:IfcValue={kind:"aggregate",value:[]}; cycle.value.push(cycle);
  expect(()=>applyIfcMutation(before,{mutation:"setEntityArg",id:1n,index:0,value:cycle})).toThrow();
});

test("unchanged edits produce empty diffs and no history event", () => {
  const before=graph(), mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:before.entities[0]!.args[0]!};
  expect(ifcMutationDiff(before,mutation)).toEqual({});
  const history=IfcEditHistory.createSync(before);
  expect(history.execute(mutation,0)).toBeUndefined();
  expect(history.revision).toBe(0);
});

test("revision-checked event history supports undo redo and diverging local edits", () => {
  const before=graph(), history=IfcEditHistory.createSync(before);
  const event=history.execute({mutation:"setEntityArg",id:1n,index:0,value:{kind:"string",value:"Edited"}},0)!;
  expect(event.revision).toBe(1);
  expect(event.scope).toBe("persistedLocal");
  expect(()=>history.execute({mutation:"removeEntity",id:1n},0)).toThrow(/revision/);
  history.undo(1); expect(history.snapshot).toEqual(before);
  history.redo(2); expect(history.snapshot.entities[0]!.args[0]).toEqual({kind:"string",value:"Edited"});
  history.undo(3); history.execute({mutation:"setEntityName",id:1n,name:"IFCWALL"},4);
  expect(history.redo(5)).toBeUndefined();
  expect(history.events.map(event=>event.kind)).toEqual(["mutation","undo","redo","undo","mutation"]);
});

test("asynchronous application reports progress and cancellation without publishing state", async () => {
  const before=graph(), controller=new AbortController(), phases:string[]=[];
  await expect(applyIfcDiffAsync(before,{},{signal:controller.signal,yieldEvery:1,onProgress:progress=>{phases.push(progress.phase);controller.abort();}})).rejects.toThrow();
  expect(phases.length).toBeGreaterThan(0);
  expect(await applyIfcDiffAsync(before,{})).toEqual(before);
});

test("mixed collection diffs satisfy inverse laws for every removal mask and insertion position", () => {
  for(let mask=0;mask<16;mask++) for(let insert=0;insert<=4;insert++) {
    const before=graph(), removed=[0,1,2,3].filter(index=>mask&(1<<index)), survivors=4-removed.length;
    if(insert>survivors) continue;
    const modified=[0,1,2,3].filter(index=>!removed.includes(index)).map(index=>({index,value:integer(index+100)}));
    const diff={entities:{modified:[{id:before.entities[1]!.id,diff:{args:{removed,modified,added:[{index:insert,value:integer(999)}]}}}]}};
    expect(applyIfcDiff(applyIfcDiff(before,diff),inverseIfcDiff(before,diff))).toEqual(before);
  }
});

test("sequential composition transports mixed argument edits and inserted entities", () => {
  for(let mask=0;mask<16;mask++) {
    const before=graph(),id=before.entities[1]!.id,removed=[0,1,2,3].filter(index=>mask&(1<<index));
    const first={entities:{modified:[{id,diff:{args:{removed,added:[{index:0,value:integer(88)}]}}}],added:[{index:1,entity:{id:3n,name:"IFCPERSON",args:[integer(5)],complex:[]}}]}};
    const intermediate=applyIfcDiff(before,first),second={entities:{modified:[{id,diff:{args:{removed:[0],added:[{index:0,value:integer(99)}]}}},{id:3n,diff:{args:{modified:[{index:0,value:integer(9)}]}}}],removed:[{id:1n,index:0}]}};
    const final=applyIfcDiff(intermediate,second),composed=composeIfcDiff(before,first,second);
    expect(applyIfcDiff(before,composed)).toEqual(final);
    expect(applyIfcDiff(final,inverseIfcDiff(before,composed))).toEqual(before);
  }
});

test("composition cancels inserted then removed rows without losing original entity positions", () => {
  const before=graph(),first={entities:{removed:[{id:1n,index:0}],added:[{index:1,entity:{id:3n,name:"IFCPERSON",args:[],complex:[]}}]}},intermediate=applyIfcDiff(before,first);
  const second={entities:{removed:[{id:3n,index:1}],added:[{index:0,entity:{id:1n,name:"IFCROOT",args:[],complex:[]}}]}},composed=composeIfcDiff(before,first,second);
  expect(applyIfcDiff(before,composed)).toEqual(applyIfcDiff(intermediate,second));
  expect(composed.entities!.removed).toEqual([{id:1n,index:0}]);
  expect(composed.entities!.added!.map(item=>item.entity.id)).toEqual([1n]);
});

test("IFC binary values preserve every bit and reject noncanonical padding", () => {
  const before=graph(),mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:{kind:"binary" as const,value:"31"}};
  const after=applyIfcMutation(before,mutation);
  expect(after.entities[0]!.args[0]).toEqual(mutation.value);
  expect(applyIfcMutation(after,inverseIfcMutation(before,mutation))).toEqual(before);
  for(const value of ["", "4F", "3F", "1f", "00G", "1"])expect(()=>applyIfcMutation(before,{...mutation,value:{kind:"binary",value}})).toThrow();
  expect(applyIfcMutation(before,{...mutation,value:{kind:"binary",value:"0"}}).entities[0]!.args[0]).toEqual({kind:"binary",value:"0"});
});

test("undefined optional lanes remain absent during sequential composition", () => {
  const before=graph(),first={fileName:[{kind:"string" as const,value:"edit.ifc"}],entities:{modified:[{id:1n,diff:{name:"IFCWALL"}}]}},second={fileName:undefined,entities:{modified:[{id:1n,diff:{name:undefined,args:undefined,complex:undefined}}]}};
  expect(applyIfcDiff(before,composeIfcDiff(before,first,second))).toEqual(applyIfcDiff(applyIfcDiff(before,first),second));
});

test("mutation and diff decimal transports satisfy their schema-first contracts", async () => {
  const {default:Ajv}=await import("ajv/dist/2020.js"),ajv=new Ajv({strict:false});
  const snapshotSchema=await Bun.file(new URL("../../🧬️schema/🔣️.json",import.meta.url)).json();ajv.addSchema(snapshotSchema);
  ajv.addSchema(await Bun.file(new URL("../../🧬️schema/🏷️templates/✍️apply/🔣️.json",import.meta.url)).json());
  const diffSchema=await Bun.file(new URL("../../🧬️schema/🔺️diff/🔣️.json",import.meta.url)).json();ajv.addSchema(diffSchema);
  const mutations=ajv.compile(await Bun.file(new URL("../../🧬️schema/🧬️mutations/🔣️.json",import.meta.url)).json());
  const diffs=ajv.getSchema(diffSchema.$id)!;
  const before=graph(),cases=[
    {mutation:"setEntityValue" as const,id:1n,index:1,path:[0,1],value:integer(77)},
    {mutation:"setEntityArg" as const,id:1n,index:0,value:{kind:"binary" as const,value:"31"}},
    {mutation:"setEntityComplex" as const,id:1n,complex:[]},
    {mutation:"removeEntity" as const,id:1n},
  ];
  for(const mutation of cases){expect(mutations(wire(mutation))).toBe(true);expect(diffs(wire(ifcMutationDiff(before,mutation)))).toBe(true);}
  expect(mutations({mutation:"setEntityValue",id:"1",index:0,path:[0],value:{kind:"missing"}})).toBe(false);
  expect(diffs({entities:{removed:["1"]}})).toBe(false);
});

test("native scalar admission never coerces malformed transport values", () => {
  const before=graph();
  for(const malformed of [{kind:"binary",value:0},{kind:"enum",value:["T"]},{kind:"typedValue",value:{name:["IFCLABEL"],items:[]}}]) {
    expect(()=>applyIfcMutation(before,{mutation:"setEntityArg",id:1n,index:0,value:malformed as unknown as IfcValue})).toThrow();
  }
  expect(()=>applyIfcDiff(before,{entities:{modified:[{id:1n,diff:{name:["IFCWALL"] as unknown as string}}]}})).toThrow();
});


test("tiny value admission never expands a wide frontier before refusing",()=>{
 let reads=0;const children=new Proxy(Array.from({length:100000},()=>integer(1)),{get(target,key,receiver){if(typeof key==="string"&&/^\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}}),before=graph();before.entities[0]!.args=[{kind:"aggregate",value:children}];
 expect(()=>applyIfcDiff(before,{}, {maxValues:3})).toThrow(/valueLimit/);expect(reads).toBeLessThanOrEqual(3);
 const replacement={kind:"aggregate" as const,value:children};reads=0;expect(()=>applyIfcMutation(graph(),{mutation:"setEntityArg",id:1n,index:0,value:replacement},{maxValues:3})).toThrow();expect(reads).toBeLessThanOrEqual(3);
});
test("direct graph results own every mutable branch",()=>{
 const before=graph(),saved=structuredClone(before),after=applyIfcDiff(before,{});after.entities[0]!.name="IFCWALL";after.entities[0]!.args[0]={kind:"string",value:"Mutated"};after.entities[0]!.complex![0]!.args.push(integer(5));after.header.fileDescription.push(integer(6));expect(before).toEqual(saved);
});


import {inverseIfcDiffAsync,applyIfcDiffTransactionAsync} from "../../🧬️schema/🔺️diff/🟦️.ts";
import {applyIfcMutationAsync,ifcMutationDiffAsync,inverseIfcMutationAsync} from "../../🧬️schema/🧬️mutations/🟦️.ts";
test("all asynchronous command and inverse routes cooperate and return owned results",async()=>{
 const before=graph(),mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:{kind:"string" as const,value:"Async"}},diff=await ifcMutationDiffAsync(before,mutation),after=await applyIfcMutationAsync(before,mutation);expect(after).toEqual(applyIfcDiff(before,diff));expect(applyIfcDiff(after,await inverseIfcDiffAsync(before,diff))).toEqual(before);expect(await applyIfcMutationAsync(after,await inverseIfcMutationAsync(before,mutation))).toEqual(before);
 let validations=0;const transaction=await applyIfcDiffTransactionAsync(before,diff,{onProgress:p=>{if(p.phase==="validateBase"&&p.completed===1&&p.total===2)validations++;}});expect(validations).toBe(1);expect(applyIfcDiff(transaction.snapshot,transaction.inverse)).toEqual(before);
 for(const operation of[(options:any)=>IfcEditHistory.create(before,"persistedLocal",options),(options:any)=>ifcMutationDiffAsync(before,mutation,options),(options:any)=>applyIfcMutationAsync(before,mutation,options),(options:any)=>inverseIfcMutationAsync(before,mutation,options),(options:any)=>inverseIfcDiffAsync(before,diff,options)]){const controller=new AbortController();await expect(operation({signal:controller.signal,yieldEvery:1,onProgress:()=>controller.abort()})).rejects.toThrow(/cancelled/);}
});
test("asynchronous history publication is atomic under cancellation and competing revisions",async()=>{
 const before=graph(),history=await IfcEditHistory.create(before),mutation={mutation:"setEntityName" as const,id:1n,name:"IFCWALL"};
 const results=await Promise.allSettled([history.executeAsync(mutation,0,{yieldEvery:1}),history.executeAsync({...mutation,name:"IFCSLAB"},0,{yieldEvery:1})]);expect(results.filter(item=>item.status==="fulfilled").length).toBe(1);const rejected=results.find(item=>item.status==="rejected") as PromiseRejectedResult;expect(rejected.reason.code).toBe("staleRevision");expect(history.revision).toBe(1);
 const after=history.snapshot,events=history.events;
 const cancel=async(operation:(options:any)=>Promise<unknown>)=>{const controller=new AbortController();await expect(operation({signal:controller.signal,yieldEvery:1,onProgress:()=>controller.abort()})).rejects.toThrow(/cancelled/);};
 await cancel(options=>history.undoAsync(1,options));expect(history.snapshot).toEqual(after);expect(history.events).toEqual(events);await history.undoAsync(1);expect(await history.readSnapshot()).toEqual(before);await cancel(options=>history.redoAsync(2,options));expect(history.revision).toBe(2);await history.redoAsync(2);expect(history.snapshot).toEqual(after);
 await cancel(options=>history.readSnapshot(options));await cancel(options=>history.readEvents(options));expect(history.revision).toBe(3);const exported=await history.readSnapshot();exported.entities[0]!.args=[];expect(history.snapshot).toEqual(after);
});
test("wide command inputs are admitted before cloning and async cancellation interrupts cursor work",async()=>{
 let reads=0;const children=new Proxy(Array.from({length:100000},()=>integer(1)),{get(target,key,receiver){if(typeof key==="string"&&/^\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}}),before=graph(),mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:{kind:"aggregate" as const,value:children}};
 await expect(ifcMutationDiffAsync(before,mutation,{maxValues:3,yieldEvery:1})).rejects.toThrow(/valueLimit/);expect(reads).toBe(0);const controller=new AbortController();let turns=0;setTimeout(()=>controller.abort(),0);await expect(applyIfcMutationAsync(before,mutation,{yieldEvery:1,signal:controller.signal,onProgress:()=>turns++})).rejects.toThrow(/cancelled/);expect(turns).toBeGreaterThan(0);expect(reads).toBe(0);
});


import {absorbIfcDiffAsync,composeIfcDiffAsync} from "../../🧬️schema/🔺️diff/🟦️.ts";
test("asynchronous sparse composition retains independent patch output and can cancel sorting",async()=>{
 const before=graph(),first=ifcMutationDiff(before,{mutation:"setEntityArg",id:1n,index:0,value:{kind:"string",value:"Edited"}}),second={entities:{modified:[{id:1n,diff:{args:{added:[{index:1,value:integer(8)}]}}}]}};
 const combined=await composeIfcDiffAsync(before,first,second);expect(applyIfcDiff(before,combined)).toEqual(applyIfcDiff(applyIfcDiff(before,first),second));expect(wire(applyIfcDiff(before,await absorbIfcDiffAsync({},first)))).toEqual(applyPatch(wire(before),fixture.jsonPatch as Parameters<typeof applyPatch>[1]).newDocument);
 const controller=new AbortController();await expect(absorbIfcDiffAsync(first,second,{yieldEvery:1,signal:controller.signal,onProgress:p=>{if(p.phase==="applyEntities")controller.abort();}})).rejects.toThrow(/cancelled/);
});
test("late cancellation leaves all history lanes and immutable internal payloads unchanged",async()=>{
 const before=graph(),history=await IfcEditHistory.create(before),mutation={mutation:"setEntityName" as const,id:1n,name:"IFCWALL"};let enteredCopy=false;const controller=new AbortController();await expect(history.executeAsync(mutation,0,{yieldEvery:1,signal:controller.signal,onProgress:p=>{if(p.phase==="copyOwned"&&p.completed>=20){enteredCopy=true;controller.abort();}}})).rejects.toThrow(/cancelled/);expect(enteredCopy).toBe(true);expect(history.revision).toBe(0);expect(history.events).toEqual([]);expect(history.snapshot).toEqual(before);expect(await history.undoAsync(0)).toBeUndefined();expect(await history.redoAsync(0)).toBeUndefined();
 const event=(await history.executeAsync(mutation,0))!;event.diff.entities!.modified![0]!.diff.name="IFCSLAB";expect(history.events[0]!.diff.entities!.modified![0]!.diff.name).toBe("IFCWALL");await history.undoAsync(1);expect(history.snapshot).toEqual(before);
});
test("tiny budgets reject header roots and proposed values before wide scalar reads",async()=>{
 let reads=0;const children=new Proxy(Array.from({length:100000},()=>integer(1)),{get(target,key,receiver){if(typeof key==="string"&&/^\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}}),before=graph();before.entities=[];before.header.fileName=children;
 expect(()=>applyIfcDiff(before,{}, {maxValues:3})).toThrow(/valueLimit/);expect(reads).toBe(0);before.header.fileName=[];before.entities=[{id:1n,name:"IFCTEST",args:[],complex:[]}];await expect(ifcMutationDiffAsync(before,{mutation:"insertEntityArg",id:1n,index:0,value:{kind:"aggregate",value:children}},{maxValues:3})).rejects.toThrow(/valueLimit/);expect(reads).toBe(0);
});


test("caller mutation during copy cannot escape final owned structural admission",async()=>{
 for(const asynchronous of[false,true]){const before=graph();let changed=false;const options={yieldEvery:1,onProgress:(p:any)=>{if(p.phase==="copyOwned"&&!changed){changed=true;(before.entities[0]!.args[0] as any).kind="integer";(before.entities[0]!.args[0] as any).value="invalid";}}};if(asynchronous)await expect(applyIfcDiffAsync(before,{},options)).rejects.toThrow(/invalidValue/);else expect(()=>applyIfcDiff(before,{},options)).toThrow(/invalidValue/);expect(changed).toBe(true);}
});
test("history refuses capture corruption and records its captured command consistently",async()=>{
 const before=graph(),history=await IfcEditHistory.create(before),value:any=integer(2);let changed=false;await expect(history.executeAsync({mutation:"setEntityArg",id:1n,index:0,value},0,{yieldEvery:1,onProgress:p=>{if(p.phase==="copyOwned"&&!changed){changed=true;value.value="invalid";}}})).rejects.toThrow(/invalidValue/);expect(history.revision).toBe(0);expect(history.events).toEqual([]);expect(history.snapshot).toEqual(before);expect(await history.undoAsync(0)).toBeUndefined();
 const valid:any=integer(3);let mutated=false;const event=(await history.executeAsync({mutation:"setEntityArg",id:1n,index:0,value:valid},0,{yieldEvery:1,onProgress:p=>{if(p.phase==="copyOwned"&&!mutated){mutated=true;valid.value=4n;}}}))!;expect(applyIfcMutation(before,event.mutation!)).toEqual(history.snapshot);expect(applyIfcDiff(before,event.diff)).toEqual(history.snapshot);expect(applyIfcDiff(history.snapshot,event.inverse)).toEqual(before);
});
test("base-free absorption refuses malformed coordinate identity and overlap domains",async()=>{
 for(const diff of[
  {entities:{modified:[{id:1n,diff:{args:{removed:[NaN]}}}]}},
  {entities:{modified:[{id:1n,diff:{args:{removed:[-1]}}}]}},
  {entities:{modified:[{id:1n,diff:{args:{added:[{index:0,value:integer(1)},{index:0,value:integer(2)}]}}}]}},
  {entities:{modified:[{id:1n,diff:{args:{removed:[0],modified:[{index:0,value:integer(2)}]}}}]}},
  {entities:{modified:[{id:0n,diff:{name:"IFCWALL"}}]}},
  {entities:{modified:[{id:1n,diff:{name:"invalid name"}}]}},
  {entities:{removed:[{id:1n,index:0},{id:1n,index:0}]}},
  {entities:{removed:[{id:1n,index:0}],modified:[{id:1n,diff:{name:"IFCWALL"}}]}},
 ])await expect(absorbIfcDiffAsync(diff as any,{})).rejects.toThrow();
});


test("omitted complex fields admit native transport and restore canonical empty collections",async()=>{
 const before=graph();before.entities=[{id:BigInt(fixture.omittedComplex.id),name:fixture.omittedComplex.name,args:fixture.omittedComplex.args.map(value=>({kind:"integer",value:BigInt(value.value)}))}];const{default:Ajv}=await import("ajv/dist/2020.js"),validate=new Ajv({strict:false}).compile(await Bun.file(new URL("../../🧬️schema/🔣️.json",import.meta.url)).json());expect(validate(wire(before))).toBe(true);
 const canonical=applyIfcDiff(before,{}),mutation={mutation:"setEntityArg" as const,id:1n,index:0,value:integer(8)},diff=ifcMutationDiff(before,mutation),after=applyIfcMutation(before,mutation);expect(canonical.entities[0]!.complex).toEqual([]);expect(after.entities[0]!.complex).toEqual([]);expect(applyIfcDiff(after,inverseIfcDiff(before,diff))).toEqual(canonical);
 const oracle=applyPatch(wire(before),[{op:"add",path:"/entities/0/complex",value:[]},{op:"replace",path:"/entities/0/args/0/value",value:"8"}]).newDocument;expect(wire(after)).toEqual(oracle);const history=await IfcEditHistory.create(before);await history.executeAsync(mutation,0);await history.undoAsync(1);expect(history.snapshot).toEqual(canonical);
});


test.each(["copyOwned","validateBase","admitInput","applyEntities","validateResult"])("yielding phase %s admits only private owned graph and replay payloads",async phase=>{
 const before=graph(),saved=structuredClone(before),value:any=integer(2),diff={entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value}]}}}]}};let changed=false;try{const transaction=await applyIfcDiffTransactionAsync(before,diff,{yieldEvery:1,onProgress:p=>{if(p.phase===phase&&!changed){changed=true;value.value="invalid";(before.entities[0]!.args[0] as any).kind="integer";(before.entities[0]!.args[0] as any).value="invalid";}}});expect(applyIfcDiff(transaction.snapshot,{})).toEqual(transaction.snapshot);expect(applyIfcDiff(saved,transaction.diff)).toEqual(transaction.snapshot);expect(applyIfcDiff(transaction.snapshot,transaction.inverse)).toEqual(saved);}catch(error){expect((error as any).code).toBe("invalidValue");}expect(changed).toBe(true);
});
test("history final-phase caller corruption and cancelled preparation preserve valid replay",async()=>{
 const original=graph(),before=structuredClone(original),history=await IfcEditHistory.create(original),value:any=integer(2);const event=(await history.executeAsync({mutation:"setEntityArg",id:1n,index:0,value},0,{yieldEvery:1,onProgress:p=>{if(p.phase==="validateResult"){value.value="invalid";(original.entities[0]!.args[0] as any).value="invalid";}}}))!;const after=history.snapshot;expect(applyIfcMutation(before,event.mutation!)).toEqual(after);expect(applyIfcDiff(before,event.diff)).toEqual(after);
 const controller=new AbortController();await expect(history.undoAsync(1,{yieldEvery:1,signal:controller.signal,onProgress:p=>{event.diff.entities!.modified![0]!.diff.args!.modified![0]!.value={kind:"integer",value:8n};if(p.phase==="validateResult")controller.abort();}})).rejects.toThrow(/cancelled/);expect(history.revision).toBe(1);expect(history.snapshot).toEqual(after);await history.undoAsync(1);expect(history.snapshot).toEqual(before);await history.redoAsync(2);expect(history.snapshot).toEqual(after);
});
test("capture refuses undeclared fields before invoking their large getters",async()=>{
 const before=graph();let reads=0;Object.defineProperty(before.entities[0]!.args[0]!,"unexpected",{enumerable:true,get(){reads++;return Array.from({length:100000},()=>integer(1));}});await expect(applyIfcDiffAsync(before,{})).rejects.toHaveProperty("code","invalidValue");expect(reads).toBe(0);
});


test("captured sparse diffs own their payloads without rereading caller inputs",async()=>{
 const before=graph(),value=integer(9),first={entities:{modified:[{id:1n,diff:{args:{modified:[{index:0,value}]}}}]}};const absorbed=await absorbIfcDiffAsync(first,{});absorbed.entities!.modified![0]!.diff.args!.modified![0]!.value={kind:"integer",value:10n};expect(first.entities.modified[0]!.diff.args.modified[0]!.value).toEqual(integer(9));
 const planned=await ifcMutationDiffAsync(before,{mutation:"setEntityArg",id:1n,index:0,value});planned.entities!.modified![0]!.diff.args!.modified![0]!.value={kind:"integer",value:11n};expect(value).toEqual(integer(9));
});
