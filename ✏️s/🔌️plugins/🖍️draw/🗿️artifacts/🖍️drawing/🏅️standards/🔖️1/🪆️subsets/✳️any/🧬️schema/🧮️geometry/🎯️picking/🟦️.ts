/** 🎯️ Resumable contour winding and stroke proximity in world coordinates. */
import type {PathGeometrySegment} from "../../🟦️.ts";
import {arcGeometry,type Point,type Matrix} from "../🟦️.ts";
type Piece={kind:"curve";points:[Point,Point,Point,Point];depth:number}|{kind:"arc";center:Point;u:Point;v:Point;start:number;sweep:number;depth:number};
function distance(p:Point,a:Point,b:Point):number {
  const dx=b[0]-a[0],dy=b[1]-a[1],length=Math.hypot(dx,dy);
  if(length===0)return Math.hypot(p[0]-a[0],p[1]-a[1]);
  const ux=dx/length,uy=dy/length,t=Math.max(0,Math.min(length,(p[0]-a[0])*ux+(p[1]-a[1])*uy));
  return Math.hypot(p[0]-a[0]-t*ux,p[1]-a[1]-t*uy);
}
const midpoint=(a:Point,b:Point):Point=>[a[0]*.5+b[0]*.5,a[1]*.5+b[1]*.5];
const ellipse=(c:Point,u:Point,v:Point,t:number):Point=>[c[0]+u[0]*Math.cos(t)+v[0]*Math.sin(t),c[1]+u[1]*Math.cos(t)+v[1]*Math.sin(t)];
export class PathHitCursor {
  private next=0;private current:Point=[0,0];private start:Point=[0,0];private open=false;private finished=false;private invalid=false;
  private winding=0;private boundary=false;private stroke=false;private work:Piece[]=[];
  maximumDepth=0;
  constructor(private point:Point,private matrix:Matrix,private radius:number,private flatness:number){
    this.invalid=![...point,...matrix,radius,flatness].every(Number.isFinite)||radius<0||flatness<=0;
    this.flatness=Math.max(flatness,1e-9);
  }
  private map(p:Point):Point {const[a,b,c,d,e,f]=this.matrix;return[a*p[0]+c*p[1]+e,b*p[0]+d*p[1]+f];}
  private line(a:Point,b:Point,stroke:boolean):void {
    if(![...a,...b].every(Number.isFinite)){this.invalid=true;return;}
    const gap=distance(this.point,a,b),rounding=Math.max(1,...a.map(Math.abs),...b.map(Math.abs),...this.point.map(Math.abs))*Number.EPSILON*8;
    this.boundary ||=gap<=this.flatness;this.stroke ||=stroke&&gap<=this.radius+rounding;
    const[x,y]=this.point;
    if((a[1]<=y&&b[1]>y)||(b[1]<=y&&a[1]>y)){const t=(y-a[1])/(b[1]-a[1]);if(a[0]*(1-t)+b[0]*t>x)this.winding+=b[1]>a[1]?1:-1;}
  }
  failed():boolean{return this.invalid;}
  contains(fill:boolean,stroke:boolean,evenOdd=false):boolean{return this.finished&&!this.invalid&&((fill&&(this.boundary||(evenOdd?this.winding%2!==0:this.winding!==0)))||(stroke&&this.stroke));}
  step(segments:readonly PathGeometrySegment[]):boolean {
    return this.stepWith(index=>segments[index]);
  }
  stepWith(segmentAt:(index:number)=>PathGeometrySegment|undefined):boolean {
    if(this.finished)return true;
    if(this.invalid){this.finished=true;return true;}
    const piece=this.work.pop();
    if(piece){
      if(piece.kind==="curve"){
        const[a,b,c,d]=piece.points;
        if(!piece.points.flat().every(Number.isFinite)){this.invalid=true;return false;}
        if(Math.max(distance(b,a,d),distance(c,a,d))<=this.flatness)this.line(a,d,true);
        else if(piece.depth>=32)this.invalid=true;
        else {
          const ab=midpoint(a,b),bc=midpoint(b,c),cd=midpoint(c,d),abc=midpoint(ab,bc),bcd=midpoint(bc,cd),m=midpoint(abc,bcd),depth=piece.depth+1;
          this.work.push({kind:"curve",points:[m,bcd,cd,d],depth},{kind:"curve",points:[a,ab,abc,m],depth});
        }
      }else{
        const{center,u,v,start,sweep,depth}=piece;
        if((Math.hypot(...u)+Math.hypot(...v))*sweep*sweep/8<=this.flatness)this.line(ellipse(center,u,v,start),ellipse(center,u,v,start+sweep),true);
        else if(depth>=32)this.invalid=true;
        else this.work.push({...piece,start:start+sweep*.5,sweep:sweep*.5,depth:depth+1},{...piece,sweep:sweep*.5,depth:depth+1});
      }
      this.maximumDepth=Math.max(this.maximumDepth,this.work.length);return false;
    }
    const segment=segmentAt(this.next++);
    if(!segment){if(this.open){this.line(this.map(this.current),this.map(this.start),false);this.open=false;}this.finished=true;return true;}
    switch(segment.kind){
      case "move":if(this.open)this.line(this.map(this.current),this.map(this.start),false);this.current=segment.to;this.start=segment.to;this.open=true;break;
      case "close":this.line(this.map(this.current),this.map(this.start),true);this.current=this.start;this.open=false;break;
      case "line":this.line(this.map(this.current),this.map(segment.to),true);this.current=segment.to;this.open=true;break;
      case "quad":{
        const a=this.current,c=segment.ctrl,d=segment.to,c1:Point=[a[0]/3+c[0]*2/3,a[1]/3+c[1]*2/3],c2:Point=[d[0]/3+c[0]*2/3,d[1]/3+c[1]*2/3];
        this.work.push({kind:"curve",points:[a,c1,c2,d].map(p=>this.map(p as Point)) as [Point,Point,Point,Point],depth:0});this.current=d;this.open=true;break;
      }
      case "cubic":this.work.push({kind:"curve",points:[this.current,segment.ctrl1,segment.ctrl2,segment.to].map(p=>this.map(p)) as [Point,Point,Point,Point],depth:0});this.current=segment.to;this.open=true;break;
      case "arc":{
        const arc=arcGeometry(this.current,[Math.abs(segment.rx),Math.abs(segment.ry)],segment.rotation,segment.largeArc,segment.sweep,segment.to);
        if(arc){const sr=Math.sin(arc.rotation),cr=Math.cos(arc.rotation),[a,b,c,d]=this.matrix;
          this.work.push({kind:"arc",center:this.map(arc.center),u:[arc.radii[0]*(a*cr+c*sr),arc.radii[0]*(b*cr+d*sr)],v:[arc.radii[1]*(-a*sr+c*cr),arc.radii[1]*(-b*sr+d*cr)],start:arc.start,sweep:arc.sweep,depth:0});
        }else this.line(this.map(this.current),this.map(segment.to),true);
        this.current=segment.to;this.open=true;break;
      }
    }
    this.maximumDepth=Math.max(this.maximumDepth,this.work.length);return false;
  }
}
