/** 🧭 Observe constructed results through the actual source context's derived attributes. */
import {IfcExpressContext,type ExpressValue} from '../index.ts';
export async function vectorOwnerValue(context:IfcExpressContext,value:ExpressValue):Promise<unknown>{
 if(value.kind==='unknown')return {kind:'unknown'};
 if(value.kind==='real'||value.kind==='integer')return {kind:'real',value:Number(value.value)};
 const ratios=async(value:ExpressValue)=>{const result=await context.evaluator.attribute(value,'DIRECTIONRATIOS');if(result.kind!=='aggregate')throw Error('Direction ratios not available');return (result.value as ExpressValue[]).map(row=>{if(row.kind!=='real'&&row.kind!=='integer')throw Error('Non-numeric ratio');return Number(row.value);});};
 if(value.kind==='aggregate')return {kind:'directions',items:await Promise.all((value.value as ExpressValue[]).map(ratios))};
 if(value.kind==='entity'&&value.types?.includes('IFCVECTOR'))return {kind:'vector',orientation:await ratios(await context.evaluator.attribute(value,'ORIENTATION')),magnitude:Number((await context.evaluator.attribute(value,'MAGNITUDE')).value)};
 if(value.kind==='entity'&&value.types?.includes('IFCDIRECTION'))return {kind:'direction',ratios:await ratios(value)};
 throw Error('Unrepresented vector owner result '+value.kind);
}
