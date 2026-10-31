/** 🔮️ Original independent Sharp and polygon-clipping coverage stay behind plain first-party values. */
import sharp from "sharp";
import polygonClipping from "polygon-clipping";
import type {CoverageInput} from "../../🟦️.ts";
export async function svgCoverage(input:CoverageInput):Promise<number[]> {
  let contours=Array.from({length:input.contours.length},(_,at)=>{const row=input.contours.at(at);if(row===undefined)throw Error("Original oracle contour absent");return Array.from({length:row.length},(_,index):[number,number]=>{const point=row.at(index),x=point?.at(0),y=point?.at(1);if(x===undefined||y===undefined)throw Error("Original oracle point absent");return[x,y];});}),transform=Array.from({length:input.transform.length},(_,at)=>{const value=input.transform.at(at);if(value===undefined)throw Error("Original oracle matrix absent");return value;});
  if(contours.length===1&&input.rule==="nonzero"&&contours[0]!.some(p=>p.some(value=>Math.abs(value)>100000))) {
    const m=transform,ring=contours[0]!.map((p):[number,number]=>[m[0]!*p[0]!+m[2]!*p[1]!+m[4]!,m[1]!*p[0]!+m[3]!*p[1]!+m[5]!]);
    contours=polygonClipping.intersection([ring],[[[0,0],[input.width,0],[input.width,input.height],[0,input.height]]]).flat();transform=[1,0,0,1,0,0];
  }
  const d=contours.map(points=>points.length?`M${points.map(point=>point.join(",")).join("L")}Z`:"").join("");
  const scale=128,svg=`<svg xmlns="http://www.w3.org/2000/svg" width="${input.width*scale}" height="${input.height*scale}" viewBox="0 0 ${input.width} ${input.height}"><path transform="matrix(${transform.join(" ")})" d="${d}" fill="red" fill-rule="${input.rule}"/></svg>`;
  const pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer();
  const coverage:number[]=[];
  for(let at=0;at<input.width*input.height;at++) {
    const x=at%input.width,y=Math.floor(at/input.width);let alpha=0;
    for(let dy=0;dy<scale;dy++)for(let dx=0;dx<scale;dx++)alpha+=pixels[((y*scale+dy)*input.width*scale+x*scale+dx)*4+3]!;
    coverage.push(Math.round(alpha/(scale*scale)));
  }
  return coverage;
}
