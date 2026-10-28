import {test,expect} from 'bun:test';
import cases from '../🧬️schema/🏷️templates/✍️apply/capture-cases.json';
import {base,application} from './template-fixtures.ts';
import {copyIfcOwnedSteps,captureIfcGraphSteps,captureIfcDiffSteps,runIfcEdit,runIfcEditAsync,applyIfcDiff,applyIfcDiffAsync} from '../🧬️schema/🔺️diff/🟦️.ts';
import {applyIfcMutation,applyIfcMutationAsync,inverseIfcMutation,inverseIfcMutationAsync,IfcEditHistory} from '../🧬️schema/🧬️mutations/🟦️.ts';
import Ajv from 'ajv/dist/2020';
const ajv=new Ajv({strict:false});
function descriptorOracle(object:object,array:any[]):boolean{
 const manifest=(value:object)=>{const descriptors=Object.getOwnPropertyDescriptors(value);return Reflect.ownKeys(descriptors).map(key=>{const descriptor=Reflect.get(descriptors,key);return{name:typeof key==='string'?key:null,data:Object.hasOwn(descriptor,'value'),enumerable:descriptor.enumerable};});};
 const row=(names?:string[],enumerable=true)=>({type:'object',required:['name','data','enumerable'],properties:{name:names?{enum:names}:{type:'string'},data:{const:true},enumerable:{const:enumerable}},additionalProperties:false});
 const objectAdmit=ajv.compile({type:'array',items:row()}),length=array.length,arrayAdmit=ajv.compile({type:'array',minItems:length+1,maxItems:length+1,items:{oneOf:[row(['length'],false),row(Array.from({length},(_,index)=>String(index)))]}});
 return !!objectAdmit(manifest(object))&&!!arrayAdmit(manifest(array));
}
function defect(name:string,object:any,array:any[],legalKey:string):()=>number{
 let invoked=0;const getter=()=>{invoked++;return'forbidden';};
 switch(name){
  case'hiddenGetter':Object.defineProperty(object,legalKey,{get:getter,configurable:true});break;
  case'hiddenAllowedData':Object.defineProperty(object,legalKey,{value:object[legalKey],enumerable:false,configurable:true});break;
  case'hiddenUnknownData':Object.defineProperty(object,'unknown',{value:true});break;
  case'objectSymbol':object[Symbol('unknown')]=true;break;
  case'arrayUnknownData':(array as any).unknown=true;break;
  case'arrayGetter':Object.defineProperty(array,'unknown',{get:getter,enumerable:true});break;
  case'arrayHiddenData':Object.defineProperty(array,'unknown',{value:true});break;
  case'arrayHiddenGetter':Object.defineProperty(array,'unknown',{get:getter});break;
  case'arraySymbol':(array as any)[Symbol('unknown')]=true;break;
  case'arrayIndexGetter':Object.defineProperty(array,'0',{get:getter,enumerable:true,configurable:true});break;
  case'arrayHiddenIndex':Object.defineProperty(array,'0',{value:array[0],enumerable:false,configurable:true});break;
  case'arrayNoncanonicalIndex':(array as any)['00']=true;break;
  case'arrayHole':delete array[0];break;
 }
 return()=>invoked;
}
for(const name of cases.reject)test(`all own keys are admitted: ${name}`,async()=>{
 for(const route of ['snapshot','diff','template','diffCommand']as const){
  const snapshot=base(),saved=structuredClone(snapshot),diff={fileName:[...snapshot.header.fileName]},app=application(),command=route==='template'?{mutation:'applyTemplate' as const,application:app}:{mutation:'applyDiff' as const,diff},input=route==='snapshot'?snapshot:route==='diff'?diff:command,object=route==='snapshot'?snapshot:route==='template'?app:diff,array=route==='snapshot'?snapshot.entities:route==='template'?app.entries:diff.fileName,legal=route==='snapshot'?'schema':route==='template'?'description':'fileName',invoked=defect(name,object,array,legal);
  expect(descriptorOracle(object,array)).toBe(cases.expected.accepted);
  expect(()=>runIfcEdit(copyIfcOwnedSteps(input,{},true),{})).toThrow('invalidValue');await expect(runIfcEditAsync(copyIfcOwnedSteps(input,{},true),{})).rejects.toThrow('invalidValue');
  if(route==='snapshot'){expect(()=>runIfcEdit(captureIfcGraphSteps(snapshot,'validateBase',{}),{})).toThrow('invalidValue');await expect(runIfcEditAsync(captureIfcGraphSteps(snapshot,'validateBase',{}),{})).rejects.toThrow('invalidValue');}
  else if(route==='diff'){expect(()=>runIfcEdit(captureIfcDiffSteps(diff,{}),{})).toThrow('invalidValue');await expect(runIfcEditAsync(captureIfcDiffSteps(diff,{}),{})).rejects.toThrow('invalidValue');expect(()=>applyIfcDiff(snapshot,diff)).toThrow('invalidValue');await expect(applyIfcDiffAsync(snapshot,diff)).rejects.toThrow('invalidValue');}
  else{
   expect(()=>applyIfcMutation(snapshot,command)).toThrow('invalidValue');await expect(applyIfcMutationAsync(snapshot,command)).rejects.toThrow('invalidValue');expect(()=>inverseIfcMutation(snapshot,command)).toThrow('invalidValue');await expect(inverseIfcMutationAsync(snapshot,command)).rejects.toThrow('invalidValue');
   for(const asynchronous of [false,true]){const history=await IfcEditHistory.create(saved);if(asynchronous)await expect(history.executeAsync(command,0)).rejects.toThrow('invalidValue');else expect(()=>history.execute(command,0)).toThrow('invalidValue');expect(history.revision).toBe(0);expect(history.events).toEqual([]);expect(history.snapshot).toEqual(saved);}
   expect(snapshot).toEqual(saved);
  }
  expect(invoked()).toBe(0);
 }
});
test('plain enumerable objects and dense frozen arrays remain valid owned data',async()=>{const snapshot=base();Object.freeze(snapshot.entities);expect(runIfcEdit(copyIfcOwnedSteps(snapshot,{},true),{})).toEqual(snapshot);expect(await runIfcEditAsync(captureIfcGraphSteps(snapshot,'validateBase',{}),{})).toEqual(snapshot);});
