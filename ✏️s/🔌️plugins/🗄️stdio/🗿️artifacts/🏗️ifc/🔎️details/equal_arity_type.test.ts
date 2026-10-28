import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/equal-arity-type.json';
import {ifc4x3Catalog as catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueDomain} from '../📚️catalog/⚖️domain/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as{kind:string,value?:unknown};if(item.kind==='real')return{kind:'real',value:{bits:BigInt((item.value as{bits:string}).bits)}};if(item.kind==='reference')return{kind:'reference',value:BigInt(String(item.value))};if(item.kind==='aggregate')return{kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value)as IfcValue;}
for(const row of contract.cases)test('original equal-arity constituent type '+row.id,()=>{
 const record=contract.sources[row.source as keyof typeof contract.sources],source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:record.name,args:record.args.map(native),complex:record.complex.map(part=>({name:part.name,args:part.args.map(native)}))},...contract.tail.map(entity=>({id:BigInt(entity.id),name:entity.name,args:entity.args.map(native),complex:[]}))]},entity=source.entities[0]!,part=entity.complex![2]!,name=row.target.toUpperCase(),before=catalog.entities[part.name]!,after=catalog.entities[name]!;
 const retained=row.path.startsWith('/entities/0/complex/')&&before.attributes.length===after.attributes.length&&part.args.length===before.attributes.length;
 expect(retained).toBe(row.retained);if(!retained)return;
 const run=()=>{if(name!==part.name)for(const [index,attribute]of after.attributes.entries())checkIfcValueDomain(catalog,attribute.typeExpr,part.args[index]!,{optional:attribute.optional});return{mutation:'setEntityComplex'as const,id:entity.id,complex:entity.complex!.map((item,index)=>({...structuredClone(item),name:index===2?name:item.name}))};};
 if(!row.admitted){expect(run).toThrow();return;}const mutation=run(),next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/complex/2/name',value:name}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(exact(next.entities[0]!.complex![2]!.args)).toBe(exact(part.args));expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
