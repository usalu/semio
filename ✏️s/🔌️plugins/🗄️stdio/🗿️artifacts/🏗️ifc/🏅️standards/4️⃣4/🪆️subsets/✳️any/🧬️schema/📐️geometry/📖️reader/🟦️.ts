import type {IfcSnapshot,IfcValue} from "../../📸️snapshot/🟦️.ts";
import type {GeometryGraph,GeometryValues,GeometryEntity,GeometryConstituent} from "../../../../../../../📐️geometry/📖️graph/🟦️.ts";
import {binary64Value} from "@semio-tech/framework/value/ieee754";
const values:GeometryValues<IfcValue>={reference:v=>v?.kind==="reference"?v.value:undefined,text:v=>v?.kind==="string"?v.value:undefined,enumeration:v=>v?.kind==="enum"?v.value:undefined,integer:v=>v?.kind==="integer"?v.value:undefined,real:v=>v?.kind==="real"?binary64Value(v.value):v?.kind==="integer"?Number(v.value):undefined,aggregate:v=>v?.kind==="aggregate"?v.value:undefined,typed:v=>v?.kind==="typedValue"?v.value:undefined,absent:v=>!v||v.kind==="unset"||v.kind==="derived",unset:v=>v?.kind==="unset",derived:v=>v?.kind==="derived"};
/** 🏗️ Borrow IFC4 values without narrowing or copying the artifact graph. */
export class Ifc4GeometryGraph implements GeometryGraph<IfcValue>{
 readonly standard="IFC4" as const;readonly values=values;
 constructor(readonly snapshot:IfcSnapshot){}
 sourceIdentity():object{return this.snapshot;}
 get length():number{return this.snapshot.entities.length;}
 entity(index:number):GeometryEntity<IfcValue>|undefined{const e=this.snapshot.entities[index];return e?{id:e.id,name:e.name,args:e.args,constituentCount:1+(e.complex?.length??0)}:undefined;}
 constituent(instance:number,index:number):GeometryConstituent<IfcValue>|undefined{const e=this.snapshot.entities[instance];return index===0&&e?{name:e.name,args:e.args}:e?.complex?.[index-1];}
}
