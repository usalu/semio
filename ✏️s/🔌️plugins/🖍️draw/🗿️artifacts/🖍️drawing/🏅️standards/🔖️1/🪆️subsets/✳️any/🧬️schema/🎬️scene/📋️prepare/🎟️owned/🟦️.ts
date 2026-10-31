/** 🏗️ Authored source values become a genuinely owned Scene body under one caller ledger. */
import {DrawingAssetsOwner,drawingDrawingArtifactGuardRefusal,type DrawingArtifact,type DrawingLayerNode,type DrawingPathSegment,DRAWING_BLEND_MODES} from "../../../🟦️.ts";
import {binary64Value} from "@semio-tech/framework/value/ieee754";
import {drawingTransformToMatrix} from "../../../🧮️geometry/↗️affine/🟦️.ts";
import {parseDrawingFontFamily} from "../../../📝️text/🔤️family/🟦️.ts";
import type {RasterSceneNode,RasterSceneGroup} from "../../📷️raster/🟦️.ts";
import type {Color} from "../../../🎨️fill/🖌️paint-model/🟦️.ts";
import type {Matrix} from "../../🗺️plan-model/🟦️.ts";
import type {PathSegment} from "@semio-tech/framework/2d";
import {ScenePlanBuildJob,type ScenePlanOwner,type ScenePlanRead,type ScenePlanNode,type ScenePlanContent,type ScenePlanAssetSamples} from "../../🗺️plan-model/🎟️owned/🟦️.ts";
import {ScenePlanCloseJob} from "../../🧹️retire/🟦️.ts";
import {RetainedWorkBuffer,RetainedWorkLedger,RetainedWorkRetirement,retireRetainedWorkBuffer,type RetainedWorkRetirementCursor,type RetainedWorkDemand} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork} from "../../🎟️work/🟦️.ts";
export type DrawingSceneSource=Pick<DrawingArtifact,"layers"|"assets">&{rootVisible?:(index:number)=>boolean};
export type DocumentSceneLimits={maxNodes:number;maxDepth:number;maxSegments:number;maxReferences:number;maxSourceBytes:number};
export type DocumentSceneProgress={phase:"assets"|"layers"|"paint"|"geometry"|"references"|"validation"|"cycles"|"complete";layers:number;assets:number;segments:number;references:number;sourceBytes:number;work:number;done:boolean};
export type DocumentSceneRetirement=RetainedWorkRetirementCursor;
export type DocumentSceneRetirementProgress=ReturnType<RetainedWorkRetirementCursor["advance"]>;
export type DocumentSceneOptions={work:DrawingSceneWorkController;operation:object;maximumWork:number;signal?:AbortSignal;quantum?:number;onProgress?:(progress:DocumentSceneProgress)=>void};
const identity:Matrix=[1,0,0,1,0,0];
function fail(message:string):never{throw new drawingDrawingArtifactGuardRefusal("$scene",message);}
const cancel=():never=>{throw new DOMException("Document scene preparation cancelled","AbortError");};
const numeric=(v:Parameters<typeof binary64Value>[0]):number=>{const n=binary64Value(v);if(!Number.isFinite(n)||Math.abs(n)>1e9)fail("Invalid authored scene coordinate");return n;};
const positive=(n:number)=>{if(n<=0)fail("Expected positive authored scene dimension");return n;};
const unit=(n:number)=>{if(n<0||n>1)fail("Expected unit paint or opacity");return n;};
const id=(v:string)=>{if(typeof v!=="string"||!v.length||v.length>4096)fail("Invalid scene identity");let bytes=0;for(const scalar of v){const code=scalar.codePointAt(0)!;bytes+=code<=127?1:code<=2047?2:code<=65535?3:4;if(bytes>4096)fail("Invalid scene identity");}return v;};
const blend=(v:string)=>{if(!(DRAWING_BLEND_MODES as readonly string[]).includes(v))fail("Invalid authored scene blend");return v as RasterSceneNode["blendMode"];};
const color=(v:readonly Parameters<typeof binary64Value>[0][]):Color=>{if(v.length!==4)fail("Invalid authored scene color");return v.map(n=>unit(numeric(n))) as Color;};
const multiply=(a:Matrix,b:Matrix):Matrix=>{const m:Matrix=[a[0]*b[0]+a[2]*b[1],a[1]*b[0]+a[3]*b[1],a[0]*b[2]+a[2]*b[3],a[1]*b[2]+a[3]*b[3],a[0]*b[4]+a[2]*b[5]+a[4],a[1]*b[4]+a[3]*b[5]+a[5]];if(!m.every(n=>Number.isFinite(n)&&Math.abs(n)<=1e9))fail("World scene matrix exceeds coordinate limit");return m.map(n=>n===0?0:n) as Matrix;};
const point=(v:readonly Parameters<typeof binary64Value>[0][]):[number,number]=>{if(v.length!==2)fail("Invalid authored scene point");return[numeric(v[0]!),numeric(v[1]!)];};
function segment(s:DrawingPathSegment):PathSegment{switch(s.kind){case"move":case"line":return{kind:s.kind,to:point(s.to)};case"quad":return{kind:s.kind,ctrl:point(s.ctrl),to:point(s.to)};case"cubic":return{kind:s.kind,ctrl1:point(s.ctrl1),ctrl2:point(s.ctrl2),to:point(s.to)};case"arc":return{kind:s.kind,rx:numeric(s.rx),ry:numeric(s.ry),rotation:numeric(s.rotation),largeArc:s.largeArc,sweep:s.sweep,to:point(s.to)};case"close":return{kind:s.kind};default:return fail("Unknown authored scene segment");}}
function shapeSegment(layer:Extract<DrawingLayerNode,{kind:"shape"}>,at:number):PathSegment|null{
 if(layer.shapeKind==="polygon"){const points=layer.polygon?.points;if(!points)fail("Missing polygon geometry");return at<points.length?{kind:at?"line":"move",to:point(points[at]!)}:points.length&&at===points.length?{kind:"close"}:null;}
 if(layer.shapeKind==="rect"){const r=layer.rect;if(!r)fail("Missing rectangle geometry");const x=numeric(r.x),y=numeric(r.y),w=numeric(r.width),h=numeric(r.height);return ([{kind:"move",to:[x,y]},{kind:"line",to:[x+w,y]},{kind:"line",to:[x+w,y+h]},{kind:"line",to:[x,y+h]},{kind:"close"}] as PathSegment[])[at]??null;}
 if(layer.shapeKind==="line"){const l=layer.line;if(!l)fail("Missing line geometry");return ([{kind:"move",to:[numeric(l.x1),numeric(l.y1)]},{kind:"line",to:[numeric(l.x2),numeric(l.y2)]}] as PathSegment[])[at]??null;}
 if(layer.shapeKind==="ellipse"||layer.shapeKind==="circle"){const e=layer.shapeKind==="ellipse"?layer.ellipse:layer.circle;if(!e)fail("Missing ellipse geometry");const cx=numeric(e.cx),cy=numeric(e.cy),rx=numeric("rx" in e?e.rx:e.r),ry=numeric("ry" in e?e.ry:e.r);if(at===5)return{kind:"close"};const to=([[cx,cy-ry],[cx+rx,cy],[cx,cy+ry],[cx-rx,cy],[cx,cy-ry]] as [number,number][])[at];return to?at===0?{kind:"move",to}:{kind:"arc",rx:Math.abs(rx),ry:Math.abs(ry),rotation:0,largeArc:false,sweep:(rx>=0)===(ry>=0),to}:null;}
 return fail("Unknown primitive shape");
}
type PreparationFrame={parent:PreparationFrame|null;layers:readonly DrawingLayerNode[];at:number;sourceIndex:number;depth:number;lockedAncestors:number;matrix:Matrix;visible:boolean;scopeId:string|null;scopeOpacity:number;scopeBlend:RasterSceneGroup["blendMode"];scopeCount:number};
type ProjectionState={layer():DrawingLayerNode;frame():PreparationFrame;index():number;visible():boolean};
type InputRows<T>={readonly length:number;at(index:number):T|undefined};
class AuthoredRows<T> implements InputRows<T>{
 constructor(private readonly extent:()=>number,private readonly read:(at:number)=>T|undefined){}
 get length(){return this.extent();}
 at(index:number){if(!Number.isInteger(index))return undefined;const length=this.length,at=index<0?index+length:index;return at<0||at>=length?undefined:this.read(at);}
}
type SceneFill=Extract<ScenePlanContent,{kind:"path"}>["fill"];
type SceneStroke=Extract<ScenePlanContent,{kind:"path"}>["stroke"];
class AuthoredPaintSource{
 readonly stops:InputRows<{offset:number;color:Color}>;readonly dash:InputRows<number>;
 constructor(protected readonly state:ProjectionState){
  this.stops=new AuthoredRows(()=>{const fill=state.layer().attributes.fill;return fill&&fill.kind!=="solid"?fill.stops.length:0;},at=>{const fill=state.layer().attributes.fill;if(!fill||fill.kind==="solid")return undefined;const value=fill.stops[at];return value===undefined?undefined:{offset:unit(numeric(value.offset)),color:color(value.color)};});
  this.dash=new AuthoredRows(()=>state.layer().attributes.stroke?.dash?.length??0,at=>{const value=state.layer().attributes.stroke?.dash?.[at];if(value===undefined)return undefined;const next=numeric(value);if(next<0)fail("Invalid scene dash");return next;});
 }
 get fillRule(){const value=this.state.layer().attributes.fillRule;if(value!=="nonzero"&&value!=="evenodd")fail("Invalid scene fill rule");return value;}
 get fill():SceneFill{const fill=this.state.layer().attributes.fill;if(!fill)return null;switch(fill.kind){case"solid":return{kind:fill.kind,color:color(fill.color)};case"linearGradient":return{kind:fill.kind,x1:numeric(fill.x1),y1:numeric(fill.y1),x2:numeric(fill.x2),y2:numeric(fill.y2),stops:this.stops};case"radialGradient":return{kind:fill.kind,cx:numeric(fill.cx),cy:numeric(fill.cy),r:positive(numeric(fill.r)),stops:this.stops};}}
 get stroke():SceneStroke{const stroke=this.state.layer().attributes.stroke;if(!stroke)return null;const width=numeric(stroke.width);if(width<0||!["butt","round","square"].includes(stroke.cap)||!["miter","round","bevel"].includes(stroke.join))fail("Invalid scene stroke");return{color:color(stroke.color),width,cap:stroke.cap,join:stroke.join,...(stroke.dash===undefined?{}:{dash:this.dash})};}
}
class AuthoredPathSource extends AuthoredPaintSource{
 readonly kind="path";readonly segments:InputRows<PathSegment>;
 constructor(source:ProjectionState){super(source);this.segments=new AuthoredRows(()=>{const layer=source.layer();if(layer.kind==="path")return layer.segments.length;if(layer.kind!=="shape")fail("Expected authored path");switch(layer.shapeKind){case"rect":return 5;case"line":return 2;case"ellipse":case"circle":return 6;case"polygon":{const points=layer.polygon?.points;if(!points)fail("Missing polygon geometry");return points.length+(points.length?1:0);}default:return fail("Unknown primitive shape");}},at=>{const layer=source.layer();if(layer.kind==="path"){const value=layer.segments[at];return value===undefined?undefined:segment(value);}if(layer.kind!=="shape")fail("Expected authored shape");return shapeSegment(layer,at)??undefined;});}
}
class AuthoredTextSource extends AuthoredPaintSource{
 readonly kind="text";
 private current(){const value=this.state.layer();if(value.kind!=="text")fail("Expected authored text");return value;}
 get content(){return this.current().content;}get x(){return numeric(this.current().x);}get y(){return numeric(this.current().y);}get size(){return positive(numeric(this.current().size));}get fontFamily(){return parseDrawingFontFamily(this.current().fontFamily);}
}
class AuthoredChildrenSource implements InputRows<string>{
 constructor(private readonly source:ProjectionState){}
 get length(){const layer=this.source.layer();if(layer.kind!=="group"&&layer.kind!=="boolean")fail("Expected authored references");return layer.children.length;}
 at(index:number){const layer=this.source.layer();if(layer.kind!=="group"&&layer.kind!=="boolean")fail("Expected authored references");const at=index<0?index+layer.children.length:index,value=layer.children[at];return value===undefined?undefined:id(typeof value==="string"?value:value.id);}
}
class AuthoredGroupSource{
 readonly kind="group";readonly children:InputRows<string>;
 constructor(private readonly source:ProjectionState){this.children=new AuthoredChildrenSource(source);}
 get isolation(){const layer=this.source.layer();if(layer.kind!=="group")fail("Expected authored group");return layer.isolation;}
}
class AuthoredBooleanSource extends AuthoredPaintSource{
 readonly kind="boolean";readonly children:InputRows<string>;
 constructor(source:ProjectionState){super(source);this.children=new AuthoredChildrenSource(source);}
 get operation(){const layer=this.state.layer();if(layer.kind!=="boolean")fail("Expected authored boolean");switch(layer.operation){case"union":case"difference":case"intersection":case"xor":return layer.operation;default:return fail("Invalid boolean scene work");}}
 get referenceTransform(){return this.state.frame().matrix;}
}
class AuthoredTraceSource extends AuthoredPaintSource{
 readonly kind="trace";
 private current(){const layer=this.state.layer();if(layer.kind!=="trace")fail("Expected authored trace");return layer;}
 get source(){return id(this.current().sourceKey);}
 get threshold(){return unit(numeric(this.current().params.threshold));}
 get simplifyEpsilon(){const value=numeric(this.current().params.simplifyEpsilon);if(value<0)fail("Invalid trace epsilon");return value;}
}
class AuthoredImageSource{
 readonly kind="image";
 constructor(private readonly source:ProjectionState){}
 private current(){const layer=this.source.layer();if(layer.kind!=="image")fail("Expected authored image");return layer;}
 get asset(){return id(this.current().imageKey);}get width(){return positive(numeric(this.current().width));}get height(){return positive(numeric(this.current().height));}
}
class AuthoredNodeSource implements ScenePlanNode{
 readonly sourcePath:InputRows<number>;readonly groups:InputRows<RasterSceneGroup>;
 readonly #path:AuthoredPathSource;readonly #text:AuthoredTextSource;readonly #group:AuthoredGroupSource;readonly #boolean:AuthoredBooleanSource;readonly #trace:AuthoredTraceSource;readonly #image:AuthoredImageSource;
 constructor(private readonly source:ProjectionState){
  this.#path=new AuthoredPathSource(source);this.#text=new AuthoredTextSource(source);this.#group=new AuthoredGroupSource(source);this.#boolean=new AuthoredBooleanSource(source);this.#trace=new AuthoredTraceSource(source);this.#image=new AuthoredImageSource(source);
  this.sourcePath=new AuthoredRows(()=>source.frame().depth+1,at=>{let frame=source.frame();if(at===frame.depth)return source.index();while(frame.parent!==null&&frame.depth>at+1)frame=frame.parent;return frame.sourceIndex;});
  this.groups=new AuthoredRows(()=>source.frame().scopeCount,at=>{let frame:PreparationFrame|null=source.frame();while(frame!==null){if(frame.scopeId!==null&&frame.scopeCount===at+1)return{id:frame.scopeId,opacity:frame.scopeOpacity,blendMode:frame.scopeBlend};frame=frame.parent;}return undefined;});
 }
 get id(){return id(this.source.layer().id);}get lockedAncestors(){const frame=this.source.frame(),layer=this.source.layer();return frame.lockedAncestors+(layer.locked?2**frame.depth:0);}
 get transform():Matrix{const t=this.source.layer().transform;return multiply(this.source.frame().matrix,drawingTransformToMatrix({x:numeric(t.x),y:numeric(t.y),scaleX:numeric(t.scaleX),scaleY:numeric(t.scaleY),shear:numeric(t.shear),rotation:numeric(t.rotation)}));}
 get visible(){return this.source.visible();}get opacity(){return unit(numeric(this.source.layer().opacity));}get blendMode(){return blend(this.source.layer().blendMode);}
 get content():ScenePlanContent{const layer=this.source.layer();switch(layer.kind){case"path":case"shape":return this.#path;case"text":return this.#text;case"group":return this.#group;case"boolean":return this.#boolean;case"trace":return this.#trace;case"image":return this.#image;}}
}


type IdentityRow={key:string;index:number};
type CycleRow={index:number;at:number};
const zeroDemand=():RetainedWorkDemand=>({copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1});
/** 🧱️ Funded traversal and source validation append directly into the canonical private builder. */
export class DocumentSceneJob{
 #document:DrawingSceneSource|null;readonly #limits:DocumentSceneLimits;readonly #maximumWork:number;readonly #builder:ScenePlanBuildJob;readonly #view:AuthoredNodeSource;readonly #assetView:ScenePlanAssetSamples;
 #runtime=new RetainedWorkProducerRuntime();#keys=new RetainedWorkBuffer<string>(8);#frames=new RetainedWorkBuffer<PreparationFrame>(136);#ids=new RetainedWorkBuffer<IdentityRow>(16);#states=new RetainedWorkBuffer<number>(8);#graph=new RetainedWorkBuffer<CycleRow>(16);
 #quote=zeroDemand();#stage="keys";#phase:DocumentSceneProgress["phase"]="assets";#work=0;#layers=0;#segments=0;#references=0;#moved=false;#cancelled=false;#failed=false;#failure:unknown;
 #entry=0;#sortAt=1;#sortProbe=0;#sortKey:string|null=null;#sortShift=false;#assetAt=0;#assetKey="";#frame:PreparationFrame|null=null;#current:DrawingLayerNode|null=null;#currentIndex=0;
 #lookupAt=0;#lookupKey="";#lookupMode="";#lookupTarget=0;#paintStage=0;#paintAt=0;#geometryAt=0;#textAt=0;#textChars=0;#validateAt=0;#referenceAt=0;#cycleAt=0;#cycleTarget=0;#cleanupAt=0;
 constructor(document:DrawingSceneSource,limits:DocumentSceneLimits,maximumWork:number){
  if(!Number.isSafeInteger(maximumWork)||maximumWork<1)fail("Invalid scene preparation semantic limit");
  for(const[key,cap]of[["maxNodes",1024],["maxDepth",32],["maxSegments",65536],["maxReferences",32768],["maxSourceBytes",268439552]]as const)if(!Number.isSafeInteger(limits[key])||limits[key]<1||limits[key]>cap)fail("Invalid scene preparation limits");
  if(!Array.isArray(document.layers)||document.layers.length>limits.maxNodes||!DrawingAssetsOwner.is(document.assets)||DrawingAssetsOwner.size(document.assets)>1024)fail("Invalid drawing document scene input");
  this.#document=document;this.#limits={...limits};this.#maximumWork=maximumWork;this.#builder=new ScenePlanBuildJob(maximumWork);
  this.#view=new AuthoredNodeSource({layer:()=>this.#layer(),frame:()=>this.#parent(),index:()=>this.#currentIndex,visible:()=>{const frame=this.#parent(),layer=this.#layer();return frame.visible&&layer.visible&&(frame.depth!==0||(this.#document!.rootVisible?.(this.#currentIndex)??true));}});
  const self=this;this.#assetView={get id(){return self.#assetKey;},get width(){return self.#asset().width;},get height(){return self.#asset().height;},get samples(){return self.#asset().samples;}};
 }
 #layer(){if(this.#current===null)fail("Scene layer source inactive");return this.#current;}
 #parent(){if(this.#frame===null)fail("Scene parent source inactive");return this.#frame;}
 #asset(){const value=this.#document!.assets[this.#assetKey];if(value===undefined)fail("Scene asset source missing");return value;}
 lastTurnWork(){return this.#runtime.lastTurnWork();}lastTurnSemanticWork(){return this.#runtime.lastTurnSemanticWork();}
 nextWorkDemand(){return this.#quote;}failurePending(){return this.#failed;}
 settlementPending(){if(this.#stage==="cleanup"||this.#stage==="seal"||this.#stage==="publication"||this.#stage==="advanceParent"||this.#stage==="cycleStateDone"||this.#stage==="cyclePop"||this.#stage==="cycleEdge")return true;if(this.#stage==="groupFrame")return this.#current!==null&&(this.#current.kind!=="group"||this.#current.children.length===0);if(this.#stage==="assetWork"||this.#stage==="nodeWork")return this.#builder.waiting()||this.#builder.settlementPending();if(this.#stage==="cycles")return this.#graph.length===0&&this.#cycleAt>=this.#ids.length;if(this.#stage==="layers"){const top=this.#frames.at(-1);return top!==undefined&&top.at>=top.layers.length;}return false;}
 resultReady(){return this.#phase==="complete"&&!this.#failed&&!this.#cancelled&&!this.#moved;}
 progress():DocumentSceneProgress{const builder=this.#builder.progress();return{phase:this.#phase,layers:this.#layers,assets:builder.assets,segments:this.#segments,references:this.#references,sourceBytes:builder.sourceBytes,work:this.#work,done:this.#phase==="complete"};}
 advance(items:number,work:RetainedWorkLedger):DocumentSceneProgress{return this.#runtime.advance(items,work,{maximumWork:this.#maximumWork,workLimitMessage:"Document scene preparation work limit exceeded",validate:()=>{if(this.#moved)throw Error("Preparation owner transferred");if(this.#cancelled)cancel();if(this.#failed)throw this.#failure;if(this.#phase!=="complete"&&!work.admitsAbsolute(this.#quote))work.requireAbsolute(this.#quote);},stopped:()=>this.#phase==="complete",semanticWork:()=>this.#work,recordSemanticWork:units=>{this.#work+=units;},settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),step:c=>{try{this.#step(c);}catch(error){this.#failed=true;this.#failure=error;throw error;}finally{this.#quote=work.nextDemand;}},progress:()=>this.progress()});}
 #writeField(c:RetainedWorkProducerStep,write:()=>void):boolean{if(!c.work.copyBytes(8))return false;write();c.commitControlWork();c.work.notePhysicalEffect();return true;}
 #privateBuffer(at:number):RetainedWorkBuffer<unknown>|null{return at===0?this.#frames:at===1?this.#keys:at===2?this.#ids:at===3?this.#states:at===4?this.#graph:null;}
 #step(c:RetainedWorkProducerStep):void{
  switch(this.#stage){
   case"keys":{if(this.#entry>=DrawingAssetsOwner.size(this.#document!.assets)){this.#stage="sort";c.commitControlWork();return;}if(!c.slot(this.#keys))return;const entry=DrawingAssetsOwner.entryAt(this.#document!.assets,this.#entry);if(entry===undefined)fail("Scene asset source missing");c.push(this.#keys,id(entry[0]));this.#entry++;return;}
   case"sort":if(this.#sortAt>=this.#keys.length){this.#stage="assetSelect";c.commitControlWork();return;}this.#sortKey=this.#keys.at(this.#sortAt)!;this.#sortProbe=this.#sortAt;this.#stage="sortCompare";c.commitControlWork();return;
   case"sortCompare":this.#sortShift=this.#sortProbe>0&&this.#keys.at(this.#sortProbe-1)!>this.#sortKey!;this.#stage=this.#sortShift?"sortShift":"sortPublish";c.commitControlWork();return;
   case"sortShift":if(!c.write(this.#keys,this.#sortProbe,this.#keys.at(this.#sortProbe-1)!))return;this.#sortProbe--;this.#stage="sortCompare";return;
   case"sortPublish":if(!c.write(this.#keys,this.#sortProbe,this.#sortKey!))return;this.#sortKey=null;this.#sortAt++;this.#stage="sort";return;
   case"assetSelect":{const key=this.#keys.at(this.#assetAt);if(key===undefined){this.#stage="root";c.commitControlWork();return;}this.#assetKey=key;const asset=this.#asset(),count=asset.width*asset.height;if(!Number.isSafeInteger(count)||count<1||count>16777216||!Array.isArray(asset.samples)||asset.samples.length!==count||count*4>this.#limits.maxSourceBytes-this.#builder.progress().sourceBytes)fail("Invalid intrinsic image extent or sample byte limit");this.#builder.appendAsset(this.#assetView);this.#stage="assetWork";c.commitSemanticWork();return;}
   case"assetWork":if(!this.#builder.waiting()){c.advanceChild(this.#builder);return;}this.#assetAt++;this.#assetKey="";this.#stage="assetSelect";c.commitControlWork();return;
   case"root":if(!c.slot(this.#frames))return;c.push(this.#frames,{parent:null,layers:this.#document!.layers,at:0,sourceIndex:0,depth:0,lockedAncestors:0,matrix:identity,visible:true,scopeId:null,scopeOpacity:1,scopeBlend:"normal",scopeCount:0});this.#phase="layers";this.#stage="layers";return;
   case"layers":{const frame=this.#frames.at(-1);if(frame===undefined){this.#phase="validation";this.#stage="validation";c.commitControlWork();return;}if(frame.at>=frame.layers.length){if(!c.pop(this.#frames))return;return;}c.requireSemanticWork();const layer=frame.layers[frame.at];if(layer===undefined){c.commitSemanticWork();fail("Missing authored scene layer");}this.#current=layer;this.#frame=frame;this.#currentIndex=frame.at;this.#lookupAt=0;this.#stage="duplicate";c.commitSemanticWork();return;}
   case"duplicate":{const value=this.#ids.at(this.#lookupAt);if(value===undefined){this.#stage="source";c.commitControlWork();return;}if(value.key===this.#layer().id){c.commitSemanticWork();fail("Duplicate scene identity or layer limit");}this.#lookupAt++;c.commitControlWork();return;}
   case"source":{try{const layer=this.#layer(),frame=this.#parent();if(this.#layers>=this.#limits.maxNodes||typeof layer.visible!=="boolean"||typeof layer.locked!=="boolean"||frame.depth>=this.#limits.maxDepth)fail("Invalid scene source ancestry or layer limit");id(layer.id);void this.#view.transform;void this.#view.opacity;void this.#view.blendMode;if(layer.kind==="group"&&(!Array.isArray(layer.children)||layer.children.length>1024||typeof layer.isolation!=="boolean"||layer.children.length>0&&frame.depth+1>=this.#limits.maxDepth))fail("Scene depth limit exceeded");if(layer.kind==="boolean"&&(!Array.isArray(layer.children)||layer.children.length>1024))fail("Invalid boolean scene work");const content=this.#view.content;if("children"in content&&this.#references+content.children.length>this.#limits.maxReferences)fail("Scene reference limit exceeded");if(content.kind==="path"&&(content.segments.length>65536||this.#segments+content.segments.length>this.#limits.maxSegments))fail("Scene segment limit exceeded");if(layer.kind==="text"&&(typeof layer.content!=="string"||layer.content.length>131072))fail("Scene text exceeds limit");this.#layers++;this.#paintStage=0;this.#paintAt=0;this.#geometryAt=0;this.#textAt=0;this.#textChars=0;this.#phase="paint";this.#stage="paint";}finally{c.commitSemanticWork();}return;}
   case"paint":{const content=this.#view.content;if(!("fill"in content)){this.#phase=content.kind==="group"?"references":"geometry";this.#stage="geometry";c.commitControlWork();return;}try{if(this.#paintStage===0){const fill=content.fill;if(fill!==null&&fill.kind!=="solid"&&(fill.stops.length<1||fill.stops.length>64))fail("Invalid scene gradient stops");this.#paintStage=1;return;}if(this.#paintStage===1){const fill=content.fill;if(fill!==null&&fill.kind!=="solid"&&this.#paintAt<fill.stops.length){if(fill.stops.at(this.#paintAt++)===undefined)fail("Missing scene gradient stop");return;}this.#paintStage=2;this.#paintAt=0;return;}if(this.#paintStage===2){const stroke=content.stroke;if(stroke?.dash!==undefined&&stroke.dash!==null&&stroke.dash.length>1024)fail("Invalid scene stroke");this.#paintStage=3;return;}const stroke=content.stroke;if(stroke?.dash!==undefined&&stroke.dash!==null&&this.#paintAt<stroke.dash.length){if(stroke.dash.at(this.#paintAt++)===undefined)fail("Missing scene dash");return;}this.#phase=content.kind==="boolean"?"references":"geometry";this.#stage="geometry";}finally{c.commitSemanticWork();}return;}
   case"geometry":{const content=this.#view.content,layer=this.#layer();if(content.kind==="path"&&this.#geometryAt<content.segments.length){try{const value=content.segments.at(this.#geometryAt++);if(value===undefined)fail("Missing scene segment");this.#validateSegment(value);this.#segments++;}finally{c.commitSemanticWork();}return;}if(layer.kind==="text"&&this.#textAt<layer.content.length){const code=layer.content.codePointAt(this.#textAt)!;this.#textAt+=code>65535?2:1;this.#textChars++;c.commitSemanticWork();if(this.#textChars>65536)fail("Scene text exceeds limit");return;}if("children"in content&&this.#geometryAt<content.children.length){try{if(content.children.at(this.#geometryAt++)===undefined)fail("Missing scene operand");this.#references++;}finally{c.commitSemanticWork();}return;}this.#stage="index";c.commitControlWork();return;}
   case"index":if(!c.slot(this.#ids))return;c.push(this.#ids,{key:this.#layer().id,index:this.#ids.length});this.#stage="state";return;
   case"state":if(!c.slot(this.#states))return;c.push(this.#states,0);this.#stage="node";return;
   case"node":this.#builder.appendNode(this.#view);this.#stage="nodeWork";c.commitControlWork();return;
   case"nodeWork":if(!this.#builder.waiting()){c.advanceChild(this.#builder);return;}this.#stage="groupFrame";c.commitControlWork();return;
   case"groupFrame":{const layer=this.#layer();if(layer.kind==="group"&&layer.children.length>0){if(!c.slot(this.#frames))return;const node=this.#builder.borrowResident().nodes.at(-1);if(node===undefined)fail("Missing completed scene group");const frame=this.#parent(),scope=layer.isolation||node.opacity!==1||node.blendMode!=="normal";c.push(this.#frames,{parent:frame,layers:layer.children,at:0,sourceIndex:this.#currentIndex,depth:frame.depth+1,lockedAncestors:node.lockedAncestors,matrix:[node.transform[0],node.transform[1],node.transform[2],node.transform[3],node.transform[4],node.transform[5]],visible:node.visible,scopeId:scope?node.id:null,scopeOpacity:node.opacity,scopeBlend:node.blendMode,scopeCount:frame.scopeCount+(scope?1:0)});this.#stage="advanceParent";return;}this.#stage="advanceParent";c.commitControlWork();return;}
   case"advanceParent":if(!this.#writeField(c,()=>{this.#parent().at=this.#currentIndex+1;}))return;this.#current=null;this.#frame=null;this.#phase="layers";this.#stage="layers";return;
   case"validation":{const node=this.#builder.borrowResident().nodes.at(this.#validateAt);if(node===undefined){this.#phase="cycles";this.#stage="cycles";c.commitControlWork();return;}const content=node.content;if(content.kind==="image"||content.kind==="trace"){this.#lookupKey=content.kind==="image"?content.asset:content.source;this.#lookupAt=0;this.#stage="assetLookup";c.commitControlWork();return;}this.#stage="references";c.commitSemanticWork();return;}
   case"assetLookup":{const value=this.#keys.at(this.#lookupAt);if(value===undefined){c.commitSemanticWork();fail("Missing scene source asset");}if(value===this.#lookupKey){this.#stage="references";c.commitSemanticWork();return;}this.#lookupAt++;c.commitControlWork();return;}
   case"references":{const node=this.#builder.borrowResident().nodes.at(this.#validateAt);if(node===undefined)fail("Missing scene validation row");const content=node.content;if("children"in content&&this.#referenceAt<content.children.length){const key=content.children.at(this.#referenceAt);if(key===undefined)fail("Missing scene operand");this.#lookupKey=key;this.#lookupAt=0;this.#lookupMode="validation";this.#stage="lookup";c.commitControlWork();return;}this.#validateAt++;this.#referenceAt=0;this.#stage="validation";c.commitSemanticWork();return;}
   case"lookup":{const value=this.#ids.at(this.#lookupAt);if(value===undefined){c.commitSemanticWork();fail("Missing scene operand");}if(value.key===this.#lookupKey){this.#lookupTarget=value.index;if(this.#lookupMode==="validation"){this.#referenceAt++;this.#stage="references";}else{this.#cycleTarget=value.index;this.#stage="cycleEdge";}c.commitSemanticWork();return;}this.#lookupAt++;c.commitControlWork();return;}
   case"cycles":{const top=this.#graph.at(-1);if(top===undefined){if(this.#cycleAt>=this.#ids.length){this.#stage="cleanup";c.commitControlWork();return;}const index=this.#cycleAt++,state=this.#states.at(index);this.#cycleTarget=index;if(state!==2)this.#stage="cycleStateStart";c.commitSemanticWork();return;}const node=this.#builder.borrowResident().nodes.at(top.index);if(node===undefined)fail("Missing cycle scene row");const content=node.content;if("children"in content&&top.at<content.children.length){const key=content.children.at(top.at);if(key===undefined)fail("Missing cycle operand");this.#lookupKey=key;this.#lookupAt=0;this.#lookupMode="cycles";this.#stage="lookup";c.commitControlWork();return;}this.#cycleTarget=top.index;this.#stage="cycleStateDone";c.commitSemanticWork();return;}
   case"cycleStateStart":if(!c.write(this.#states,this.#cycleTarget,1))return;this.#stage="cyclePush";return;
   case"cyclePush":if(!c.slot(this.#graph))return;c.push(this.#graph,{index:this.#cycleTarget,at:0});this.#stage="cycles";return;
   case"cycleEdge":{const top=this.#graph.at(-1);if(top===undefined)fail("Missing cycle frame");if(!this.#writeField(c,()=>{top.at++;}))return;this.#stage="cycleCheck";return;}
   case"cycleCheck":{const state=this.#states.at(this.#cycleTarget);c.commitSemanticWork();if(state===1)fail("Cyclic scene operands");this.#stage=state===0?"cycleStateStart":"cycles";return;}
   case"cycleStateDone":if(!c.write(this.#states,this.#cycleTarget,2))return;this.#stage="cyclePop";return;
   case"cyclePop":if(!c.pop(this.#graph))return;this.#stage="cycles";return;
   case"cleanup":{const values=this.#privateBuffer(this.#cleanupAt);if(values===null){this.#stage="seal";c.commitControlWork();return;}if(!values.retireOne(c.work,c.retainedTurn)||c.hasActualWork())return;this.#cleanupAt++;c.commitControlWork();return;}
   case"seal":this.#builder.seal();this.#stage="publication";c.commitControlWork();return;
   case"publication":if(!this.#builder.resultReady()){c.advanceChild(this.#builder);return;}this.#phase="complete";c.commitControlWork();return;
   default:throw Error("Unknown genuine preparation phase");
  }
 }
 #validateSegment(value:PathSegment):void{const scalar=(n:number)=>{if(!Number.isFinite(n)||Math.abs(n)>1e9)fail("Derived primitive coordinates exceed scene limit");},point=(p:readonly[number,number])=>{scalar(p[0]);scalar(p[1]);};switch(value.kind){case"close":return;case"move":case"line":point(value.to);return;case"quad":point(value.to);point(value.ctrl);return;case"cubic":point(value.to);point(value.ctrl1);point(value.ctrl2);return;case"arc":point(value.to);scalar(value.rx);scalar(value.ry);scalar(value.rotation);return;}}
 result():ScenePlanRead{if(this.#cancelled)cancel();if(this.#failed)throw this.#failure;if(!this.resultReady())fail("Document scene preparation incomplete");return this.#builder.result();}
 retireOutput(output:ScenePlanOwner):RetainedWorkRetirementCursor{return new ScenePlanCloseJob(output);}
 cancel():void{this.#cancelled=true;this.#builder.cancel();}
 intoRetirement():{job:RetainedWorkRetirementCursor;output:ScenePlanOwner|null}{if(this.#moved)throw Error("Preparation ownership already transferred");this.#moved=true;if(this.#cancelled||this.#failed||this.#phase!=="complete")this.#builder.cancel();const moved=this.#builder.intoRetirement();let child:RetainedWorkRetirementCursor|null=moved.job,slot=0,cleared=false;const job=new RetainedWorkRetirement((work,turn)=>{if(child!==null){let done=false;try{done=child.advance(1,work).done;}finally{turn.commitActualWork(child.lastTurnWork());}if(!done||turn.actualWork()>0)return false;child=null;turn.commitActualWork();return false;}const values=this.#privateBuffer(slot);if(values!==null){if(!retireRetainedWorkBuffer(values,work,turn))return false;slot++;turn.commitActualWork();return false;}if(!cleared){if(!work.canStep())return false;this.#document=null;this.#current=null;this.#frame=null;this.#sortKey=null;this.#assetKey="";this.#lookupKey="";cleared=true;turn.commitActualWork();return false;}return true;});return{job,output:moved.output};}
}
/** 🚦️ The caller's finite controller retains producer/output custody through every async refusal. */
export async function prepareDocumentScene(document:DrawingSceneSource,limits:DocumentSceneLimits,options:DocumentSceneOptions):Promise<ScenePlanOwner>{
 options.work.checkBinding(options.operation,document,"advance");if(options.signal?.aborted)cancel();if(options.quantum!==undefined&&(!Number.isSafeInteger(options.quantum)||options.quantum<1))throw RangeError("Invalid Drawing semantic quantum");const job=new DocumentSceneJob(document,limits,options.maximumWork);
 return driveDrawingSceneWork(options.work,options.operation,document,{advance:(items,work)=>job.advance(Math.min(items,options.quantum??items),work),nextWorkDemand:()=>job.nextWorkDemand(),lastTurnWork:()=>job.lastTurnWork(),cancel:()=>job.cancel(),intoRetirement:()=>job.intoRetirement(),retireOutput:output=>job.retireOutput(output),beforeTurn:async()=>{options.work.checkBinding(options.operation,document,"advance");if(options.signal?.aborted)cancel();}},options.onProgress);
}
