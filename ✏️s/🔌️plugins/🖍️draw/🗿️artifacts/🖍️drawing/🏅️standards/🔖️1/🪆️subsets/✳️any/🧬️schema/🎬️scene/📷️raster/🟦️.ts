/** 🖼️ Resolved world-space scene inputs with isolated scopes and cropped private paint. */
import {PathRasterJob,type PathRasterInput} from "../../🧮️geometry/📷️raster/🟦️.ts";
import type {WorkRetirement} from "@semio-tech/framework/2d/retire";
import {advanceCursor} from "../🎨️paint/🟦️.ts";
import {OwnedPixelImage} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🔲️pixels/✍️editing/📦️owned/🟦️.ts";
import {RetainedWorkRetirement,type RetainedWorkDemand,type RetainedWorkLedger,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {RetainedWorkProducerRuntime,type RetainedWorkProducerStep} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/📈️producer/🟦️.ts";
import {DrawingSceneWorkController,driveDrawingSceneWork} from "../🎟️work/🟦️.ts";
import type {PixelImage} from "@semio-tech/framework/pixels/editing";
import type {CompositeBlend,CompositeAffine} from "@semio-tech/framework/pixels/compositing";
export type RasterSceneGroup={readonly id:string;readonly opacity:number;readonly blendMode:CompositeBlend};
export type RasterSceneAsset={readonly id:string;readonly image:PixelImage};
export type RasterSceneContent=({readonly kind:"path"}&Pick<PathRasterInput,"segments"|"fillRule"|"fill"|"stroke">)|{readonly kind:"pixels";readonly image:PixelImage}|{readonly kind:"image";readonly asset:string;readonly width:number;readonly height:number};
export type RasterSceneNode={readonly id:string;readonly groups:readonly RasterSceneGroup[];readonly transform:CompositeAffine;readonly opacity:number;readonly blendMode:CompositeBlend;readonly visible:boolean;readonly content:RasterSceneContent};
export type RasterSceneInput={width:number;height:number;origin:readonly[number,number];tolerance:number;maxPixels:number;maxSourceBytes:number;assets:readonly RasterSceneAsset[];nodes:readonly RasterSceneNode[]};
export type RasterSceneProgress={phase:string;nodes:number;totalNodes:number;pixels:number;assets:number;totalAssets:number;sourceBytes:number;admittedImages:number;work:number;done:boolean};
export type RasterSceneOptions={signal?:AbortSignal;workBudget?:number;onProgress?:(progress:RasterSceneProgress)=>void};
import {CompositeJob,type CompositeLayer} from "@semio-tech/framework/pixels/compositing";
import {validateExtent,validateImage} from "@semio-tech/framework/pixels/editing";
import {arcGeometry} from "../../🧮️geometry/🟦️.ts";
import {AffineImageJob} from "@semio-tech/framework/pixels/sampling/affine";
const identity:CompositeAffine=[1,0,0,1,0,0];
const coordinate=(v:number)=>Number.isFinite(v)&&Math.abs(v)<=1e9;
function fail(message:string):never{throw new RangeError(message);}
const cancelled=():never=>{throw new DOMException("Scene raster cancelled","AbortError");};
const blends:readonly string[]=["normal","multiply","screen","overlay","darken","lighten","colorDodge","colorBurn","hardLight","softLight","difference","exclusion","hue","saturation","color","luminosity"];
const validStyle=(s:{opacity:number;blendMode:string})=>Number.isFinite(s.opacity)&&s.opacity>=0&&s.opacity<=1&&blends.includes(s.blendMode);
const validId=(id:string)=>typeof id==="string"&&id.length>0&&id.length<=4096&&new TextEncoder().encode(id).length<=4096;
type Scope={group:RasterSceneGroup;layers:CompositeLayer[]};
/** 🧱️ Cropped path candidates and admitted assets remain private until tiled scene completion. */
export class RasterSceneJob {
 private retiredChildren:RetainedWorkRetirementCursor[]=[];private retiredOld:WorkRetirement[]=[];private ownedImages:OwnedPixelImage[]=[];readonly #runtime=new RetainedWorkProducerRuntime();#demand:RetainedWorkDemand={copyBytes:0,capacityBytes:0,releaseBytes:0,depth:1};
 private readonly width:number;private readonly height:number;private readonly origin:readonly[number,number];private readonly tolerance:number;private readonly maxPixels:number;private readonly totalNodes:number;
 private readonly maxSourceBytes:number;private readonly totalAssets:number;
 private assetSource:readonly RasterSceneAsset[];private assetAt=0;private assetChar=0;private assetCurrent:RasterSceneAsset|null=null;private assets=0;private sourceBytes=0;private admittedImages=0;
 private catalog=new Map<string,RasterSceneAsset>();private admitted=new Map<string,PixelImage>();
 private source:readonly RasterSceneNode[];private at=0;private groupAt=0;private nodes=0;private pixels=0;private work=0;private entries=0;private phase="assets";private current:RasterSceneNode|null=null;
 private stack:Scope[]=[];private root:CompositeLayer[]=[];private closed=new Set<string>();private ids=new Set<string>();private images:Record<string,PixelImage>=Object.create(null);
 private segment=0;private from:[number,number]=[0,0];private start:[number,number]=[0,0];private contour=false;private bounds:[number,number,number,number]=[Infinity,Infinity,-Infinity,-Infinity];
 private crop:CompositeAffine=identity;private painter:PathRasterJob|null=null;private sampler:AffineImageJob|null=null;private compositor:CompositeJob|null=null;private output:PixelImage|null=null;private aborted=false;private failure:unknown=null;
 private painterRetirement:RetainedWorkRetirementCursor|null=null;private painted:PixelImage|null=null;
 private samplerRetirement:RetainedWorkRetirementCursor|null=null;private sampled:OwnedPixelImage|null=null;
 private compositorRetirement:WorkRetirement|null=null;private transferred=false;
 constructor(input:RasterSceneInput) {
  validateExtent(input.width,input.height);
  if(input.origin.length!==2||!input.origin.every(coordinate)||!Number.isFinite(input.tolerance)||input.tolerance<1e-6||input.tolerance>16||!Number.isSafeInteger(input.maxPixels)||input.maxPixels<1||input.maxPixels>67108864||input.nodes.length>1024||!Array.isArray(input.assets)||input.assets.length>1024||!Number.isSafeInteger(input.maxSourceBytes)||input.maxSourceBytes<1||input.maxSourceBytes>268439552)fail("Invalid scene raster contract");
  this.width=input.width;this.height=input.height;this.origin=[...input.origin];this.tolerance=input.tolerance;this.maxPixels=input.maxPixels;this.source=input.nodes;this.totalNodes=input.nodes.length;this.maxSourceBytes=input.maxSourceBytes;this.assetSource=input.assets;this.totalAssets=input.assets.length;
 }
 private layers():CompositeLayer[]{return this.stack.at(-1)?.layers??this.root;}
 private active():boolean{return this.current!.visible&&this.current!.opacity>0&&this.stack.every(s=>s.group.opacity>0);}
 private entry():void{if(++this.entries>1024)fail("Scene node count exceeds compositor budget");}
 private close():void{const scope=this.stack.pop()!;this.closed.add(scope.group.id);this.layers().push({kind:"group",children:scope.layers,opacity:scope.group.opacity,blend:scope.group.blendMode,visible:true,transform:identity,mask:null});}
 private reserve(count:number):void{if(this.pixels+count>this.maxPixels)fail("Scene aggregate pixel budget exceeded");this.pixels+=count;}
 private add(image:PixelImage,matrix:CompositeAffine):void{const n=this.current!,key=String(this.nodes);this.images[key]=image;this.layers().push({kind:"pixels",image:key,opacity:n.opacity,blend:n.blendMode,visible:true,transform:matrix,mask:null});this.current=null;this.nodes++;this.phase="nodes";}
 private point(p:readonly number[]):[number,number]{if(p.length!==2||!p.every(coordinate))fail("Invalid scene path coordinate");return [p[0]!,p[1]!];}
 private include(p:readonly[number,number]):void{if(!p.every(Number.isFinite))fail("Scene path bounds exceed numeric limits");this.bounds[0]=Math.min(this.bounds[0],p[0]);this.bounds[1]=Math.min(this.bounds[1],p[1]);this.bounds[2]=Math.max(this.bounds[2],p[0]);this.bounds[3]=Math.max(this.bounds[3],p[1]);}
 private map(p:readonly[number,number]):[number,number]{const m=this.current!.transform;return [m[0]*p[0]+m[2]*p[1]+m[4],m[1]*p[0]+m[3]*p[1]+m[5]];}
 private bound():void{
  const c=this.current!.content;if(c.kind!=="path")fail("Expected scene path");
  const s=c.segments[this.segment++];if(!s){this.paint(c);return;}
  if(s.kind==="move"){this.from=this.point(s.to);this.start=this.from;this.contour=true;this.include(this.map(this.from));return;}
  if(!this.contour)fail("Scene drawing segment requires a moveto");
  this.include(this.map(this.from));
  if(s.kind==="close"){this.from=this.start;this.include(this.map(this.from));return;}
  if(s.kind!=="line"&&s.kind!=="quad"&&s.kind!=="cubic"&&s.kind!=="arc")fail("Unknown scene path segment");
  const to=this.point(s.to);
  if(s.kind==="quad")this.include(this.map(this.point(s.ctrl)));
  else if(s.kind==="cubic"){this.include(this.map(this.point(s.ctrl1)));this.include(this.map(this.point(s.ctrl2)));}
  else if(s.kind==="arc"){
   if(![s.rx,s.ry,s.rotation].every(coordinate)||typeof s.largeArc!=="boolean"||typeof s.sweep!=="boolean")fail("Invalid scene arc");
   if(s.rx!==0&&s.ry!==0&&(this.from[0]!==to[0]||this.from[1]!==to[1])){
    const arc=arcGeometry(this.from,[Math.abs(s.rx),Math.abs(s.ry)],s.rotation%360,s.largeArc,s.sweep,to);
    if(arc){const m=this.current!.transform,[rx,ry]=arc.radii,a=Math.cos(arc.rotation),b=Math.sin(arc.rotation),center=this.map(arc.center),ex=Math.hypot(m[0]*rx*a+m[2]*rx*b,-m[0]*ry*b+m[2]*ry*a),ey=Math.hypot(m[1]*rx*a+m[3]*rx*b,-m[1]*ry*b+m[3]*ry*a);this.include([center[0]-ex,center[1]-ey]);this.include([center[0]+ex,center[1]+ey]);}
    else{this.include([this.origin[0],this.origin[1]]);this.include([this.origin[0]+this.width,this.origin[1]+this.height]);}
   }
  }
  this.from=to;this.include(this.map(to));
 }
 private paint(c:Extract<RasterSceneContent,{kind:"path"}>):void{
  const n=this.current!,m=n.transform;
  if(!this.active()||!c.fill&&!c.stroke||m[0]*m[3]-m[1]*m[2]===0||this.bounds[0]===Infinity){this.current=null;this.nodes++;this.phase="nodes";return;}
  const pad=(c.stroke?.width??0)*2,px=pad*Math.hypot(m[0],m[2]),py=pad*Math.hypot(m[1],m[3]);
  const left=Math.max(0,Math.floor(this.bounds[0]-px-this.origin[0])),top=Math.max(0,Math.floor(this.bounds[1]-py-this.origin[1])),right=Math.min(this.width,Math.ceil(this.bounds[2]+px-this.origin[0])),bottom=Math.min(this.height,Math.ceil(this.bounds[3]+py-this.origin[1]));
  if(right<=left||bottom<=top){this.current=null;this.nodes++;this.phase="nodes";return;}
  this.reserve((right-left)*(bottom-top));const origin:[number,number]=[this.origin[0]+left,this.origin[1]+top];this.crop=[1,0,0,1,...origin];
  this.painter=new PathRasterJob({width:right-left,height:bottom-top,origin,transform:m,tolerance:this.tolerance,segments:c.segments,fillRule:c.fillRule,fill:c.fill,stroke:c.stroke});this.phase="path";
 }
 private skip():void{this.current=null;this.nodes++;this.phase="nodes";}
 private assetStep():void{const asset=this.assetSource[this.assetAt++];if(!asset){this.assetSource=[];this.phase="nodes";return;}if(!validId(asset.id)||this.catalog.has(asset.id))fail("Invalid scene asset catalog");validateImage(asset.image);if(asset.image.pixels.length>this.maxSourceBytes-this.sourceBytes)fail("Scene aggregate sample byte budget exceeded");this.sourceBytes+=asset.image.pixels.length;this.catalog.set(asset.id,asset);this.assets++;}
 private imageCrop(width:number,height:number):[number,number,number,number]|null{
  const m=this.current!.transform;if(!this.active()||m[0]*m[3]-m[1]*m[2]===0)return null;
  const points=[[0,0],[width,0],[width,height],[0,height]].map(p=>this.map(p as [number,number]));
  if(!points.every(p=>p.every(Number.isFinite)))fail("Scene image bounds exceed numeric limits");
  const left=Math.max(0,Math.floor(Math.min(...points.map(p=>p[0]))-this.origin[0])),right=Math.min(this.width,Math.ceil(Math.max(...points.map(p=>p[0]))-this.origin[0])),top=Math.max(0,Math.floor(Math.min(...points.map(p=>p[1]))-this.origin[1])),bottom=Math.min(this.height,Math.ceil(Math.max(...points.map(p=>p[1]))-this.origin[1]));
  return right<=left||bottom<=top?null:[left,top,right,bottom];
 }
 private assetImage(c:Extract<RasterSceneContent,{kind:"image"}>,image:PixelImage):void{
  const n=this.current!,m=n.transform,sx=c.width/image.width,sy=c.height/image.height;
  this.current={...n,transform:[m[0]*sx,m[1]*sx,m[2]*sy,m[3]*sy,m[4],m[5]]};this.image(image,true);
 }
 private prepareImage(c:Extract<RasterSceneContent,{kind:"image"}>):void{
  if(!validId(c.asset)||!coordinate(c.width)||c.width<=0||!coordinate(c.height)||c.height<=0)fail("Invalid authored scene image");
  const cached=this.admitted.get(c.asset),asset=this.catalog.get(c.asset);if(!cached&&!asset)fail("Missing scene image asset: "+c.asset);
  if(!this.imageCrop(c.width,c.height)){this.skip();return;}
  if(cached){this.assetImage(c,cached);return;}
  const available=Math.min(16777216,this.maxPixels-this.pixels);if(available<1)fail("Scene aggregate pixel budget exceeded");
  const image=asset!.image,count=image.width*image.height;if(count>available)fail("Scene aggregate pixel budget exceeded");this.reserve(count);this.catalog.delete(c.asset);this.admitted.set(c.asset,image);this.admittedImages++;this.assetImage(c,image);
 }
 private image(image:PixelImage,charged=false):void{
  validateImage(image);const count=validateExtent(image.width,image.height),m=this.current!.transform,crop=this.imageCrop(image.width,image.height);
  if(!crop){this.skip();return;}const [left,top,right,bottom]=crop;if(!charged)this.reserve(count);
  if(m.slice(0,4).every(v=>v===-1||v===0||v===1)&&m[0]*m[0]+m[1]*m[1]===1&&m[2]*m[2]+m[3]*m[3]===1&&m[0]*m[2]+m[1]*m[3]===0&&Number.isInteger(m[4]-this.origin[0])&&Number.isInteger(m[5]-this.origin[1])){this.add(image,m);return;}
  this.reserve((right-left)*(bottom-top));const origin:[number,number]=[this.origin[0]+left,this.origin[1]+top];this.crop=[1,0,0,1,...origin];
  this.sampler=new AffineImageJob({source:image,width:right-left,height:bottom-top,origin,transform:m,sampling:"auto"});this.phase="image";
 }
 private unit():void{
  if(this.phase==="assets"){this.assetStep();return;}
  
  if(this.phase==="nodes"){
   if(!this.current){
    if(this.at===this.source.length){if(this.stack.length){this.close();return;}this.compositor=new CompositeJob({width:this.width,height:this.height,origin:this.origin,images:this.images,layers:this.root});this.phase="compositing";return;}
    const n=this.source[this.at++]!;
    if(!validId(n.id)||this.ids.has(n.id)||!validStyle(n)||typeof n.visible!=="boolean"||n.transform.length!==6||!n.transform.every(coordinate)||n.groups.length>32||!n.content||!["path","pixels","image"].includes(n.content.kind))fail("Invalid resolved scene node");
    this.ids.add(n.id);this.entry();this.current=n;this.groupAt=0;return;
   }
   const n=this.current,g=n.groups[this.groupAt],scope=this.stack[this.groupAt];
   if(g&&(!validId(g.id)||!validStyle(g)))fail("Invalid scene group");
   if(scope&&(!g||scope.group.id!==g.id)){this.close();return;}
   if(g){
    if(scope){if(scope.group.opacity!==g.opacity||scope.group.blendMode!==g.blendMode)fail("Inconsistent scene group style");}
    else{if(this.closed.has(g.id)||this.stack.some(s=>s.group.id===g.id))fail("Scene group scope cannot reopen");this.entry();this.stack.push({group:{...g},layers:[]});}
    this.groupAt++;return;
   }
   const c=n.content;
   if(c.kind==="pixels")this.image(c.image);
   else if(c.kind==="image")this.prepareImage(c);
   else{if(c.segments.length>65536)fail("Scene path exceeds segment budget");new PathRasterJob({width:1,height:1,origin:[0,0],transform:n.transform,tolerance:this.tolerance,segments:[],fillRule:c.fillRule,fill:c.fill,stroke:c.stroke});this.segment=0;this.from=[0,0];this.start=[0,0];this.contour=false;this.bounds=[Infinity,Infinity,-Infinity,-Infinity];this.phase="bounds";}
  }else if(this.phase==="bounds")this.bound();
  else if(this.phase==="pathCleanup"){this.retiredChildren.push(this.painterRetirement!);this.painterRetirement=null;{const image=this.painted!;this.painted=null;this.add(image,this.crop);}}
  else if(this.phase==="imageCleanup"){this.retiredChildren.push(this.samplerRetirement!);this.samplerRetirement=null;{const owned=this.sampled!;this.sampled=null;this.ownedImages.push(owned);this.add(owned.borrow(),this.crop);}}
  else if(this.phase==="compositingCleanup"){this.retiredOld.push(this.compositorRetirement!);this.compositorRetirement=null;{this.current=null;this.phase="complete";}}
 }
  private step(c:RetainedWorkProducerStep):void{
   if(this.phase==="path"){if(c.advanceChild(this.painter!).done){const retired=this.painter!.intoRetirement();this.painterRetirement=retired.job;this.painted=retired.output!;this.painter=null;this.phase="pathCleanup";}return;}
   if(this.phase==="image"){if(c.advanceChild(this.sampler!).done){const retired=this.sampler!.intoRetirement();this.samplerRetirement=retired.job;this.sampled=retired.output!;this.sampler=null;this.phase="imageCleanup";}return;}
   if(this.phase==="compositing"){if(this.compositor!.advance(1).done){const retired=this.compositor!.intoRetirement();this.compositorRetirement=retired.job;this.output=retired.output!;this.compositor=null;this.phase="compositingCleanup";}c.commitSemanticWork();return;}
   this.unit();c.commitSemanticWork();
  }
  lastTurnWork():number{return this.#runtime.lastTurnWork();}
  lastTurnSemanticWork():number{return this.#runtime.lastTurnSemanticWork();}
  nextWorkDemand():RetainedWorkDemand{return this.#demand;}
  failurePending():boolean{return this.failure!==null||this.painter?.failurePending()===true||this.sampler?.failurePending()===true;}
  settlementPending():boolean{return this.phase==="complete"||(this.phase==="path"&&this.painter?.settlementPending()===true)||(this.phase==="image"&&this.sampler?.settlementPending()===true);}
  progress():RasterSceneProgress{return {phase:this.phase,nodes:this.nodes,totalNodes:this.totalNodes,pixels:this.pixels,assets:this.assets,totalAssets:this.totalAssets,sourceBytes:this.sourceBytes,admittedImages:this.admittedImages,work:this.work,done:this.phase==="complete"};}
  advance(items:number,work:RetainedWorkLedger):RasterSceneProgress{
   try{return this.#runtime.advance(items,work,{maximumWork:Number.MAX_SAFE_INTEGER,workLimitMessage:"Scene raster work limit exceeded",validate:()=>{if(this.aborted||this.transferred)cancelled();if(this.failure!==null)throw this.failure;},stopped:()=>this.phase==="complete",settlementPending:()=>this.settlementPending(),failurePending:()=>this.failurePending(),semanticWork:()=>this.work,recordSemanticWork:n=>{this.work+=n;},step:c=>{try{this.step(c);}catch(error){this.failure=error;throw error;}finally{this.#demand=c.work.nextDemand;}},progress:()=>this.progress()});}
   catch(error){if(Number.isSafeInteger(items)&&items>=1&&this.failure===null&&!this.aborted)this.failure=error;throw error;}
  }
  /** 🧹️ Plain composed pixels carry no funded owner; the published image closes in one actual step. */
  retireOutput(_output:PixelImage):RetainedWorkRetirementCursor{let done=false;return new RetainedWorkRetirement((work,turn)=>{if(done)return true;if(!work.canStep())return false;done=true;turn.commitActualWork();return false;});}
 /** 🧹️ Adopts genuine paint, sampler, compositor and remaining scene owners before relinquishing the job. */
  intoRetirement():{job:RetainedWorkRetirementCursor;output:PixelImage|null}{
   if(this.transferred)throw Error("Raster scene ownership already transferred");this.transferred=true;const children=this.retiredChildren;this.retiredChildren=[];const olds=this.retiredOld;this.retiredOld=[];const images:PixelImage[]=[];const owned:RetainedWorkRetirementCursor[]=[];
   if(this.painter){const moved=this.painter.intoRetirement();children.push(moved.job);if(moved.output)images.push(moved.output);this.painter=null;}
   if(this.sampler){const moved=this.sampler.intoRetirement();children.push(moved.job);if(moved.output)owned.push(moved.output.intoRetirement());this.sampler=null;}
   if(this.compositor){const moved=this.compositor.intoRetirement();olds.push(moved.job);if(moved.output)images.push(moved.output);this.compositor=null;}
   for(const child of [this.painterRetirement,this.samplerRetirement])if(child)children.push(child);this.painterRetirement=null;this.samplerRetirement=null;if(this.compositorRetirement)olds.push(this.compositorRetirement);this.compositorRetirement=null;
   if(this.painted)images.push(this.painted);this.painted=null;if(this.sampled)owned.push(this.sampled.intoRetirement());this.sampled=null;for(const image of this.ownedImages)owned.push(image.intoRetirement());this.ownedImages=[];
   const output=this.phase==="complete"&&!this.aborted&&!this.failure?this.output:null;if(this.output&&!output)images.push(this.output);this.output=null;this.aborted=true;
   let source=this.source,assetSource=this.assetSource,current=this.current,assetCurrent=this.assetCurrent;this.source=[];this.assetSource=[];this.current=null;this.assetCurrent=null;let sourceAt=0,assetAt=0;
   const catalog=this.catalog,admitted=this.admitted,ids=this.ids,closed=this.closed;this.catalog=new Map();this.admitted=new Map();this.ids=new Set();this.closed=new Set();
   const layers=this.root;this.root=[];while(this.stack.length){const scope=this.stack.pop()!;layers.push({kind:"group",children:scope.layers,opacity:scope.group.opacity,blend:scope.group.blendMode,visible:true,transform:identity,mask:null});}
   const imageRecords=this.images;this.images=Object.create(null);const keys=Object.keys(imageRecords);let slot=0;
   const entry=(map:Map<unknown,unknown>|Set<unknown>)=>{const next=map.keys().next();if(!next.done){map.delete(next.value);return false;}return true;};
   return{output,job:new RetainedWorkRetirement((work,turn)=>{
    if(slot===0&&advanceCursor(children.at(-1)??owned.at(-1)??null,work,turn))return false;
    if(!work.canStep())return false;
    turn.commitActualWork();
    switch(slot){
     case 0:if(children.length){children.pop();return false;}if(owned.length){owned.pop();return false;}break;
     case 1:{const child=olds.at(-1);if(child){if(child.advance(1).done)olds.pop();return false;}break;}
     case 2:if(sourceAt<source.length){sourceAt++;return false;}source=[];break;
     case 3:if(assetAt<assetSource.length){assetAt++;return false;}assetSource=[];break;
     case 4:current=null;assetCurrent=null;break;
     case 5:if(!entry(catalog))return false;break;
     case 6:if(!entry(admitted))return false;break;
     case 7:if(!entry(ids))return false;break;
     case 8:if(!entry(closed))return false;break;
     case 9:{const layer=layers.pop();if(layer){if(layer.kind==="group")layers.push(...layer.children);return false;}break;}
     case 10:{const key=keys.pop();if(key!==undefined){delete imageRecords[key];return false;}break;}
     case 11:if(images.pop())return false;break;
     default:return current===null&&assetCurrent===null;
    }
    slot++;return false;
   })};
  }
 cancel():void{this.aborted=true;}
 result():PixelImage{if(this.aborted)cancelled();if(this.failure)throw this.failure;if(this.phase!=="complete")throw Error("Scene raster incomplete");return this.output!;}
}
/** ⏳️ Caller-funded turns publish only a fully composed candidate after its owned close. */
export async function rasterizeScene(input:RasterSceneInput,control:{work:DrawingSceneWorkController;operation:object;source:object;onProgress?:(progress:RasterSceneProgress)=>void}):Promise<PixelImage>{return driveDrawingSceneWork(control.work,control.operation,control.source,new RasterSceneJob(input),control.onProgress);}