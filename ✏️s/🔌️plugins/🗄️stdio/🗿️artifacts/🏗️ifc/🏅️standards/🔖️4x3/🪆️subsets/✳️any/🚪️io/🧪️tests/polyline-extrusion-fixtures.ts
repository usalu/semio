/** 🏗️ Closed planar polyline profiles keep holes distinct from relational opening subtraction. */
import type {IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {ifcReal} from '../step.ts';
export interface IfcPolylineCase {id:string;outer:number[][];holes?:number[][][];depth:number;direction:number[];millimetres?:boolean}
/** 🪢️ Emit actual declared curve references, including one explicit closing coordinate. */
export function ifcPolylineFixture(row:IfcPolylineCase){
 const graph=createIfc4x3ExchangeGraph(row.id),{snapshot,add,context}=graph;
 const unset:IfcValue={kind:'unset'},a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),s=(value:string):IfcValue=>({kind:'string',value}),e=(value:string):IfcValue=>({kind:'enum',value});
 const point=(xyz:number[])=>add('IFCCARTESIANPOINT',[a(xyz.map(ifcReal))]),curve=(points:number[][])=>add('IFCPOLYLINE',[a(points.map(point))]);
 if(row.millimetres)snapshot.entities.find(entity=>entity.name==='IFCSIUNIT')!.args[2]=e('MILLI');
 const profile=add(row.holes===undefined?'IFCARBITRARYCLOSEDPROFILEDEF':'IFCARBITRARYPROFILEDEFWITHVOIDS',[e('AREA'),unset,curve(row.outer),...(row.holes===undefined?[]:[a(row.holes.map(curve))])]);
 const solid=add('IFCEXTRUDEDAREASOLID',[profile,unset,add('IFCDIRECTION',[a(row.direction.map(ifcReal))]),ifcReal(row.depth)]);
 const representation=add('IFCSHAPEREPRESENTATION',[context,s('Body'),s('SweptSolid'),a([solid])]),shape=add('IFCPRODUCTDEFINITIONSHAPE',[unset,unset,a([representation])]);
 const placement=add('IFCLOCALPLACEMENT',[unset,add('IFCAXIS2PLACEMENT3D',[point([0,0,0]),unset,unset])]);
 const product=add('IFCBUILDINGELEMENTPROXY',[s('0000000000000000000002'),unset,s(row.id),unset,unset,placement,shape,unset,unset]);
 return{...graph,profile,solid,product};
}
