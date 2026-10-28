import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/primary-zero-slot-type.json';
import {ifc4x3Catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation,type IfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
for(const row of contract.cases)test('original primary zero-slot type '+row.id,()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:'IFCREPRESENTATIONITEM',args:[],complex:[{name:'IFCGEOMETRICREPRESENTATIONITEM',args:[]},{name:'IFCPOINT',args:[]},{name:'IFCCARTESIANPOINT',args:[{kind:'aggregate',value:[{kind:'real',value:{bits:9223372036854775808n}},{kind:'real',value:{bits:4607182418800017408n}}]}]}]}]};source.entities.push({...structuredClone(source.entities[0]!),id:'duplicate'in row&&row.duplicate?source.entities[0]!.id:2n});const entity=source.entities[0]!,name=row.value.$ifcType.toUpperCase();
 const run=():IfcMutation[]=>{const before=ifc4x3Catalog.entities[entity.name],after=ifc4x3Catalog.entities[name];if(!entity.complex?.length||entity.args.length||!before||!after||before.attributes.length||after.attributes.length||source.entities.filter(item=>item.id===entity.id).length!==1)throw new Error('primary-type');return name===entity.name?[]:[{mutation:'setEntityName',id:entity.id,name}];};
 if(!row.admitted){expect(run).toThrow();return;}const mutations=run(),next=mutations.reduce(applyIfcMutation,source),oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/name',value:name}],true,true).newDocument;
 expect(mutations.length).toBe(row.mutations);expect(exact(next)).toBe(exact(oracle));expect(exact(next.entities[0]!.complex)).toBe(exact(entity.complex));expect(exact(mutations.flatMap(mutation=>inverseIfcMutation(source,mutation)).reduce(applyIfcMutation,next))).toBe(exact(source));
});
