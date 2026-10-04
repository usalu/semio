import {validateSceneSourceAddress} from "../📋️prepare/🟦️.ts";
/** 🔀️ Complete prepared documents resolve referenced filled geometry under grants. */
import type {DocumentScenePlan,DocumentSceneNode} from "../📋️prepare/🟦️.ts";
import {PathBooleanJob,type PathBooleanOperand,type PathBooleanProgress} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🔀️booleans/🛤️paths/🟦️.ts";
import type {PathSegment,Vec2} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";
import {UnitRetirement,type WorkRetirement,type WorkRetirementProgress} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🧹️retire/🟦️.ts";
import {ScenePlanCloseJob} from "../🧹️retire/🟦️.ts";
import {SceneNodeCopyJob} from "../📋️prepare/📋️copy/🟦️.ts";
export type DocumentBooleanRetirement=WorkRetirement;
export type DocumentBooleanRetirementProgress=WorkRetirementProgress;
export type DocumentBooleanLimits={tolerance:number;epsilon:number;maxDepth:number;maxReferences:number;maxEdges:number;maxParameters:number;maxAtomicEdges:number;maxSegments:number;maxRetainedSegments:number;maxWork:number};
export type DocumentBooleanInput={plan:DocumentScenePlan;limits:DocumentBooleanLimits};
export type DocumentBooleanProgress={phase:"indexing"|"validation"|"cycles"|"requirements"|"traversal"|"operands"|"geometry"|"remapping"|"publishing"|"complete";nodes:number;resolved:number;references:number;segments:number;work:number;geometry:PathBooleanProgress|null;done:boolean};
export type DocumentBooleanOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(p:DocumentBooleanProgress)=>void};
type Matrix=[number,number,number,number,number,number];
type Frame={index:number;at:number};
const identity:Matrix=[1,0,0,1,0,0];
function invalid(message:string):never{throw new RangeError(message);}
function matrix(value:readonly number[]):Matrix{if(!Array.isArray(value)||value.length!==6||!value.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))invalid("Invalid Boolean document matrix");return [...value] as Matrix;}
function inverse(m:Matrix):Matrix{const d=m[0]*m[3]-m[1]*m[2];if(!Number.isFinite(d)||d===0)invalid("Boolean reference space is singular");return matrix([m[3]/d,-m[1]/d,-m[2]/d,m[0]/d,(m[2]*m[5]-m[3]*m[4])/d,(m[1]*m[4]-m[0]*m[5])/d]);}
function magnification(n:DocumentSceneNode):number{
 if(n.content.kind!=="boolean")return 1;const m=n.transform,r=n.content.referenceTransform,d=r[0]*r[3]-r[1]*r[2];if(d===0)return 1;
 const a=(m[0]*r[3]-m[2]*r[1])/d,b=(m[1]*r[3]-m[3]*r[1])/d,c=(m[2]*r[0]-m[0]*r[2])/d,e=(m[3]*r[0]-m[1]*r[2])/d,scale=Math.max(Math.abs(a),Math.abs(b),Math.abs(c),Math.abs(e));
 if(!Number.isFinite(scale))invalid("Boolean document precision exceeds numeric limits");if(scale===0)return 1;
 const x=a/scale,y=b/scale,z=c/scale,w=e/scale,norm=scale*Math.sqrt((x*x+y*y+z*z+w*w+Math.hypot(x*x+y*y-z*z-w*w,2*(x*z+y*w)))/2);
 if(!Number.isFinite(norm))invalid("Boolean document precision exceeds numeric limits");return Math.max(1,norm);
}
function point(p:Vec2,m:Matrix):Vec2{const q:Vec2=[m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];if(!q.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))invalid("Boolean document point exceeds coordinate limit");return[q[0]===0?0:q[0],q[1]===0?0:q[1]];}
function copyPoint(p:Vec2):Vec2{if(!Array.isArray(p)||p.length!==2)invalid("Invalid Boolean document point");return[p[0],p[1]];}
function copy(s:PathSegment):PathSegment{if(!s||typeof s!=="object")invalid("Invalid Boolean document segment");switch(s.kind){case"move":case"line":return{kind:s.kind,to:copyPoint(s.to)};case"quad":return{kind:s.kind,ctrl:copyPoint(s.ctrl),to:copyPoint(s.to)};case"cubic":return{kind:s.kind,ctrl1:copyPoint(s.ctrl1),ctrl2:copyPoint(s.ctrl2),to:copyPoint(s.to)};case"arc":return{kind:s.kind,rx:s.rx,ry:s.ry,rotation:s.rotation,largeArc:s.largeArc,sweep:s.sweep,to:copyPoint(s.to)};case"close":return{kind:s.kind};default:return invalid("Invalid Boolean document segment");}}
const refs=(node:DocumentSceneNode):readonly string[]=>node.content.kind==="group"||node.content.kind==="boolean"?node.content.children:[];
/** ⏱️ Indexes references, resolves shared world fills and preserves independently moved result-local paint. */
export class DocumentBooleanJob{
 private phase:DocumentBooleanProgress["phase"]="indexing";private work=0;private nodes=0;private resolved=0;private references=0;private retained=0;private sourceSegments=0;private validateAt=0;private refAt=0;private rootAt=0;private current=-1;private publishAt=0;
 private ids=new Map<string,number>();private visited=new Set<number>();private visiting=new Set<number>();private stack:Frame[]=[];private cache=new Map<number,PathSegment[]>();private local=new Map<number,PathSegment[]>();
 private order:number[]=[];private impact=new Map<number,number>();private quality=new Map<number,number>();
 private operands:PathBooleanOperand[]=[];private operandAt=0;private copyAt=0;private copied=0;private operand:PathBooleanOperand|null=null;private child:PathBooleanJob|null=null;private geometry:PathBooleanProgress|null=null;
 private raw:PathSegment[]=[];private remapAt=0;private reference:Matrix=identity;private localResult:PathSegment[]=[];private worldResult:PathSegment[]=[];private output:DocumentScenePlan={assets:[],nodes:[]};private failure:unknown=null;private cancelled=false;private transferred=false;
 private copyNode:SceneNodeCopyJob|null=null;private publishAssetAt=0;private cleanupSlot=0;private cleanupEntries:Iterator<unknown>|null=null;
 constructor(private input:DocumentBooleanInput){
  const l=input.limits;if(!input.plan||!Array.isArray(input.plan.nodes)||input.plan.nodes.length>1024||!Array.isArray(input.plan.assets)||input.plan.assets.length>1024||!Number.isFinite(l.tolerance)||l.tolerance<1e-6||l.tolerance>16)invalid("Invalid Boolean document contract");
  for(const [n,cap]of [[l.maxDepth,1024],[l.maxReferences,32768],[l.maxRetainedSegments,262144]])if(!Number.isSafeInteger(n)||n!<1||n!>cap!)invalid("Invalid Boolean document limits");
  const check=new PathBooleanJob({...l,operation:"union",operands:[{segments:[],transform:identity,tolerance:l.tolerance,fillRule:"nonzero"}]});check.cancel();this.input={plan:input.plan,limits:{...l}};
 }
 private push(index:number):void{if(this.stack.length>=this.input.limits.maxDepth)invalid("Boolean document dependency depth exceeded");if(this.visiting.has(index))invalid("Cyclic Boolean document operands");this.visiting.add(index);this.stack.push({index,at:0});}
 private index():void{
  const n=this.input.plan.nodes[this.nodes];if(this.nodes===this.input.plan.nodes.length){this.phase="validation";return;}
  if(!n||typeof n.id!=="string"||!n.id.length||n.id.length>4096||new TextEncoder().encode(n.id).length>4096||this.ids.has(n.id)||!n.content||!["path","group","boolean","text","trace","image"].includes(n.content.kind))invalid("Invalid or duplicate Boolean document node");
  validateSceneSourceAddress(n);matrix(n.transform);const c=n.content;
  if(c.kind==="path"){if(!Array.isArray(c.segments)||c.segments.length>65536||!["nonzero","evenodd"].includes(c.fillRule))invalid("Invalid Boolean document path");this.sourceSegments+=c.segments.length;if(this.sourceSegments>65536)invalid("Boolean document source segment cap exceeded");}
  if(c.kind==="group"||c.kind==="boolean"){if(!Array.isArray(c.children)||c.children.length>1024)invalid("Invalid Boolean document references");this.references+=c.children.length;if(this.references>this.input.limits.maxReferences)invalid("Boolean document reference cap exceeded");}
  if(c.kind==="boolean"){if(!["union","difference","intersection","xor"].includes(c.operation))invalid("Invalid Boolean document operation");matrix(c.referenceTransform);}
  this.ids.set(n.id,this.nodes++);
 }
 private traversal(cycles:boolean):void{
  if(!this.stack.length){const n=this.input.plan.nodes[this.rootAt];if(!n){if(cycles){if(this.removeEntry(this.visited))return;this.visited.clear();this.rootAt=0;this.phase="requirements";}else this.phase="publishing";return;}const index=this.rootAt++;if(cycles?this.visited.has(index):n.content.kind!=="boolean"||this.cache.has(index))return;this.push(index);return;}
  const top=this.stack.at(-1)!,n=this.input.plan.nodes[top.index]!,children=refs(n);
  if(top.at<children.length){const index=this.ids.get(children[top.at++]!)!;if(cycles?this.visited.has(index):this.cache.has(index))return;this.push(index);return;}
  if(cycles){this.stack.pop();this.visiting.delete(top.index);this.visited.add(top.index);this.order.push(top.index);return;}
  if(n.content.kind!=="path"&&n.content.kind!=="group"&&n.content.kind!=="boolean")invalid("Unsupported Boolean operand "+n.content.kind+": "+n.id);
  this.current=top.index;this.operandAt=0;this.copyAt=0;this.copied=0;this.operand=null;this.operands=[];this.phase="operands";
 }
 private requirements():void{
  if(this.current<0){const index=this.order.pop();if(index===undefined){if(this.removeEntry(this.impact))return;this.impact.clear();this.phase="traversal";return;}this.current=index;this.refAt=0;const factor=(this.impact.get(index)??1)*magnification(this.input.plan.nodes[index]!);if(!Number.isFinite(factor))invalid("Boolean document precision exceeds numeric limits");this.quality.set(index,factor);return;}
  const children=refs(this.input.plan.nodes[this.current]!);if(this.refAt===children.length){this.current=-1;return;}const child=this.ids.get(children[this.refAt++]!)!,factor=this.quality.get(this.current)!;this.impact.set(child,Math.max(this.impact.get(child)??1,factor));
 }
 private prepare():void{
  const n=this.input.plan.nodes[this.current]!,c=n.content,children=refs(n),count=c.kind==="path"?1:children.length,factor=this.quality.get(this.current)??1,tolerance=this.input.limits.tolerance/factor,epsilon=this.input.limits.epsilon/factor;
  if(tolerance<1e-6||epsilon<1e-12)invalid("Boolean document precision exceeds supported tolerance");
  if(!this.operand){if(this.operandAt===count){if(!this.operands.length)this.operands.push({segments:[],transform:identity,tolerance,fillRule:"nonzero"});this.child=new PathBooleanJob({...this.input.limits,epsilon,operation:c.kind==="boolean"?c.operation:"union",operands:this.operands});this.operands=[];this.phase="geometry";return;}
   this.operand={segments:[],transform:c.kind==="path"?[...n.transform]:identity,tolerance,fillRule:c.kind==="path"?c.fillRule:"nonzero"};this.copyAt=0;return;}
  const source=c.kind==="path"?c.segments:this.cache.get(this.ids.get(children[this.operandAt]!)!)!,segments=this.operand.segments as PathSegment[];
  if(this.copyAt<source.length){if(this.copied>=this.input.limits.maxEdges)invalid("Boolean document operand segment budget exceeded");segments.push(copy(source[this.copyAt++]!));this.copied++;return;}
  this.operands.push(this.operand);this.operand=null;this.operandAt++;
 }
 private completeGeometry():void{
  const node=this.input.plan.nodes[this.current]!,content=node.content;
  if(content.kind!=="boolean"){if(this.retained+this.raw.length>this.input.limits.maxRetainedSegments)invalid("Boolean document retained geometry cap exceeded");this.retained+=this.raw.length;this.cache.set(this.current,this.raw);this.raw=[];this.finishNode();return;}
  this.reference=this.raw.length?inverse(matrix(content.referenceTransform)):identity;this.remapAt=0;this.localResult=[];this.worldResult=[];this.phase="remapping";
 }
 private finishNode():void{this.resolved++;const top=this.stack.pop()!;this.visiting.delete(top.index);this.current=-1;this.phase="traversal";}
 private step():void{
  if(this.phase==="indexing"){this.index();return;}
  if(this.phase==="validation"){const n=this.input.plan.nodes[this.validateAt];if(!n){this.phase="cycles";return;}const children=refs(n);if(this.refAt<children.length){const id=children[this.refAt++];if(typeof id!=="string"||!this.ids.has(id))invalid("Missing Boolean document operand");return;}this.validateAt++;this.refAt=0;return;}
  if(this.phase==="cycles"||this.phase==="traversal"){this.traversal(this.phase==="cycles");return;}
  if(this.phase==="requirements"){this.requirements();return;}
  if(this.phase==="operands"){this.prepare();return;}
  if(this.phase==="geometry"){this.geometry=this.child!.advance(1);if(this.geometry.done){this.raw=this.child!.result();this.child=null;this.completeGeometry();}return;}
  if(this.phase==="remapping"){
   const s=this.raw[this.remapAt];if(this.remapAt===this.raw.length){this.cache.set(this.current,this.worldResult);this.local.set(this.current,this.localResult);this.raw=[];this.worldResult=[];this.localResult=[];this.finishNode();return;}
   if(this.retained+2>this.input.limits.maxRetainedSegments)invalid("Boolean document retained geometry cap exceeded");
   if(s!.kind==="move"||s!.kind==="line"){const local=point(s!.to,this.reference),world=point(local,matrix(this.input.plan.nodes[this.current]!.transform));this.localResult.push({kind:s!.kind,to:local});this.worldResult.push({kind:s!.kind,to:world});}else if(s!.kind==="close"){this.localResult.push({kind:"close"});this.worldResult.push({kind:"close"});}else invalid("Unresolved Boolean document curve");
   this.retained+=2;this.remapAt++;return;
  }
  if(this.phase==="publishing")this.publish();
 }
 private removeEntry(collection:Map<unknown,unknown>|Set<unknown>):boolean{this.cleanupEntries??=collection.keys();const next=this.cleanupEntries.next();if(next.done){this.cleanupEntries=null;return false;}collection.delete(next.value);return true;}
 private cleanup():boolean{
  const collection=this.cleanupSlot===0?this.cache:this.cleanupSlot===1?this.local:this.cleanupSlot===2?this.ids:this.cleanupSlot===3?this.quality:this.cleanupSlot===4?this.impact:this.cleanupSlot===5?this.visited:this.cleanupSlot===6?this.visiting:null;
  if(collection){if(this.removeEntry(collection))return false;collection.clear();}
  else if(this.cleanupSlot===7)this.stack=[];else if(this.cleanupSlot===8)this.order=[];
  return ++this.cleanupSlot===9;
 }
 private publish():void{
  const source=this.input.plan.nodes[this.publishAt];
  if(!source){const a=this.input.plan.assets[this.publishAssetAt++];if(a){this.output.assets.push({id:a.id,mime:a.mime,data:a.data});return;}if(this.cleanup())this.phase="complete";return;}
  if(!this.copyNode){this.copyNode=new SceneNodeCopyJob(source,source.content.kind==="boolean"?this.local.get(this.publishAt)!:null);if(source.content.kind==="boolean")this.local.delete(this.publishAt);return;}
  if(this.copyNode.advanceOne()){this.output.nodes.push(this.copyNode.take());this.copyNode=null;this.publishAt++;}
 }

