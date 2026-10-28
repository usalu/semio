import {expect,test} from 'bun:test';
import Ajv from 'ajv/dist/2020.js';
import {applyStagedFormBinding,stagedFormStatus,type StagedFormRequest} from '@semio-tech/framework/manifest/staging';
import law from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-number-binary-cases.json';
import schema from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/✏️editor/guided-type-number-binary-cases.schema.json';
import models from './🧪️tests/type-number-binary-input.json';
import {typeInputForm} from './type_input.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
function value(input:unknown):IfcValue{const row=input as {kind:string;value?:unknown};if(row.kind==='reference'||row.kind==='integer')return{kind:row.kind,value:BigInt(String(row.value))};if(row.kind==='real')return{kind:'real',value:{bits:BigInt((row.value as {bits:string}).bits)}};if(row.kind==='aggregate')return{kind:'aggregate',value:(row.value as unknown[]).map(value)};return structuredClone(input) as IfcValue;}
function snapshot(name:keyof typeof models.sources):IfcSnapshot{return{schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[{kind:'aggregate',value:[{kind:'string',value:'IFC4X3_ADD2'}]}]},entities:models.sources[name].entities.map(row=>({id:BigInt(row.id),name:row.name,args:row.args.map(value),complex:row.complex.map(part=>({name:part.name,args:part.args.map(value)}))}))};}
const exact=(value:unknown)=>JSON.stringify(value,(_key,value)=>typeof value==='bigint'?String(value):value);
function at(root:unknown,path:readonly string[]):unknown{let value=root;for(const key of path){if(value===null||typeof value!=='object')return undefined;value=(value as Record<string,unknown>)[key];}return value;}
test('closed native/web numeric control vectors share the actual typed carrier schema',()=>{expect(new Ajv({strict:true}).compile(schema)(law)).toBe(true);});
for(const row of law.cases)test('actual numeric control path and editable raw carrier '+row.id,async()=>{
 const source=snapshot(row.source as keyof typeof models.sources),before=exact(source),request:StagedFormRequest={token:{action:'setSnapshotValue',revision:'source-revision',window:'original-window',generation:'1'},locale:'de',args:{path:'/entities/0',value:{$ifcType:row.target,$ifcInputs:row.input===null?[]:[{index:row.index,value:row.input}]}}};
 const args=exact(request.args),result=await typeInputForm(source,'source-revision',request,{signal:new AbortController().signal});
 expect(result.response.ready).toBe(row.ready);expect(result.response.fields.length).toBeLessThanOrEqual(16);
 expect(exact(result.response.fields.map(field=>{const current=at(request.args,field.path);return{kind:field.kind,format:field.kind==='text'?field.format:null,value:field.kind==='text'?current??'':current&&typeof current==='object'?(current as {kind?:string}).kind??null:null};}))).toBe(exact(row.fields));
 if(row.input!==null)for(const [index,field] of result.response.fields.entries()){
  const supplied=field.kind==='choice'?row.input.kind:at(request.args,field.path);expect(typeof supplied).toBe('string');
  expect(exact(applyStagedFormBinding(request.args,stagedFormStatus(result.response),result.bindings[index]!,supplied as string))).toBe(args);
 }
 expect(exact(request.args)).toBe(args);expect(exact(source)).toBe(before);
});
