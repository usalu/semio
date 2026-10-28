/** 🧮 Actual argument views keep derived arrays and declared source attributes authoritative. */
import type {IfcExpressContext,ExpressValue} from '../index.ts';
export async function aggregateOwnerArguments(context:IfcExpressContext,inputs:readonly unknown[]):Promise<ExpressValue[]>{
 const result:ExpressValue[]=[];for(const input of inputs){const row=input as {kind:string;id:string;name?:string;declaredBy?:string},value=context.entities.get(BigInt(row.id));if(!value)throw Error('Unavailable source entity');if(row.kind==='entity')result.push(value);else if(row.kind==='attribute'&&row.name&&row.declaredBy)result.push(await context.attribute(value,row.name,row.declaredBy));else throw Error('Unrepresented aggregate argument');}return result;
}