 advance(budget:number):DocumentBooleanProgress{
  if(!Number.isSafeInteger(budget)||budget<1)invalid("Invalid Boolean document work grant");if(this.cancelled)throw new DOMException("Document Boolean cancelled","AbortError");if(this.failure)throw this.failure;
  try{for(let at=0;at<budget&&this.phase!=="complete";at++){if(this.work>=this.input.limits.maxWork)invalid("Boolean document work cap exceeded");this.step();this.work++;}}catch(error){this.failure=error;throw error;}
  return{phase:this.phase,nodes:this.nodes,resolved:this.resolved,references:this.references,segments:this.retained,work:this.work,geometry:this.geometry,done:this.phase==="complete"};
 }
 result():DocumentScenePlan{if(this.cancelled)throw new DOMException("Document Boolean cancelled","AbortError");if(this.failure)throw this.failure;if(this.phase!=="complete")invalid("Document Boolean incomplete");return this.output;}
 /** 🧹️ Return genuine source and valid output while composing every private child and scene owner. */
 intoRetirement():{job:DocumentBooleanRetirement;input:DocumentScenePlan;output:DocumentScenePlan|null}{
  if(this.transferred)throw Error("Document Boolean ownership already transferred");this.transferred=true;
  let child:WorkRetirement|null=null;if(this.child){const moved=this.child.intoRetirement();child=moved.job;if(moved.output)this.raw=moved.output;this.child=null;}
  const input=this.input.plan,output=this.phase==="complete"&&!this.cancelled&&!this.failure?this.output:null;
  this.input={plan:{assets:[],nodes:[]},limits:this.input.limits};let draft:ScenePlanCloseJob|null=output?null:new ScenePlanCloseJob(this.output);this.output={assets:[],nodes:[]};
  const partial=this.copyNode?.takePartial();let node:ScenePlanCloseJob|null=partial?new ScenePlanCloseJob({assets:[],nodes:[partial]}):null;this.copyNode=null;this.cleanupEntries=null;this.cancelled=true;
  let slot=0;const job=new UnitRetirement(()=>{
   switch(slot){
    case 0:if(child){if(!child.advance(1).done)return false;child=null;}break;
    case 1:if(draft){if(!draft.advance(1).done)return false;draft=null;}break;
    case 2:if(node){if(!node.advance(1).done)return false;node=null;}break;
    case 3:if(this.removeEntry(this.ids))return false;this.ids.clear();break;
    case 4:if(this.removeEntry(this.visited))return false;this.visited.clear();break;
    case 5:if(this.removeEntry(this.visiting))return false;this.visiting.clear();break;
    case 6:if(this.removeEntry(this.cache))return false;this.cache.clear();break;
    case 7:if(this.removeEntry(this.local))return false;this.local.clear();break;
    case 8:if(this.removeEntry(this.impact))return false;this.impact.clear();break;
    case 9:if(this.removeEntry(this.quality))return false;this.quality.clear();break;
    case 10:this.stack=[];break;
    case 11:this.order=[];break;
    case 12:if(this.operands.pop())return false;this.operands=[];break;
    case 13:this.operand=null;break;
    case 14:this.raw=[];break;
    case 15:this.localResult=[];break;
    case 16:this.worldResult=[];break;
    case 17:break;
    case 18:break;
    case 19:this.input.plan={assets:[],nodes:[]};break;
    case 20:this.output={assets:[],nodes:[]};break;
   }
   return ++slot===21;
  });return{job,input,output};
 }
 private clear():void{this.child?.cancel();this.child=null;this.copyNode=null;this.cleanupEntries=null;this.input={plan:{assets:[],nodes:[]},limits:this.input.limits};this.ids.clear();this.visited.clear();this.visiting.clear();this.stack=[];this.order=[];this.impact.clear();this.quality.clear();this.cache.clear();this.local.clear();this.operands=[];this.operand=null;this.raw=[];this.localResult=[];this.worldResult=[];this.output={assets:[],nodes:[]};}
 cancel():void{this.cancelled=true;if(!this.transferred)this.clear();}
}
/** ⏳️ Yields reference/curve/region work and checks observers before private plan publication. */
export async function resolveDocumentBooleans(input:DocumentBooleanInput,options:DocumentBooleanOptions={}):Promise<DocumentScenePlan>{
 const check=()=>{if(options.signal?.aborted)throw new DOMException("Document Boolean cancelled","AbortError");};check();const job=new DocumentBooleanJob(input);
 try{for(;;){check();const p=job.advance(options.workBudget??4096);options.onProgress?.(p);check();if(p.done)return job.result();await new Promise<void>(resolve=>setTimeout(resolve,0));}}catch(error){job.cancel();throw error;}
}
