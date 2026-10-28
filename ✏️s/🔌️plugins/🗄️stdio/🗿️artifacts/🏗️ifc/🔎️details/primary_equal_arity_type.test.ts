import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/primary-equal-arity-type.json';
import {ifc4x3Catalog as catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueDomain} from '../📚️catalog/⚖️domain/🟦️.ts';
import {ifcSubtype} from '../📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation,type IfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as{kind:string,value?:unknown};if(item.kind==='real')return{kind:'real',value:{bits:BigInt((item.value as{bits:string}).bits)}};if(item.kind==='reference')return{kind:'reference',value:BigInt(String(item.value))};if(item.kind==='aggregate')return{kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value)as IfcValue;}
for(const row of contract.cases)test('original primary equal-arity type '+row.id,()=>{
 const record=contract.sources[row.source as keyof typeof contract.sources],source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:record.name,args:record.args.map(native),complex:record.complex.map(part=>({name:part.name,args:part.args.map(native)}))},...contract.tail.map(entity=>({id:BigInt(entity.id),name:entity.name,args:entity.args.map(native),complex:[]}))]},entity=source.entities[0]!,name=row.target.toUpperCase(),before=catalog.entities[entity.name]!,after=catalog.entities[name]!;
 const retained=!!entity.complex?.length&&before.attributes.length===after.attributes.length&&entity.args.length===before.attributes.length;
 expect(retained).toBe(row.retained);if(!retained)return;
 const run=():IfcMutation[]=>{if(name===entity.name)return[];for(const [index,attribute]of after.attributes.entries())checkIfcValueDomain(catalog,attribute.typeExpr,entity.args[index]!,{optional:attribute.optional,reference:function*(id,expected){yield;const matches=source.entities.filter(candidate=>candidate.id===id);if(matches.length!==1)return false;const candidate=matches[0]!,names=[candidate===entity?name:candidate.name,...(candidate.complex??[]).map(part=>part.name)];return names.some(actual=>ifcSubtype(catalog,actual,expected));}});return[{mutation:'setEntityName',id:entity.id,name}];};
 if(!row.admitted){expect(run).toThrow();return;}const mutations=run(),next=mutations.reduce(applyIfcMutation,source),oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/name',value:name}],true,true).newDocument;
 expect(mutations.length).toBe(row.mutations??0);expect(exact(next)).toBe(exact(oracle));expect(exact(next.entities[0]!.args)).toBe(exact(entity.args));expect(exact(next.entities[0]!.complex)).toBe(exact(entity.complex));expect(exact(mutations.flatMap(mutation=>inverseIfcMutation(source,mutation)).reduce(applyIfcMutation,next))).toBe(exact(source));
});
