/** 🔎️ Declared slot queries preserve original values, owner qualification and exact source positions. */
import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020';
import cases from './cases.json';
import schema from './schema.json';
import {ifcAttributeQuerySteps} from '../🟦️.ts';
import {ifc4x3Catalog} from '../../../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {runIfcIoSteps,type IfcIoControl} from '../../../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🚪️io/control.ts';
const drain=<T>(steps:Generator<void,T,void>):T=>{for(;;){const step=steps.next();if(step.done)return step.value;}};
test('declared query laws have a closed language-neutral schema',()=>expect(new Ajv({strict:true}).compile(schema)(cases)).toBe(true));
for(const row of cases.cases)test('declared source query '+row.id,()=>{
 const wrap=(part:{name:string;args:string[]})=>({name:part.name,args:part.args.map(marker=>({marker}))}),entity={...wrap(row.entity),complex:row.entity.complex?.map(wrap)},before=structuredClone(entity);
 const result=drain(ifcAttributeQuerySteps(ifc4x3Catalog,{entity,ordinal:7},row.name,row.owner));
 expect(String(result.status)).toBe(row.status);expect(result.matches.map(match=>match.value?.marker)).toEqual(row.values);expect(result.matches.map(match=>match.pointer)).toEqual(row.pointers);
 for(const match of result.matches){const part=match.part===0?entity:entity.complex![match.part-1]!;expect(match.value).toBe(part.args[match.index]);expect(match.declaration.declaredBy.toUpperCase()).toBe(row.owner.toUpperCase());}
 expect(entity).toEqual(before);
});
for(const ordinal of [-1,1.5,Number.NaN,Number.MAX_SAFE_INTEGER+1])test('query refuses invalid source ordinal '+ordinal,()=>expect(()=>drain(ifcAttributeQuerySteps(ifc4x3Catalog,{ordinal,entity:{name:'IFCRECTANGLEPROFILEDEF',args:[]}},'XDim'))).toThrow(RangeError));
test('locating declaration metadata does not discharge source value admission',()=>{
 const result=drain(ifcAttributeQuerySteps(ifc4x3Catalog,{ordinal:7,entity:{name:'IFCRECTANGLEPROFILEDEF',args:[]}},'XDim'));
 expect(result.status).toBe('unique');expect(result.matches[0]!.value).toBeUndefined();expect(result.matches[0]!.pointer).toBe('/entities/7/args/3');
});
for(const frontier of ['initial','interior','completion'])for(const fault of ['error','abort','undefined'])test('original source query '+frontier+' '+fault,async()=>{
 const entity={name:'IFCRECTANGLEPROFILEDEF',args:['x','y'],complex:Array.from({length:128},()=>({name:'IFCREPRESENTATIONITEM',args:[] as string[]}))},before=structuredClone(entity),events:number[]=[];
 const steps=()=>ifcAttributeQuerySteps(ifc4x3Catalog,{entity,ordinal:7},'XDim','IFCRECTANGLEPROFILEDEF');await runIfcIoSteps(steps(),{batchSize:128,progress:event=>events.push(event.completed)},'query');expect(events.length).toBeGreaterThan(2);
 const target=frontier==='initial'?0:frontier==='interior'?1:events.length-1,original=new AbortController(),replacement=new AbortController(),failure=fault==='error'?new Error('caller'):undefined;let calls=0,replacementCalls=0,caught=false;
 const control:IfcIoControl={signal:original.signal,batchSize:128,progress(){if(calls++!==target)return;control.signal=replacement.signal;control.progress=()=>replacementCalls++;if(fault==='abort')original.abort();else throw failure;}};
 try{await runIfcIoSteps(steps(),control,'query');}catch(error){caught=true;if(fault==='abort')expect(error).toMatchObject({code:'io.cancelled'});else expect(error).toBe(failure);}
 expect(caught).toBe(true);expect(calls).toBe(target+1);expect(replacementCalls).toBe(0);expect(entity).toEqual(before);
});
