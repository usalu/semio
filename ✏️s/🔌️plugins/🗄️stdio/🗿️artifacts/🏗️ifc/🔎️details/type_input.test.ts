import {test,expect} from 'bun:test';
import Ajv from 'ajv';
import Ajv2020 from 'ajv/dist/2020.js';
import {applyPatch} from 'fast-json-patch';
import {applyStagedFormBinding,stagedFormStatus} from '@semio-tech/framework/manifest/staging';
import contract from './🧪️tests/type-input.json';
import schema from './🧪️tests/type-input.schema.json';
import remap from './🧪️tests/primary-remap.json';
import guided from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-cases.json';
import guidedSchema from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type.schema.json';
import {typeInputForm,typeInputMutations} from './type_input.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
import type {StagedFormRequest} from '@semio-tech/framework/manifest/staging';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?String(item):item);
function native(value:unknown):IfcValue{const item=value as{kind:string,value?:unknown};if(item.kind==='reference'||item.kind==='integer')return{kind:item.kind,value:BigInt(String(item.value))};if(item.kind==='real')return{kind:'real',value:{bits:BigInt((item.value as{bits:string}).bits)}};if(item.kind==='aggregate')return{kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value)as IfcValue;}
function source(profileId='7'):IfcSnapshot{const record=remap.sources.constituent;return{schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:[{id:1n,name:record.name,args:record.args.map(native),complex:record.complex.map(part=>({name:part.name,args:part.args.map(native)}))},...remap.tail.map(entity=>({id:BigInt(entity.id==='7'?profileId:entity.id),name:entity.name,args:entity.args.map(native),complex:[]}))]};}
const token={action:'setSnapshotValue',window:'original-window',generation:'1',revision:'original-revision'},control=()=>({signal:new AbortController().signal});
for(const row of contract.cases)test('required type reference '+row.id,async()=>{
 const snapshot=source(row.profileId),request:StagedFormRequest={token,locale:'en',args:{path:'/entities/0',value:{$ifcType:contract.target,$ifcInputs:row.inputs}}};
 if(!row.admitted){await expect(typeInputMutations(snapshot,token.revision,request,control())).rejects.toThrow();return;}
 expect(new Ajv().compile(schema)(request.args.value)).toBe(true);
 const mutations=await typeInputMutations(snapshot,token.revision,request,control());expect(mutations.length).toBe(row.mutations??0);let result=snapshot;const inverses=[];for(const mutation of mutations){inverses.push(inverseIfcMutation(result,mutation));result=applyIfcMutation(result,mutation);}
 const args=[...JSON.parse(exact(snapshot.entities[0]!.args.slice(0,3))),{kind:'reference',value:row.profileId??'7'},{kind:'unset'},JSON.parse(exact(snapshot.entities[0]!.args[4]))];const oracle=applyPatch(JSON.parse(exact(snapshot)),[{op:'replace',path:'/entities/0/name',value:contract.target},{op:'replace',path:'/entities/0/args',value:args}],true,true).newDocument;expect(exact(result)).toBe(exact(oracle));for(const inverse of inverses.reverse())result=inverse.reduce(applyIfcMutation,result);expect(exact(result)).toBe(exact(snapshot));
});
test('actual guided binding supplies caller Profile without default ID and rejects stale source',async()=>{
 const snapshot=source(),request:StagedFormRequest={token,locale:'de',args:{path:'/entities/0',value:{$ifcType:contract.target,$ifcInputs:[]}}},before=exact(snapshot),form=await typeInputForm(snapshot,token.revision,request,control());
 expect(form.response.ready).toBe(false);expect(form.response.fields[0]?.label).toEqual({en:'Profile reference',de:'Profilreferenz'});expect(form.bindings[0]).toBeDefined();expect(exact(form)).not.toContain('"value":"7"');
 const args=applyStagedFormBinding(request.args,stagedFormStatus(form.response),form.bindings[0]!,'7'),next={...request,args},ready=await typeInputForm(snapshot,token.revision,next,control());expect(ready.response.ready).toBe(true);expect(ready.response.fields.length).toBe(1);expect((await typeInputMutations(snapshot,token.revision,next,control())).length).toBe(4);expect(exact(snapshot)).toBe(before);
 for(const partial of ['', '-', '07', '6', '999']){const draft=applyStagedFormBinding(next.args,stagedFormStatus(ready.response),ready.bindings[0]!,partial),result=await typeInputForm(snapshot,token.revision,{...next,args:draft},control());expect(result.response.ready).toBe(false);expect(result.response.fields.length).toBe(1);}
 await expect(typeInputForm(snapshot,'substituted-revision',request,control())).rejects.toThrow('staging.stale');const abort=new AbortController();abort.abort();await expect(typeInputForm(snapshot,token.revision,request,{signal:abort.signal})).rejects.toThrow();
});
test('actual controller neutral guidance cases agree with the executable form producer',async()=>{
 expect(new Ajv2020().compile(guidedSchema)(guided)).toBe(true);
 for(const row of guided.cases){const marker:Record<string,any>={$ifcType:guided.target};if(row.name!=='missing-inputs')marker.$ifcInputs=row.name==='null-inputs'?null:row.draft===null?[]:[{index:guided.index,value:{kind:'reference',value:row.draft}}];const request:StagedFormRequest={token,locale:'en',args:{path:guided.path,value:marker}};if(!row.output){await expect(typeInputForm(source(),row.name==='wrong-revision'?'substituted-revision':token.revision,request,control())).rejects.toThrow(row.name==='wrong-revision'?'staging.stale':'type.input-rows');continue;}const result=await typeInputForm(source(),token.revision,request,control());expect(result.response.ready).toBe(row.ready);expect(result.response.fields.length).toBe(1);expect(result.bindings[0]).toBeDefined();if(row.name==='missing-inputs'){const args=applyStagedFormBinding(request.args,stagedFormStatus(result.response),result.bindings[0]!,'7');expect((await typeInputForm(source(),token.revision,{...request,args},control())).response.ready).toBe(true);}}
});
test('guided required rows remain ascending when a later caller reference already exists',async()=>{
 const snapshot=source();snapshot.entities[0]={id:1n,name:'IFCRELASSOCIATESMATERIAL',args:[{kind:'reference',value:7n}],complex:[{name:'IFCREPRESENTATIONITEM',args:[]}]};for(const id of [8n,9n])snapshot.entities.push({id,name:'IFCBUILDINGELEMENTPROXY',args:[],complex:[]});
 const request:StagedFormRequest={token,locale:'en',args:{path:'/entities/0',value:{$ifcType:'IFCRELCONNECTSELEMENTS',$ifcInputs:[{index:'2',value:{kind:'reference',value:'8'}}]}}},first=await typeInputForm(snapshot,token.revision,request,control()),args=applyStagedFormBinding(request.args,stagedFormStatus(first.response),first.bindings[0]!,'9');
 expect((args.value as any).$ifcInputs.map((row:any)=>row.index)).toEqual(['1','2']);expect((await typeInputForm(snapshot,token.revision,{...request,args},control())).response.ready).toBe(true);
});
test('null input roster refuses and progress retains its original callback',async()=>{
 const request:StagedFormRequest={token,locale:'en',args:{path:'/entities/0',value:{$ifcType:contract.target,$ifcInputs:null}}};await expect(typeInputForm(source(),token.revision,request,control())).rejects.toThrow('type.input-rows');
 await expect(typeInputForm(source(),token.revision,{...request,args:{path:'/entities/0',value:{$ifcType:contract.target,$ifcInputs:[{index:'3',value:{kind:'reference',value:7}}]}}},control())).rejects.toThrow('type.input-row');
 let original=0,substituted=0;const owned={signal:new AbortController().signal,progress:()=>{original++;owned.progress=()=>{substituted++;};}};await typeInputForm(source(),token.revision,{...request,args:{path:'/entities/0',value:{$ifcType:contract.target}}},owned);expect(original).toBeGreaterThan(1);expect(substituted).toBe(0);
});
                                                                                                                                                                  