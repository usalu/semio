/** 🎨️ Prepared local-space paint with stable stops and logarithmic lookup. */
import type {Color,Fill,Stop} from "../🟦️.ts";
import {UnitRetirement,type WorkRetirement} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts";

type PaintStop={readonly offset:number;readonly color:Readonly<Color>};
type PaintFill={readonly kind:"solid";readonly color:Readonly<Color>}|(Omit<Extract<Fill,{kind:"linearGradient"}>,"stops">&{readonly stops:readonly PaintStop[]})|(Omit<Extract<Fill,{kind:"radialGradient"}>,"stops">&{readonly stops:readonly PaintStop[]});

export const MAX_PAINT_STOPS=4096;
const unit=(value:number):boolean=>Number.isFinite(value)&&value>=0&&value<=1;
const validColor=(color:Readonly<Color>):boolean=>color.length===4&&color.every(unit);

export class GradientRamp {
  #stops:Stop[];#transferred=false;
  constructor(stops:readonly PaintStop[]) {
    if(stops.length>MAX_PAINT_STOPS || stops.some(stop=>!unit(stop.offset)||!validColor(stop.color))) throw new Error("Invalid gradient stops");
    this.#stops=stops.map(stop=>({offset:stop.offset,color:[...stop.color] as Color})).sort((a,b)=>a.offset-b.offset);
  }
  constantColor():Color|null {if(this.#transferred)throw Error("Gradient ramp ownership already transferred");return this.#stops.length<=1 ? [...(this.#stops[0]?.color??[0,0,0,0])] as Color : null;}
  sample(offset:number):Color {
    if(this.#transferred)throw Error("Gradient ramp ownership already transferred");
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
  /** 🧹️ Drains each copied fixed-width stop owner before releasing the ramp header. */
  intoRetirement():WorkRetirement {
    if(this.#transferred)throw Error("Gradient ramp ownership already transferred");this.#transferred=true;let slot=0;
    return new UnitRetirement(()=>{if(slot===0){if(this.#stops.length){this.#stops.pop();return false;}this.#stops=[];slot=1;return false;}return true;});
  }
}

type Geometry={kind:"solid";color:Color}|{kind:"linear";origin:[number,number];unit:[number,number];length:number}|{kind:"radial";center:[number,number];radius:number};

export class PreparedFill {
  #geometry:Geometry|null;
  #ramp:GradientRamp|null;#transferred=false;
  constructor(fill:PaintFill) {
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
  constantColor():Color|null {
    if(this.#transferred)throw Error("Prepared fill ownership already transferred");const geometry=this.#geometry!;
    if(geometry.kind==="solid") return [...geometry.color];
    if(geometry.kind==="linear" ? geometry.length===0 : geometry.radius===0) return this.#ramp!.sample(1);
    return this.#ramp!.constantColor();
  }
  sample(point:readonly [number,number]):Color {
    if(this.#transferred)throw Error("Prepared fill ownership already transferred");
    if(!point.every(Number.isFinite)) throw new Error("Fill sample position must be finite");
    const geometry=this.#geometry!;
    if(geometry.kind==="solid") return [...geometry.color];
    const offset=geometry.kind==="linear" ? geometry.length===0 ? 1 : ((point[0]-geometry.origin[0])*geometry.unit[0]+(point[1]-geometry.origin[1])*geometry.unit[1])/geometry.length
      : geometry.radius===0 ? 1 : Math.hypot(point[0]-geometry.center[0],point[1]-geometry.center[1])/geometry.radius;
    return this.#ramp!.sample(offset);
  }
  /** 🖌️ Adopts the actual ramp and admits each child step before closing the paint header. */
  intoRetirement():WorkRetirement {
    if(this.#transferred)throw Error("Prepared fill ownership already transferred");this.#transferred=true;let ramp=this.#ramp?.intoRetirement()??null,slot=0;
    return new UnitRetirement(()=>{if(slot===0){if(ramp&&!ramp.terminalIsEmpty()){ramp.advance(1);return false;}ramp=null;this.#ramp=null;slot=1;return false;}this.#geometry=null;return true;});
  }
}
