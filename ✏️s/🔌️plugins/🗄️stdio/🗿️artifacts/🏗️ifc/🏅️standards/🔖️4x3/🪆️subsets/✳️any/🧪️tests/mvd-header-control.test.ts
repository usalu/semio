/** 🧪️ Public released-view header wrappers preserve original authority across nested generators. */
import {test,expect} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import {applyPatch} from 'fast-json-patch';
import cases from '../🧬️schema/👁️mvd/header-control-cases.json';
import schema from '../🧬️schema/👁️mvd/header-control.schema.json';
import {ifc4x3MvdDeclarations,ifc4x3MvdDeclarationsAsync,ifc4x3MvdHeaderMutation,ifc4x3MvdHeaderMutationAsync} from '../🧬️schema/👁️mvd/header.ts';
import {IfcEditError,type IfcApplyOptions} from '../🧬️schema/🔺️diff/🟦️.ts';
import {applyIfcMutationAsync} from '../🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcValue} from '../🧬️schema/📸️snapshot/🟦️.ts';
import {base} from './template-fixtures.ts';
const values=():IfcValue[]=>[{kind:'aggregate',value:[{kind:'string',value:'ViewDefinition ['+cases.initialView+']'}]},{kind:'string',value:'2;1'}];
const routes:[string,(options:IfcApplyOptions)=>unknown][]=[['declarations-sync',o=>ifc4x3MvdDeclarations(values(),o)],['declarations-async',o=>ifc4x3MvdDeclarationsAsync(values(),o)],['selection-sync',o=>ifc4x3MvdHeaderMutation(values(),{view:'ReferenceView'},o)],['selection-async',o=>ifc4x3MvdHeaderMutationAsync(values(),{view:'ReferenceView'},o)]];
const result=async(run:()=>unknown)=>{try{return{failed:false,value:await run(),error:undefined};}catch(error){return{failed:true,value:undefined,error};}};
test('closed neutral corpus lists exactly the actual four wrappers',()=>{expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true);expect(routes.map(row=>row[0])).toEqual(cases.routes);});
for(const [name,run]of routes){
 test(name+' cannot raise original depth through callback closure',async()=>{let calls=0;const options:IfcApplyOptions={maxDepth:cases.originalDepth,onProgress(){calls++;options.maxDepth=cases.replacementDepth;}};const out=await result(()=>run(options));expect(calls).toBeGreaterThan(0);expect(out.failed).toBe(true);expect(out.error).toMatchObject({code:cases.depthRefusal});});
 test(name+' captures each original accessor once',async()=>{const reads=new Map<string,number>(),options={}as IfcApplyOptions;let calls=0;for(const key of cases.properties)Object.defineProperty(options,key,{get(){reads.set(key,(reads.get(key)??0)+1);return key==='onProgress'?()=>calls++:key==='yieldEvery'?8:undefined;}});expect((await result(()=>run(options))).failed).toBe(false);expect(calls).toBeGreaterThan(0);expect(Object.fromEntries(reads)).toEqual(Object.fromEntries(cases.properties.map(key=>[key,1])));});
 for(const fault of cases.faults)test(name+' original '+fault+' survives original receiver/closure replacement',async()=>{const abort=new AbortController(),failure=fault==='editError'?new IfcEditError('cancelled','original callback'):undefined;let calls=0,redirected=0,receiver:unknown='unset';const options:IfcApplyOptions={signal:abort.signal,onProgress:function(this:unknown){receiver=this;calls++;options.signal=new AbortController().signal;options.onProgress=()=>redirected++;if(fault==='abort')abort.abort();else throw failure;}};const out=await result(()=>run(options));expect(out.failed).toBe(true);if(fault==='abort')expect(out.error).toMatchObject({code:'cancelled'});else expect(out.error).toBe(failure);expect(calls).toBe(1);expect(redirected).toBe(0);expect(receiver).toBeUndefined();});
 test(name+' original value-zero is never replaced by first callback',async()=>{const options:IfcApplyOptions={maxValues:0,onProgress(){options.maxValues=100000;}};const out=await result(()=>run(options));expect(out.failed).toBe(true);expect(out.error).toMatchObject({code:'valueLimit'});});
 test(name+' zero entity admission remains valid for header-only values',async()=>expect((await result(()=>run({maxEntities:0}))).failed).toBe(false));
}
test('selected command applies through central mutation and independent JSON Patch',async()=>{const source=base();source.header.fileDescription=values();const command=await ifc4x3MvdHeaderMutationAsync(source.header.fileDescription,{view:'ReferenceView'}),next=await applyIfcMutationAsync(source,command),wire=(value:unknown)=>JSON.parse(JSON.stringify(value,(_,item)=>typeof item==='bigint'?String(item):item));const external=applyPatch(wire(source),[{op:'replace',path:'/header/fileDescription',value:wire(command.values)}],true,false).newDocument;expect(wire(next)).toEqual(external);expect(ifc4x3MvdDeclarations(next.header.fileDescription)[0]!.known).toEqual(['ReferenceView']);});
for(const [name,run]of routes)test(name+' capture failure precedes source traversal',async()=>{const failure={original:true},options=Object.defineProperty({},'onProgress',{get(){throw failure;}});const out=await result(()=>run(options));expect(out.failed).toBe(true);expect(out.error).toBe(failure);});
