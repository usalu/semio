/** 👁️ Complete scene records borrow geometry and preserve semantic text and images. */
import type {DocumentScenePlan,DocumentSceneNode} from "../📋️prepare/🟦️.ts";
import {sceneSelectionRelation} from "../📋️prepare/🟦️.ts";
import type {RasterSceneGroup} from "../📷️raster/🟦️.ts";
import type {PathSegment} from "../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";
import type {Fill} from "../../🎨️fill/🟦️.ts";
import type {PathRasterStroke} from "../../🧮️geometry/📷️raster/🟦️.ts";
import {multiply,segmentBounds,type Matrix,type Point} from "../../🧮️geometry/🟦️.ts";
export interface PreparedSceneNode{readonly id:string;readonly groups:readonly RasterSceneGroup[];readonly transform:Matrix;readonly segments:readonly PathSegment[];readonly opacity:number;readonly blendMode:DocumentSceneNode["blendMode"];readonly visible:boolean;readonly fillRule?:"nonzero"|"evenodd";readonly fill?:Fill;readonly stroke?:PathRasterStroke;readonly image?:{readonly assetId:string;readonly width:number;readonly height:number};}
/** 🎯️ Group selection includes all rendered descendants, with authored identity preserved. */
export function preparedSceneSelection(plan:DocumentScenePlan,ids:readonly string[]):ReadonlySet<string>{const index=new Map(plan.nodes.map(node=>[node.id,node])),selected=new Set<string>(),pending=[...ids];while(pending.length){const id=pending.pop()!;if(selected.has(id))continue;selected.add(id);const node=index.get(id);if(node?.content.kind==="group")pending.push(...node.content.children);}return selected;}
/** 🎯️ Resolve membership through the actual prepared group graph. */
export function preparedSceneSelected(plan:DocumentScenePlan,ids:readonly string[],id:string):boolean{return preparedSceneSelection(plan,ids).has(id);}
/** 🎬️ Borrow paths and paints; complete plans contain no unresolved algorithm nodes. */
export function preparedSceneNodes(plan:DocumentScenePlan,transformation?:readonly[readonly string[],Matrix]):PreparedSceneNode[]{const out:PreparedSceneNode[]=[],selected=transformation?preparedSceneSelection(plan,transformation[0]):null;for(const node of plan.nodes){const content=node.content;if(content.kind==="group")continue;if(content.kind==="boolean"||content.kind==="trace"||content.kind==="text")throw Error("Unresolved algorithm in completed canvas plan");let transform:Matrix=[...node.transform];if(transformation&&selected!.has(node.id))transform=multiply(transformation[1],transform);const common={id:node.id,groups:node.groups,transform,segments:content.kind==="path"||content.kind==="glyphs"?content.segments:[],opacity:node.opacity,blendMode:node.blendMode,visible:node.visible};if(content.kind==="image"){const asset=plan.assets.find(asset=>asset.id===content.asset);if(!asset)throw Error("Missing completed scene image asset");out.push({...common,image:{assetId:asset.id,width:content.width,height:content.height}});}else out.push({...common,fillRule:content.fillRule,...(content.fill?{fill:content.fill}:{}),...(content.stroke?{stroke:content.stroke}:{})});}return out;}

export type PreparedSceneBounds=[number,number,number,number];
/** 🪢️ Visible descendants contribute handles only through an unlocked selected prefix. */
export function preparedSceneSelectionBounds(plan:DocumentScenePlan,ids:readonly string[]):PreparedSceneBounds|null{
 if(ids.length>256)throw Error("Scene selection exceeds ancestry capacity");
 const index=new Map(plan.nodes.map(node=>[node.id,node])),selected=ids.flatMap(id=>{const node=index.get(id);return node?[node.sourcePath]:[];});
 let bounds:PreparedSceneBounds|null=null;
 for(const node of preparedSceneNodes(plan)){
  if(!node.visible||node.opacity<=0||node.groups.some(group=>group.opacity<=0))continue;
  const source=index.get(node.id)!;
  if(sceneSelectionRelation(source.sourcePath,source.lockedAncestors,selected).boundsSelection===null)continue;
  const next=preparedSceneNodeBounds(node);if(next)bounds=preparedSceneUnion(bounds,next);
 }
 return bounds;
}
/** 📐️ World extrema include affine stroke envelopes and explicit semantic extent fallbacks. */
export function preparedSceneNodeBounds(node:PreparedSceneNode):PreparedSceneBounds|null{
 const extent=node.image?[node.image.width,node.image.height]:null;
 const segments:readonly PathSegment[]=extent?[{kind:"move",to:[0,0]},{kind:"line",to:[extent[0]!,0]},{kind:"line",to:[extent[0]!,extent[1]!]},{kind:"line",to:[0,extent[1]!]},{kind:"close"}]:node.segments;
 let bounds:PreparedSceneBounds|null=null,current:Readonly<Point>=[0,0],start:Readonly<Point>=current;
 for(const segment of segments){if(segment.kind==="close"&&!bounds)continue;const[x,y,w,h]=segmentBounds(segment,current,start,node.transform);bounds=preparedSceneUnion(bounds,[x,y,x+w,y+h]);if(segment.kind==="move"){current=segment.to;start=current;}else if(segment.kind==="close")current=start;else current=segment.to;}
 if(!bounds||!bounds.every(Number.isFinite))return null;
 const radius=(node.stroke?.width??0)*.5,dx=radius*Math.hypot(node.transform[0],node.transform[2]),dy=radius*Math.hypot(node.transform[1],node.transform[3]);return[bounds[0]-dx,bounds[1]-dy,bounds[2]+dx,bounds[3]+dy];
}
/** 📷️ Framing unions actual visible geometry with the authored artboard. */
export function preparedSceneBounds(artboard:{readonly width:number;readonly height:number}|null,nodes:readonly PreparedSceneNode[]):PreparedSceneBounds{
 let bounds:PreparedSceneBounds|null=artboard&&artboard.width>0&&artboard.height>0?[0,0,artboard.width,artboard.height]:null;
 for(const node of nodes){if(!node.visible||node.opacity<=0||node.groups.some(group=>group.opacity<=0))continue;const next=preparedSceneNodeBounds(node);if(next)bounds=preparedSceneUnion(bounds,next);}return bounds??[0,0,1024,1024];
}
/** 🧩️ Retain extrema without changing the caller's rectangle. */
export function preparedSceneUnion(old:PreparedSceneBounds|null,next:PreparedSceneBounds):PreparedSceneBounds{return old?[Math.min(old[0],next[0]),Math.min(old[1],next[1]),Math.max(old[2],next[2]),Math.max(old[3],next[3])]:[...next];}
