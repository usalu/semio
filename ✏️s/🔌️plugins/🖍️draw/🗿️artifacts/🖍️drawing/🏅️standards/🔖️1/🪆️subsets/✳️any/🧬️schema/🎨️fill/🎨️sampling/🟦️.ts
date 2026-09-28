/** 🎨️ Prepared local-space paint with stable stops and logarithmic lookup. */
import type {Color,Fill,Stop} from "../🟦️.ts";

export const MAX_PAINT_STOPS=4096;
const unit=(value:number):boolean=>Number.isFinite(value)&&value>=0&&value<=1;
const validColor=(color:Color):boolean=>color.length===4&&color.every(unit);

export class GradientRamp {
  readonly #stops:Stop[];
  constructor(stops:readonly Stop[]) {
    if(stops.length>MAX_PAINT_STOPS || stops.some(stop=>!unit(stop.offset)||!validColor(stop.color))) throw new Error("Invalid gradient stops");
    this.#stops=stops.map(stop=>({offset:stop.offset,color:[...stop.color] as Color})).sort((a,b)=>a.offset-b.offset);
  }
  sample(offset:number):Color {
    if(Number.isNaN(offset)) throw new Error("Invalid gradient position");
    const stops=this.#stops;
    if(!stops.length) return [0,0,0,0];
    let lo=0,hi=stops.length;
    while(lo<hi) {const mid=lo+Math.floor((hi-lo)/2);if(stops[mid]!.offset<=offset) lo=mid+1;else hi=mid;}
    if(lo===0) return [...stops[0]!.color];
    const left=stops[lo-1]!;
    if(lo===stops.length) return [...left.color];
    const right=stops[lo]!,t=(offset-left.offset)/(right.offset-left.offset);
    return left.color.map((value,index)=>value+(right.color[index]!-value)*t) as Color;
  }
}

type Geometry={kind:"solid";color:Color}|{kind:"linear";origin:[number,number];unit:[number,number];length:number}|{kind:"radial";center:[number,number];radius:number};

export class PreparedFill {
  readonly #geometry:Geometry;
  readonly #ramp:GradientRamp|null;
  constructor(fill:Fill) {
    if(fill.kind==="solid") {
      if(!validColor(fill.color)) throw new Error("Invalid fill color");
      this.#geometry={kind:"solid",color:[...fill.color]};this.#ramp=null;
    } else {
      this.#ramp=new GradientRamp(fill.stops);
      if(fill.kind==="linearGradient") {
        const dx=fill.x2-fill.x1,dy=fill.y2-fill.y1,length=Math.hypot(dx,dy);
        if(![fill.x1,fill.y1,fill.x2,fill.y2,length].every(Number.isFinite)) throw new Error("Invalid linear gradient geometry");
        this.#geometry={kind:"linear",origin:[fill.x1,fill.y1],unit:length===0?[0,0]:[dx/length,dy/length],length};
      } else {
        if(![fill.cx,fill.cy,fill.r].every(Number.isFinite)||fill.r<0) throw new Error("Invalid radial gradient geometry");
        this.#geometry={kind:"radial",center:[fill.cx,fill.cy],radius:fill.r};
      }
    }
  }
  sample(point:readonly [number,number]):Color {
    if(!point.every(Number.isFinite)) throw new Error("Fill sample position must be finite");
    const geometry=this.#geometry;
    if(geometry.kind==="solid") return [...geometry.color];
    const offset=geometry.kind==="linear" ? geometry.length===0 ? 1 : ((point[0]-geometry.origin[0])*geometry.unit[0]+(point[1]-geometry.origin[1])*geometry.unit[1])/geometry.length
      : geometry.radius===0 ? 1 : Math.hypot(point[0]-geometry.center[0],point[1]-geometry.center[1])/geometry.radius;
    return this.#ramp!.sample(offset);
  }
}
