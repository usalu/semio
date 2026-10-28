import type {IfcExpressContext,ExpressValue} from '../index.ts';
import {placementOwnerArguments} from '../🧪️placement/fixture.ts';
/** 📏 Exact dimension arguments and source entity references enter the actual issued functions. */
export function projectionOwnerArguments(context:IfcExpressContext,args:readonly unknown[]):ExpressValue[]{return args.map(arg=>{const row=arg as {kind:string;value?:string};return row.kind==='integer'&&typeof row.value==='string'?{kind:'integer',value:BigInt(row.value)}:placementOwnerArguments(context,[arg])[0]!;});}
/** 🧩 A partially indeterminate axes LIST keeps every original result position. */
export async function projectionOwnerValue(context:IfcExpressContext,value:ExpressValue):Promise<unknown>{
 if(value.kind==='unknown')return {kind:'unknown'};
 const direction=async(value:ExpressValue)=>{if(value.kind==='unknown')return null;if(value.kind!=='entity')throw Error('Projection result is not a direction');const ratios=await context.evaluator.attribute(value,'DIRECTIONRATIOS');if(ratios.kind!=='aggregate')throw Error('Projection direction ratios unavailable');return(ratios.value as ExpressValue[]).map(item=>{if(item.kind!=='real'&&item.kind!=='integer')throw Error('Projection ratio is not numeric');return Number(item.value);});};
 return value.kind==='aggregate'?{kind:'axes',items:await Promise.all((value.value as ExpressValue[]).map(direction))}:{kind:'direction',ratios:await direction(value)};
}
