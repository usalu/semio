/** 🎬️ Owned vector plans and node-indexed painted caches publish as one completed scene. */
import {PaintedPathPrepareJob,advanceCursor,PaintedRegionsCloseJob,type PaintedPathRegions} from "../🟦️.ts";
import {ScenePlanCloseJob} from "../../🧹️retire/🟦️.ts";
import {ScenePlanOwner,type ScenePlanRead} from "../../🗺️plan-model/🎟️owned/🟦️.ts";
import {segmentBounds} from "../../../🧮️geometry/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkTurn} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
type Point=readonly[number,number];type Bounds=[number,number,number,number];
export type PaintedSceneLimits={maxNodes:number;maxSegments:number;maxPoints:number;maxContours:number;maxWork:number};
export type PreparedSceneCounts={nodes:number;segments:number;points:number;contours:number};
export type PreparedPaint={kind:"empty"}|{kind:"path";regions:PaintedPathRegions}|{kind:"image";corners:Point[]};
export type PreparedGeometry={geometryBounds:Bounds|null;bounds:Bounds|null;paint:PreparedPaint};
export type PreparedScene={owner:ScenePlanOwner;plan:ScenePlanRead;geometry:PreparedGeometry[];flatness:number;counts:PreparedSceneCounts};
export type ScenePaintProgress=PreparedSceneCounts&{phase:"nodes"|"bounds"|"pathInput"|"path"|"pathCleanup"|"complete";work:number;done:boolean};
const valid=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
const invalid=(message:string):never=>{throw new RangeError(message);};
const union=(a:Bounds|null,b:Bounds):Bounds=>a?[Math.min(a[0],b[0]),Math.min(a[1],b[1]),Math.max(a[2],b[2]),Math.max(a[3],b[3])]:b;
function rectangle(x:number,y:number,w:number,h:number,m:readonly number[]):PreparedGeometry{const corners=[[x,y],[x+w,y],[x+w,y+h],[x,y+h]].map(([a,b])=>[m[0]!*a!+m[2]!*b!+m[4]!,m[1]!*a!+m[3]!*b!+m[5]!]as Point);let bounds:Bounds|null=null;for(const p of corners){if(!p.every(valid))invalid("Painted scene coordinate limit exceeded");bounds=union(bounds,[p[0],p[1],p[0],p[1]]);}return{geometryBounds:bounds,bounds,paint:{kind:"image",corners}};}
/** 🧹️ Adopts actual cache entries and their real vector-plan close frontier. */
export class PreparedSceneCloseJob implements RetainedWorkRetirementCursor{
 private geometry:PreparedGeometry[];private extra:PreparedGeometry|null;private plan:ScenePlanCloseJob|null;private regions:RetainedWorkRetirementCursor|null=null;private fixed:Point[]|null=null;private phase=0;private cursor:RetainedWorkRetirement;
 constructor(scene:PreparedScene,extra:PreparedGeometry|null=null){this.geometry=scene.geometry;this.extra=extra;this.plan=new ScenePlanCloseJob(scene.owner);this.cursor=new RetainedWorkRetirement((work,turn)=>this.step(work,turn));}
 private step(work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{
  const again=():boolean=>{turn.commitActualWork();return false;};
  if(advanceCursor(this.regions??(this.phase===0?null:this.plan),work,turn))return false;
  if(!work.canStep())return false;
  if(this.regions){this.regions=null;return again();}
  if(this.fixed){if(this.fixed.length)this.fixed.pop();else this.fixed=null;return again();}
  if(this.phase===0){const g=this.extra??this.geometry.pop();this.extra=null;if(g){if(g.paint.kind==="path")this.regions=new PaintedRegionsCloseJob(g.paint.regions);else if(g.paint.kind!=="empty")this.fixed=g.paint.corners;return again();}this.geometry=[];this.phase=1;return again();}
  if(this.plan){this.plan=null;return again();}
  turn.commitActualWork();return true;
 }
 terminalIsEmpty():boolean{return this.cursor.terminalIsEmpty();}
 advance(grant:number,work:RetainedWorkLedger){return this.cursor.advance(grant,work);}
 nextDemand():RetainedWorkDemand{return this.cursor.nextDemand();}
 lastTurnWork():number{return this.cursor.lastTurnWork();}
}
/** 🧱️ Constructor adoption keeps refused owned input available for real retirement. */
export class ScenePaintJob{
 private owner:ScenePlanOwner|null;private plan:ScenePlanRead|null;private limits:PaintedSceneLimits;private geometry:PreparedGeometry[]=[];private counts:PreparedSceneCounts={nodes:0,segments:0,points:0,contours:0};private phase:ScenePaintProgress["phase"]="nodes";private work=0;private validated=false;private index=0;private segment=0;private pen:Point=[0,0];private origin:Point=[0,0];private bounds:Bounds|null=null;private matrix:[number,number,number,number,number,number]=[1,0,0,1,0,0];
 private path:PaintedPathPrepareJob|null=null;private pathRetirement:RetainedWorkRetirementCursor|null=null;private retiredPaths:RetainedWorkRetirementCursor[]=[];private pending:PreparedGeometry|null=null;private output:PreparedScene|null=null;private cancelled=false;private failure:unknown=null;private transferred=false;#runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 constructor(plan:ScenePlanOwner,private flatness:number,limits:PaintedSceneLimits){this.plan=ScenePlanOwner.prototype.borrow.call(plan);this.owner=ScenePlanOwner.transfer(plan);this.limits={...limits};}
 private validate():void{const l=this.limits;if(!Number.isFinite(this.flatness)||this.flatness<1e-6||this.flatness>16||!([['maxNodes',1024],['maxSegments',65536],['maxPoints',262144],['maxContours',65536],['maxWork',1e9]]as const).every(([key,max])=>Number.isSafeInteger(l[key])&&l[key]>0&&l[key]<=max))invalid("Invalid painted scene contract");if(this.plan!.nodes.length>l.maxNodes)invalid("Painted scene node limit exceeded");this.validated=true;}
 private admit(g:PreparedGeometry):void{const p=g.paint,points=p.kind==="path"?p.regions.points:p.kind==="empty"?0:4,contours=p.kind==="path"?p.regions.contours:p.kind==="empty"?0:1;if(this.counts.points+points>this.limits.maxPoints||this.counts.contours+contours>this.limits.maxContours)invalid("Painted scene cache count limit exceeded");this.geometry.push(g);this.pending=null;this.counts.points+=points;this.counts.contours+=contours;this.counts.nodes++;this.index++;this.phase="nodes";}
 private unit():void{
  if(!this.validated){this.validate();return;}
  const node=this.plan!.nodes.at(this.index);
  switch(this.phase){
   case"nodes":{if(!node){this.output={owner:this.owner!,plan:this.plan!,geometry:this.geometry,flatness:this.flatness,counts:{...this.counts}};this.plan=null;this.owner=null;this.geometry=[];this.phase="complete";break;}if(node.transform.length!==6||!node.transform.every(valid))invalid("Painted scene transform limit exceeded");this.matrix=[...node.transform];const c=node.content;if(c.kind==="boolean"||c.kind==="trace")invalid("Unresolved algorithm in painted scene");if((c.kind==="path"||c.kind==="glyphs")){if(this.counts.segments+c.segments.length>this.limits.maxSegments)invalid("Painted scene segment limit exceeded");this.counts.segments+=c.segments.length;}this.bounds=null;this.segment=0;this.pen=[0,0];this.origin=[0,0];if(!node.visible||c.kind==="group"){this.admit({geometryBounds:null,bounds:null,paint:{kind:"empty"}});}else if((c.kind==="path"||c.kind==="glyphs")){if(c.stroke&&(c.stroke.dash?.length??0)>1024)invalid("Painted scene stroke limit exceeded");this.phase="bounds";}else if(c.kind==="image"){this.pending=rectangle(0,0,c.width,c.height,node.transform);this.admit(this.pending);}else invalid("Unresolved semantic text in painted scene");break;}
   case"bounds":{const c=node!.content;if(c.kind!=="path"&&c.kind!=="glyphs")throw new RangeError("Painted scene path owner changed");const s=c.segments.at(this.segment);if(s){if(!(s.kind==="close"&&this.bounds===null)){const b=segmentBounds(s,this.pen,this.origin,this.matrix),world:Bounds=[b[0],b[1],b[0]+b[2],b[1]+b[3]];if(!world.every(valid))invalid("Painted scene bounds limit exceeded");this.bounds=union(this.bounds,world);}if(s.kind==="move")this.origin=this.pen=s.to;else if(s.kind==="close")this.pen=this.origin;else this.pen=s.to;this.segment++;}break;}
   case"pathCleanup":this.admit(this.pending!);break;
   case"complete":break;
  }
 }
 private step(c:RetainedWorkProducerStep):void{
  if(this.validated&&this.phase==="bounds"){const content=this.plan!.nodes.at(this.index)!.content;if((content.kind==="path"||content.kind==="glyphs")&&this.segment>=content.segments.length){this.phase="pathInput";c.commitControlWork();return;}}
  if(this.validated&&this.phase==="pathInput"){this.path=PaintedPathPrepareJob.fromPrepared(this.plan!.nodes.at(this.index)!,this.flatness,{maxSegments:65536,maxPoints:262144,maxContours:65536,maxWork:this.limits.maxWork-this.work});this.phase="path";c.commitControlWork();return;}
  if(this.validated&&this.phase==="path"){const node=this.plan!.nodes.at(this.index)!,content=node.content;if(content.kind!=="path"&&content.kind!=="glyphs")throw new RangeError("Painted scene path owner changed");if(c.advanceChild(this.path!.asChild(index=>content.segments.at(index))).done){const moved=this.path!.intoRetirement();this.path=null;if(this.pathRetirement)this.retiredPaths.push(this.pathRetirement);this.pathRetirement=moved.job;this.pending={geometryBounds:this.bounds,bounds:moved.output!.bounds,paint:{kind:"path",regions:moved.output!}};this.phase="pathCleanup";}return;}
  this.unit();c.commitSemanticWork();
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.path?.failurePending()===true;}
 settlementPending():boolean{return this.phase==="complete"||this.phase==="pathInput"||(this.phase==="path"&&this.path?.settlementPending()===true);}
 advance(items:number,work:RetainedWorkLedger):ScenePaintProgress{const job=this;try{return this.#runtime.advance(items,work,{get maximumWork(){return job.validated?job.limits.maxWork:Number.MAX_SAFE_INTEGER;},workLimitMessage:"Painted scene work limit exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Painted scene cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>({phase:this.phase,work:this.work,...this.counts,done:this.phase==="complete"})});}catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.cancelled)this.failure=error;throw error;}}
 result():PreparedScene{if(this.cancelled)throw new DOMException("Painted scene cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Painted scene incomplete");return this.output!;}
 cancel():void{this.cancelled=true;}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:PreparedScene|null}{if(this.transferred)invalid("Painted scene ownership already transferred");this.transferred=true;const success=this.phase==="complete"&&!this.cancelled&&!this.failure,output=success?this.output:null;let discarded:PreparedSceneCloseJob|null=null;if(this.output&&!success)discarded=new PreparedSceneCloseJob(this.output);else if(this.plan)discarded=new PreparedSceneCloseJob({owner:this.owner!,plan:this.plan,geometry:this.geometry,flatness:this.flatness,counts:this.counts},this.pending);this.output=null;this.plan=null;this.owner=null;this.geometry=[];this.pending=null;this.cancelled=true;if(this.path){this.path.cancel();if(this.pathRetirement)this.retiredPaths.push(this.pathRetirement);this.pathRetirement=this.path.intoRetirement().job;this.path=null;}return{output,job:new RetainedWorkRetirement((work,turn)=>{const retired=this.retiredPaths.at(-1);if(advanceCursor(retired??this.pathRetirement??discarded,work,turn))return false;if(!work.canStep())return false;turn.commitActualWork();if(retired){this.retiredPaths.pop();return false;}if(this.pathRetirement){this.pathRetirement=null;return false;}if(discarded){discarded=null;return false;}this.failure=null;return true;})};}
}
