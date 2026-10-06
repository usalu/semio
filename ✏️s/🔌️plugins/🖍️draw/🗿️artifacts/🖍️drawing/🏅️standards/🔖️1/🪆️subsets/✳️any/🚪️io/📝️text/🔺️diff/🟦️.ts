/** 📝️ Text representation for `drawing.drawing.diff`. */
export type DrawingDiffText = string;
import { parseDrawingDiff, type DrawingDiff } from "../../../🧬️schema/🔺️diff/🟦️.ts";
import { binary64 } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";

function decodedDrawingValue(value:unknown):unknown {
  if(typeof value==="number")return binary64(value);
  if(Array.isArray(value))return value.map(decodedDrawingValue);
  if(value!==null&&typeof value==="object")return Object.fromEntries(Object.entries(value).map(([key,item])=>[key,decodedDrawingValue(item)]));
  return value;
}

/** 📝️ Admit JSON numeric carriers before the decoded diff guards run. */
export function decodeDrawingDiffJson(text:string):DrawingDiff {
  const value:unknown=JSON.parse(text);
  if(value!==null&&typeof value==="object"){
    const row=value as Record<string,unknown>;
    const layers=row.layers as {patched?:{patch:Record<string,unknown>}[];added?:{layer:unknown}[]}|undefined;
    for(const entry of layers?.patched??[])for(const field of ["transform","fill","stroke","traceParams","layer"])if(entry.patch[field]!=null)entry.patch[field]=decodedDrawingValue(entry.patch[field]);
    for(const entry of layers?.added??[])entry.layer=decodedDrawingValue(entry.layer);
    if(row.artboard!=null)row.artboard=decodedDrawingValue(row.artboard);
    if(row.artifact!==null&&typeof row.artifact==="object"){
      const artifact=row.artifact as {layers?:unknown[];artboard?:unknown};
      if(artifact.layers)artifact.layers=artifact.layers.map(decodedDrawingValue);
      if(artifact.artboard!=null)artifact.artboard=decodedDrawingValue(artifact.artboard);
    }
  }
  return parseDrawingDiff(value);
}
