/** 📐️ Complete layer-transform mutation wire contract. */
import {parseRasterTransform,type RasterTransform} from "../../🟦️.ts";
import {inverse} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/🧩️compositing/🟦️.ts";
export type ChangeLayerTransform={layerId:string;expected:RasterTransform;transform:RasterTransform};
export function parseChangeLayerTransform(value:unknown):ChangeLayerTransform {
  if(!value||typeof value!=="object"||Array.isArray(value))throw new TypeError("Invalid transform mutation");
  const row=value as Record<string,unknown>;
  if(typeof row.layerId!=="string"||!row.layerId||Object.keys(row).some(key=>!["layerId","expected","transform"].includes(key)))throw new TypeError("Invalid transform mutation");
  const expected=parseRasterTransform(row.expected),transform=parseRasterTransform(row.transform);
  for(const t of [expected,transform])inverse([t.a,t.b,t.c,t.d,t.x,t.y]);
  return {layerId:row.layerId,expected,transform};
}
