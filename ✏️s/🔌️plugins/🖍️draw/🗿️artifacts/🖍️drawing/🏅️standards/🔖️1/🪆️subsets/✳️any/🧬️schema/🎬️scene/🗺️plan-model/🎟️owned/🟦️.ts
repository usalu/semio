/** 🗝️ Only genuinely funded scene cells can issue a consuming plan owner. */
import {RetainedWorkBuffer,RetainedWorkBytes,RetainedWorkLedger,RetainedWorkRetirement,type RetainedWorkInputSequence,type RetainedWorkRetirementCursor,type RetainedWorkTurn,type RetainedWorkDemand} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import type {DocumentSceneNode,DocumentSceneContent} from "../🟦️.ts";
import type {RasterSceneAsset,RasterSceneGroup} from "../../📷️raster/🟦️.ts";
import type {PathSegment} from "@semio-tech/framework/2d";
type Sequence<T>=RetainedWorkInputSequence<T>;
type Matrix=readonly[number,number,number,number,number,number];
type Color=readonly[number,number,number,number];
type Stop={readonly offset:number;readonly color:Color};
type Fill={readonly kind:"solid";readonly color:Color}|{readonly kind:"linearGradient";readonly x1:number;readonly y1:number;readonly x2:number;readonly y2:number;readonly stops:Sequence<Stop>}|{readonly kind:"radialGradient";readonly cx:number;readonly cy:number;readonly r:number;readonly stops:Sequence<Stop>};
type OriginalPaint=Extract<DocumentSceneContent,{kind:"path"}>;
type Stroke=Omit<NonNullable<OriginalPaint["stroke"]>,"color"|"dash">&{readonly color:Color;readonly dash?:Sequence<number>|null};
type Paint={readonly fillRule:"nonzero"|"evenodd";readonly fill:Fill|null;readonly stroke:Stroke|null};
type TextFields=Pick<Extract<DocumentSceneContent,{kind:"text"}>,"content"|"x"|"y"|"size"|"fontFamily">;
export type ScenePlanContent=({readonly kind:"path";readonly segments:Sequence<PathSegment>}&Paint)|({readonly kind:"glyphs";readonly segments:Sequence<PathSegment>}&TextFields&Paint)|({readonly kind:"text"}&TextFields&Paint)|Extract<DocumentSceneContent,{kind:"image"}>|{readonly kind:"group";readonly children:Sequence<string>;readonly isolation:boolean}|({readonly kind:"boolean";readonly operation:"union"|"difference"|"intersection"|"xor";readonly children:Sequence<string>;readonly referenceTransform:Matrix}&Paint)|({readonly kind:"trace";readonly source:string;readonly threshold:number;readonly simplifyEpsilon:number}&Paint);
export type ScenePlanNode=Omit<DocumentSceneNode,"sourcePath"|"groups"|"content"|"transform">&{readonly sourcePath:Sequence<number>;readonly transform:Matrix;readonly groups:Sequence<RasterSceneGroup>;readonly content:ScenePlanContent};
export interface ScenePlanRead{readonly assets:Sequence<RasterSceneAsset>;readonly nodes:Sequence<ScenePlanNode>}
type Resource=({readonly buffer:RetainedWorkBuffer<unknown>}|{readonly bytes:RetainedWorkBytes})&{rowOwners:number;finalOwners:number};
const emptyDemand=():RetainedWorkDemand=>({copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1});
function row<T>(source:Sequence<T>,at:number):T{const value=source.at(at);if(value===undefined)throw Error("Missing scene source row");return value;}
function matrix(source:Matrix):Matrix{return Object.freeze([source[0],source[1],source[2],source[3],source[4],source[5]]);}
function color(source:Color):Color{return Object.freeze([source[0],source[1],source[2],source[3]]);}
function point(source:readonly[number,number]):[number,number]{const value:[number,number]=[source[0],source[1]];Object.freeze(value);return value;}
function segment(value:PathSegment):PathSegment{
 switch(value.kind){case"move":case"line":return Object.freeze({kind:value.kind,to:point(value.to)});case"quad":return Object.freeze({kind:value.kind,ctrl:point(value.ctrl),to:point(value.to)});case"cubic":return Object.freeze({kind:value.kind,ctrl1:point(value.ctrl1),ctrl2:point(value.ctrl2),to:point(value.to)});case"arc":return Object.freeze({kind:value.kind,rx:value.rx,ry:value.ry,rotation:value.rotation,largeArc:value.largeArc,sweep:value.sweep,to:point(value.to)});case"close":return Object.freeze({kind:value.kind});}
}
class Borrow<T> implements Sequence<T>{
 #body:PlanStorage;#rows:RetainedWorkBuffer<T>;
 constructor(body:PlanStorage,rows:RetainedWorkBuffer<T>){this.#body=body;this.#rows=rows;Object.freeze(this);}
 get length(){this.#body.check();return this.#rows.length;}
 at(index:number):T|undefined{this.#body.check();return this.#rows.at(index);}
 static matches(body:PlanStorage,value:unknown,buffer:RetainedWorkBuffer<unknown>):boolean{return value!==null&&typeof value==="object"&&#body in value&&value.#body===body&&value.#rows===buffer;}
}
class PlanStorage{
 readonly nodes=new RetainedWorkBuffer<ScenePlanNode>(112);readonly assets=new RetainedWorkBuffer<RasterSceneAsset>(32);readonly resources=new RetainedWorkBuffer<Resource>(24);readonly rewrite=new RetainedWorkBuffer<ScenePlanNode>(112);
 readonly view:ScenePlanRead;#closing=false;#closed=false;
 static authentic(value:unknown):value is PlanStorage{return value!==null&&typeof value==="object"&&#closing in value;}
 constructor(){this.view=Object.freeze({assets:new Borrow(this,this.assets),nodes:new Borrow(this,this.nodes)});}
 check():void{if(this.#closing||this.#closed)throw Error("Scene plan borrow expired");}
 borrow<T>(values:RetainedWorkBuffer<T>):Sequence<T>{return new Borrow(this,values);}
 close():RetainedWorkRetirementCursor&{owners():number}{
  if(this.#closing||this.#closed)throw Error("Scene plan storage already closing");this.#closing=true;
  let owners=0,active:Resource|null=null,activeFinished=false,root=0,finished=false;
  const closeBuffer=(resource:Resource,work:RetainedWorkLedger,turn:RetainedWorkTurn):boolean=>{const before="buffer"in resource?resource.buffer.length:0,used=turn.actualWork(),done="buffer"in resource?resource.buffer.retireOne(work,turn):resource.bytes.retire(work,turn);owners+=(before-("buffer"in resource?resource.buffer.length:0))*resource.rowOwners;if(done){owners+=resource.finalOwners;activeFinished=true;}return done&&turn.actualWork()===used;};
  const driver=new RetainedWorkRetirement((work,turn)=>{
   if(active){if(!activeFinished){if(!closeBuffer(active,work,turn))return false;}if(!work.pop(this.resources,turn))return false;active=null;activeFinished=false;return false;}
   if(this.resources.length){active=this.resources.at(-1)!;if(!closeBuffer(active,work,turn))return false;if(!work.pop(this.resources,turn))return false;active=null;activeFinished=false;return false;}
   if(!this.resources.retireOne(work,turn))return false;if(turn.actualWork()>0)return false;
   while(root<3){const values=root===0?this.nodes:root===1?this.assets:this.rewrite,before=values.length,used=turn.actualWork(),done=values.retireOne(work,turn);owners+=(before-values.length)*(root<2?3:0);if(done){if(root<2)owners++;root++;}if(!done||turn.actualWork()>used)return false;}
   if(!finished){if(!work.canStep())return false;this.#closed=true;finished=true;owners++;turn.commitActualWork();return false;}
   return true;
  });
  return{advance:(items,work)=>driver.advance(items,work),lastTurnWork:()=>driver.lastTurnWork(),nextDemand:()=>driver.nextDemand(),terminalIsEmpty:()=>driver.terminalIsEmpty(),owners:()=>owners};
 }
}
let issue:(body:PlanStorage)=>ScenePlanOwner;let inspectOwner:(value:ScenePlanOwner)=>PlanStorage;let takeOwner:(value:ScenePlanOwner)=>PlanStorage;
/** 🔐️ A unique private body is transferred once; borrowed rows never grant retirement authority. */
export class ScenePlanOwner{
 #body:PlanStorage;#taken=false;
 private constructor(body:PlanStorage){if(!PlanStorage.authentic(body))throw Error("Authentic scene storage required");this.#body=body;}
 static{issue=body=>new ScenePlanOwner(body);inspectOwner=value=>{if(value===null||typeof value!=="object"||!(#body in value)||value.#taken)throw Error("Authentic unconsumed scene owner required");value.#body.check();return value.#body;};takeOwner=value=>{const body=inspectOwner(value);value.#taken=true;return body;};}
 borrow():ScenePlanRead{return inspectOwner(this).view;}
 static transfer(value:ScenePlanOwner):ScenePlanOwner{return issue(takeOwner(value));}
 static consume(value:ScenePlanOwner):RetainedWorkRetirementCursor&{owners():number}{return takeOwner(value).close();}
}
function resourceBuffer<T>(body:PlanStorage,bytes:number,rowOwners:number,finalOwners:number,context:RetainedWorkProducerStep):RetainedWorkBuffer<T>|null{
 if(!context.slot(body.resources))return null;const values=new RetainedWorkBuffer<T>(bytes);context.push(body.resources,{buffer:values,rowOwners,finalOwners});return values;
}
class SceneContentCopy{
 #runtime=new RetainedWorkProducerRuntime();#phase="path";#index=0;#work=0;#ready=false;#failed=false;#failure:unknown;#source:ScenePlanContent|null;#reuseFill:Fill|null=null;#reuseStroke:Stroke|null=null;
 #segments:RetainedWorkBuffer<PathSegment>|null=null;#children:RetainedWorkBuffer<string>|null=null;#matrix:RetainedWorkBuffer<Matrix>|null=null;#content:RetainedWorkBuffer<ScenePlanContent>|null=null;#fills:RetainedWorkBuffer<Fill>|null=null;#stops:RetainedWorkBuffer<Stop>|null=null;#strokes:RetainedWorkBuffer<Stroke>|null=null;#dashes:RetainedWorkBuffer<number>|null=null;
 constructor(private readonly body:PlanStorage,source:ScenePlanContent,readonly maximumWork:number,previous:ScenePlanContent|null){this.#source=source;if(previous!==null&&"fill"in previous&&"fill"in source&&source.fill!==null&&source.fill===previous.fill)this.#reuseFill=source.fill;if(previous!==null&&"stroke"in previous&&"stroke"in source&&source.stroke!==null&&source.stroke===previous.stroke)this.#reuseStroke=source.stroke;}
 lastTurnWork(){return this.#runtime.lastTurnWork();}lastTurnSemanticWork(){return this.#runtime.lastTurnSemanticWork();}
 settlementPending(){return this.#phase==="publication";}failurePending(){return this.#failed;}
 progress(){return{phase:this.#phase,work:this.#work,done:this.#ready};}resultReady(){return this.#ready;}result(){if(!this.#ready)throw Error("Scene content copy incomplete");return this.#content!.at(0)!;}
 #buffer<T>(bytes:number,rowOwners:number,finalOwners:number,context:RetainedWorkProducerStep){return resourceBuffer<T>(this.body,bytes,rowOwners,finalOwners,context);}
  #paint():Paint{const source=this.#source!;if(!("fillRule"in source))throw Error("Scene content has no paint");return{fillRule:source.fillRule,fill:this.#reuseFill??this.#fills?.at(0)??null,stroke:this.#reuseStroke??this.#strokes?.at(0)??null};}

 advance(items:number,work:RetainedWorkLedger){return this.#runtime.advance(items,work,{maximumWork:this.maximumWork,workLimitMessage:"Scene content work limit",validate:()=>{if(this.#failed)throw this.#failure;},stopped:()=>this.#ready,semanticWork:()=>this.#work,recordSemanticWork:n=>{this.#work+=n;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>{try{this.#step(c);}catch(error){this.#failed=true;this.#failure=error;throw error;}},progress:()=>this.progress()});}
 #step(c:RetainedWorkProducerStep):void{switch(this.#phase){
    case"path":{const content=this.#source!;if("segments"in content){if(!this.#segments){this.#segments=this.#buffer(64,0,1,c);return;}if(this.#index<content.segments.length){if(!c.slot(this.#segments))return;c.push(this.#segments,segment(row(content.segments,this.#index)));this.#index++;c.postSemanticWork();return;}}this.#index=0;this.#phase="children";c.commitControlWork();return;}
   case"children":{const content=this.#source!;if("children"in content){if(!this.#children){this.#children=this.#buffer(8,1,1,c);return;}if(this.#index<content.children.length){if(!c.slot(this.#children))return;c.push(this.#children,row(content.children,this.#index));this.#index++;c.postSemanticWork();return;}}if(content.kind==="boolean"){if(!this.#matrix){this.#matrix=this.#buffer(48,0,0,c);return;}if(this.#matrix.length===0){if(!c.slot(this.#matrix))return;c.push(this.#matrix,matrix(content.referenceTransform));c.postSemanticWork();return;}}this.#index=0;this.#phase="paint";c.commitControlWork();return;}
   case"paint":{if(this.#reuseFill!==null){this.#phase="dash";this.#index=0;c.commitControlWork();return;}const input=this.#source!;if(!("fill"in input)||input.fill===null){this.#phase="dash";c.commitControlWork();return;}const fill=input.fill;
    if(!this.#fills){this.#fills=this.#buffer(fill.kind==="solid"?40:fill.kind==="linearGradient"?48:40,1,0,c);return;}
    if(fill.kind!=="solid"){if(!this.#stops){this.#stops=this.#buffer(40,0,1,c);return;}if(this.#index<fill.stops.length){if(!c.slot(this.#stops))return;const input=row(fill.stops,this.#index);c.push(this.#stops,Object.freeze({offset:input.offset,color:color(input.color)}));this.#index++;c.postSemanticWork();return;}}
    if(this.#fills.length===0){if(!c.slot(this.#fills))return;const copied:Fill=fill.kind==="solid"?Object.freeze({kind:fill.kind,color:color(fill.color)}):fill.kind==="linearGradient"?Object.freeze({kind:fill.kind,x1:fill.x1,y1:fill.y1,x2:fill.x2,y2:fill.y2,stops:this.body.borrow(this.#stops!)}):Object.freeze({kind:fill.kind,cx:fill.cx,cy:fill.cy,r:fill.r,stops:this.body.borrow(this.#stops!)});c.push(this.#fills,copied);c.postSemanticWork();return;}this.#phase="dash";this.#index=0;c.commitControlWork();return;
   }
   case"dash":{if(this.#reuseStroke!==null){this.#phase="content";c.commitControlWork();return;}const input=this.#source!;if(!("stroke"in input)||input.stroke===null){this.#phase="content";c.commitControlWork();return;}const stroke=input.stroke;
    if(!this.#strokes){this.#strokes=this.#buffer(stroke.dash===undefined?56:64,1,0,c);return;}
    if(stroke.dash!==undefined&&stroke.dash!==null){if(!this.#dashes){this.#dashes=this.#buffer(8,0,1,c);return;}if(this.#index<stroke.dash.length){if(!c.slot(this.#dashes))return;c.push(this.#dashes,row(stroke.dash,this.#index));this.#index++;c.postSemanticWork();return;}}
    if(this.#strokes.length===0){if(!c.slot(this.#strokes))return;const copied:Stroke=Object.freeze({color:color(stroke.color),width:stroke.width,cap:stroke.cap,join:stroke.join,...(stroke.dash!==undefined?{dash:stroke.dash===null?null:this.body.borrow(this.#dashes!)}:{})});c.push(this.#strokes,copied);c.postSemanticWork();return;}this.#phase="content";c.commitControlWork();return;
   }
   case"content":{const input=this.#source!;
    if(!this.#content){const bytes=input.kind==="path"?40:input.kind==="glyphs"?80:input.kind==="text"?72:input.kind==="image"?32:input.kind==="group"?24:56,owners=input.kind==="glyphs"||input.kind==="text"||input.kind==="image"||input.kind==="boolean"||input.kind==="trace"?2:1;this.#content=this.#buffer(bytes,owners,0,c);return;}
    if(!c.slot(this.#content))return;
    let copied:ScenePlanContent;
    switch(input.kind){
     case"path":copied={kind:input.kind,segments:this.body.borrow(this.#segments!),...this.#paint()};break;
     case"glyphs":copied={kind:input.kind,content:input.content,x:input.x,y:input.y,size:input.size,fontFamily:input.fontFamily,segments:this.body.borrow(this.#segments!),...this.#paint()};break;
     case"text":copied={kind:input.kind,content:input.content,x:input.x,y:input.y,size:input.size,fontFamily:input.fontFamily,...this.#paint()};break;
     case"image":copied={kind:input.kind,asset:input.asset,width:input.width,height:input.height};break;
     case"group":copied={kind:input.kind,children:this.body.borrow(this.#children!),isolation:input.isolation};break;
     case"boolean":copied={kind:input.kind,operation:input.operation,children:this.body.borrow(this.#children!),referenceTransform:this.#matrix!.at(0)!,...this.#paint()};break;
     case"trace":copied={kind:input.kind,source:input.source,threshold:input.threshold,simplifyEpsilon:input.simplifyEpsilon,...this.#paint()};break;
    }c.push(this.#content,Object.freeze(copied));this.#phase="publication";c.postSemanticWork();return;
   }

 case"publication":this.#ready=true;c.commitControlWork();return;
 default:throw Error("Unknown scene content copy phase");
 }}
}

class SceneNodeAppend{
 #runtime=new RetainedWorkProducerRuntime();#phase="address";#index=0;#work=0;#ready=false;#failure:unknown;#failed=false;
 #paths:RetainedWorkBuffer<number>|null=null;#groups:RetainedWorkBuffer<RasterSceneGroup>|null=null;#content:SceneContentCopy|null=null;
 constructor(private readonly body:PlanStorage,private readonly source:ScenePlanNode,readonly maximumWork:number){}
 lastTurnWork(){return this.#runtime.lastTurnWork();}lastTurnSemanticWork(){return this.#runtime.lastTurnSemanticWork();}
 resultReady(){return this.#ready;}failurePending(){return this.#failed;}
 settlementPending(){return this.#phase==="path"&&this.#content?.settlementPending()===true||this.#phase==="complete";}
 phase(){return this.#phase==="path"&&this.#content?this.#content.progress().phase:this.#phase;}
 advance(items:number,work:RetainedWorkLedger):{done:boolean}{return this.#runtime.advance(items,work,{maximumWork:this.maximumWork,workLimitMessage:"Scene node work limit",validate:()=>{if(this.#failed)throw this.#failure;},stopped:()=>this.#ready,semanticWork:()=>this.#work,recordSemanticWork:units=>{this.#work+=units;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>{try{this.#step(c);}catch(error){this.#failure=error;this.#failed=true;throw error;}},progress:()=>({done:this.#ready})});}
 #step(c:RetainedWorkProducerStep):void{
  switch(this.#phase){
   case"address":if(!this.#paths){this.#paths=resourceBuffer(this.body,8,0,1,c);return;}if(this.#index<this.source.sourcePath.length){c.requireSemanticWork();if(!c.slot(this.#paths))return;const value=row(this.source.sourcePath,this.#index);if(!Number.isSafeInteger(value)||value<0||value>65535)throw Error("Invalid scene source ancestry");c.push(this.#paths,value);this.#index++;c.postSemanticWork();return;}if(this.#paths.length<1||this.#paths.length>32||!Number.isSafeInteger(this.source.lockedAncestors)||this.source.lockedAncestors<0||this.source.lockedAncestors>=2**this.#paths.length)throw Error("Invalid scene source ancestry");this.#phase="groups";this.#index=0;c.commitSemanticWork();return;
   case"groups":if(!this.#groups){this.#groups=resourceBuffer(this.body,24,3,1,c);return;}if(this.#index<this.source.groups.length){c.requireSemanticWork();if(!c.slot(this.#groups))return;const input=row(this.source.groups,this.#index);c.push(this.#groups,Object.freeze({id:input.id,opacity:input.opacity,blendMode:input.blendMode}));this.#index++;c.postSemanticWork();return;}this.#index=0;this.#phase="path";c.commitControlWork();return;
   case"path":if(!this.#content){this.#content=new SceneContentCopy(this.body,this.source.content,this.maximumWork,null);c.commitControlWork();return;}if(!this.#content.resultReady()){c.advanceChild(this.#content);return;}this.#phase="nodePublication";c.commitControlWork();return;
   case"nodePublication":{c.requireSemanticWork();if(!c.slot(this.body.nodes))return;const input=this.source;c.push(this.body.nodes,Object.freeze({sourcePath:this.body.borrow(this.#paths!),lockedAncestors:input.lockedAncestors,id:input.id,transform:matrix(input.transform),visible:input.visible,opacity:input.opacity,blendMode:input.blendMode,groups:this.body.borrow(this.#groups!),content:this.#content!.result()}));this.#phase="complete";this.#ready=true;c.postSemanticWork();return;}
   default:throw Error("Unknown owned node phase");
  }
 }
}

export type ScenePlanCopyProgress={phase:string;work:number;nodes:number;assets:number;done:boolean};
/** 🧱️ Actual page slots precede every copied scene record and immutable variable container. */
export class ScenePlanCopyJob{
 #sourceChecked=false;#body=new PlanStorage();#runtime=new RetainedWorkProducerRuntime();#phase="source";#work=0;#cancelled=false;#moved=false;#ready=false;#failure:unknown;#hasFailure=false;#quote=emptyDemand();
 #source:ScenePlanRead|null;#asset=0;#node=0;#nodeJob:SceneNodeAppend|null=null;
 readonly maximumWork:number;
 constructor(source:ScenePlanRead,options:{maxWork:number}){if(!Number.isSafeInteger(options.maxWork)||options.maxWork<1)throw Error("Invalid scene copy work limit");if(source===null||typeof source!=="object")throw Error("Invalid scene source");this.#source=source;this.maximumWork=options.maxWork;}
 lastTurnWork(){return this.#runtime.lastTurnWork();}lastTurnSemanticWork(){return this.#runtime.lastTurnSemanticWork();}
 settlementPending(){return !this.#moved&&(this.#phase==="path"&&this.#nodeJob?.settlementPending()===true||this.#source!==null&&(this.#phase==="assets"||this.#phase==="node"||this.#phase==="publication")&&this.#asset>=this.#source.assets.length&&this.#node>=this.#source.nodes.length);}failurePending(){return this.#hasFailure;}
 nextWorkDemand(){return this.#quote;}resultReady(){return this.#ready&&!this.#cancelled&&!this.#moved&&!this.#hasFailure;}
 result():ScenePlanRead{if(!this.resultReady())throw Error("Scene plan not available");return this.#body.view;}
 cancel():void{this.#cancelled=true;}
 progress():ScenePlanCopyProgress{return{phase:this.#phase==="path"&&this.#nodeJob?this.#nodeJob.phase():this.#phase,work:this.#work,nodes:this.#node,assets:this.#asset,done:this.#ready};}
 advance(items:number,work:RetainedWorkLedger):ScenePlanCopyProgress{
  return this.#runtime.advance(items,work,{maximumWork:this.maximumWork,workLimitMessage:"Scene copy work limit",validate:()=>{if(this.#moved)throw Error("Scene copy owner transferred");if(this.#cancelled)throw new DOMException("Scene copy cancelled","AbortError");if(this.#hasFailure)throw this.#failure;if(!this.#ready&&!work.admitsAbsolute(this.#quote))work.requireAbsolute(this.#quote);},stopped:()=>this.#ready,semanticWork:()=>this.#work,recordSemanticWork:units=>{this.#work+=units;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:context=>{try{this.#step(context);}catch(error){this.#failure=error;this.#hasFailure=true;throw error;}finally{this.#quote=work.nextDemand;}},progress:()=>this.progress()});
 }
 #buffer<T>(bytes:number,rowOwners:number,finalOwners:number,context:RetainedWorkProducerStep):RetainedWorkBuffer<T>|null{
  return resourceBuffer<T>(this.#body,bytes,rowOwners,finalOwners,context);
 }
 #step(c:RetainedWorkProducerStep):void{
  const source=this.#source!;
  switch(this.#phase){
   case"source":if(!this.#sourceChecked){if(!Number.isSafeInteger(source.nodes.length)||source.nodes.length<0||!Number.isSafeInteger(source.assets.length)||source.assets.length<0)throw Error("Invalid scene row count");this.#sourceChecked=true;c.commitSemanticWork();return;}this.#phase="assets";c.commitControlWork();return;
   case"assets":if(this.#asset<source.assets.length){if(!c.slot(this.#body.assets))return;const input=row(source.assets,this.#asset);c.push(this.#body.assets,Object.freeze({id:input.id,image:Object.freeze({width:input.image.width,height:input.image.height,pixels:input.image.pixels})}));this.#asset++;c.postSemanticWork();return;}this.#phase="node";c.commitControlWork();return;
   case"node":if(this.#node>=source.nodes.length){this.#phase="publication";c.commitControlWork();return;}c.requireSemanticWork();this.#nodeJob=new SceneNodeAppend(this.#body,row(source.nodes,this.#node),this.maximumWork);this.#phase="path";c.commitSemanticWork();return;
   case"path":if(!this.#nodeJob!.resultReady()){c.advanceChild(this.#nodeJob!);return;}this.#nodeJob=null;this.#node++;this.#phase="node";c.commitControlWork();return;
   case"publication":this.#ready=true;c.commitControlWork();return;
   default:throw Error("Unknown owned scene phase");
  }
 }
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{
  if(this.#moved)throw Error("Scene copy owner transferred twice");this.#moved=true;
  const output=this.#ready&&!this.#cancelled&&!this.#hasFailure?issue(this.#body):null,closing=output===null?this.#body.close():null;
  let cleared=false;const job=new RetainedWorkRetirement((work,turn)=>{if(closing){let done=false;try{done=closing.advance(1,work).done;}finally{turn.commitActualWork(closing.lastTurnWork());}if(!done||turn.actualWork()>0)return false;}if(!cleared){if(!work.canStep())return false;this.#source=null;this.#nodeJob=null;cleared=true;turn.commitActualWork();return false;}return true;});
  return{job,output};
 }
}

export type ScenePlanAssetSamples={readonly id:string;readonly width:number;readonly height:number;readonly samples:Sequence<readonly[number,number,number,number]>};
/** 🏗️ Authored producers append borrowed records into one genuine private scene body. */
export class ScenePlanBuildJob{
 #body=new PlanStorage();#runtime=new RetainedWorkProducerRuntime();#phase="waiting";#work=0;#assets=0;#nodes=0;#pixel=0;#sourceBytes=0;#sealed=false;#ready=false;#cancelled=false;#moved=false;#failed=false;#failure:unknown;#quote=emptyDemand();
 #asset:ScenePlanAssetSamples|null=null;#bytes:RetainedWorkBytes|null=null;#node:ScenePlanNode|null=null;#child:SceneNodeAppend|null=null;
 constructor(readonly maximumWork:number){if(!Number.isSafeInteger(maximumWork)||maximumWork<1)throw Error("Invalid scene builder semantic limit");}
 lastTurnWork(){return this.#runtime.lastTurnWork();}lastTurnSemanticWork(){return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand(){return this.#quote;}failurePending(){return this.#failed;}resultReady(){return this.#ready&&!this.#failed&&!this.#cancelled&&!this.#moved;}
 settlementPending(){return this.#phase==="assetPublication"||this.#phase==="assetPixels"&&this.#asset!==null&&this.#pixel===this.#asset.width*this.#asset.height*4||this.#phase==="node"&&(this.#child?.resultReady()===true||this.#child?.settlementPending()===true)||this.#phase==="waiting"&&this.#sealed;}
 waiting(){return this.#phase==="waiting"&&!this.#sealed&&!this.#cancelled&&!this.#moved&&!this.#failed;}
 borrowResident():ScenePlanRead{if(this.#cancelled||this.#moved||this.#failed)throw Error("Scene builder borrow unavailable");return this.#body.view;}
 appendAsset(source:ScenePlanAssetSamples):void{if(!this.waiting())throw Error("Scene builder has resident input");this.#asset=source;this.#phase="assetSource";this.#quote=emptyDemand();}
 appendNode(source:ScenePlanNode):void{if(!this.waiting())throw Error("Scene builder has resident input");this.#node=source;this.#phase="node";this.#quote=emptyDemand();}
 seal():void{if(!this.waiting())throw Error("Scene builder cannot seal resident input");this.#sealed=true;this.#quote=emptyDemand();}
 result():ScenePlanRead{if(!this.resultReady())throw Error("Scene builder incomplete");return this.#body.view;}
 progress(){return{phase:this.#phase==="node"&&this.#child?this.#child.phase():this.#phase,work:this.#work,assets:this.#assets,nodes:this.#nodes,sourceBytes:this.#sourceBytes,done:this.#ready,waiting:this.waiting()};}
 advance(items:number,work:RetainedWorkLedger){return this.#runtime.advance(items,work,{maximumWork:this.maximumWork,workLimitMessage:"Scene builder work limit",validate:()=>{if(this.#moved)throw Error("Scene builder transferred");if(this.#cancelled)throw new DOMException("Scene builder cancelled","AbortError");if(this.#failed)throw this.#failure;if(!this.#ready&&!work.admitsAbsolute(this.#quote))work.requireAbsolute(this.#quote);},stopped:()=>this.#ready||this.waiting(),semanticWork:()=>this.#work,recordSemanticWork:units=>{this.#work+=units;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>{try{this.#step(c);}catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#quote=work.nextDemand;}},progress:()=>this.progress()});}
 #step(c:RetainedWorkProducerStep):void{
  switch(this.#phase){
   case"assetSource":{const source=this.#asset!;if(typeof source.id!=="string"||source.id.length===0||!Number.isSafeInteger(source.width)||!Number.isSafeInteger(source.height)||source.width<1||source.height<1||!Number.isSafeInteger(source.width*source.height)||source.samples.length!==source.width*source.height)throw Error("Invalid produced scene asset");this.#pixel=0;this.#phase="assetOwner";c.commitSemanticWork();return;}
   case"assetOwner":if(!c.slot(this.#body.resources))return;this.#bytes=new RetainedWorkBytes();c.push(this.#body.resources,{bytes:this.#bytes,rowOwners:0,finalOwners:0});this.#phase="assetCapacity";return;
   case"assetCapacity":if(!this.#bytes!.admit(this.#asset!.width*this.#asset!.height*4,c.work,c.retainedTurn))return;this.#phase="assetPixels";return;
   case"assetPixels":{const source=this.#asset!;if(this.#pixel===source.width*source.height*4){this.#phase="assetPublication";c.commitControlWork();return;}c.requireSemanticWork();const sample=row(source.samples,Math.floor(this.#pixel/4)),value=sample[this.#pixel%4];if(value===undefined||!Number.isInteger(value)||value<0||value>255)throw Error("Invalid produced scene RGBA sample");if(!this.#bytes!.set(this.#pixel,value,c.work,c.retainedTurn))return;this.#pixel++;if(this.#pixel%4===0){this.#sourceBytes+=4;c.postSemanticWork();}return;}
   case"assetPublication":if(!c.slot(this.#body.assets))return;c.push(this.#body.assets,Object.freeze({id:this.#asset!.id,image:Object.freeze({width:this.#asset!.width,height:this.#asset!.height,pixels:this.#bytes!.bytes})}));this.#asset=null;this.#bytes=null;this.#assets++;this.#phase="waiting";return;
   case"node":if(!this.#child){this.#child=new SceneNodeAppend(this.#body,this.#node!,this.maximumWork);c.commitControlWork();return;}if(!this.#child.resultReady()){c.advanceChild(this.#child);return;}this.#child=null;this.#node=null;this.#nodes++;this.#phase="waiting";c.commitControlWork();return;
   case"waiting":if(!this.#sealed)throw Error("Scene builder input absent");this.#ready=true;c.commitControlWork();return;
   default:throw Error("Unknown scene builder phase");
  }
 }
 cancel():void{this.#cancelled=true;}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{if(this.#moved)throw Error("Scene builder transferred twice");this.#moved=true;const output=this.#ready&&!this.#failed&&!this.#cancelled?issue(this.#body):null,closing=output===null?this.#body.close():null;let cleared=false;const job=new RetainedWorkRetirement((work,turn)=>{if(closing){let done=false;try{done=closing.advance(1,work).done;}finally{turn.commitActualWork(closing.lastTurnWork());}if(!done||turn.actualWork()>0)return false;}if(!cleared){if(!work.canStep())return false;this.#asset=null;this.#bytes=null;this.#node=null;this.#child=null;cleared=true;turn.commitActualWork();return false;}return true;});return{job,output};}
}

function oldContentResource(body:PlanStorage,content:ScenePlanContent,replacement:ScenePlanContent,resource:Resource):boolean{
 if(!("buffer"in resource))return false;const buffer=resource.buffer;if(buffer.at(0)===content)return true;
 if("segments"in content&&Borrow.matches(body,content.segments,buffer))return true;
 if("children"in content&&Borrow.matches(body,content.children,buffer))return true;
 if(content.kind==="boolean"&&buffer.at(0)===content.referenceTransform)return true;
 if("fill"in content&&content.fill!==null&&!("fill"in replacement&&replacement.fill===content.fill)){if(buffer.at(0)===content.fill)return true;if(content.fill.kind!=="solid"&&Borrow.matches(body,content.fill.stops,buffer))return true;}
 if("stroke"in content&&content.stroke!==null&&!("stroke"in replacement&&replacement.stroke===content.stroke)){if(buffer.at(0)===content.stroke)return true;if(content.stroke.dash!==undefined&&content.stroke.dash!==null&&Borrow.matches(body,content.stroke.dash,buffer))return true;}
 return false;
}
/** 📝️ Replaces only funded content and node cells while retaining the same original private body. */
export class ScenePlanContentReplaceJob{
 #sourceChecked=false;#body:PlanStorage;#old:ScenePlanNode|null;#source:ScenePlanContent|null;#child:SceneContentCopy|null=null;#runtime=new RetainedWorkProducerRuntime();#phase="source";#work=0;#ready=false;#cancelled=false;#moved=false;#failed=false;#failure:unknown;#quote=emptyDemand();#scan=0;#oldResources=0;#matched=false;
 readonly maximumWork:number;
 constructor(owner:ScenePlanOwner,private readonly index:number,source:ScenePlanContent,options:{maxWork:number}){
  const body=inspectOwner(owner);if(!Number.isSafeInteger(index)||index<0||index>=body.nodes.length||source===null||typeof source!=="object"||!Number.isSafeInteger(options.maxWork)||options.maxWork<1)throw Error("Invalid scene content replacement");
  this.maximumWork=options.maxWork;this.#old=row(body.nodes,index);this.#source=source;this.#oldResources=body.resources.length;this.#body=takeOwner(owner);
 }
 lastTurnWork(){return this.#runtime.lastTurnWork();}lastTurnSemanticWork(){return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand(){return this.#quote;}failurePending(){return this.#failed;}
 settlementPending(){return this.#child?.resultReady()===true||this.#child?.settlementPending()===true;}
 progress(){return{phase:this.#phase==="content"&&this.#child?this.#child.progress().phase:this.#phase,work:this.#work,done:this.#ready};}
 resultReady(){return this.#ready&&!this.#cancelled&&!this.#failed&&!this.#moved;}
 result():ScenePlanRead{if(!this.resultReady())throw Error("Scene replacement incomplete");return this.#body.view;}
 cancel(){this.#cancelled=true;}
 advance(items:number,work:RetainedWorkLedger){
  return this.#runtime.advance(items,work,{maximumWork:this.maximumWork,workLimitMessage:"Scene replacement work limit",validate:()=>{if(this.#moved)throw Error("Scene replacement transferred");if(this.#cancelled)throw new DOMException("Scene replacement cancelled","AbortError");if(this.#failed)throw this.#failure;if(!this.#ready&&!work.admitsAbsolute(this.#quote))work.requireAbsolute(this.#quote);},stopped:()=>this.#ready,semanticWork:()=>this.#work,recordSemanticWork:units=>{this.#work+=units;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>{try{this.#step(c);}catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#quote=work.nextDemand;}},progress:()=>this.progress()});
 }
 #step(c:RetainedWorkProducerStep):void{
  switch(this.#phase){
   case"source":if(!this.#sourceChecked){if(this.#body.nodes.at(this.index)!==this.#old)throw Error("Original scene node changed");this.#sourceChecked=true;c.commitSemanticWork();return;}this.#child=new SceneContentCopy(this.#body,this.#source!,this.maximumWork,this.#old!.content);this.#phase="content";c.commitControlWork();return;
   case"content":if(!this.#child!.resultReady()){c.advanceChild(this.#child!);return;}this.#phase="nodeRewrite";c.commitControlWork();return;
   case"nodeRewrite":if(this.#body.rewrite.length===0){if(!c.slot(this.#body.rewrite))return;c.push(this.#body.rewrite,Object.freeze({...this.#old!,content:this.#child!.result()}));return;}if(!c.write(this.#body.nodes,this.index,this.#body.rewrite.at(0)!))return;this.#phase="stagedClear";return;
   case"stagedClear":if(!c.pop(this.#body.rewrite))return;this.#phase="reclassification";return;
   case"reclassification":if(this.#scan===this.#oldResources){this.#phase="publication";c.commitControlWork();return;}else{const resource=row(this.#body.resources,this.#scan);if(!this.#matched){if((resource.rowOwners!==0||resource.finalOwners!==0)&&oldContentResource(this.#body,this.#old!.content,this.#child!.result(),resource))this.#matched=true;else this.#scan++;c.commitControlWork();return;}if(!c.work.copyBytes(16))return;resource.rowOwners=0;resource.finalOwners=0;c.work.notePhysicalEffect();this.#matched=false;this.#scan++;c.commitControlWork();return;}
   case"publication":this.#ready=true;c.commitControlWork();return;
   default:throw Error("Unknown scene replacement phase");
  }
 }
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{
  if(this.#moved)throw Error("Scene replacement transferred twice");this.#moved=true;
  const output=this.#ready&&!this.#cancelled&&!this.#failed?issue(this.#body):null,closing=output===null?this.#body.close():null;let cleared=false;
  const job=new RetainedWorkRetirement((work,turn)=>{if(closing){let done=false;try{done=closing.advance(1,work).done;}finally{turn.commitActualWork(closing.lastTurnWork());}if(!done||turn.actualWork()>0)return false;}if(!cleared){if(!work.canStep())return false;this.#old=null;this.#source=null;this.#child=null;cleared=true;turn.commitActualWork();return false;}return true;});return{job,output};
 }
}
