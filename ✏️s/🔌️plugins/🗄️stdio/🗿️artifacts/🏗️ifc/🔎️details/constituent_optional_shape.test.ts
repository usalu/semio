import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/constituent-optional-shape.json';
import {ifc4x3Catalog as catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueDomain} from '../📚️catalog/⚖️domain/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as{kind:string,value?:unknown};if(item.kind==='real')return{kind:'real',value:{bits:BigInt((item.value as{bits:string}).bits)}};if(item.kind==='reference')return{kind:'reference',value:BigInt(String(item.value))};if(item.kind==='aggregate')return{kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value)as IfcValue;}
for(const row of contract.cases)test('original constituent optional shape '+row.id,()=>{
 const record=contract.sources[row.source as keyof typeof contract.sources],source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt(row.entityId??'1'),name:record.name,args:record.args.map(native),complex:record.complex.map(part=>({name:part.name,args:part.args.map(native)}))}]},entity=source.entities[0]!,part=entity.complex![2]!,name=row.target.toUpperCase(),old=catalog.entities[part.name]!,next=catalog.entities[name]!;
 const retained=row.path.startsWith('/entities/0/complex/')&&old.attributes.length===part.args.length&&old.attributes.length!==next.attributes.length&&(next.attributes.length===0||old.attributes.length===0&&next.attributes.every(attribute=>attribute.optional&&!attribute.derived&&attribute.name!=='GlobalId'));
 expect(retained).toBe(row.retained);if(!retained)return;
 const args:IfcValue[]=next.attributes.map(attribute=>{const value:IfcValue={kind:'unset'};checkIfcValueDomain(catalog,attribute.typeExpr,value,{optional:attribute.optional});return value;});expect(exact(args)).toBe(exact(row.args));
 const mutation={mutation:'setEntityComplex'as const,id:entity.id,complex:entity.complex!.map((item,index)=>index===2?{name,args}:{...structuredClone(item)})},nextSource=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/complex/2/name',value:name},{op:'replace',path:'/entities/0/complex/2/args',value:row.args}],true,true).newDocument;
 expect(exact(nextSource)).toBe(exact(oracle));expect(exact(nextSource.entities[0]!.args)).toBe(exact(entity.args));expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,nextSource))).toBe(exact(source));
});
