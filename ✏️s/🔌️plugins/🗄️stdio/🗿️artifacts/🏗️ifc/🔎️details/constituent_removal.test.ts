import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/constituent-removal.json';
import {ifc4x3Catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueChange} from '../📚️catalog/⚖️domain/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as {kind:string,value?:unknown};if(item.kind==='real'){const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,Number(item.value));return {kind:'real',value:{bits:view.getBigUint64(0)}};}if(item.kind==='integer')return {kind:'integer',value:BigInt(String(item.value))};if(item.kind==='aggregate')return {kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value) as IfcValue;}
for(const row of contract.cases)test('retained constituent argument removal '+row.id,()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:row.root,args:[],complex:[{name:row.name,args:row.args.map(native)}]}]},entity=source.entities[0]!,part=entity.complex![0]!,index=Number(row.path.split('/').at(-1));
 const run=()=>{const declaration=ifc4x3Catalog.entities[part.name];if(index>=part.args.length||part.args.length-1!==declaration?.attributes.length)throw new Error('argument-arity');const args=part.args.filter((_,position)=>position!==index);for(const [position,after]of args.entries()){const attribute=declaration.attributes[position]!;checkIfcValueChange(ifc4x3Catalog,attribute.typeExpr,part.args[position]!,after,[],'remove',{optional:attribute.optional});}return {mutation:'setEntityComplex' as const,id:entity.id,complex:[{name:part.name,args}]};};
 if(!row.admitted){expect(run).toThrow();return;}const mutation=run(),next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'remove',path:row.path}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(exact(next.entities[0]!.args)).toBe(exact(entity.args));expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
