/** 📉️ Work-granted Douglas–Peucker simplification preserves authored curve segments. */
import {parsePathGeometrySegment,type PathGeometrySegment} from "../../../🟦️.ts";
type Point=[number,number];
type Span={start:number;end:number;next:number;farthest:number;distance:number;force:number};
export type PathSimplifyProgress={phase:"scanning"|"reducing"|"building"|"complete";completed:number;total:number;work:number;done:boolean};
function distance(p:Point,a:Point,b:Point):number {
  const scale=Math.max(1,...p.map(Math.abs),...a.map(Math.abs),...b.map(Math.abs));
  p=[p[0]/scale,p[1]/scale];a=[a[0]/scale,a[1]/scale];b=[b[0]/scale,b[1]/scale];
  const dx=b[0]-a[0],dy=b[1]-a[1],length=dx*dx+dy*dy,t=length===0?0:Math.max(0,Math.min(1,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length));
  return Math.hypot(p[0]-a[0]-t*dx,p[1]-a[1]-t*dy)*scale;
}
/** ⏱️ Owns a private candidate until every line-run scan and distance test has completed. */
export class PathSimplifyJob {
  private keep:boolean[];private output:PathGeometrySegment[]=[];private stack:Span[]=[];private span:Span|null=null;private at=0;private run:number|null=null;private build=0;private work=0;private done=false;private cancelled=false;private contour=false;private failed:unknown=null;
  constructor(private source:readonly PathGeometrySegment[],private tolerance:number) {
    if(!Number.isFinite(tolerance)||tolerance<1e-6||tolerance>1e6||source.length>65536)throw new Error("Invalid path simplification contract");
    this.keep=Array(source.length).fill(true);
  }
  private point(index:number):Point {const segment=this.source[index]!;if(segment.kind==="close")throw new Error("Invalid line-run endpoint");return segment.to;}
  private makeSpan(start:number,end:number,force:number):Span {return {start,end,next:start+1,farthest:start,distance:0,force};}
  private step():void {
    this.span??=this.stack.pop()??null;
    if(this.span) {
      const span=this.span;this.span=null;
      if(span.next<span.end){const value=distance(this.point(span.next),this.point(span.start),this.point(span.end));if(value>span.distance){span.distance=value;span.farthest=span.next;}span.next++;this.span=span;return;}
      if(span.farthest>span.start&&(span.distance>this.tolerance||span.force>0)){this.keep[span.farthest]=true;const force=Math.max(0,span.force-1);this.stack.push(this.makeSpan(span.farthest,span.end,force),this.makeSpan(span.start,span.farthest,force));}
      return;
    }
    if(this.at<this.source.length) {
      const item=parsePathGeometrySegment(this.source[this.at]);
      if(item.kind!=="line"&&this.run!==null){const start=this.run,end=this.at-1;this.run=null;const a=this.point(start),b=this.point(end),force=item.kind==="close"&&this.source[start]?.kind==="move"?(a[0]===b[0]&&a[1]===b[1]?2:1):0;this.stack.push(this.makeSpan(start,end,force));return;}
      if(item.kind==="move")this.contour=true;else if(!this.contour)throw new Error("A contour must start with a move");else if(item.kind==="close")this.contour=false;
      if(item.kind==="line") {
        if(this.at===0||this.source[this.at-1]?.kind==="close")throw new Error("A contour must start with a move");
        this.run??=this.at-1;
        if(this.source[this.at+1]?.kind==="line")this.keep[this.at]=false;
      }
      this.at++;return;
    }
    if(this.run!==null){this.stack.push(this.makeSpan(this.run,this.at-1,0));this.run=null;return;}
    if(this.build<this.source.length){if(this.keep[this.build])this.output.push(parsePathGeometrySegment(this.source[this.build]));this.build++;return;}
    this.done=true;
  }
  advance(grant:number):PathSimplifyProgress {
    if(!Number.isSafeInteger(grant)||grant<=0)throw new Error("Path simplification requires a positive work grant");
    if(this.cancelled)throw new DOMException("Path simplification cancelled","AbortError");
    if(this.failed)throw this.failed;
    try{for(let index=0;index<grant&&!this.done;index++){if(this.work>=10000000)throw new Error("Path simplification exceeds work capacity");this.step();this.work++;}}catch(error){this.failed=error;throw error;}
    return {phase:this.done?"complete":this.span||this.stack.length?"reducing":this.at===this.source.length?"building":"scanning",completed:this.at,total:this.source.length,work:this.work,done:this.done};
  }
  cancel():void {this.cancelled=true;}
  close(grant:number):{released:number;done:boolean} {
    if(!Number.isSafeInteger(grant)||grant<0)throw new Error("Invalid path retirement grant");
    this.cancel();let released=0;
    while(released<grant){if(this.output.length)this.output.pop();else if(this.stack.length)this.stack.pop();else if(this.keep.length)this.keep.pop();else {this.span=null;this.source=[];return {released,done:true};}released++;}
    return {released,done:false};
  }
  result():PathGeometrySegment[] {if(this.cancelled)throw new DOMException("Path simplification cancelled","AbortError");if(this.failed)throw this.failed;if(!this.done)throw new Error("Path simplification is incomplete");return this.output;}
}
