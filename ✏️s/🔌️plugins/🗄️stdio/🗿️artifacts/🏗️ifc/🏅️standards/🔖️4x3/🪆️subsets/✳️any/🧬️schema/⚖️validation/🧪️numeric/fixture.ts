/** 🧮 Numeric fixtures extend existing constructor inputs with genuine declared enum resolution and field observations. */
import {ExpressEvaluator,parseExpressExpression,type ExpressValue} from '../index.ts';
import {issuedOwnerArguments,issuedOwnerValue} from '../🧪️coverage/fixture.ts';
export async function numericOwnerArguments(evaluator:ExpressEvaluator,inputs:readonly unknown[]):Promise<ExpressValue[]>{
 const result:ExpressValue[]=[];
 for(const input of inputs){const row=input as {kind:string;value?:string};result.push(row.kind==='enum'?await evaluator.evaluate(parseExpressExpression(row.value!)):(await issuedOwnerArguments(evaluator,[input]))[0]!);}
 return result;
}
export async function numericOwnerValue(evaluator:ExpressEvaluator,value:ExpressValue,projection?:readonly string[]):Promise<unknown>{
 if(!projection)return issuedOwnerValue(value);
 const values:Record<string,unknown>={};for(const name of projection)values[name]=issuedOwnerValue(await evaluator.attribute(value,name.toUpperCase()));return{kind:'attributes',values};
}
export function numericOwnerSnapshot(value:ExpressValue):unknown{
 if(value.kind!=='entity'&&value.kind!=='aggregate')return issuedOwnerValue(value);
 if(value.kind==='aggregate')return{kind:value.kind,aggregate:value.aggregate,lower:String(value.lower),items:(value.value as ExpressValue[]).map(numericOwnerSnapshot)};
 return{kind:value.kind,types:value.types,fields:Object.fromEntries([...(value.fields??[])].map(([name,item])=>[name,numericOwnerSnapshot(item)]))};
}
