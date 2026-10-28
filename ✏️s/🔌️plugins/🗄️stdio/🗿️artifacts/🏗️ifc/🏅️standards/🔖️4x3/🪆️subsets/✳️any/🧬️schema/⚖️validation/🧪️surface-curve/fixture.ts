import type {ExpressValue} from '../index.ts';
/** 🌊 Surface projections retain canonical identity rather than equal geometric fields. */
export function surfaceCurveOwnerValue(value:ExpressValue):unknown{
 if(value.kind==='unknown')return {kind:'unknown'};
 if(value.kind==='integer')return {kind:'integer',value:String(value.value)};
 const identity=(value:ExpressValue)=>{if(value.kind!=='entity'||typeof value.identity!=='bigint'||!value.types?.includes('IFCSURFACE'))throw Error('Original surface identity unavailable');return String(value.identity);};
 if(value.kind==='entity')return {kind:'surface',value:identity(value)};
 if(value.kind==='aggregate'){if(value.aggregate!=='SET')throw Error('Basis surfaces are not a SET');return {kind:'surfaces',value:(value.value as ExpressValue[]).map(identity).sort((a,b)=>BigInt(a)<BigInt(b)?-1:BigInt(a)>BigInt(b)?1:0)};}
 throw Error('Unsupported surface/curve owner result '+value.kind);
}
