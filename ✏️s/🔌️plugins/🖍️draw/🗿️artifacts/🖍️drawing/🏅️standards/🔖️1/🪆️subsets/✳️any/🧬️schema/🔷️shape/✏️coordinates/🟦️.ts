/** 🔷️ Scalar authored shape coordinates support sparse semantic diffs. */
import type {DrawingLayerNode} from "../../🟦️.ts";
import {binary64,binary64Value,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export const SHAPE_COORDINATE_FIELDS=["rectX","rectY","rectWidth","rectHeight","ellipseCx","ellipseCy","ellipseRx","ellipseRy","circleCx","circleCy","circleR","lineX1","lineY1","lineX2","lineY2","polygonX","polygonY"] as const;
export type ShapeCoordinateField=typeof SHAPE_COORDINATE_FIELDS[number];
type Shape=Extract<DrawingLayerNode,{kind:"shape"}>;
function coordinate(shape:Shape,field:ShapeCoordinateField,index?:number):{kind:"rect"|"ellipse"|"circle"|"line"|"polygon";key:string} {
  if(!SHAPE_COORDINATE_FIELDS.includes(field))throw new Error("Unknown shape coordinate");
  const kind=field.startsWith("rect")?"rect":field.startsWith("ellipse")?"ellipse":field.startsWith("circle")?"circle":field.startsWith("line")?"line":"polygon";
  if(shape.shapeKind!==kind||!shape[kind])throw new Error("The coordinate does not belong to this shape");
  if(kind==="polygon"){if(!Number.isSafeInteger(index)||index!<0||!shape.polygon!.points[index!])throw new Error("Missing polygon vertex");}
  else if(index!==undefined)throw new Error("Only polygon coordinates require a vertex index");
  const suffix=field.slice(kind.length);return {kind,key:suffix[0]!.toLowerCase()+suffix.slice(1)};
}
/** 📏️ Reads one canonical IEEE scalar while preserving its owned identity. */
export function shapeCoordinate(shape:Shape,field:ShapeCoordinateField,index?:number):number {
  const {kind,key}=coordinate(shape,field,index);
  return binary64Value(kind==="polygon"?shape.polygon!.points[index!]![field==="polygonX"?0:1]:(shape[kind] as unknown as Record<string,Binary64>)[key]!);
}
/** 🩹️ Replaces exactly one admitted scalar and preserves unrelated canonical records. */
export function setShapeCoordinate(shape:Shape,field:ShapeCoordinateField,index:number|undefined,value:number):Shape {
  const {kind,key}=coordinate(shape,field,index);
  if(!Number.isFinite(value)||(["rectWidth","rectHeight","ellipseRx","ellipseRy","circleR"].includes(field)&&value<0))throw new Error("Enter a finite coordinate and nonnegative dimensions");
  if(kind==="polygon") {const points=shape.polygon!.points.slice(),point=[...points[index!]!] as [Binary64,Binary64];point[field==="polygonX"?0:1]=binary64(value);points[index!]=point;return {...shape,polygon:{points}};}
  return {...shape,[kind]:{...shape[kind],[key]:binary64(value)}};
}
