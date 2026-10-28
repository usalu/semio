import {expressAggregate,type ExpressValue,type IfcExpressContext} from '../index.ts';
/** 🧭 Closed classification inputs borrow exact source identities and declared SET membership. */
export function classificationOwnerArguments(context:IfcExpressContext,args:readonly unknown[]):ExpressValue[]{
 const entity=(id:string)=>{const value=context.entities.get(BigInt(id));if(!value)throw Error('Unavailable classification source identity '+id);return value;};
 return args.map(input=>{const row=input as {kind:string;id?:string;value?:string;ids?:string[]};switch(row.kind){case'unknown':return{kind:'unknown'};case'entity':return entity(row.id!);case'string':return{kind:'string',value:row.value!};case'set':return expressAggregate(row.ids!.map(entity),'SET');default:throw Error('Unsupported classification argument');}});
}
/** ⚖️ Logical UNKNOWN remains distinct from an indeterminate RETURN question mark. */
export function classificationOwnerValue(value:ExpressValue):unknown{if(value.kind==='unknown')return{kind:'indeterminate'};if(value.kind==='logical')return{kind:'logical',value:String(value.value)};throw Error('Unsupported classification result '+value.kind);}
