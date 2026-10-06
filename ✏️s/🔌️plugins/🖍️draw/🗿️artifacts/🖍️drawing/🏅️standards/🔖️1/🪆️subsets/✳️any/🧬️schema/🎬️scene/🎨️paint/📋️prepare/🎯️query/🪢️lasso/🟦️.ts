/** 🪢️ A borrowed painted segment is partitioned and enclosed under polygon-edge work grants. */
type Point=readonly[number,number];
import type {PreparedPaint} from "../../🟦️.ts";
class SegmentPartitionCursor{
 private cuts=[0,1];private at=0;
 constructor(private from:Point,private to:Point){}
 advance(polygon:readonly Point[]):readonly number[]|null{
  if(this.at===polygon.length){this.cuts.sort((a,b)=>a-b);this.cuts=this.cuts.filter((t,i)=>i===0||t!==this.cuts[i-1]);return this.cuts;}
  const a=polygon[this.at]!,b=polygon[(++this.at)%polygon.length]!,dx=this.to[0]-this.from[0],dy=this.to[1]-this.from[1],ex=b[0]-a[0],ey=b[1]-a[1],ox=a[0]-this.from[0],oy=a[1]-this.from[1],denominator=dx*ey-dy*ex,length=Math.hypot(dx,dy);
  if(length===0)return null;
  if(Math.abs(denominator)>Number.EPSILON*16*length*Math.hypot(ex,ey)){const t=(ox*ey-oy*ex)/denominator,u=(ox*dy-oy*dx)/denominator;if(t>0&&t<1&&u>=0&&u<=1)this.cuts.push(t);}
  else if(Math.abs(ox*dy-oy*dx)<=Number.EPSILON*8*Math.max(1,Math.abs(a[0]),Math.abs(a[1]),Math.abs(this.from[0]),Math.abs(this.from[1]))*length){for(const p of [a,b]){const t=Math.abs(dx)>=Math.abs(dy)?(p[0]-this.from[0])/dx:(p[1]-this.from[1])/dy;if(t>0&&t<1)this.cuts.push(t);}}
  return null;
 }
}
export class SegmentEnclosureCursor{
 private phase:"intersections"|"samples"|"complete"="intersections";private cuts:readonly number[]=[0,1];private at=0;private sample=0;private inside=false;private boundary=false;private enclosed=true;private point:Point;private partition:SegmentPartitionCursor|null;
 constructor(private from:Point,private to:Point){this.point=from;this.partition=new SegmentPartitionCursor(from,to);}
 advance(polygon:readonly Point[]):boolean|null{
  if(this.phase==="complete")return this.enclosed;
  if(this.phase==="intersections"){const cuts=this.partition!.advance(polygon);if(cuts){this.cuts=cuts;this.partition=null;this.phase="samples";}return null;}
  if(this.at===polygon.length){if(!this.inside&&!this.boundary){this.enclosed=false;this.phase="complete";return false;}this.sample++;if(this.sample===this.cuts.length*2-1){this.phase="complete";return true;}const index=this.sample>>1,t=this.sample%2?(this.cuts[index]!+this.cuts[index+1]!)/2:this.cuts[index]!;this.point=[this.from[0]*(1-t)+this.to[0]*t,this.from[1]*(1-t)+this.to[1]*t];this.at=0;this.inside=false;this.boundary=false;return null;}
  const p=this.point,a=polygon[this.at]!,b=polygon[(this.at+1)%polygon.length]!,dx=b[0]-a[0],dy=b[1]-a[1],length=Math.hypot(dx,dy),along=length?Math.max(0,Math.min(length,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length)):0,rounding=Math.max(1,Math.abs(a[0]),Math.abs(a[1]),Math.abs(b[0]),Math.abs(b[1]),Math.abs(p[0]),Math.abs(p[1]))*Number.EPSILON*8;
  this.boundary ||=Math.hypot(p[0]-a[0]-(length?dx*along/length:0),p[1]-a[1]-(length?dy*along/length:0))<=rounding;
  if((a[1]>p[1])!==(b[1]>p[1])&&p[0]<(b[0]-a[0])*(p[1]-a[1])/(b[1]-a[1])+a[0])this.inside=!this.inside;
  this.at++;return null;
 }
}
function sideWinding(p:Point,normal:Point,side:number,a:Point,b:Point):number{
 const above=(y:number)=>y===p[1]?Math.sign(-side*normal[1]):Math.sign(y-p[1]),ay=above(a[1]),by=above(b[1]),dx=b[0]-a[0],dy=b[1]-a[1],px=p[0]-a[0],py=p[1]-a[1],cross=dx*py-dy*px,error=(Math.abs(dx*py)+Math.abs(dy*px))*Number.EPSILON*8,orientation=Math.abs(cross)<=error?Math.sign(side*(dx*normal[1]-dy*normal[0])):Math.sign(cross);
 return ay<=0&&by>0&&orientation>0?1:by<=0&&ay>0&&orientation<0?-1:0;
}
/** 🕳️ Infinitesimal lasso sides reject internal exclusions without inventing epsilon-sized holes. */
export class LassoInteriorCursor{
 private phase:"partition"|"lasso"|"paint"|"complete"="partition";private outer=0;private partition:SegmentPartitionCursor|null=null;private cuts:readonly number[]=[];private interval=0;private point:Point=[0,0];private normal:Point=[0,0];private at=0;private slot=0;private contour=0;private edge=0;private winding=[0,0];private outside=[false,false];private painted=[false,false];private enclosed=true;
 constructor(private bounds:readonly[number,number,number,number]){this.bounds=[...bounds];}
 private sample(polygon:readonly Point[]):void{const a=polygon[this.outer]!,b=polygon[(this.outer+1)%polygon.length]!,t=(this.cuts[this.interval]!+this.cuts[this.interval+1]!)/2;this.point=[a[0]*(1-t)+b[0]*t,a[1]*(1-t)+b[1]*t];this.normal=[a[1]-b[1],b[0]-a[0]];this.at=0;this.winding=[0,0];this.phase="lasso";}
 private next(polygon:readonly Point[]):void{this.interval++;if(this.interval+1===this.cuts.length){this.outer++;this.cuts=[];this.phase="partition";}else this.sample(polygon);}
 advance(polygon:readonly Point[],paint:PreparedPaint):boolean|null{
  if(this.phase==="complete")return this.enclosed;
  if(this.phase==="partition"){if(this.outer===polygon.length){this.phase="complete";return true;}const a=polygon[this.outer]!,b=polygon[(this.outer+1)%polygon.length]!,[x,y,r,d]=this.bounds;if(a[0]===b[0]&&a[1]===b[1]||Math.max(a[0],b[0])<x||Math.min(a[0],b[0])>r||Math.max(a[1],b[1])<y||Math.min(a[1],b[1])>d){this.outer++;return null;}this.partition??=new SegmentPartitionCursor(a,b);const cuts=this.partition.advance(polygon);if(cuts){this.cuts=cuts;this.partition=null;this.interval=0;this.sample(polygon);}return null;}
  if(this.phase==="lasso"){if(this.at<polygon.length){const a=polygon[this.at]!,b=polygon[(++this.at)%polygon.length]!;for(let side=0;side<2;side++)this.winding[side]!+=sideWinding(this.point,this.normal,side?1:-1,a,b);return null;}this.outside=this.winding.map(n=>n%2===0);if(!this.outside.some(Boolean)){this.next(polygon);return null;}this.slot=0;this.contour=0;this.edge=0;this.winding=[0,0];this.painted=[false,false];this.phase="paint";return null;}
  if(this.slot===(paint.kind==="path"?2:1)){if(this.outside.some((outside,side)=>outside&&this.painted[side])){this.enclosed=false;this.phase="complete";return false;}this.next(polygon);return null;}
  const contours=paint.kind==="path"?(this.slot===0?paint.regions.fill:paint.regions.stroke):paint.kind==="empty"?[]:[paint.corners],contour=contours[this.contour];
  if(!contour){for(let side=0;side<2;side++)this.painted[side] ||=paint.kind==="path"&&this.slot===0&&paint.regions.fillRule==="evenodd"?this.winding[side]!%2!==0:this.winding[side]!==0;this.slot++;this.contour=0;this.edge=0;this.winding=[0,0];return null;}
  if(this.edge===contour.length){this.contour++;this.edge=0;return null;}
  const a=contour[this.edge]!,b=contour[(++this.edge)%contour.length]!;for(let side=0;side<2;side++)this.winding[side]!+=sideWinding(this.point,this.normal,side?1:-1,a,b);return null;
 }
}
