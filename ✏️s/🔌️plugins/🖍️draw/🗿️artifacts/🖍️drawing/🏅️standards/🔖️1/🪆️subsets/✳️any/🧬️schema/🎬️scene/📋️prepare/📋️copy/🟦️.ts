/** 📋️ Incrementally own all mutable scene metadata while borrowing immutable source text. */
import type {DocumentSceneNode,DocumentSceneContent} from "../🟦️.ts";
import {validateSceneSourceAddress} from "../🟦️.ts";
import type {PathSegment,Vec2} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🟦️.ts";
import type {Fill,Color} from "../../../🎨️fill/🟦️.ts";
import type {PathRasterStroke} from "../../../🧮️geometry/📷️raster/🟦️.ts";
function invalid(message:string):never{throw new RangeError(message);}
function copyPoint(p:Vec2):Vec2{if(!Array.isArray(p)||p.length!==2)invalid("Invalid scene copy point");return[p[0],p[1]];}
function copy(s:PathSegment):PathSegment{if(!s||typeof s!=="object")invalid("Invalid scene copy segment");switch(s.kind){case"move":case"line":return{kind:s.kind,to:copyPoint(s.to)};case"quad":return{kind:s.kind,ctrl:copyPoint(s.ctrl),to:copyPoint(s.to)};case"cubic":return{kind:s.kind,ctrl1:copyPoint(s.ctrl1),ctrl2:copyPoint(s.ctrl2),to:copyPoint(s.to)};case"arc":return{kind:s.kind,rx:s.rx,ry:s.ry,rotation:s.rotation,largeArc:s.largeArc,sweep:s.sweep,to:copyPoint(s.to)};case"close":return{kind:s.kind};default:return invalid("Invalid scene copy segment");}}
function ownedFill(fill:Fill|null):Fill|null{
 if(!fill)return null;if(fill.kind==="solid")return{kind:"solid",color:[...fill.color]};
 return fill.kind==="linearGradient"?{kind:fill.kind,x1:fill.x1,y1:fill.y1,x2:fill.x2,y2:fill.y2,stops:[]}:{kind:fill.kind,cx:fill.cx,cy:fill.cy,r:fill.r,stops:[]};
}
function ownedStroke(stroke:PathRasterStroke|null):PathRasterStroke|null{return stroke?{color:[...stroke.color],width:stroke.width,cap:stroke.cap,join:stroke.join,...(stroke.dash!==undefined?{dash:stroke.dash?[]:null}:{})}:null;}
function ownedNode(n:DocumentSceneNode,segments:PathSegment[]|null):DocumentSceneNode{
 validateSceneSourceAddress(n);
 const c=n.content,paint="fill" in c?{fillRule:c.fillRule,fill:ownedFill(c.fill),stroke:ownedStroke(c.stroke)}:null;let content:DocumentSceneContent;
 switch(c.kind){
  case "path":content={kind:"path",segments:[],...paint!};break;
  case "glyphs":content={kind:"glyphs",content:c.content,x:c.x,y:c.y,size:c.size,fontFamily:c.fontFamily,segments:[],...paint!};break;
  case "boolean":content=segments?{kind:"path",segments,...paint!}:{kind:"boolean",operation:c.operation,children:[],referenceTransform:[...c.referenceTransform],...paint!};break;
  case "group":content={kind:"group",children:[],isolation:c.isolation};break;
  case "image":content={kind:"image",asset:c.asset,width:c.width,height:c.height};break;
  case "text":content={kind:"text",content:c.content,x:c.x,y:c.y,size:c.size,fontFamily:c.fontFamily,...paint!};break;
  case "trace":content=segments?{kind:"path",segments,...paint!}:{kind:"trace",source:c.source,threshold:c.threshold,simplifyEpsilon:c.simplifyEpsilon,...paint!};break;
 }
 return{sourcePath:n.sourcePath.slice(),lockedAncestors:n.lockedAncestors,id:n.id,transform:[...n.transform],visible:n.visible,opacity:n.opacity,blendMode:n.blendMode,groups:[],content};
}
/** 🧺️ Copies one group, paint stop, dash, segment or reference per step. */
export class SceneNodeCopyJob{
 private node:DocumentSceneNode|null;private part=0;private at=0;private done=false;
 constructor(private readonly source:DocumentSceneNode,segments:PathSegment[]|null=null){this.node=ownedNode(source,segments);}
 advanceOne():boolean{
  if(!this.node)invalid("Scene copy ownership already transferred");if(this.done)return true;const node=this.node!,c=this.source.content,d=node.content,at=this.at;
  switch(this.part){
   case 0:{const g=this.source.groups[at];if(g){node.groups.push({id:g.id,opacity:g.opacity,blendMode:g.blendMode});this.at++;return false;}break;}
   case 1:if("fill" in c&&"fill" in d&&c.fill&&d.fill&&c.fill.kind!=="solid"&&d.fill.kind!=="solid"){const stop=c.fill.stops[at];if(stop){d.fill.stops.push({offset:stop.offset,color:[...stop.color] as Color});this.at++;return false;}}break;
   case 2:if("stroke" in c&&"stroke" in d&&c.stroke?.dash&&d.stroke?.dash){const dash=c.stroke.dash[at];if(dash!==undefined){(d.stroke.dash as number[]).push(dash);this.at++;return false;}}break;
   case 3:if((c.kind==="path"||c.kind==="glyphs")&&(d.kind==="path"||d.kind==="glyphs")){const segment=c.segments[at];if(segment){d.segments.push(copy(segment));this.at++;return false;}}break;
   case 4:if((c.kind==="group"&&d.kind==="group")||(c.kind==="boolean"&&d.kind==="boolean")){const id=c.children[at];if(id!==undefined){d.children.push(id);this.at++;return false;}}break;
   case 5:this.done=true;return true;
  }this.part++;this.at=0;return false;
 }
 take():DocumentSceneNode{if(!this.done)invalid("Scene copy incomplete");return this.takePartial()!;}
 takePartial():DocumentSceneNode{if(!this.node)invalid("Scene copy ownership already transferred");const node=this.node;this.node=null;this.done=true;return node;}
}
