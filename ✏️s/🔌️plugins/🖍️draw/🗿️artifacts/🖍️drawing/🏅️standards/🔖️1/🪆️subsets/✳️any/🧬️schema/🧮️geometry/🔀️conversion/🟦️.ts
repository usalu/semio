/** 🛤️ Editable path geometry matching the native primitive renderer. */
import {parsePathSegment,type PathSegment} from "../../🟦️.ts";

export function shapePath(kind:string,geometry:Record<string,unknown>):PathSegment[] {
  const number=(key:string):number=>{
    const value=geometry[key];
    if(typeof value!=="number" || !Number.isFinite(value)) throw new Error(`Invalid shape coordinate ${key}`);
    return value;
  };
  let segments:PathSegment[];
  if(kind==="rect") {
    const x=number("x"),y=number("y"),w=number("width"),h=number("height");
    segments=[{kind:"move",to:[x,y]},{kind:"line",to:[x+w,y]},{kind:"line",to:[x+w,y+h]},{kind:"line",to:[x,y+h]},{kind:"close"}];
  } else if(kind==="line") segments=[{kind:"move",to:[number("x1"),number("y1")]},{kind:"line",to:[number("x2"),number("y2")]}];
  else if(kind==="polygon") {
    if(!Array.isArray(geometry.points) || geometry.points.length===0) throw new Error("A polygon needs points");
    segments=geometry.points.map((to,index)=>parsePathSegment({kind:index===0?"move":"line",to}));
    segments.push({kind:"close"});
  } else if(kind==="ellipse" || kind==="circle") {
    const cx=number("cx"),cy=number("cy"),rx=number(kind==="circle"?"r":"rx"),ry=number(kind==="circle"?"r":"ry"),k=.5522847498,x=rx*k,y=ry*k;
    segments=[
      {kind:"move",to:[cx,cy-ry]},
      {kind:"cubic",ctrl1:[cx+x,cy-ry],ctrl2:[cx+rx,cy-y],to:[cx+rx,cy]},
      {kind:"cubic",ctrl1:[cx+rx,cy+y],ctrl2:[cx+x,cy+ry],to:[cx,cy+ry]},
      {kind:"cubic",ctrl1:[cx-x,cy+ry],ctrl2:[cx-rx,cy+y],to:[cx-rx,cy]},
      {kind:"cubic",ctrl1:[cx-rx,cy-y],ctrl2:[cx-x,cy-ry],to:[cx,cy-ry]},
      {kind:"close"},
    ];
  } else throw new Error("Select a supported geometric shape");
  return segments.map(segment=>parsePathSegment(segment));
}
