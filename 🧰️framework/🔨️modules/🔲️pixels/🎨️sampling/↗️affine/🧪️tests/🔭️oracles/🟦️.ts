/** 🔭️ Independent area intersections and native canvas image filtering for tests. */
import polygonClipping from "polygon-clipping";
import {createCanvas,ImageData} from "@napi-rs/canvas";
import type {AffineImageInput} from "../../🟦️.ts";
function polygonArea(ring:readonly(readonly number[])[]):number{let total=0;const anchor=ring[0]!;for(let i=1;i+1<ring.length;i++){const a=ring[i]!,b=ring[i+1]!;total+=(a[0]!-anchor[0]!)*(b[1]!-anchor[1]!)-(a[1]!-anchor[1]!)*(b[0]!-anchor[0]!);}return Math.abs(total)/2;}
export function areaOracle(v:AffineImageInput):Uint8Array{
 const output=new Uint8Array(v.width*v.height*4),m=v.transform;if(m[0]!*m[3]!-m[1]!*m[2]===0)return output;
 const map=(x:number,y:number):[number,number]=>[m[0]!*x+m[2]!*y+m[4]!-v.origin[0],m[1]!*x+m[3]!*y+m[5]!-v.origin[1]];
 for(let y=0;y<v.height;y++)for(let x=0;x<v.width;x++){
  const target:[number,number][]=[[x,y],[x+1,y],[x+1,y+1],[x,y+1],[x,y]];let alpha=0;const rgb=[0,0,0];
  for(let sy=0;sy<v.source.height;sy++)for(let sx=0;sx<v.source.width;sx++){
   const ring=[map(sx,sy),map(sx+1,sy),map(sx+1,sy+1),map(sx,sy+1),map(sx,sy)];
   const clipped=polygonClipping.intersection([ring],[target]);let area=0;for(const poly of clipped)for(let i=0;i<poly.length;i++)area+=(i===0?1:-1)*polygonArea(poly[i]!);
   const at=(sy*v.source.width+sx)*4,amount=v.source.pixels[at+3]!*area;alpha+=amount;for(let c=0;c<3;c++)rgb[c]!+=v.source.pixels[at+c]!*amount;
  }
  const at=(y*v.width+x)*4,a=Math.round(Math.min(255,alpha));if(a){for(let c=0;c<3;c++)output[at+c]=Math.round(rgb[c]!/alpha);output[at+3]=a;}
 }
 return output;
}
export function canvasOracle(v:AffineImageInput):Uint8Array{
 const source=createCanvas(v.source.width,v.source.height);source.getContext("2d").putImageData(new ImageData(new Uint8ClampedArray(v.source.pixels),v.source.width,v.source.height),0,0);
 const canvas=createCanvas(v.width,v.height),ctx=canvas.getContext("2d"),m=v.transform;
 ctx.setTransform(m[0]!,m[1]!,m[2]!,m[3]!,m[4]!-v.origin[0],m[5]!-v.origin[1]);ctx.imageSmoothingEnabled=v.sampling!=="nearest";ctx.imageSmoothingQuality="low";ctx.drawImage(source,0,0);
 return new Uint8Array(ctx.getImageData(0,0,v.width,v.height).data);
}
export function delta(a:Uint8Array,b:Uint8Array):number{let max=0;for(let at=0;at<a.length;at+=4){max=Math.max(max,Math.abs(a[at+3]!-b[at+3]!));for(let c=0;c<3;c++)max=Math.max(max,Math.abs(a[at+c]!*a[at+3]!/255-b[at+c]!*b[at+3]!/255));}return max;}
