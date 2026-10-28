import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/constituent-move.json';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
for(const row of contract.cases)test('original complete constituent move '+row.id,()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:'IFCREPRESENTATIONITEM',args:[],complex:contract.names.map((name,index)=>({name,args:index===2?[{kind:'aggregate',value:[{kind:'real',value:{bits:9223372036854775808n}},{kind:'real',value:{bits:4607182418800017408n}}]}]:[]}))}]};source.entities.push({...structuredClone(source.entities[0]!),id:'duplicate'in row&&row.duplicate?source.entities[0]!.id:2n});
 const selected=source.entities[0]!,parts=selected.complex!,origin=row.from.split('/'),destination=row.path.split('/'),from=Number(origin[4]),to=destination[4]==='-'?parts.length-1:Number(destination[4]);
 const run=()=>{if(origin.length!==5||destination.length!==5||origin.slice(0,4).join('/')!==destination.slice(0,4).join('/')||from>=parts.length||to>=parts.length||source.entities.filter(entity=>entity.id===selected.id).length!==1)throw new Error('constituent-move');const order=Array.from({length:parts.length},(_,index)=>index===to?from:from<to&&index>=from&&index<to?index+1:from>to&&index>to&&index<=from?index-1:index);return {mutation:'setEntityComplex' as const,id:selected.id,complex:order.map(index=>structuredClone(parts[index]!))};};
 if(!row.admitted){expect(run).toThrow();return;}const mutation=run(),next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'move',from:row.from,path:row.path}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(next.entities[0]!.complex!.map(part=>part.name)).toEqual(('order'in row?row.order!:[]).map(index=>contract.names[index]!));expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
