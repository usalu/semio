/** 🛤️ Editable path geometry matching the native primitive renderer. */
import {parsePathGeometrySegment,type PathGeometrySegment} from "../../🟦️.ts";

export function shapeSegment(kind:string,geometry:Record<string,unknown>,index:number):PathGeometrySegment|undefined {
  if(!Number.isSafeInteger(index)||index<0)return undefined;
  const number=(key:string):number=>{
    const value=geometry[key];
    if(typeof value!=="number" || !Number.isFinite(value)) throw new Error(`Invalid shape coordinate ${key}`);
    return value;
  };
  let segments:PathGeometrySegment[];
  if(kind==="rect") {
    const x=number("x"),y=number("y"),w=number("width"),h=number("height");
    segments=[{kind:"move",to:[x,y]},{kind:"line",to:[x+w,y]},{kind:"line",to:[x+w,y+h]},{kind:"line",to:[x,y+h]},{kind:"close"}];
  } else if(kind==="line") segments=[{kind:"move",to:[number("x1"),number("y1")]},{kind:"line",to:[number("x2"),number("y2")]}];
  else if(kind==="polygon") {
    if(!Array.isArray(geometry.points) || geometry.points.length===0) throw new Error("A polygon needs points");
    return index<geometry.points.length?parsePathGeometrySegment({kind:index===0?"move":"line",to:geometry.points[index]}):index===geometry.points.length?{kind:"close"}:undefined;
  } else if(kind==="ellipse" || kind==="circle") {
    const cx=number("cx"),cy=number("cy"),rx=number(kind==="circle"?"r":"rx"),ry=number(kind==="circle"?"r":"ry");
    const arc=(to:[number,number]):PathGeometrySegment=>({kind:"arc",rx:Math.abs(rx),ry:Math.abs(ry),rotation:0,largeArc:false,sweep:(rx>=0)===(ry>=0),to});
    segments=[{kind:"move",to:[cx,cy-ry]},arc([cx+rx,cy]),arc([cx,cy+ry]),arc([cx-rx,cy]),arc([cx,cy-ry]),{kind:"close"}];
  } else throw new Error("Select a supported geometric shape");
  return segments[index]===undefined?undefined:parsePathGeometrySegment(segments[index]);
}

/** 🔷️ Materializes primitive segments for a semantic shape-to-path edit. */
export function shapePath(kind:string,geometry:Record<string,unknown>):PathGeometrySegment[] {
  const segments:PathGeometrySegment[]=[];
  for(let index=0;;index++) {const segment=shapeSegment(kind,geometry,index);if(!segment)return segments;segments.push(segment);}
}
