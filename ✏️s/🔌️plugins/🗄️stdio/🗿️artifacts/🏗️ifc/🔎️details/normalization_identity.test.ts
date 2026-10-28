import {test,expect} from 'bun:test';
import Ajv from 'ajv';
import {applyPatch} from 'fast-json-patch';
import contract from './🧪️tests/normalization-identity.json';
import {ifc4x3Catalog} from '../🏅️standards/🔖️4x3/🪆️subsets/✳️any/🧬️schema/📚️catalog/🟦️.ts';
import {applyIfcMutation,inverseIfcMutation} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🟦️.ts';
import type {IfcSnapshot,IfcValue} from '../🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🟦️.ts';
const exact=(value:unknown)=>JSON.stringify(value,(_key,item)=>typeof item==='bigint'?item.toString():item),valid=new Ajv().compile({type:'string',minLength:1});
for(const row of contract.cases)test('original direct identity normalization '+row.id,()=>{
 const args:IfcValue[]='missing'in row&&row.missing?[]:[structuredClone(row.globalId) as IfcValue,{kind:'unset'},{kind:'string',value:'建物🌍'},{kind:'unset'}],source:IfcSnapshot={schema:'stdio.ifc',header:{fileDescription:[],fileName:[],fileSchema:[]},entities:[{id:BigInt('entityId'in row?row.entityId!:'1'),name:'IFCROOT',args,complex:structuredClone(contract.parts) as IfcSnapshot['entities'][number]['complex']}]},entity=source.entities[0]!,parts=entity.complex!;
 const candidate={name:'IFCREPRESENTATIONITEM',args:[]},roles=[{name:entity.name,args:entity.args},...parts,candidate];let admitted=true;
 for(const role of roles){const declaration=ifc4x3Catalog.entities[role.name]!;if(declaration.allAttributes.some(attribute=>attribute.derived))admitted=false;for(const [index,attribute]of declaration.attributes.entries())if(attribute.name==='GlobalId'){const value=role.args[index];admitted&&=value?.kind==='string'&&value.value.length>0;}}
 expect(admitted).toBe(row.retained);const value=entity.args[0];expect(Boolean(valid(value?.kind==='string'?value.value:undefined))).toBe(row.retained);if(!admitted)return;
 const mutation={mutation:'setEntityComplex' as const,id:entity.id,complex:[...structuredClone(parts),candidate]},next=applyIfcMutation(source,mutation),oracle=applyPatch(JSON.parse(exact(source)),[{op:'add',path:'/entities/0/complex/-',value:candidate}],true,true).newDocument;
 expect(exact(next)).toBe(exact(oracle));expect(exact(next.entities[0]!.args)).toBe(exact(entity.args));expect(exact(inverseIfcMutation(source,mutation).reduce(applyIfcMutation,next))).toBe(exact(source));
});
