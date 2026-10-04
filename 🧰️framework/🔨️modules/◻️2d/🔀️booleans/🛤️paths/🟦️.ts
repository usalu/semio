/** 🛤️ Curved transformed fill operands enter work-granted planar booleans. */
import type {PathSegment,Vec2} from "../../🟦️.ts";
import {BooleanJob,type BooleanInput,type BooleanOperand,type BooleanProgress} from "../🟦️.ts";
import {PathFlattenJob,type FlatContour,type PathFlattenInput,type PathFlattenProgress} from "../../🛤️path/📏️flatten/🟦️.ts";
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "../../🧹️retire/🟦️.ts";
export type PathBooleanRetirement=WorkRetirement;
export type PathBooleanRetirementProgress=WorkRetirementProgress;
export type PathBooleanOperand=PathFlattenInput&{fillRule:"nonzero"|"evenodd"};
export type PathBooleanInput=Omit<BooleanInput,"operands">&{operands:readonly PathBooleanOperand[]};
export type PathBooleanProgress={phase:"admitting"|"flattening"|"transforming"|"boolean"|"complete";operands:number;sourceSegments:number;points:number;work:number;flatten:PathFlattenProgress|null;boolean:BooleanProgress|null;done:boolean};
export type PathBooleanOptions={workBudget?:number;signal?:AbortSignal;onProgress?:(p:PathBooleanProgress)=>void};
function invalid(message:string):never{throw new RangeError(message);}
function copyPoint(p:Vec2):Vec2{if(!Array.isArray(p)||p.length!==2)invalid("Invalid boolean path point");return[p[0],p[1]];}
function copySegment(s:PathSegment):PathSegment{
 if(!s||typeof s!=="object")invalid("Invalid boolean path segment");
 switch(s.kind){
  case "move":case "line":return{kind:s.kind,to:copyPoint(s.to)};
  case "quad":return{kind:s.kind,ctrl:copyPoint(s.ctrl),to:copyPoint(s.to)};
  case "cubic":return{kind:s.kind,ctrl1:copyPoint(s.ctrl1),ctrl2:copyPoint(s.ctrl2),to:copyPoint(s.to)};
  case "arc":return{kind:s.kind,rx:s.rx,ry:s.ry,rotation:s.rotation,largeArc:s.largeArc,sweep:s.sweep,to:copyPoint(s.to)};
  case "close":return{kind:s.kind};
  default:return invalid("Invalid boolean path segment");
 }
}
/** ⏱️ Admits paths, flattens in world tolerance and transforms one point per grant. */
export class PathBooleanJob{
 private phase:PathBooleanProgress["phase"]="admitting";private work=0;private operands=0;private sourceSegments=0;private points=0;private at=0;private contourAt=0;private pointAt=0;
 private current:PathBooleanOperand|null=null;private admitted:PathSegment[]=[];private local:(FlatContour|null)[]=[];private world:Vec2[][]=[];private contour:Vec2[]|null=null;private prepared:BooleanOperand[]=[];
 private flatten:PathFlattenJob|null=null;private flatProgress:PathFlattenProgress|null=null;private boolean:BooleanJob|null=null;private booleanProgress:BooleanProgress|null=null;private output:PathSegment[]=[];
 private flatRetirement:WorkRetirement|null=null;private booleanRetirement:WorkRetirement|null=null;private retained:BooleanOperand[]=[];private retainedOperand:BooleanOperand|null=null;
 private cancelled=false;private transferred=false;private failure:unknown=null;
 constructor(private input:PathBooleanInput){
  if(!Array.isArray(input.operands)||input.operands.length<1||input.operands.length>1024)invalid("Invalid boolean path operands");
  const check=new BooleanJob({...input,operands:[{contours:[],fillRule:"nonzero"}]});check.cancel();this.input={...input};
 }
 private step():void{
  if(this.phase==="admitting"){
   if(!this.current){
    if(this.operands===this.input.operands.length){this.boolean=new BooleanJob({...this.input,operands:this.prepared});this.input={...this.input,operands:[]};this.prepared=[];this.phase="boolean";return;}const operand=this.input.operands[this.operands];
    if(!operand||!Array.isArray(operand.segments)||operand.segments.length>65536||!Array.isArray(operand.transform)||operand.transform.length!==6||!operand.transform.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9)||!Number.isFinite(operand.tolerance)||operand.tolerance<1e-6||operand.tolerance>16||!["nonzero","evenodd"].includes(operand.fillRule))invalid("Invalid boolean path operand");
    this.current={segments:operand.segments,transform:[...operand.transform],tolerance:operand.tolerance,fillRule:operand.fillRule};this.at=0;this.admitted=[];return;
   }
   if(this.at<this.current.segments.length){if(this.sourceSegments>=this.input.maxEdges)invalid("Boolean paths exceed source segment budget");this.admitted.push(copySegment(this.current.segments[this.at++]!));this.sourceSegments++;return;}
   this.flatten=new PathFlattenJob({...this.current,segments:this.admitted});this.admitted=[];this.phase="flattening";return;
  }
  if(this.phase==="flattening"){
   if(this.flatRetirement){if(this.flatRetirement.advance(1).done){this.flatRetirement=null;this.phase="transforming";}return;}
   const p=this.flatten!.advance(1);this.flatProgress=p;if(this.points+p.points>this.input.maxEdges)invalid("Boolean paths exceed flattened point budget");
   if(p.done){const transferred=this.flatten!.intoRetirement();this.local=transferred.output!;this.flatRetirement=transferred.job;this.flatten=null;this.world=[];this.contour=null;this.contourAt=0;this.pointAt=0;}return;
  }
  if(this.phase==="transforming"){
   const local=this.local[this.contourAt];if(!local){this.prepared.push({contours:this.world,fillRule:this.current!.fillRule});this.local=[];this.world=[];this.current=null;this.operands++;this.phase="admitting";return;}
   if(!this.contour){this.contour=[];this.world.push(this.contour);return;}
   const p=local.points[this.pointAt++];if(p){const m=this.current!.transform,q:Vec2=[m[0]!*p[0]+m[2]!*p[1]+m[4]!,m[1]!*p[0]+m[3]!*p[1]+m[5]!];if(!q.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))invalid("Boolean world point exceeds coordinate budget");if(this.points>=this.input.maxEdges)invalid("Boolean paths exceed flattened point budget");this.contour.push(q);this.points++;return;}
   this.local[this.contourAt]=null;this.contourAt++;this.pointAt=0;this.contour=null;return;
  }
  if(this.phase==="boolean"){
   if(this.booleanRetirement){if(this.booleanRetirement.advance(1).done)this.booleanRetirement=null;return;}
   if(!this.boolean){if(this.retireOperands(this.retained))this.phase="complete";return;}
   this.booleanProgress=this.boolean.advance(1);if(this.booleanProgress.done){const transferred=this.boolean.intoRetirement();this.output=transferred.output!;this.booleanRetirement=transferred.job;this.retained=transferred.operands as BooleanOperand[];this.boolean=null;}
  }
 }
 advance(budget:number):PathBooleanProgress{
  if(!Number.isSafeInteger(budget)||budget<=0)invalid("Invalid boolean path work grant");if(this.cancelled)throw new DOMException("Path boolean cancelled","AbortError");if(this.failure)throw this.failure;
  try{for(let at=0;at<budget&&this.phase!=="complete";at++){if(this.work>=this.input.maxWork)invalid("Boolean paths exceed work budget");this.step();this.work++;}}catch(error){this.failure=error;throw error;}
  return{phase:this.phase,operands:this.operands,sourceSegments:this.sourceSegments,points:this.points,work:this.work,flatten:this.flatProgress,boolean:this.booleanProgress,done:this.phase==="complete"};
 }
 result():PathSegment[]{if(this.cancelled)throw new DOMException("Path boolean cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Path boolean is incomplete");return this.output;}
 private retireOperands(items:BooleanOperand[]):boolean{
  if(this.retainedOperand){const contours=this.retainedOperand.contours as Vec2[][];if(contours.length){contours.pop();return false;}this.retainedOperand=null;return false;}
  this.retainedOperand=items.pop()??null;return this.retainedOperand===null;
 }
 /** 🧹️ Compose child cursors while private results remain unavailable to callers. */
 intoRetirement():{job:PathBooleanRetirement;output:PathSegment[]|null}{
  if(this.transferred)throw Error("Path boolean ownership already transferred");this.transferred=true;
  if(this.flatten){const moved=this.flatten.intoRetirement();this.flatRetirement=moved.job;if(moved.output)this.local=moved.output;this.flatten=null;}
  if(this.boolean){const moved=this.boolean.intoRetirement();this.booleanRetirement=moved.job;this.retained=moved.operands as BooleanOperand[];if(moved.output)this.output=moved.output;this.boolean=null;}
  const output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;if(output)this.output=[];this.cancelled=true;
  let slot=0;const job=new UnitRetirement(()=>{
   switch(slot){
    case 0:if(this.flatRetirement){if(!this.flatRetirement.advance(1).done)return false;this.flatRetirement=null;}break;
    case 1:if(this.booleanRetirement){if(!this.booleanRetirement.advance(1).done)return false;this.booleanRetirement=null;}break;
    case 2:if(!this.retireOperands(this.retained))return false;this.retained=[];break;
    case 3:this.input={...this.input,operands:[]};break;
    case 4:this.current=null;break;
    case 5:this.admitted=[];break;
    case 6:if(this.local.length){this.local.pop();return false;}this.local=[];break;
    case 7:this.contour=null;break;
    case 8:if(this.world.length){this.world.pop();return false;}this.world=[];break;
    case 9:if(!this.retireOperands(this.prepared))return false;this.prepared=[];break;
    case 10:this.output=[];break;
    case 11:this.input.operands=[];break;
   }
   return ++slot===12;
  });return{job,output};
 }
 private clear():void{this.flatten?.cancel();this.boolean?.cancel();this.flatten=null;this.boolean=null;this.flatRetirement=null;this.booleanRetirement=null;this.retained=[];this.retainedOperand=null;this.current=null;this.input={...this.input,operands:[]};this.admitted=[];this.local=[];this.world=[];this.contour=null;this.prepared=[];this.output=[];}
 cancel():void{this.cancelled=true;if(!this.transferred)this.clear();}
}
/** ⏳️ Publishes only complete paths after observers and cancellation checks. */
export async function booleanPaths(input:PathBooleanInput,options:PathBooleanOptions={}):Promise<PathSegment[]>{
 const abort=()=>{if(options.signal?.aborted)throw new DOMException("Path boolean cancelled","AbortError");};abort();const job=new PathBooleanJob(input);
 try{for(;;){abort();const p=job.advance(options.workBudget??4096);options.onProgress?.(p);abort();if(p.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();throw error;}
}
