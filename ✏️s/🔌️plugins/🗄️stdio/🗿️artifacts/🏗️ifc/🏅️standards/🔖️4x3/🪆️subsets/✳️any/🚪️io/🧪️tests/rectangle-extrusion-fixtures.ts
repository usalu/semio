/** 🏗️ Construct declared rectangular extrusion roles with independently measured expected geometry. */
import type {IfcValue} from '../../🧬️schema/📸️snapshot/🟦️.ts';
import {createIfc4x3ExchangeGraph} from '../gltf.ts';
import {ifcReal} from '../step.ts';
export interface IfcRectangleCase {id:string;x:number;y:number;depth:number;direction:number[];profileOrigin?:number[];profileX?:number[];solidOrigin?:number[];solidZ?:number[];solidX?:number[];millimetres?:boolean;profileType?:string;}
/** 📐️ Build one genuine product and context rather than matching fixture-specific entity IDs. */
export function ifcRectangleFixture(row:IfcRectangleCase){
 const graph=createIfc4x3ExchangeGraph(row.id),{snapshot,add,context}=graph;
 const unset:IfcValue={kind:'unset'},a=(value:IfcValue[]):IfcValue=>({kind:'aggregate',value}),s=(value:string):IfcValue=>({kind:'string',value}),e=(value:string):IfcValue=>({kind:'enum',value});
 const point=(xyz:number[])=>add('IFCCARTESIANPOINT',[a(xyz.map(ifcReal))]),direction=(xyz:number[])=>add('IFCDIRECTION',[a(xyz.map(ifcReal))]);
 if(row.millimetres)snapshot.entities.find(entity=>entity.name==='IFCSIUNIT')!.args[2]=e('MILLI');
 const profilePosition=row.profileOrigin?add('IFCAXIS2PLACEMENT2D',[point(row.profileOrigin),direction(row.profileX!)]):unset;
 const solidPosition=row.solidOrigin?add('IFCAXIS2PLACEMENT3D',[point(row.solidOrigin),direction(row.solidZ!),direction(row.solidX!)]):unset;
 const profile=add('IFCRECTANGLEPROFILEDEF',[e(row.profileType??'AREA'),unset,profilePosition,ifcReal(row.x),ifcReal(row.y)]);
 const solid=add('IFCEXTRUDEDAREASOLID',[profile,solidPosition,direction(row.direction),ifcReal(row.depth)]);
 const representation=add('IFCSHAPEREPRESENTATION',[context,s('Body'),s('SweptSolid'),a([solid])]),shape=add('IFCPRODUCTDEFINITIONSHAPE',[unset,unset,a([representation])]);
 const placement=add('IFCLOCALPLACEMENT',[unset,add('IFCAXIS2PLACEMENT3D',[point([0,0,0]),unset,unset])]);
 const product=add('IFCBUILDINGELEMENTPROXY',[s('0000000000000000000002'),unset,s(row.id),unset,unset,placement,shape,unset,unset]);
 return{...graph,profile,solid,product};
}
