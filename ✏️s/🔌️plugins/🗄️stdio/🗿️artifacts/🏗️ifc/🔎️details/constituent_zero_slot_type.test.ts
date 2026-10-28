import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/constituent-zero-slot-type.json';
import {ifc4x3Catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
for(const row of contract.cases)test('original zero-slot constituent type '+row.id,()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:'IFCREPRESENTATIONITEM',args:[],complex:contract.names.map((name,index)=>({name,args:index===2?[{kind:'aggregate',value:[{kind:'real',value:{bits:9223372036854775808n}},{kind:'real',value:{bits:4607182418800017408n}}]}]:[]}))}]};source.entities.push({...structuredClone(source.entities[0]!),id:'duplicate'in row&&row.duplicate?source.entities[0]!.id:2n});
 const entity=source.entities[0]!,parts=entity.complex!,name=(typeof row.value==='string'?row.value:row.value.$ifcType).toUpperCase();
 const run=()=>{const before=ifc4x3Catalog.entities[parts[1]!.name],after=ifc4x3Catalog.entities[name];if(!before||!after||before.attributes.length||after.attributes.length||parts[1]!.args.length||source.entities.filter(item=>item.id===entity.id).length!==1)throw new Error('constituent-type');return {mutation:'setEntityComplex' as const,id:entity.id,complex:parts.map((part,index)=>({...structuredClone(part),name:index===1?name:part.name}))};};
 if(!row.admitted){expect(run).toThrow();return;}const mutation=run(),next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/complex/1/name',value:name}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(next.entities[0]!.complex![1]!.name).toBe(row.name);expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
