/** 🧫️ Neutral fixture values use actual public EXPRESS constructors and complex entity composition. */
import {ExpressEvaluator,type ExpressValue} from '../index.ts';
import {ifc4x3Catalog} from '../../📚️catalog/🟦️.ts';
export async function issuedOwnerArguments(evaluator:ExpressEvaluator,inputs:readonly unknown[]):Promise<ExpressValue[]>{
 const identities=new Map<string,ExpressValue>();
 const value=async(input:unknown):Promise<ExpressValue>=>{
  const row=input as Record<string,any>;
  if(row.kind==='unknown')return{kind:'unknown'};
  if(row.kind==='integer')return{kind:'integer',value:BigInt(row.value)};
  if(row.kind==='real')return{kind:'real',value:Number(row.value)};
  if(row.kind==='string'||row.kind==='logical')return{kind:row.kind,value:row.value};
  if(row.kind==='reference'){const prior=identities.get(row.key);if(!prior)throw Error('Unresolved fixture identity');return prior;}
  if(row.kind==='aggregate'){const items:ExpressValue[]=[];for(const item of row.items)items.push(await value(item));return{kind:'aggregate',aggregate:row.aggregate,lower:BigInt(row.lower),value:items};}
  if(row.kind!=='entity')throw Error('Unknown closed fixture value');
  const chain:string[]=[];let name:string|null=row.type;while(name){chain.unshift(name);name=ifc4x3Catalog.entities[name]!.supertype;}
  let entity:ExpressValue|undefined;
  for(const name of chain){const args:ExpressValue[]=[];for(const attribute of ifc4x3Catalog.entities[name]!.attributes)args.push(await value(row.fields[attribute.name]??{kind:'unknown'}));const partial=await evaluator.call(name,args);entity=entity?await evaluator.binary('||',entity,partial):partial;}
  if(!entity)throw Error('Empty entity fixture');
  if(row.key)identities.set(row.key,entity);
  return entity;
 };
 const result:ExpressValue[]=[];for(const input of inputs)result.push(await value(input));return result;
}
export function issuedOwnerValue(value:ExpressValue):unknown{
 if(value.kind==='unknown')return{kind:'unknown'};
 if(value.kind==='aggregate')return{kind:'aggregate',aggregate:value.aggregate??'LIST',lower:String(value.lower??1n),items:(value.value as ExpressValue[]).map(issuedOwnerValue)};
 return{kind:value.kind,value:String(value.value)};
}
