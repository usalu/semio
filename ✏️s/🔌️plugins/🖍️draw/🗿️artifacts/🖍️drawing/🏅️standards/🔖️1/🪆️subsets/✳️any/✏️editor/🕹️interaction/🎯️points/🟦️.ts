/** 🎯️ Snapshot-bound point references for framework-owned node selection. */
import { Blake3Hasher } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔏️hash/🟦️.ts";
import { parsePathSegment, type PathSegment } from "../../../🧬️schema/🟦️.ts";
import type { PathPoint } from "../../../🧬️schema/🧮️geometry/✏️editing/🟦️.ts";

export interface PointSelectionRef { readonly layerId:string; readonly geometry:string; readonly index:number; readonly point:PathPoint; }

export function pointSlots(segment:PathSegment):readonly PathPoint[] {
  return segment.kind==="close"?[]:segment.kind==="cubic"?["anchor","control1","control2"]:segment.kind==="quad"?["anchor","control1"]:["anchor"];
}

export function parsePointId(id:string):PointSelectionRef|null {
  const end=id.lastIndexOf(":"),middle=id.lastIndexOf(":",end-1),start=id.lastIndexOf(":",middle-1);
  if(start<1 || middle<=start || end<=middle)return null;
  const layerId=id.slice(0,start),geometry=id.slice(start+1,middle),digits=id.slice(middle+1,end),point=id.slice(end+1);
  if(!/^[0-9a-f]{64}$/.test(geometry) || !/^(0|[1-9][0-9]*)$/.test(digits) || (!Number.isSafeInteger(Number(digits)) || Number(digits)>0xffffffff) || !["anchor","control1","control2"].includes(point))return null;
  return {layerId,geometry,index:Number(digits),point:point as PathPoint};
}

export function pointId(reference:PointSelectionRef):string|null {
  const id=`${reference.layerId}:${reference.geometry}:${reference.index}:${reference.point}`;
  const parsed=parsePointId(id);
  return parsed && parsed.layerId===reference.layerId && parsed.geometry===reference.geometry && parsed.index===reference.index && parsed.point===reference.point?id:null;
}

export function geometryId(segments:readonly PathSegment[]):string|null {
  const hasher=new Blake3Hasher(),bytes=new Uint8Array(8),view=new DataView(bytes.buffer);
  hasher.update(new TextEncoder().encode("draw-points-v1"));
  const number=(value:number):void=>{view.setFloat64(0,value===0?0:value,true);hasher.update(bytes);};
  const point=(value:readonly number[]):void=>{number(value[0]!);number(value[1]!);};
  try {
    for(const source of segments) {
      const segment=parsePathSegment(source);
      hasher.update(Uint8Array.of({move:0,line:1,quad:2,cubic:3,arc:4,close:5}[segment.kind]));
      if(segment.kind==="quad")point(segment.ctrl);
      if(segment.kind==="cubic"){point(segment.ctrl1);point(segment.ctrl2);}
      if(segment.kind==="arc"){number(segment.rx);number(segment.ry);number(segment.rotation);hasher.update(Uint8Array.of(Number(segment.largeArc),Number(segment.sweep)));}
      if(segment.kind!=="close")point(segment.to);
    }
  } catch {return null;}
  return Array.from(hasher.digest(),byte=>byte.toString(16).padStart(2,"0")).join("");
}

/** 🖱️ A press on a selected point retains its drag group; modified presses change membership. */
export function pickPointSelection(current:readonly string[],hit:string|null,mode:"replace"|"toggle"|"add"):string[] {
  if(hit===null)return mode==="replace"?[]:[...current];
  const selected=current.includes(hit);
  if(mode==="replace")return selected?[...current]:[hit];
  if(mode==="toggle" && selected)return current.filter(id=>id!==hit);
  return selected?[...current]:[...current,hit];
}

/** ▧️ Marquees address anchors in world coordinates, including rectangle edges. */
export function anchorInMarquee(segment:PathSegment,matrix:readonly number[],start:readonly number[],end:readonly number[]):boolean {
  if(segment.kind==="close" || ![...matrix,...start,...end].every(Number.isFinite))return false;
  const [x,y]=segment.to;
  const px=matrix[0]!*x+matrix[2]!*y+matrix[4]!,py=matrix[1]!*x+matrix[3]!*y+matrix[5]!;
  return Number.isFinite(px)&&Number.isFinite(py)&&px>=Math.min(start[0]!,end[0]!)&&px<=Math.max(start[0]!,end[0]!)&&py>=Math.min(start[1]!,end[1]!)&&py<=Math.max(start[1]!,end[1]!);
}

/** 🧮️ A region merge applies membership once per distinct hit. */
export function mergePointSelection(current:readonly string[],hits:readonly string[],mode:"replace"|"add"|"toggle"):string[] {
  const incoming=new Set(hits),existing=new Set(current);
  if(mode==="replace")return [...incoming];
  const result=[...existing].filter(id=>mode!=="toggle"||!incoming.has(id));
  for(const id of incoming)if(!existing.has(id))result.push(id);
  return result;
}
