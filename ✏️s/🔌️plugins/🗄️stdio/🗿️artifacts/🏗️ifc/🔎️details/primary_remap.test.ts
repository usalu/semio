import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/primary-remap.json';
import {ifc4x3Catalog as catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueDomain} from '../📚️catalog/⚖️domain/🟦️.ts';
import {ifcSubtype} from '../📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as{kind:string,value?:unknown};if(item.kind==='real')return{kind:'real',value:{bits:BigInt((item.value as{bits:string}).bits)}};if(item.kind==='integer'||item.kind==='reference')return{kind:item.kind,value:BigInt(String(item.value))};if(item.kind==='aggregate')return{kind:'aggregate',value:(item.value as unknown[]).map(native)};if(item.kind==='typedValue'){const typed=item.value as{name:string,items:unknown[]};return{kind:'typedValue',value:{name:typed.name,items:typed.items.map(native)}};}return structuredClone(value)as IfcValue;}
for(const row of contract.cases)test('original primary sparse remap '+row.id,()=>{
 const record=contract.sources[row.source as keyof typeof contract.sources],source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt(row.entityId??'1'),name:record.name,args:record.args.map(native),complex:record.complex.map(part=>({name:part.name,args:part.args.map(native)}))},...contract.tail.map(entity=>({id:BigInt(entity.id),name:entity.name,args:entity.args.map(native),complex:[]}))]},entity=source.entities[0]!,name=row.target.toUpperCase(),old=catalog.entities[entity.name]!,next=catalog.entities[name]!;
 const retained=entity.complex!.length>0&&old.attributes.length===entity.args.length&&old.attributes.length>0&&next.attributes.length>0&&old.attributes.length!==next.attributes.length;expect(retained).toBe(row.retained);if(!retained)return;
 const run=()=>{const map=next.attributes.map(attribute=>{const index=old.attributes.findIndex(previous=>previous.name===attribute.name&&exact(previous.typeExpr)===exact(attribute.typeExpr));if(index>=0&&entity.args[index]!.kind!=='derived'&&(attribute.optional||entity.args[index]!.kind!=='unset'))return index;if(!attribute.optional)throw Error('explicit-required-slot');return null;});const args=map.map(index=>index===null?{kind:'unset'as const}:entity.args[index]!);for(const [index,attribute]of next.attributes.entries())checkIfcValueDomain(catalog,attribute.typeExpr,args[index]!,{optional:attribute.optional,reference:function*(id,expected){yield;const rows=source.entities.filter(candidate=>candidate.id===id);return rows.length===1&&[rows[0]===entity?name:rows[0]!.name,...(rows[0]!.complex??[]).map(candidate=>candidate.name)].some(actual=>ifcSubtype(catalog,actual,expected));}});return{map,args};};
 if(!row.admitted){expect(run).toThrow();return;}
 const {map,args}=run();expect(map).toEqual(row.map??[]);expect(exact(args)).toBe(exact(row.args));
 const batch:IfcMutation[]=[{mutation:'setEntityName',id:entity.id,name}];
 for(let index=entity.args.length-1;index>=args.length;index--)batch.push({mutation:'removeEntityArg',id:entity.id,index});
 for(const [index,value]of args.entries()){if(map[index]===index||map[index]===null&&entity.args[index]?.kind==='unset')continue;batch.push(index<entity.args.length?{mutation:'setEntityArg',id:entity.id,index,value}:{mutation:'insertEntityArg',id:entity.id,index,value});}
 expect(batch.length).toBe(row.mutations??0);
 const inverses:IfcMutation[][]=[];let result=source;for(const mutation of batch){inverses.push(inverseIfcMutation(result,mutation));result=applyIfcMutation(result,mutation);}
 const oracle=applyPatch(JSON.parse(exact(source)),[{op:'replace',path:'/entities/0/name',value:name},{op:'replace',path:'/entities/0/args',value:row.args}],true,true).newDocument;
 expect(exact(result)).toBe(exact(oracle));for(const inverse of inverses.reverse())result=inverse.reduce(applyIfcMutation,result);expect(exact(result)).toBe(exact(source));
});
