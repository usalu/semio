/** 🎯 Polygon containment twin for lasso selection. */
import type { Point } from "../🟦️.ts";
export function polygonContainsPoint(polygon: readonly Point[],point: Point): boolean {
  if(polygon.length<3 || !point.every(Number.isFinite)) return false;
  let inside=false,previous=polygon[polygon.length-1]!;
  for(const current of polygon) {
    const cross=(current[0]-previous[0])*(point[1]-previous[1])-(current[1]-previous[1])*(point[0]-previous[0]);
    if(Math.abs(cross)<1e-10 && point[0]>=Math.min(current[0],previous[0]) && point[0]<=Math.max(current[0],previous[0]) && point[1]>=Math.min(current[1],previous[1]) && point[1]<=Math.max(current[1],previous[1])) return true;
    if((current[1]>point[1])!==(previous[1]>point[1]) && point[0]<(previous[0]-current[0])*(point[1]-current[1])/(previous[1]-current[1])+current[0]) inside=!inside;
    previous=current;
  }
  return inside;
}
export function polygonEnclosesBounds(polygon: readonly Point[],bounds: [number,number,number,number]): boolean {
  const [x,y,width,height]=bounds;
  if(!bounds.every(Number.isFinite) || width<0 || height<0) return false;
  const corners: Point[]=[[x,y],[x+width,y],[x+width,y+height],[x,y+height]];
  if(!corners.every(point=>polygonContainsPoint(polygon,point))) return false;
  if(width===0 || height===0) return polygonEnclosesSegment(polygon,[x,y],[x+width,y+height]);
  let previous=polygon[polygon.length-1]!;
  for(const current of polygon) {
    let lower=0,upper=1,possible=true;
    for(const axis of [0,1] as const) {
      const min=bounds[axis],max=min+bounds[axis+2]!,delta=current[axis]-previous[axis];
      if(delta===0) { if(previous[axis]<=min || previous[axis]>=max) possible=false; }
      else { const a=(min-previous[axis])/delta,b=(max-previous[axis])/delta; lower=Math.max(lower,Math.min(a,b));upper=Math.min(upper,Math.max(a,b)); }
    }
    if(possible && lower<upper) return false;
    previous=current;
  }
  return true;
}

function polygonEnclosesSegment(polygon: readonly Point[],from: Point,to: Point): boolean {
  const direction: Point=[to[0]-from[0],to[1]-from[1]],length=direction[0]**2+direction[1]**2;
  if(length===0) return polygonContainsPoint(polygon,from);
  const cross=(a:Point,b:Point)=>a[0]*b[1]-a[1]*b[0],cuts=[0,1];
  let previous=polygon[polygon.length-1]!;
  for(const current of polygon) {
    const edge: Point=[current[0]-previous[0],current[1]-previous[1]],offset:Point=[previous[0]-from[0],previous[1]-from[1]],denominator=cross(direction,edge);
    if(denominator!==0) {
      const t=cross(offset,edge)/denominator,u=cross(offset,direction)/denominator;
      if(t>0 && t<1 && u>=0 && u<=1) cuts.push(t);
    } else if(Math.abs(cross(offset,direction))<1e-10) {
      for(const point of [previous,current]) { const t=((point[0]-from[0])*direction[0]+(point[1]-from[1])*direction[1])/length; if(t>0 && t<1) cuts.push(t); }
    }
    previous=current;
  }
  cuts.sort((a,b)=>a-b);
  return cuts.slice(1).every((end,index)=>{const t=(cuts[index]!+end)/2;return polygonContainsPoint(polygon,[from[0]+direction[0]*t,from[1]+direction[1]*t]);});
}
