/** 📝️ AP214 wireframe, fill and text annotation entities over owned canonical Part21 values. */
import type { Part21Instance, Part21Value } from "../../../../../../../../📇️registry/🧬️contract/📐️part21/🟦️.ts";
export type { Part21Instance, Part21Value };
export type Point3 = readonly [number, number, number];
export type Rgb = readonly [number, number, number];
const str = (value: string): Part21Value => ({kind:"string",value});
const ref = (value: bigint): Part21Value => ({kind:"reference",value});
const enumeration = (value: string): Part21Value => ({kind:"enum",value});
const aggregate = (value: Part21Value[]): Part21Value => ({kind:"aggregate",value});
const refs = (values: readonly bigint[]): Part21Value => aggregate(values.map(ref));
const real = (value: number): Part21Value => { if (!Number.isFinite(value)) throw new Error("step.draughting.number"); const parts = String(value).toUpperCase().split("E"); return {kind:"real",value:(parts[0].includes(".")?parts[0]:parts[0]+".")+(parts.length>1?"E"+parts[1]:"")}; };
const xyz = (point: Point3): Part21Value => aggregate(point.map(real));
const validPoint = (point: Point3): boolean => point.every(Number.isFinite);
/** 🧵️ Retains only id/context scalars; each primitive returns one bounded entity batch. */
export class DraughtingWriter {
  private context = 0n; private normal = 0n; private font = 0n; private curveFont = 0n; private curveWidth=.5*25.4/72;
  private constructor(private nextId: bigint) {}
  private alloc(out: Part21Instance[], name: string, args: Part21Value[]): bigint { const id=this.nextId++;out.push({id,entities:[{name,args}]});return id; }
  private complex(out: Part21Instance[], entities: Part21Instance["entities"]): bigint { const id=this.nextId++;out.push({id,entities});return id; }
  private color(out: Part21Instance[], color: Rgb): bigint { if(color.some(value=>!Number.isFinite(value)||value<0||value>1))throw new Error("step.draughting.color");return this.alloc(out,"COLOUR_RGB",[str(""),...color.map(real)]); }
  private assignment(out: Part21Instance[], style: bigint): bigint { return this.alloc(out,"PRESENTATION_STYLE_ASSIGNMENT",[refs([style])]); }
  private model(out: Part21Instance[], name: string, item: bigint): void { this.alloc(out,"DRAUGHTING_MODEL",[str(name),refs([item]),ref(this.context)]); }
  private wireframe(out: Part21Instance[], name: string, curve: bigint, stroke: Rgb|null): void { const set=this.alloc(out,"GEOMETRIC_CURVE_SET",[str(name),refs([curve])]);this.alloc(out,"GEOMETRICALLY_BOUNDED_WIREFRAME_SHAPE_REPRESENTATION",[str(name),refs([set]),ref(this.context)]);if(stroke){const color=this.color(out,stroke),style=this.alloc(out,"CURVE_STYLE",[str(name),ref(this.curveFont),{kind:"typedValue",value:{name:"POSITIVE_LENGTH_MEASURE",items:[real(this.curveWidth)]}},ref(color)]),assignment=this.assignment(out,style),styled=this.alloc(out,"STYLED_ITEM",[str(name),refs([assignment]),ref(curve)]);this.model(out,name+"/stroke",styled);} }
  /** 🧭️ Defines explicit millimetre/radian/steradian context and ISO3098 draughting defaults. */
  static start(firstId=1n): {writer:DraughtingWriter;instances:Part21Instance[]} {
    if(firstId<1n||firstId>9223372036854775807n)throw new Error("step.draughting.id");const writer=new DraughtingWriter(firstId),out:Part21Instance[]=[];
    const length=writer.complex(out,[{name:"LENGTH_UNIT",args:[]},{name:"NAMED_UNIT",args:[{kind:"derived"}]},{name:"SI_UNIT",args:[enumeration("MILLI"),enumeration("METRE")]}]);
    const angle=writer.complex(out,[{name:"NAMED_UNIT",args:[{kind:"derived"}]},{name:"PLANE_ANGLE_UNIT",args:[]},{name:"SI_UNIT",args:[{kind:"unset"},enumeration("RADIAN")]}]);
    const solid=writer.complex(out,[{name:"NAMED_UNIT",args:[{kind:"derived"}]},{name:"SI_UNIT",args:[{kind:"unset"},enumeration("STERADIAN")]},{name:"SOLID_ANGLE_UNIT",args:[]}]);
    writer.context=writer.complex(out,[{name:"GEOMETRIC_REPRESENTATION_CONTEXT",args:[{kind:"integer",value:3n}]},{name:"GLOBAL_UNIT_ASSIGNED_CONTEXT",args:[refs([length,angle,solid])]},{name:"REPRESENTATION_CONTEXT",args:[str("Layout"),str("3D")]}]);
    writer.normal=writer.alloc(out,"DIRECTION",[str(""),xyz([0,0,1])]);writer.font=writer.alloc(out,"DRAUGHTING_PRE_DEFINED_TEXT_FONT",[str("ISO 3098")]);writer.curveFont=writer.alloc(out,"DRAUGHTING_PRE_DEFINED_CURVE_FONT",[str("continuous")]);return {writer,instances:out};
  }
  /** 🖋️ Chooses a positive physical pen width with fixed scalar state. */
  setCurveWidth(width:number):void{if(!Number.isFinite(width)||width<=0)throw Error("step.draughting.curve-width");this.curveWidth=width;}
  /** 🧱️ Connects a page solid to the explicit physical context. */
  solidRepresentation(name:string,solid:bigint):Part21Instance[]{const out:Part21Instance[]=[];this.alloc(out,"ADVANCED_BREP_SHAPE_REPRESENTATION",[str(name),refs([solid]),ref(this.context)]);return out;}
  /** 📐️ Retains real paths and closed fill geometry with AP214-valid polyline cardinality. */
  path(name:string,points:readonly Point3[],stroke:Rgb|null=null,fill:Rgb|null=null):Part21Instance[]{
    if(new TextEncoder().encode(name).length>512||points.length<2||points.length>65||points.some(point=>!validPoint(point)))throw new Error("step.draughting.path");const first=points[0],last=points.at(-1)!,closed=first.every((value,index)=>value===last[index]);if(fill&&!closed)throw new Error("step.draughting.open-fill");const out:Part21Instance[]=[],ids:bigint[]=[];
    points.forEach((value,index)=>{if(index===points.length-1&&closed)ids.push(ids[0]);else ids.push(this.alloc(out,"CARTESIAN_POINT",[str(""),xyz(value)]));if(points.length===2&&index===0)ids.push(this.alloc(out,"CARTESIAN_POINT",[str(""),xyz([(value[0]+last[0])/2,(value[1]+last[1])/2,(value[2]+last[2])/2])]));});
    const curve=this.alloc(out,"POLYLINE",[str(name),refs(ids)]);this.wireframe(out,name,curve,stroke);if(fill){const color=this.color(out,fill),colorStyle=this.alloc(out,"FILL_AREA_STYLE_COLOUR",[str(name),ref(color)]),style=this.alloc(out,"FILL_AREA_STYLE",[str(name),refs([colorStyle])]),assignment=this.assignment(out,style),area=this.alloc(out,"ANNOTATION_FILL_AREA",[str(name),refs([curve])]),occurrence=this.alloc(out,"ANNOTATION_FILL_AREA_OCCURRENCE",[str(name),refs([assignment]),ref(area),ref(ids[0])]);this.model(out,name+"/fill",occurrence);}return out;
  }
  /** ⭕️ Preserves an exact XY CIRCLE instead of baking its shape into rectangle geometry. */
  circle(name:string,center:Point3,radius:number,stroke:Rgb|null=null):Part21Instance[]{if(new TextEncoder().encode(name).length>512||!validPoint(center)||!Number.isFinite(radius)||radius<=0)throw new Error("step.draughting.circle");const out:Part21Instance[]=[],origin=this.alloc(out,"CARTESIAN_POINT",[str(""),xyz(center)]),axis=this.alloc(out,"AXIS2_PLACEMENT_3D",[str(""),ref(origin),ref(this.normal),{kind:"unset"}]),curve=this.alloc(out,"CIRCLE",[str(name),ref(axis),real(radius)]);this.wireframe(out,name,curve,stroke);return out;}
  /** 🔤️ Emits actual Unicode text annotations with position, extent, rotation and character height. */
  text(name:string,literal:string,position:Point3,rotation:number,width:number,height:number,fontSize:number,color:Rgb|null=null):Part21Instance[]{if(new TextEncoder().encode(name).length>512||new TextEncoder().encode(literal).length>512||!validPoint(position)||![rotation,width,height,fontSize].every(Number.isFinite)||width<=0||height<=0||fontSize<=0)throw new Error("step.draughting.text");const out:Part21Instance[]=[],origin=this.alloc(out,"CARTESIAN_POINT",[str(""),xyz(position)]),direction=this.alloc(out,"DIRECTION",[str(""),xyz([Math.cos(rotation),Math.sin(rotation),0])]),axis=this.alloc(out,"AXIS2_PLACEMENT_3D",[str(""),ref(origin),ref(this.normal),ref(direction)]),extent=this.alloc(out,"PLANAR_EXTENT",[str(name),real(width),real(height)]),item=this.alloc(out,"TEXT_LITERAL_WITH_EXTENT",[str(name),str(literal),ref(axis),str("bottom left"),enumeration("RIGHT"),ref(this.font),ref(extent)]),rgb=this.color(out,color??[0,0,0]),appearance=this.alloc(out,"TEXT_STYLE_FOR_DEFINED_FONT",[ref(rgb)]),style=this.alloc(out,"TEXT_STYLE_WITH_BOX_CHARACTERISTICS",[str(name),ref(appearance),aggregate([{kind:"typedValue",value:{name:"BOX_HEIGHT",items:[real(fontSize)]}}])]),assignment=this.assignment(out,style),occurrence=this.alloc(out,"ANNOTATION_TEXT_OCCURRENCE",[str(name),refs([assignment]),ref(item)]);this.model(out,name+"/text",occurrence);return out;}
}

export { readDraughting } from "./📖️read/🟦️.ts";
export type { DraughtingProjection,CurvePaint,TextAnnotation } from "./📖️read/🟦️.ts";
