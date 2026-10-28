import {test,expect} from 'bun:test';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/argument-insertion.json';
import {ifc4x3Catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {checkIfcValueDomain} from '../📚️catalog/⚖️domain/🟦️.ts';
import {ifcSubtype} from '../📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item);
function native(value:unknown):IfcValue{const item=value as {kind:string,value?:unknown};if(item.kind==='real'){const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,Number(item.value));return {kind:'real',value:{bits:view.getBigUint64(0)}};}if(item.kind==='integer')return {kind:'integer',value:BigInt(String(item.value))};if(item.kind==='reference')return {kind:'reference',value:BigInt(String(item.value))};if(item.kind==='aggregate')return {kind:'aggregate',value:(item.value as unknown[]).map(native)};return structuredClone(value) as IfcValue;}
for(const row of contract.cases)test('original root argument insertion '+row.id,()=>{
 const source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:row.name,args:row.args.map(native),complex:[]},...('targets'in row?row.targets!:[]).map(target=>({id:BigInt(target.id),name:target.name,args:[],complex:[]}))]},entity=source.entities[0]!,last=row.path.split('/').at(-1)!,index=last==='-'?entity.args.length:Number(last),value=native(row.value);
 const run=()=>{if(index!==entity.args.length)throw new Error('argument-index');const attribute=ifc4x3Catalog.entities[entity.name]?.allAttributes[index];if(!attribute)throw new Error('argument-arity');checkIfcValueDomain(ifc4x3Catalog,attribute.typeExpr,value,{optional:attribute.optional,*reference(id,expected){let count=0,accepted=false;for(const target of source.entities){yield;if(target.id===id){count++;accepted ||= ifcSubtype(ifc4x3Catalog,target.name,expected);}}return count===1&&accepted;}});return {mutation:'insertEntityArg' as const,id:entity.id,index,value};};
 if(!row.admitted){expect(run).toThrow();return;}
 const mutation=run(),next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'add',path:row.path,value:JSON.parse(exact(value))}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(next.entities[0]!.id).toBe(entity.id);expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
