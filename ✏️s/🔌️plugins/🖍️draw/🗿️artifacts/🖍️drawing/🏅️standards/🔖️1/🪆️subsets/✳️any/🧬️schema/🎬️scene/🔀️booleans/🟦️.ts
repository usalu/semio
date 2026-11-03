import { validateSceneSourceAddress } from "../🗺️plan-model/🟦️.ts";
/** 🔀️ Complete prepared documents resolve referenced filled geometry under caller grants. */
import {ScenePlanOwner,ScenePlanContentReplaceJob,type ScenePlanRead,type ScenePlanNode,type ScenePlanContent} from "../🗺️plan-model/🎟️owned/🟦️.ts";
import {PathBooleanJob,type PathBooleanOperand,type PathBooleanProgress} from "@semio-tech/framework/2d/booleans/paths";
import type {PathSegment,Vec2} from "@semio-tech/framework/2d";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor,type RetainedWorkRetirementProgress,type RetainedWorkTurn} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork} from "../🎟️work/🟦️.ts";
export type DocumentBooleanRetirement=RetainedWorkRetirementCursor;
export type DocumentBooleanRetirementProgress=RetainedWorkRetirementProgress;
export type DocumentBooleanLimits={tolerance:number;epsilon:number;maxDepth:number;maxReferences:number;maxEdges:number;maxParameters:number;maxAtomicEdges:number;maxSegments:number;maxRetainedSegments:number;maxWork:number};
export type DocumentBooleanInput={plan:ScenePlanOwner;limits:DocumentBooleanLimits};
export type DocumentBooleanProgress={phase:"indexing"|"validation"|"cycles"|"requirements"|"traversal"|"operands"|"geometry"|"remapping"|"publishing"|"complete";nodes:number;resolved:number;references:number;segments:number;work:number;geometry:PathBooleanProgress|null;done:boolean};
type Matrix=[number,number,number,number,number,number];
type Frame={index:number;at:number};
const identity:Matrix=[1,0,0,1,0,0];
function invalid(message:string):never{throw new RangeError(message);}
function matrix(value:readonly number[]):Matrix{if(!Array.isArray(value)||value.length!==6||!value.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))invalid("Invalid Boolean document matrix");return [...value] as Matrix;}
function inverse(m:Matrix):Matrix{const d=m[0]*m[3]-m[1]*m[2];if(!Number.isFinite(d)||d===0)invalid("Boolean reference space is singular");return matrix([m[3]/d,-m[1]/d,-m[2]/d,m[0]/d,(m[2]*m[5]-m[3]*m[4])/d,(m[1]*m[4]-m[0]*m[5])/d]);}
function magnification(n:ScenePlanNode):number{
 if(n.content.kind!=="boolean")return 1;const m=n.transform,r=n.content.referenceTransform,d=r[0]*r[3]-r[1]*r[2];if(d===0)return 1;
 const a=(m[0]*r[3]-m[2]*r[1])/d,b=(m[1]*r[3]-m[3]*r[1])/d,c=(m[2]*r[0]-m[0]*r[2])/d,e=(m[3]*r[0]-m[1]*r[2])/d,scale=Math.max(Math.abs(a),Math.abs(b),Math.abs(c),Math.abs(e));
 if(!Number.isFinite(scale))invalid("Boolean document precision exceeds numeric limits");if(scale===0)return 1;
 const x=a/scale,y=b/scale,z=c/scale,w=e/scale,norm=scale*Math.sqrt((x*x+y*y+z*z+w*w+Math.hypot(x*x+y*y-z*z-w*w,2*(x*z+y*w)))/2);
 if(!Number.isFinite(norm))invalid("Boolean document precision exceeds numeric limits");return Math.max(1,norm);
}
function point(p:Vec2,m:Matrix):Vec2{const q:Vec2=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];if(!q.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))invalid("Boolean document point exceeds coordinate limit");return[q[0]===0?0:q[0],q[1]===0?0:q[1]];}
function copyPoint(p:Vec2):Vec2{if(!Array.isArray(p)||p.length!==2)invalid("Invalid Boolean document point");return[p[0],p[1]];}
function copy(s:PathSegment):PathSegment{if(!s||typeof s!=="object")invalid("Invalid Boolean document segment");switch(s.kind){case"move":case"line":return{kind:s.kind,to:copyPoint(s.to)};case"quad":return{kind:s.kind,ctrl:copyPoint(s.ctrl),to:copyPoint(s.to)};case"cubic":return{kind:s.kind,ctrl1:copyPoint(s.ctrl1),ctrl2:copyPoint(s.ctrl2),to:copyPoint(s.to)};case"arc":return{kind:s.kind,rx:s.rx,ry:s.ry,rotation:s.rotation,largeArc:s.largeArc,sweep:s.sweep,to:copyPoint(s.to)};case"close":return{kind:s.kind};default:return invalid("Invalid Boolean document segment");}}
type Sequence<T>={readonly length:number;at(index:number):T|undefined};
const none:Sequence<string>={length:0,at:()=>undefined};
function row<T>(source:Sequence<T>,at:number):T{const value=source.at(at);if(value===undefined)invalid("Missing Boolean document row");return value;}
const refs=(node:ScenePlanNode):Sequence<string>=>node.content.kind==="group"||node.content.kind==="boolean"?node.content.children:none;
const address=(n:ScenePlanNode)=>validateSceneSourceAddress({sourcePath:Array.from({length:n.sourcePath.length},(_,at)=>row(n.sourcePath,at)),lockedAncestors:n.lockedAncestors});
function drain(cursor:RetainedWorkRetirementCursor,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean{const before=turn.actualWork();let done=false;try{done=cursor.advance(1,work).done;}finally{turn.commitActualWork(cursor.lastTurnWork());}return done&&turn.actualWork()===before;}
/** ⏱️ Indexes references, resolves shared world fills and preserves independently moved result-local paint. */
export class DocumentBooleanJob{
 readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private phase:DocumentBooleanProgress["phase"]|"geometryTake"|"childClose"|"replaceBirth"|"replace"|"replaceTake"|"publishNext"="indexing";private work=0;private nodes=0;private resolved=0;private references=0;private retained=0;private sourceSegments=0;private validateAt=0;private refAt=0;private rootAt=0;private current=-1;private publishAt=0;
 private ids=new Map<string,number>();private visited=new Set<number>();private visiting=new Set<number>();private stack:Frame[]=[];private cache=new Map<number,PathSegment[]>();private local=new Map<number,PathSegment[]>();
 private order:number[]=[];private impact=new Map<number,number>();private quality=new Map<number,number>();
 private operands:PathBooleanOperand[]=[];private operandAt=0;private copyAt=0;private copied=0;private operand:PathBooleanOperand|null=null;private child:PathBooleanJob|null=null;private geometry:PathBooleanProgress|null=null;
 private raw:PathSegment[]=[];private remapAt=0;private reference:Matrix=identity;private localResult:PathSegment[]=[];private worldResult:PathSegment[]=[];private failure:unknown=null;private cancelled=false;private transferred=false;
 private owner:ScenePlanOwner|null;private view:ScenePlanRead|null;private limits:DocumentBooleanLimits;private closing:RetainedWorkRetirementCursor|null=null;private afterClose="";private replacement:ScenePlanContentReplaceJob|null=null;
 /** 🔒️ Limits validate without a plan so owners of other jobs can refuse early. */
 static checkLimits(limits:DocumentBooleanLimits):void{
  if(!Number.isFinite(limits.tolerance)||limits.tolerance<1e-6||limits.tolerance>16)invalid("Invalid Boolean document contract");
  for(const [n,cap]of [[limits.maxDepth,1024],[limits.maxReferences,32768],[limits.maxRetainedSegments,262144]])if(!Number.isSafeInteger(n)||n!<1||n!>cap!)invalid("Invalid Boolean document limits");
  const check=new PathBooleanJob({...limits,operation:"union",operands:[{segments:[],transform:identity,tolerance:limits.tolerance,fillRule:"nonzero"}]});check.cancel();
 }
 constructor(input:DocumentBooleanInput){
  const view=ScenePlanOwner.prototype.borrow.call(input.plan);
  if(view.nodes.length>1024||view.assets.length>1024)invalid("Invalid Boolean document contract");
  DocumentBooleanJob.checkLimits(input.limits);
  this.limits={...input.limits};this.owner=ScenePlanOwner.transfer(input.plan);this.view=view;
 }
 private nodesView():Sequence<ScenePlanNode>{if(this.view===null)invalid("Boolean document source transferred");return this.view.nodes;}
 private at(index:number):ScenePlanNode{return row(this.nodesView(),index);}
 private push(index:number):void{if(this.stack.length>=this.limits.maxDepth)invalid("Boolean document dependency depth exceeded");if(this.visiting.has(index))invalid("Cyclic Boolean document operands");this.visiting.add(index);this.stack.push({index,at:0});}
 private index():void{
  const view=this.nodesView();if(this.nodes===view.length){this.phase="validation";return;}
  const n=view.at(this.nodes);
  if(!n||typeof n.id!=="string"||!n.id.length||n.id.length>4096||new TextEncoder().encode(n.id).length>4096||this.ids.has(n.id)||!n.content||!["path","glyphs","group","boolean","text","trace","image"].includes(n.content.kind))invalid("Invalid or duplicate Boolean document node");
  address(n);matrix(n.transform);const c=n.content;
  if((c.kind==="path"||c.kind==="glyphs")){if(typeof c.segments?.length!=="number"||c.segments.length>65536||!["nonzero","evenodd"].includes(c.fillRule))invalid("Invalid Boolean document path");this.sourceSegments+=c.segments.length;if(this.sourceSegments>65536)invalid("Boolean document source segment cap exceeded");}
  if(c.kind==="group"||c.kind==="boolean"){if(typeof c.children?.length!=="number"||c.children.length>1024)invalid("Invalid Boolean document references");this.references+=c.children.length;if(this.references>this.limits.maxReferences)invalid("Boolean document reference cap exceeded");}
  if(c.kind==="boolean"){if(!["union","difference","intersection","xor"].includes(c.operation))invalid("Invalid Boolean document operation");matrix(c.referenceTransform);}
  this.ids.set(n.id,this.nodes++);
 }
 private traversal(cycles:boolean):void{
  const view=this.nodesView();
  if(!this.stack.length){const n=view.at(this.rootAt);if(!n){if(cycles){this.visited.clear();this.rootAt=0;this.phase="requirements";}else this.phase="publishing";return;}const index=this.rootAt++;if(cycles?this.visited.has(index):n.content.kind!=="boolean"||this.cache.has(index))return;this.push(index);return;}
  const top=this.stack.at(-1)!,n=this.at(top.index),children=refs(n);
  if(top.at<children.length){const index=this.ids.get(row(children,top.at++))!;if(cycles?this.visited.has(index):this.cache.has(index))return;this.push(index);return;}
  if(cycles){this.stack.pop();this.visiting.delete(top.index);this.visited.add(top.index);this.order.push(top.index);return;}
  if(n.content.kind!=="path"&&n.content.kind!=="glyphs"&&n.content.kind!=="group"&&n.content.kind!=="boolean")invalid("Unsupported Boolean operand "+n.content.kind+": "+n.id);
  this.current=top.index;this.operandAt=0;this.copyAt=0;this.copied=0;this.operand=null;this.operands=[];this.phase="operands";
 }
 private requirements():void{
  if(this.current<0){const index=this.order.pop();if(index===undefined){this.impact.clear();this.phase="traversal";return;}this.current=index;this.refAt=0;const factor=(this.impact.get(index)??1)*magnification(this.at(index));if(!Number.isFinite(factor))invalid("Boolean document precision exceeds numeric limits");this.quality.set(index,factor);return;}
  const children=refs(this.at(this.current));if(this.refAt===children.length){this.current=-1;return;}const child=this.ids.get(row(children,this.refAt++))!,factor=this.quality.get(this.current)!;this.impact.set(child,Math.max(this.impact.get(child)??1,factor));
 }
 private prepare():void{
  const n=this.at(this.current),c=n.content,children=refs(n),count=(c.kind==="path"||c.kind==="glyphs")?1:children.length,factor=this.quality.get(this.current)??1,tolerance=this.limits.tolerance/factor,epsilon=this.limits.epsilon/factor;
  if(tolerance<1e-6||epsilon<1e-12)invalid("Boolean document precision exceeds supported tolerance");
  if(!this.operand){if(this.operandAt===count){if(!this.operands.length)this.operands.push({segments:[],transform:identity,tolerance,fillRule:"nonzero"});this.child=new PathBooleanJob({...this.limits,epsilon,operation:c.kind==="boolean"?c.operation:"union",operands:this.operands});this.operands=[];this.phase="geometry";return;}
   this.operand={segments:[],transform:(c.kind==="path"||c.kind==="glyphs")?[...n.transform]:identity,tolerance,fillRule:(c.kind==="path"||c.kind==="glyphs")?c.fillRule:"nonzero"};this.copyAt=0;return;}
  const source:Sequence<PathSegment>=(c.kind==="path"||c.kind==="glyphs")?c.segments:this.cache.get(this.ids.get(row(children,this.operandAt))!)!,segments=this.operand.segments as PathSegment[];
  if(this.copyAt<source.length){if(this.copied>=this.limits.maxEdges)invalid("Boolean document operand segment budget exceeded");segments.push(copy(row(source,this.copyAt++)));this.copied++;return;}
  this.operands.push(this.operand);this.operand=null;this.operandAt++;
 }
 private completeGeometry():void{
  const node=this.at(this.current),content=node.content;
  if(content.kind!=="boolean"){if(this.retained+this.raw.length>this.limits.maxRetainedSegments)invalid("Boolean document retained geometry cap exceeded");this.retained+=this.raw.length;this.cache.set(this.current,this.raw);this.raw=[];this.finishNode();return;}
  this.reference=this.raw.length?inverse(matrix(content.referenceTransform)):identity;this.remapAt=0;this.localResult=[];this.worldResult=[];this.phase="remapping";
 }
 private finishNode():void{this.resolved++;const top=this.stack.pop()!;this.visiting.delete(top.index);this.current=-1;this.phase="traversal";}
 private remap():void{
  const s=this.raw[this.remapAt];if(this.remapAt===this.raw.length){this.cache.set(this.current,this.worldResult);this.local.set(this.current,this.localResult);this.raw=[];this.worldResult=[];this.localResult=[];this.finishNode();return;}
  if(this.retained+2>this.limits.maxRetainedSegments)invalid("Boolean document retained geometry cap exceeded");
  if(s!.kind==="move"||s!.kind==="line"){const local=point(s!.to,this.reference),world=point(local,matrix(this.at(this.current).transform));this.localResult.push({kind:s!.kind,to:local});this.worldResult.push({kind:s!.kind,to:world});}else if(s!.kind==="close"){this.localResult.push({kind:"close"});this.worldResult.push({kind:"close"});}else invalid("Unresolved Boolean document curve");
  this.retained+=2;this.remapAt++;
 }
 private unit():void{
  switch(this.phase){
   case"indexing":this.index();return;
   case"validation":{const n=this.nodesView().at(this.validateAt);if(!n){this.phase="cycles";return;}const children=refs(n);if(this.refAt<children.length){const id=children.at(this.refAt++);if(typeof id!=="string"||!this.ids.has(id))invalid("Missing Boolean document operand");return;}this.validateAt++;this.refAt=0;return;}
   case"cycles":case"traversal":this.traversal(this.phase==="cycles");return;
   case"requirements":this.requirements();return;
   case"operands":this.prepare();return;
   case"remapping":this.remap();return;
   case"geometryTake":this.child=null;this.completeGeometry();return;
   default:throw Error("Unknown Boolean document phase");
  }
 }
 private stage(cursor:RetainedWorkRetirementCursor,after:string):void{if(this.closing!==null)throw Error("Boolean document resident child close already occupied");this.closing=cursor;this.afterClose=after;this.phase="childClose";}
 private step(c:RetainedWorkProducerStep):void{
  switch(this.phase){
   case"childClose":{let done=false;try{done=this.closing!.advance(1,c.work).done;}finally{c.retainedTurn.commitActualWork(this.closing!.lastTurnWork());}if(!done||c.hasActualWork())return;this.closing=null;this.phase=this.afterClose as typeof this.phase;this.afterClose="";c.commitControlWork();return;}
   case"geometry":{this.geometry=c.advanceChild(this.child!);if(this.geometry.done){const moved=this.child!.intoRetirement();this.raw=moved.output!;this.stage(moved.job,"geometryTake");c.commitControlWork();}return;}
   case"publishing":{
    const view=this.nodesView();if(this.publishAt===view.length){this.phase="complete";c.commitControlWork();return;}
    const node=this.at(this.publishAt);if(node.content.kind!=="boolean"){this.publishAt++;c.commitSemanticWork();return;}
    c.requireSemanticWork();const old=node.content,content:ScenePlanContent={kind:"path",segments:this.local.get(this.publishAt)!,fillRule:old.fillRule,fill:old.fill,stroke:old.stroke};
    this.replacement=new ScenePlanContentReplaceJob(this.owner!,this.publishAt,content,{maxWork:this.limits.maxWork});this.owner=null;this.phase="replace";c.commitSemanticWork();return;
   }
   case"replace":if(c.advanceChild(this.replacement!).done)this.phase="replaceTake";return;
   case"replaceTake":{const moved=this.replacement!.intoRetirement();if(moved.output===null)throw Error("Completed Boolean Scene owner absent");this.owner=moved.output;this.replacement=null;this.stage(moved.job,"publishNext");c.commitControlWork();return;}
   case"publishNext":this.local.delete(this.publishAt);this.publishAt++;this.phase="publishing";c.commitControlWork();return;
   case"complete":return;
  }
  this.unit();c.commitSemanticWork();
 }
 lastTurnWork():number{return this.#runtime.lastTurnWork();}
 lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand():RetainedWorkDemand{return this.#demand;}
 failurePending():boolean{return this.failure!==null||this.child?.failurePending()===true||this.replacement?.failurePending()===true;}
 settlementPending():boolean{
  if(this.transferred)return false;
  switch(this.phase){
   case"complete":case"childClose":case"geometryTake":case"replaceTake":case"publishNext":return true;
   case"geometry":return this.child?.settlementPending()===true;
   case"replace":return this.replacement?.settlementPending()===true;
   case"publishing":return this.view!==null&&this.publishAt===this.view.nodes.length;
   default:return false;
  }
 }
 private publicPhase():DocumentBooleanProgress["phase"]{switch(this.phase){case"geometryTake":return"geometry";case"childClose":return this.afterClose==="geometryTake"?"geometry":"publishing";case"replaceBirth":case"replace":case"replaceTake":case"publishNext":return"publishing";default:return this.phase;}}
 private progress():DocumentBooleanProgress{return{phase:this.publicPhase(),nodes:this.nodes,resolved:this.resolved,references:this.references,segments:this.retained,work:this.work,geometry:this.geometry,done:this.phase==="complete"};}
 advance(items:number,work:RetainedWorkLedger):DocumentBooleanProgress{
  try{return this.#runtime.advance(items,work,{maximumWork:this.limits.maxWork,workLimitMessage:"Boolean document work cap exceeded",validate:()=>{if(this.cancelled||this.transferred)throw new DOMException("Document Boolean cancelled","AbortError");if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.progress()});}
  catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.cancelled)this.failure=error;throw error;}
 }
 result():ScenePlanRead{if(this.cancelled||this.transferred)throw new DOMException("Document Boolean cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")invalid("Document Boolean incomplete");return this.owner!.borrow();}
 retireOutput(output:ScenePlanOwner):RetainedWorkRetirementCursor{return new ScenePlanCloseJob(output);}
 /** 🧹️ Return the valid output owner while composing every private child and scene owner. */
 intoRetirement():{job:DocumentBooleanRetirement;output:ScenePlanOwner|null}{
  if(this.transferred)throw Error("Document Boolean ownership already transferred");this.transferred=true;
  const output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.owner:null;this.cancelled=true;
  let prior=this.closing,childClose:RetainedWorkRetirementCursor|null=null,replacementClose:RetainedWorkRetirementCursor|null=null;this.closing=null;
  if(this.child){const moved=this.child.intoRetirement();childClose=moved.job;this.child=null;}
  if(this.replacement){const moved=this.replacement.intoRetirement();replacementClose=moved.job;if(moved.output!==null)this.owner=moved.output;this.replacement=null;}
  let scene=this.owner!==null&&this.owner!==output?new ScenePlanCloseJob(this.owner):null;this.owner=null;let cleared=false;
  const job=new RetainedWorkRetirement((work,turn)=>{
   if(prior){if(!drain(prior,work,turn))return false;prior=null;turn.commitActualWork();return false;}
   if(childClose){if(!drain(childClose,work,turn))return false;childClose=null;turn.commitActualWork();return false;}
   if(replacementClose){if(!drain(replacementClose,work,turn))return false;replacementClose=null;turn.commitActualWork();return false;}
   if(scene){if(!drain(scene,work,turn))return false;scene=null;turn.commitActualWork();return false;}
   if(!cleared){if(!work.canStep())return false;this.clear();cleared=true;turn.commitActualWork();return false;}
   return true;
  });
  return{job,output};
 }
 private clear():void{this.view=null;this.ids.clear();this.visited.clear();this.visiting.clear();this.stack=[];this.order=[];this.impact.clear();this.quality.clear();this.cache.clear();this.local.clear();this.operands=[];this.operand=null;this.raw=[];this.localResult=[];this.worldResult=[];this.afterClose="";}
 cancel():void{this.cancelled=true;}
}
/** ⏳️ Caller-funded turns publish the resolved plan only after its owned close. */
export async function resolveDocumentBooleans(input:DocumentBooleanInput,control:{work:DrawingSceneWorkController;operation:object;source:object;onProgress?:(p:DocumentBooleanProgress)=>void}):Promise<ScenePlanOwner>{
 return driveDrawingSceneWork(control.work,control.operation,control.source,new DocumentBooleanJob(input),control.onProgress);
}
