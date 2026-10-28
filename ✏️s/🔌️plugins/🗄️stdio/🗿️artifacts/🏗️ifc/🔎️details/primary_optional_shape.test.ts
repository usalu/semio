import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/primary-optional-shape.json';
import {ifc4x3Catalog as catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueDomain} from '../📚️catalog/⚖️domain/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation,type IfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as{kind:string,value?:unknown};if(item.kind==='real')return{kind:'real',value:{bits:BigInt((item.value as{bits:string}).bits)}};if(item.kind==='reference')return{kind:'reference',value:BigInt(String(item.value))};if(item.kind==='aggregate')return{kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value)as IfcValue;}
for(const row of contract.cases)test('original primary optional shape '+row.id,()=>{
 const record=contract.sources[row.source as keyof typeof contract.sources],source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt(row.entityId??'1'),name:record.name,args:record.args.map(native),complex:record.complex.map(part=>({name:part.name,args:part.args.map(native)}))}]},entity=source.entities[0]!,name=row.target.toUpperCase(),old=catalog.entities[entity.name]!,next=catalog.entities[name]!;
 const retained=!!entity.complex?.length&&old.attributes.length===entity.args.length&&old.attributes.length!==next.attributes.length&&(next.attributes.length===0||old.attributes.length===0&&next.attributes.every(attribute=>attribute.optional&&!attribute.derived&&attribute.name!=='GlobalId'));
 expect(retained).toBe(row.retained);if(!retained)return;
 const args:IfcValue[]=next.attributes.map(attribute=>{const value:IfcValue={kind:'unset'};checkIfcValueDomain(catalog,attribute.typeExpr,value,{optional:attribute.optional});return value;});expect(exact(args)).toBe(exact(row.args));
 const mutations:IfcMutation[]=[{mutation:'setEntityName',id:entity.id,name}];if(args.length)args.forEach((value,index)=>mutations.push({mutation:'insertEntityArg',id:entity.id,index,value}));else for(let index=entity.args.length-1;index>=0;index--)mutations.push({mutation:'removeEntityArg',id:entity.id,index});
 const inverses:IfcMutation[]=[];let result=source;for(const mutation of mutations){inverses.unshift(...inverseIfcMutation(result,mutation));result=applyIfcMutation(result,mutation);}
 const oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/name',value:name},{op:'replace',path:'/entities/0/args',value:row.args}],true,true).newDocument;
 expect(mutations.length).toBe(row.mutations??0);expect(exact(result)).toBe(exact(oracle));expect(exact(result.entities[0]!.complex)).toBe(exact(entity.complex));expect(exact(inverses.reduce(applyIfcMutation,result))).toBe(exact(source));
});
