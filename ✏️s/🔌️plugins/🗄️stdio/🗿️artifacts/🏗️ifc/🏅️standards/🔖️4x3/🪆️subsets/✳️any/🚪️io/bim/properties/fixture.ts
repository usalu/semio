/** 🏷️ Closed occurrence-property descriptors preserve binary64 words and actual supported spatial fixture records. */
import {ifcBimAxisFixture} from '../../🧪️tests/bim-axis-fixtures.ts';
import type {bimSpatialToIfc4x3} from '../../bim.ts';
export type BimPropertyFixture=Parameters<typeof bimSpatialToIfc4x3>[0];
export function propertyValue(row:{kind:string;value?:unknown;bits?:string}):any{let value=row.value;if(row.kind==='Real'){const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,BigInt('0x'+row.bits!),false);value=view.getFloat64(0,false);}if(row.kind==='Integer')value=Number(row.value);return{[row.kind]:{value}};}
export function propertyModel():BimPropertyFixture{return{...ifcBimAxisFixture(.3),storeys:{storey:{building:'building',name:'Storey',level:0,height:3,cut_height:1.2}}};}
