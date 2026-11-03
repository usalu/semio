/** 🛤️ Curved transformed fill operands enter work-granted planar booleans. */
import type {PathSegment,Vec2} from "../../📐️drawing-contract/🟦️.ts";
import {BooleanJob,type BooleanInput,type BooleanOperand,type BooleanProgress,type BooleanRetirement} from "../🟦️.ts";
import {PathFlattenJob,type FlatContours,type PathFlattenInput,type PathFlattenProgress} from "../../🛤️path/📏️flatten/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkRetirementProgress} from "../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
export type PathBooleanRetirement=RetainedWorkRetirementCursor;
export type PathBooleanRetirementProgress=RetainedWorkRetirementProgress;
export type PathBooleanOperand=PathFlattenInput&{fillRule:"nonzero"|"evenodd"};
export type PathBooleanInput=Omit<BooleanInput,"operands">&{operands:readonly PathBooleanOperand[]};
export type PathBooleanProgress={phase:"admitting"|"flattening"|"transforming"|"boolean"|"complete";operands:number;sourceSegments:number;points:number;work:number;flatten:PathFlattenProgress|null;boolean:BooleanProgress|null;done:boolean};
function invalid(message:string):never{throw new RangeError(message);}
const finiteSequence=(values:{length:number;at(index:number):number|undefined},length:number)=>{if(values.length!==length)return false;for(let at=0;at<length;at++){const value=values.at(at);if(value===undefined||!Number.isFinite(value)||Math.abs(value)>1e9)return false;}return true;};
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
/** ⏱️ The original path operands stay resident while caller-funded children publish world-space contours. */
export class PathBooleanJob{
 #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private phase:PathBooleanProgress["phase"]="admitting";private work=0;private operands=0;private sourceSegments=0;private points=0;private at=0;private contourAt=0;private pointAt=0;
 private current:PathBooleanOperand|null=null;private admitted:PathSegment[]=[];private local:FlatContours|null=null;private world:Vec2[][]=[];private contour:Vec2[]|null=null;private prepared:BooleanOperand[]=[];
 private flatten:PathFlattenJob|null=null;private flatProgress:PathFlattenProgress|null=null;private boolean:BooleanJob|null=null;private booleanProgress:BooleanProgress|null=null;private output:PathSegment[]=[];
 private retiredFlats:RetainedWorkRetirementCursor[]=[];
 private booleanRetirement:BooleanRetirement|null=null;private retained:BooleanOperand[]=[];private retainedOperand:BooleanOperand|null=null;
 private cancelled=false;private transferred=false;private failure:unknown=null;
 constructor(private input:PathBooleanInput){
  if(!Array.isArray(input.operands)||input.operands.length<1||input.operands.length>1024)invalid("Invalid boolean path operands");
  const check=new BooleanJob({...input,operands:[{contours:[],fillRule:"nonzero"}]});check.cancel();this.input={...input};
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.flatten?.failurePending()===true;}
 settlementPending():boolean{
  switch(this.phase){
   case "flattening":return this.flatten?.settlementPending()===true;
   case "transforming":{const contour=this.local?.at(this.contourAt);return contour===undefined||this.contour!==null&&this.pointAt>=contour.points.length;}
   case "complete":return true;
   case "admitting":return this.current!==null||this.operands>=this.input.operands.length;
   default:return false;
  }
 }
 private step(c:RetainedWorkProducerStep):void{
  if(this.phase==="admitting"){
   if(!this.current){
    if(this.operands===this.input.operands.length){this.boolean=new BooleanJob({...this.input,operands:this.prepared});this.input={...this.input,operands:[]};this.prepared=[];this.phase="boolean";c.commitControlWork();return;}const operand=this.input.operands[this.operands];
    if(!operand||typeof operand.segments?.length!=="number"||operand.segments.length>65536||typeof operand.transform?.length!=="number"||!finiteSequence(operand.transform,6)||!Number.isFinite(operand.tolerance)||operand.tolerance<1e-6||operand.tolerance>16||!["nonzero","evenodd"].includes(operand.fillRule))invalid("Invalid boolean path operand");
    this.current={segments:operand.segments,transform:Array.from({length:6},(_,at)=>operand.transform.at(at)!),tolerance:operand.tolerance,fillRule:operand.fillRule};this.at=0;this.admitted=[];c.commitSemanticWork();return;
   }
   if(this.at<this.current.segments.length){if(this.sourceSegments>=this.input.maxEdges)invalid("Boolean paths exceed source segment budget");this.admitted.push(copySegment(this.current.segments.at(this.at++)!));this.sourceSegments++;c.commitSemanticWork();return;}
   this.flatten=new PathFlattenJob({...this.current,segments:this.admitted});this.admitted=[];this.phase="flattening";c.commitControlWork();return;
  }
  if(this.phase==="flattening"){
   const flatten=this.flatten!,p=c.advanceChild(flatten);this.flatProgress=p;if(this.points+p.points>this.input.maxEdges)invalid("Boolean paths exceed flattened point budget");
   if(p.done){const transferred=flatten.intoRetirement();this.retiredFlats.push(transferred.job);this.local=transferred.output;this.phase="transforming";this.flatten=null;this.world=[];this.contour=null;this.contourAt=0;this.pointAt=0;c.commitControlWork();}return;
  }
  if(this.phase==="transforming"){
   const local=this.local!.at(this.contourAt);if(!local){this.prepared.push({contours:this.world,fillRule:this.current!.fillRule});this.retiredFlats.push(this.local!.intoRetirement());this.local=null;this.world=[];this.current=null;this.operands++;this.phase="admitting";c.commitSemanticWork();return;}
   if(!this.contour){this.contour=[];this.world.push(this.contour);c.commitControlWork();return;}
   const p=local.points.at(this.pointAt++);if(p){const m=this.current!.transform as number[],q:Vec2=[m[0]!*p[0]+m[2]!*p[1]+m[4]!,m[1]!*p[0]+m[3]!*p[1]+m[5]!];if(!q.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))invalid("Boolean world point exceeds coordinate budget");if(this.points>=this.input.maxEdges)invalid("Boolean paths exceed flattened point budget");this.contour.push(q);this.points++;c.commitSemanticWork();return;}
   this.contourAt++;this.pointAt=0;this.contour=null;c.commitControlWork();return;
  }
  if(this.phase==="boolean"){
   this.booleanProgress=c.advanceChild(this.boolean!);if(this.booleanProgress.done){const transferred=this.boolean!.intoRetirement();this.output=transferred.output!;this.booleanRetirement=transferred.job;this.retained=transferred.operands as BooleanOperand[];this.boolean=null;this.phase="complete";}
  }
 }
 private progress():PathBooleanProgress{return{phase:this.phase,operands:this.operands,sourceSegments:this.sourceSegments,points:this.points,work:this.work,flatten:this.flatProgress,boolean:this.booleanProgress,done:this.phase==="complete"};}
 advance(items:number,work:RetainedWorkLedger):PathBooleanProgress{return this.#runtime.advance(items,work,{maximumWork:this.input.maxWork,workLimitMessage:"Boolean paths exceed work budget",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Path boolean cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.progress()});}
 result():PathSegment[]{if(this.cancelled)throw new DOMException("Path boolean cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Path boolean is incomplete");return this.output;}
 private retireOperands(items:BooleanOperand[]):boolean{
  if(this.retainedOperand){const contours=this.retainedOperand.contours as Vec2[][];if(contours.length){contours.pop();return false;}this.retainedOperand=null;return false;}
  this.retainedOperand=items.pop()??null;return this.retainedOperand===null;
 }
 /** 🧹️ Compose child cursors while private results remain unavailable to callers. */
 intoRetirement():{job:PathBooleanRetirement;output:PathSegment[]|null}{
  if(this.transferred)throw Error("Path boolean ownership already transferred");this.transferred=true;
  if(this.flatten){const moved=this.flatten.intoRetirement();this.retiredFlats.push(moved.job);if(moved.output)this.retiredFlats.push(moved.output.intoRetirement());this.flatten=null;}
  if(this.local){this.retiredFlats.push(this.local.intoRetirement());this.local=null;}
  if(this.boolean){const moved=this.boolean.intoRetirement();this.booleanRetirement=moved.job;this.retained=moved.operands as BooleanOperand[];if(moved.output)this.output=moved.output;this.boolean=null;}
  const output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;if(output)this.output=[];this.cancelled=true;
  let slot=0;const job=new RetainedWorkRetirement((work,turn)=>{
   const child=slot===0?this.retiredFlats[0]:slot===1?this.booleanRetirement??undefined:undefined;
   if(child!==undefined&&!child.terminalIsEmpty()){try{child.advance(1,work);}finally{turn.commitActualWork(child.lastTurnWork());}if(!child.terminalIsEmpty()||turn.actualWork()>0)return false;}
   if(!work.canStep())return false;
   const next=():boolean=>{slot++;turn.commitActualWork();return slot===11;};
   switch(slot){
    case 0:if(child!==undefined){this.retiredFlats.shift();turn.commitActualWork();return false;}return next();
    case 1:this.booleanRetirement=null;return next();
    case 2:if(!this.retireOperands(this.retained)){turn.commitActualWork();return false;}this.retained=[];return next();
    case 3:this.input={...this.input,operands:[]};return next();
    case 4:this.current=null;return next();
    case 5:this.admitted=[];return next();
    case 6:this.contour=null;return next();
    case 7:if(this.world.length){this.world.pop();turn.commitActualWork();return false;}this.world=[];return next();
    case 8:if(!this.retireOperands(this.prepared)){turn.commitActualWork();return false;}this.prepared=[];return next();
    case 9:this.output=[];return next();
    default:this.input.operands=[];return next();
   }
  });return{job,output};
 }
 cancel():void{this.cancelled=true;}
}
