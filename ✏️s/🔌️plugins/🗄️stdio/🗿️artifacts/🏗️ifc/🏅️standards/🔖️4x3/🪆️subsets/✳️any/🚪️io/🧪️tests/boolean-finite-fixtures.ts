import {ifcBlockFixture} from "./csg-block-fixtures.ts";
import type {IfcEntity,IfcValue} from "../../🧬️schema/📸️snapshot/🟦️.ts";
import {ifcReal} from "../step.ts";
import type{IfcMappingLayer}from'./mapping-fixtures.ts';
export interface IfcFiniteBooleanCase{id:string;operation:string;root:string;cut:number[][];unitPrefix:string|null;mapping:IfcMappingLayer[];expected:string;volume:number|null;}
/** 🧱🪚 Explicit original Boolean operands and CSG/direct representation roles. */
export function ifcFiniteBooleanFixture(row:IfcFiniteBooleanCase){
 const fixture=ifcBlockFixture({id:row.id,dimensions:[4,4,4],position:{origin:[0,0,0],axis:null,refDirection:null},unitPrefix:row.unitPrefix,mapping:row.mapping,expected:row.expected,bounds:null,volume:row.volume,damage:"none"}),unset:IfcValue={kind:"unset"},ref=(entity:IfcEntity):IfcValue=>({kind:"reference",value:entity.id}),aggregate=(value:IfcValue[]):IfcValue=>({kind:"aggregate",value}),put=(name:string,args:IfcValue[])=>{fixture.add(name,args);return fixture.snapshot.entities.at(-1)!;},point=put("IFCCARTESIANPOINT",[aggregate(row.cut[0]!.map(ifcReal))]),position=put("IFCAXIS2PLACEMENT3D",[ref(point),unset,unset]),right=put("IFCBLOCK",[ref(position),...row.cut[1]!.map((value,axis)=>ifcReal(value-row.cut[0]![axis]!))]),boolean=put("IFCBOOLEANRESULT",[{kind:"enum",value:row.operation},ref(fixture.block),ref(right)]);
 fixture.solid.args[0]=ref(boolean);
 if(row.root==="direct"){for(const entity of fixture.snapshot.entities)if(entity.name==="IFCSHAPEREPRESENTATION"&&entity.args[3]?.kind==="aggregate")for(let at=0;at<entity.args[3].value.length;at++){const item=entity.args[3].value[at];if(item?.kind==="reference"&&item.value===fixture.solid.id)entity.args[3].value[at]=ref(boolean);}fixture.snapshot.entities=fixture.snapshot.entities.filter(entity=>entity.id!==fixture.solid.id);}
 return {...fixture,left:fixture.block,right,boolean,root:row.root==="direct"?boolean:fixture.solid};
}

