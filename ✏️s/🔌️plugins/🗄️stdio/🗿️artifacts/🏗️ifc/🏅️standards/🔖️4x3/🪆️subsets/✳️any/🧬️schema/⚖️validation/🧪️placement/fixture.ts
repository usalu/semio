/** 📐 Closed arguments borrow actual context entities and preserve explicit indeterminate inputs. */
import type {IfcExpressContext,ExpressValue} from '../index.ts';
export function placementOwnerArguments(context:IfcExpressContext,inputs:readonly unknown[]):ExpressValue[]{
 return inputs.map(input=>{const row=input as {kind:string;id?:string;value?:string};if(row.kind==='unknown')return {kind:'unknown'};if(row.kind==='entity'&&typeof row.id==='string'){const value=context.entities.get(BigInt(row.id));if(value)return value;}if(row.kind==='real'&&typeof row.value==='string'&&Number.isFinite(Number(row.value)))return {kind:'real',value:Number(row.value)};throw Error('Unavailable placement owner argument');});
}
