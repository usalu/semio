/** 🖱️ Pointer selection borrows one complete cache and retains only bounded query metadata. */
import {PreparedRegionQueryJob,type PreparedRegionStamp} from "../../🟦️.ts";
import {sceneIdentityMatches,type SceneIdentity} from "../../../🪪️identity/🟦️.ts";
import {sceneSelectionRelation} from "../../../📋️prepare/🧭️selection/🟦️.ts";
import {SegmentEnclosureCursor,LassoInteriorCursor} from "./🪢️lasso/🟦️.ts";
import type {PreparedScene} from "../🟦️.ts";
type Point=readonly[number,number];
export type PreparedSceneIdentity={readonly source:SceneIdentity;readonly build:string;readonly flatness:number};
export type PreparedScenePick={kind:"point";point:Point;tolerance:number;requiredFlatness:number}|{kind:"rectangle";start:Point;end:Point;crossing:boolean}|{kind:"lasso";points:readonly Point[]};
export type PreparedScenePickProgress={phase:"nodes"|"paint"|"complete";work:number;done:boolean};
const valid=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
const point=(p:Point)=>p.length===2&&p.every(valid);
const visible=(n:PreparedScene["plan"]["nodes"][number])=>n.visible&&n.opacity>0&&n.groups.every(g=>g.opacity>0);
const stamp=(identity:PreparedSceneIdentity,entry:number):PreparedRegionStamp=>({...identity,entry});
const matches=(a:PreparedSceneIdentity,b:PreparedSceneIdentity)=>sceneIdentityMatches(a.source,b.source)&&a.build===b.build&&a.flatness===b.flatness;
function segmentCrossesRectangle(a:Point,b:Point,min:Point,max:Point):boolean{let lo=0,hi=1;for(const axis of [0,1]as const){const d=b[axis]-a[axis];if(d===0){if(a[axis]<min[axis]||a[axis]>max[axis])return false;}else{const t0=(min[axis]-a[axis])/d,t1=(max[axis]-a[axis])/d;lo=Math.max(lo,Math.min(t0,t1));hi=Math.min(hi,Math.max(t0,t1));if(lo>hi)return false;}}return true;}
function quadCrossesRectangle(quad:readonly Point[],min:Point,max:Point):boolean{return quadContains(quad,min,0)||quad.some((a,i)=>segmentCrossesRectangle(a,quad[(i+1)%4]!,min,max));}
function quadContains(quad:readonly Point[],p:Point,tolerance:number):boolean{let winding=0;for(let i=0;i<4;i++){const a=quad[i]!,b=quad[(i+1)%4]!,dx=b[0]-a[0],dy=b[1]-a[1],length=Math.hypot(dx,dy),t=length?Math.max(0,Math.min(length,((p[0]-a[0])*dx+(p[1]-a[1])*dy)/length)):0,distance=Math.hypot(p[0]-a[0]-(length?dx*t/length:0),p[1]-a[1]-(length?dy*t/length:0)),rounding=Math.max(1,...a.map(Math.abs),...b.map(Math.abs),...p.map(Math.abs))*Number.EPSILON*8;if(distance<=tolerance+rounding)return true;if((a[1]<=p[1]&&b[1]>p[1])||(b[1]<=p[1]&&a[1]>p[1])){const t=(p[1]-a[1])/(b[1]-a[1]);if(a[0]*(1-t)+b[0]*t>p[0])winding++;}}return winding%2!==0;}
/** 🎯️ Partial candidate indices stay private until the exact cache query is complete. */
export class PreparedScenePickJob{
 private identity:PreparedSceneIdentity;private query:PreparedScenePick;private next:number|null=null;private active=0;private path:PreparedRegionQueryJob|null=null;private rectangle:{min:Point;max:Point;slot:number;contour:number;edge:number}|null=null;private lassoBounds:[number,number,number,number]|null=null;private lasso:{slot:number;contour:number;edge:number;seen:boolean;segment:SegmentEnclosureCursor|null;interior:LassoInteriorCursor|null}|null=null;private hits:number[]=[];private phase:PreparedScenePickProgress["phase"]="nodes";private work=0;private cancelled=false;private failure:unknown=null;
 constructor(identity:PreparedSceneIdentity,query:PreparedScenePick,private maxHits:number){
  new PreparedRegionQueryJob(stamp(identity,0),[0,0],0,16);if(!Number.isSafeInteger(maxHits)||maxHits<1||maxHits>256)throw new RangeError("Invalid prepared scene query contract");
  switch(query.kind){case"point":new PreparedRegionQueryJob(stamp(identity,0),query.point,query.tolerance,query.requiredFlatness);this.query={...query,point:[...query.point]};break;case"rectangle":if(!point(query.start)||!point(query.end)||typeof query.crossing!=="boolean")throw new RangeError("Invalid prepared scene rectangle");this.query={...query,start:[...query.start],end:[...query.end]};break;case"lasso":if(query.points.length<3||query.points.length>256||!query.points.every(point))throw new RangeError("Invalid prepared scene lasso");this.query={...query,points:query.points.map(p=>[...p]as[number,number])};break;default:throw new RangeError("Invalid prepared scene query");}
  this.identity={...identity,source:{...identity.source,revision:[...identity.source.revision]}};if(this.query.kind==="lasso"){this.lassoBounds=[Infinity,Infinity,-Infinity,-Infinity];for(const p of this.query.points){this.lassoBounds[0]=Math.min(this.lassoBounds[0],p[0]);this.lassoBounds[1]=Math.min(this.lassoBounds[1],p[1]);this.lassoBounds[2]=Math.max(this.lassoBounds[2],p[0]);this.lassoBounds[3]=Math.max(this.lassoBounds[3],p[1]);}}
 }
 private authority(live:PreparedSceneIdentity):void{if(this.cancelled)throw new DOMException("Prepared scene query cancelled","AbortError");if(this.failure)throw this.failure;if(!matches(this.identity,live))throw new RangeError("Prepared scene query authority changed");}
 private admit(index:number):void{if(this.hits.length>=this.maxHits)throw new RangeError("Prepared scene query exceeds result capacity");this.hits.push(index);}
 private step(scene:PreparedScene,live:PreparedSceneIdentity):void{
  if(this.phase==="paint"){
   if(this.lasso){const cursor=this.lasso,query=this.query;if(query.kind!=="lasso")throw new RangeError("Prepared scene lasso owner changed");const paint=scene.geometry[this.active]!.paint;
    if(paint.kind==="path"&&paint.regions.flatness!==live.flatness)throw new RangeError("Prepared scene cache precision changed");
    if(cursor.interior){const enclosed=cursor.interior.advance(query.points,paint);if(enclosed!==null){if(enclosed)this.admit(this.active);this.lasso=null;this.phase="nodes";}return;}
    if(cursor.segment){const enclosed=cursor.segment.advance(query.points);if(enclosed===false){this.lasso=null;this.phase="nodes";}else if(enclosed===true){cursor.segment=null;cursor.edge++;}return;}
    const contours=paint.kind==="path"?(cursor.slot===0?paint.regions.fill:paint.regions.stroke):paint.kind==="empty"?[]:[paint.corners];
    if(cursor.slot===(paint.kind==="path"?2:1)){if(cursor.seen)cursor.interior=new LassoInteriorCursor(scene.geometry[this.active]!.bounds!);else{this.lasso=null;this.phase="nodes";}return;}
    const contour=contours[cursor.contour];if(!contour){cursor.slot++;cursor.contour=0;cursor.edge=0;return;}if(cursor.edge===contour.length){cursor.contour++;cursor.edge=0;return;}cursor.segment=new SegmentEnclosureCursor(contour[cursor.edge]!,contour[(cursor.edge+1)%contour.length]!);cursor.seen=true;return;
   }
   const s=stamp(live,this.active),paint=scene.geometry[this.active]!.paint;if(paint.kind!=="path")throw new RangeError("Prepared scene cache entry changed");if(paint.regions.flatness!==live.flatness)throw new RangeError("Prepared scene cache precision changed");
   if(this.path){if(this.path.advance(paint.regions,s,1).done){const hit=this.path.result(s);this.path=null;if(hit){this.admit(this.active);this.phase=this.rectangle?"nodes":"complete";this.rectangle=null;}else if(!this.rectangle)this.phase="nodes";}return;}
   const q=this.rectangle!;if(q.slot===2){this.rectangle=null;this.phase="nodes";return;}const contours=q.slot===0?paint.regions.fill:paint.regions.stroke,c=contours[q.contour];if(!c){q.slot++;q.contour=0;q.edge=0;return;}if(q.edge>=c.length){q.contour++;q.edge=0;return;}const a=c[q.edge]!,b=c[(q.edge+1)%c.length]!;q.edge++;if(segmentCrossesRectangle(a,b,q.min,q.max)){this.admit(this.active);this.rectangle=null;this.phase="nodes";}return;
  }
  if(this.next===null){this.next=scene.plan.nodes.length;return;}if(this.next===0){this.phase="complete";return;}const index=--this.next,node=scene.plan.nodes[index]!;if(!visible(node)||node.lockedAncestors!==0)return;const g=scene.geometry[index]!,q=this.query;
  switch(q.kind){case"point":switch(g.paint.kind){case"empty":break;case"path":this.path=new PreparedRegionQueryJob(stamp(live,index),q.point,q.tolerance,q.requiredFlatness);this.active=index;this.phase="paint";break;case"image":case"textFallback":if(quadContains(g.paint.corners,q.point,q.tolerance)){this.admit(index);this.phase="complete";}break;}break;
   case"rectangle":if(g.bounds){const[x,y,r,b]=g.bounds,min:Point=[Math.min(q.start[0],q.end[0]),Math.min(q.start[1],q.end[1])],max:Point=[Math.max(q.start[0],q.end[0]),Math.max(q.start[1],q.end[1])];if(!q.crossing){if(x>=min[0]&&y>=min[1]&&r<=max[0]&&b<=max[1])this.admit(index);}else if(min[0]<=r&&max[0]>=x&&min[1]<=b&&max[1]>=y){switch(g.paint.kind){case"empty":break;case"path":this.rectangle={min,max,slot:0,contour:0,edge:0};this.path=new PreparedRegionQueryJob(stamp(live,index),min,0,live.flatness);this.active=index;this.phase="paint";break;case"image":case"textFallback":if(quadCrossesRectangle(g.paint.corners,min,max))this.admit(index);break;}}}break;
   case"lasso":if(g.bounds){const[x,y,r,b]=g.bounds;if(x>=this.lassoBounds![0]&&y>=this.lassoBounds![1]&&r<=this.lassoBounds![2]&&b<=this.lassoBounds![3]){this.lasso={slot:0,contour:0,edge:0,seen:false,segment:null,interior:null};this.active=index;this.phase="paint";}}break;
  }
 }
 advance(scene:PreparedScene,live:PreparedSceneIdentity,grant:number):PreparedScenePickProgress{if(!Number.isSafeInteger(grant)||grant<1)throw new RangeError("Invalid prepared scene query work grant");try{this.authority(live);if(scene.flatness!==this.identity.flatness||scene.geometry.length!==scene.plan.nodes.length||scene.geometry.length>1024)throw new RangeError("Prepared scene cache precision or structure changed");for(let at=0;at<grant&&this.phase!=="complete";at++){this.step(scene,live);this.work++;}}catch(error){if(!this.cancelled&&!this.failure)this.failure=error;throw error;}return{phase:this.phase,work:this.work,done:this.phase==="complete"};}
 result(live:PreparedSceneIdentity):readonly number[]{this.authority(live);if(this.phase!=="complete")throw new Error("Prepared scene query incomplete");return this.hits;}
 cancel():void{this.cancelled=true;}
}
/** 📐️ Selection presentation and pointer handles share cached bounds and unlocked ancestry. */
export function preparedSelectionBounds(scene:PreparedScene,ids:readonly string[]):[number,number,number,number]|null{
 if(ids.length>256||ids.reduce((n,id)=>n+new TextEncoder().encode(id).length,0)>16384)throw new RangeError("Scene selection exceeds identity capacity");
 const selected=ids.flatMap(id=>{const node=scene.plan.nodes.find(n=>n.id===id);return node?[node.sourcePath]:[];});let bounds:[number,number,number,number]|null=null;
 scene.plan.nodes.forEach((node,index)=>{if(!visible(node)||sceneSelectionRelation(node.sourcePath,node.lockedAncestors,selected).boundsSelection===null)return;const next=scene.geometry[index]!.bounds??scene.geometry[index]!.geometryBounds;if(next)bounds=bounds?[Math.min(bounds[0],next[0]),Math.min(bounds[1],next[1]),Math.max(bounds[2],next[2]),Math.max(bounds[3],next[3])]:[...next];});return bounds;
}
