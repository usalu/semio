/** 🧭️ Language-neutral placement cases drive real graph queries and independent tools. */
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {ifcReal} from '../step.ts';
import type {IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';

export interface IfcAxisCase {id:string;dimension:number;origin:{kind:string;ratios:number[]};axis:{kind:string;ratios:number[]}|null;ref:{kind:string;ratios:number[]}|null;matrix?:number[];error?:string}
export function ifcAxisFixture(row:IfcAxisCase) {
 const graph=createIfc4x3ExchangeGraph(row.id),unset:IfcValue={kind:'unset'};
 const coordinate=(value:IfcAxisCase['origin']|null):IfcValue=>value?graph.add(value.kind,[{kind:'aggregate',value:value.ratios.map(ifcReal)}]):unset;
 const origin=coordinate(row.origin),axis=coordinate(row.axis),ref=coordinate(row.ref);
 const placement=graph.add(row.dimension===3?'IFCAXIS2PLACEMENT3D':'IFCAXIS2PLACEMENT2D',row.dimension===3?[origin,axis,ref]:[origin,ref]);
 return{snapshot:graph.snapshot,placement};
}

