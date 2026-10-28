import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/zero-slot-constituent-insertion.json';
import {ifc4x3Catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
for(const row of contract.cases)test('original zero-slot whole constituent insertion '+row.id,()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:'IFCREPRESENTATIONITEM',args:[],complex:[{name:'IFCGEOMETRICREPRESENTATIONITEM',args:[]},{name:'IFCPOINT',args:[]},{name:'IFCCARTESIANPOINT',args:[{kind:'aggregate',value:[{kind:'real',value:{bits:9223372036854775808n}},{kind:'real',value:{bits:4607182418800017408n}}]}]}]}]};source.entities.push({...structuredClone(source.entities[0]!),id:'duplicate'in row&&row.duplicate?source.entities[0]!.id:2n});const entity=source.entities[0]!,parts=entity.complex!,name=row.value.name.toUpperCase(),index=row.path.endsWith('/-')?parts.length:Number(row.path.split('/')[4]);
 const run=()=>{const declaration=ifc4x3Catalog.entities[name];if(!parts.length||!declaration||declaration.attributes.length||row.value.args.length||index>parts.length||source.entities.filter(item=>item.id===entity.id).length!==1)throw new Error('constituent-insertion');return {mutation:'setEntityComplex' as const,id:entity.id,complex:Array.from({length:parts.length+1},(_,ordinal)=>ordinal===index?{name,args:[]} :structuredClone(parts[ordinal-(ordinal>index?1:0)]!))};};
 if(!row.admitted){expect(run).toThrow();return;}const mutation=run(),next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'add',path:row.path,value:{name,args:[]}}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(next.entities[0]!.complex!.map(part=>part.name)).toEqual(('order'in row?row.order!:[]).map(index=>index<0?name:parts[index]!.name));expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
