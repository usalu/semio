/** 📄️ Incrementally construct an editable SVG hierarchy with explicit unsupported-feature failures. */
import type {DrawingSnapshot} from "../../../../../../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {DrawingLayerNode,PathSegment} from "../../../../../../../../🧬️schema/🟦️.ts";
import {parseFillRule} from "../../../../../../../../🧬️schema/🎨️fill/🌀️rule/🟦️.ts";
import {drawingMatrixToTransform,drawingTransformToMatrix} from "../../../../../../../../🧬️schema/🧮️geometry/↗️affine/🟦️.ts";
import {multiply,segmentBounds,type Matrix,type Point} from "../../../../../../../../🧬️schema/🧮️geometry/🟦️.ts";
import {parseEditableSvgPath} from "../🛤️path/🟦️.ts";
import {parseEditableSvgTransform} from "../↗️transform/🟦️.ts";
type Style=Record<string,string>;
type Layer=DrawingLayerNode&{id:string;children?:Layer[]};
type Pending={element:Element;style:Style;id:string;parent:Layer[]};
export interface SvgImportProgress {readonly completed:number;readonly pending:number;readonly done:boolean}
const inherited=["fill","fill-rule","fill-opacity","stroke","stroke-width","stroke-opacity","stroke-linecap","stroke-linejoin","stroke-dasharray","visibility","font-size","color"];
const unsupported=["clip-path","mask","filter","vector-effect","stroke-dashoffset","font-family","font-weight","font-style","text-anchor"];
function scalar(value:string):number {if(!/^[+-]?(?:\d+\.?\d*|\.\d+)(?:[eE][+-]?\d+)?$/.test(value.trim())||!Number.isFinite(+value))throw new Error(`Invalid SVG number: ${value}`);return +value;}
function length(value:string|undefined,fallback=0):number {if(value===undefined)return fallback;const match=/^(.*?)(px|pt|pc|mm|cm|in)?$/.exec(value.trim())!;return scalar(match[1]!)*({px:1,pt:96/72,pc:16,mm:96/25.4,cm:96/2.54,in:96}[match[2] as "px"]??1);}
function nonnegative(value:number):number {if(!Number.isFinite(value)||value<0)throw new Error("Invalid SVG dimension");return value;}
function fraction(value:string|undefined):number {return Math.max(0,Math.min(1,value===undefined?1:scalar(value)));}
function color(source:string,alpha:number):[number,number,number,number] {
  const names:Record<string,string>={black:"000000",white:"ffffff",red:"ff0000",green:"008000",blue:"0000ff",yellow:"ffff00",gray:"808080",grey:"808080",silver:"c0c0c0",maroon:"800000",purple:"800080",fuchsia:"ff00ff",lime:"00ff00",olive:"808000",navy:"000080",teal:"008080",aqua:"00ffff",orange:"ffa500"};
  if(source==="transparent")return [0,0,0,0];
  let hex=source.startsWith("#")?source.slice(1):names[source.toLowerCase()];
  if(hex&&/^[0-9a-f]+$/i.test(hex)&&[3,4,6,8].includes(hex.length)){
    if(hex.length<5)hex=[...hex].map(v=>v+v).join("");
    return [parseInt(hex.slice(0,2),16)/255,parseInt(hex.slice(2,4),16)/255,parseInt(hex.slice(4,6),16)/255,alpha*(hex.length===8?parseInt(hex.slice(6,8),16)/255:1)];
  }
  const rgb=/^rgb\(([^)]+)\)$/.exec(source);
  if(rgb){const values=rgb[1]!.split(",");if(values.length===3){const channels=values.map(v=>Math.max(0,Math.min(1,v.trim().endsWith("%")?scalar(v.trim().slice(0,-1))/100:scalar(v)/255)));return [channels[0]!,channels[1]!,channels[2]!,alpha];}}
  throw new Error(`Unsupported SVG paint: ${source}`);
}
function properties(element:Element,parent:Style):Style {
  const style:Style={};for(const key of inherited)if(parent[key]!==undefined)style[key]=parent[key]!;
  for(const key of [...inherited,...unsupported,"opacity","display","mix-blend-mode","isolation"])if(element.hasAttribute(key))style[key]=element.getAttribute(key)!;
  for(const declaration of (element.getAttribute("style")??"").split(";")){if(!declaration.trim())continue;const split=declaration.indexOf(":");if(split<1)throw new Error("Invalid SVG style declaration");const key=declaration.slice(0,split).trim(),value=declaration.slice(split+1).trim();if(![...inherited,...unsupported,"opacity","display","mix-blend-mode","isolation"].includes(key))throw new Error(`Unsupported SVG style: ${key}`);style[key]=value;}
  for(const key of Object.keys(style))if(style[key]==="inherit"){if(parent[key]===undefined)delete style[key];else style[key]=parent[key]!;}
  for(const key of unsupported)if(style[key]!==undefined&&style[key]!=="none")throw new Error(`Unsupported SVG property: ${key}`);
  return style;
}
function blendMode(value="normal"):string {
  const modes:Record<string,string>={normal:"normal",multiply:"multiply",screen:"screen",overlay:"overlay",darken:"darken",lighten:"lighten","color-dodge":"colorDodge","color-burn":"colorBurn","hard-light":"hardLight","soft-light":"softLight",difference:"difference",exclusion:"exclusion",hue:"hue",saturation:"saturation",color:"color",luminosity:"luminosity"};
  if(!Object.hasOwn(modes,value))throw new Error(`Unsupported SVG blend mode: ${value}`);
  return modes[value]!;
}
function isolation(value:string|undefined):boolean {
  if(value!==undefined&&value!=="auto"&&value!=="isolate")throw new Error("Invalid SVG isolation");
  return value==="isolate";
}
function attributes(style:Style){
  const result:Record<string,unknown>={fillRule:parseFillRule(style["fill-rule"]??"nonzero")};
  const resolve=(paint:string)=>paint==="currentColor"?style.color??"black":paint;
  const fill=style.fill??"black";if(fill!=="none")result.fill={kind:"solid",color:color(resolve(fill),fraction(style["fill-opacity"]))};
  if(style.stroke&&style.stroke!=="none"){
    const cap=style["stroke-linecap"]??"butt",join=style["stroke-linejoin"]??"miter";
    if(!["butt","round","square"].includes(cap)||!["miter","round","bevel"].includes(join))throw new Error("Invalid SVG stroke ending");
    const dash=style["stroke-dasharray"]&&style["stroke-dasharray"]!=="none"?style["stroke-dasharray"]!.trim().split(/[ ,]+/).map(v=>nonnegative(length(v))):undefined;
    result.stroke={color:color(resolve(style.stroke),fraction(style["stroke-opacity"])),width:nonnegative(length(style["stroke-width"],1)),cap,join,...(dash?{dash}: {})};
  }
  return result;
}
function geometry(element:Element):PathSegment[] {
  const tag=element.localName,n=(key:string,fallback=0)=>length(element.hasAttribute(key)?element.getAttribute(key)!:undefined,fallback);
  const m=(x:number,y:number):PathSegment=>({kind:"move",to:[x,y]}),l=(x:number,y:number):PathSegment=>({kind:"line",to:[x,y]}),close:PathSegment={kind:"close"};
  if(tag==="path")return parseEditableSvgPath(element.getAttribute("d")??"");
  if(tag==="line")return [m(n("x1"),n("y1")),l(n("x2"),n("y2"))];
  if(tag==="rect"){
    const x=n("x"),y=n("y"),w=nonnegative(n("width")),h=nonnegative(n("height"));
    if(!w||!h)return [];
    const rx=Math.min(w/2,nonnegative(n("rx",n("ry")))),ry=Math.min(h/2,nonnegative(n("ry",n("rx"))));
    if(!rx||!ry)return [m(x,y),l(x+w,y),l(x+w,y+h),l(x,y+h),close];
    const a=(x:number,y:number):PathSegment=>({kind:"arc",rx,ry,rotation:0,largeArc:false,sweep:true,to:[x,y]});
    return [m(x+rx,y),l(x+w-rx,y),a(x+w,y+ry),l(x+w,y+h-ry),a(x+w-rx,y+h),l(x+rx,y+h),a(x,y+h-ry),l(x,y+ry),a(x+rx,y),close];
  }
  if(tag==="circle"||tag==="ellipse"){
    const x=n("cx"),y=n("cy"),rx=nonnegative(n(tag==="circle"?"r":"rx")),ry=nonnegative(n(tag==="circle"?"r":"ry"));
    if(!rx||!ry)return [];
    const a=(x:number):PathSegment=>({kind:"arc",rx,ry,rotation:0,largeArc:false,sweep:true,to:[x,y]});return [m(x+rx,y),a(x-rx),a(x+rx),close];
  }
  if(tag==="polyline"||tag==="polygon"){
    const source=(element.getAttribute("points")??"").trim();if(!source)return [];
    const points=source.split(/[ ,\t\r\n]+/).map(scalar);if(points.length%2)throw new Error("SVG points need coordinate pairs");
    const segments:PathSegment[]=[];for(let i=0;i<points.length;i+=2)segments.push(i?l(points[i]!,points[i+1]!):m(points[i]!,points[i+1]!));if(tag==="polygon")segments.push(close);return segments;
  }
  throw new Error(`Unsupported SVG element: ${tag}`);
}
function viewport(root:Element):{width:number;height:number;matrix:Matrix}{
  const box=root.hasAttribute("viewBox")?root.getAttribute("viewBox")!.trim().split(/[ ,\t\r\n]+/).map(scalar):undefined;
  if(box&&(box.length!==4||box[2]!<=0||box[3]!<=0))throw new Error("SVG viewBox needs positive width and height");
  const width=nonnegative(length(root.hasAttribute("width")?root.getAttribute("width")!:undefined,box?.[2]??300)),height=nonnegative(length(root.hasAttribute("height")?root.getAttribute("height")!:undefined,box?.[3]??150));
  if(!box)return {width,height,matrix:[1,0,0,1,0,0]};
  let sx=width/box[2]!,sy=height/box[3]!,dx=0,dy=0;
  const [align="xMidYMid",mode="meet",extra]=(root.getAttribute("preserveAspectRatio")??"xMidYMid meet").trim().split(/\s+/);
  if(extra||!["meet","slice"].includes(mode)||!(align==="none"||/^x(Min|Mid|Max)Y(Min|Mid|Max)$/.test(align)))throw new Error("Invalid SVG aspect ratio");
  if(align!=="none"){sx=sy=mode==="slice"?Math.max(sx,sy):Math.min(sx,sy);const part=(value:string)=>value==="Min"?0:value==="Mid"?.5:1;dx=(width-box[2]!*sx)*part(align.slice(1,4));dy=(height-box[3]!*sy)*part(align.slice(5));}
  return {width,height,matrix:[sx,0,0,sy,dx-box[0]!*sx,dy-box[1]!*sy]};
}
function gradientFill(reference:string,definitions:Map<string,Element>,segments:PathSegment[],viewport:[number,number],alpha:number){
  const match=/^url\(\s*["']?#([^"'()\s]+)["']?\s*\)$/.exec(reference);if(!match)throw new Error("SVG paint needs a local gradient reference");
  let element=definitions.get(match[1]!);if(!element)throw new Error("Missing SVG gradient");
  if(element.localName!=="linearGradient")throw new Error("Radial SVG gradient import requires an authored gradient coordinate system");
  const attrs:Record<string,string>={},seen=new Set<Element>();let sourceStops:Element[]=[];
  while(element){
    if(seen.has(element))throw new Error("Cyclic SVG gradient reference");seen.add(element);
    let interpolation=element.getAttribute("color-interpolation")||"sRGB";
    for(const declaration of (element.getAttribute("style")??"").split(";")){const split=declaration.indexOf(":");if(declaration.slice(0,split).trim()==="color-interpolation")interpolation=declaration.slice(split+1).trim();}
    if(interpolation!=="sRGB"&&interpolation!=="auto")throw new Error("Unsupported SVG gradient color interpolation");
    for(const attr of Array.from(element.attributes))if(attrs[attr.name]===undefined)attrs[attr.name]=attr.value;
    if(!sourceStops.length)sourceStops=Array.from(element.childNodes).filter(n=>n.nodeType===1&&(n as Element).localName==="stop") as Element[];
    const href=element.getAttribute("href")||element.getAttribute("xlink:href");if(!href)break;
    if(!href.startsWith("#"))throw new Error("SVG gradient references must be local");element=definitions.get(href.slice(1));if(!element)throw new Error("Missing SVG gradient reference");
  }
  if(attrs.spreadMethod&&attrs.spreadMethod!=="pad")throw new Error("Unsupported SVG gradient spread");
  let last=0;
  const stops=sourceStops.map(stop=>{
    const style:Record<string,string>={};for(const declaration of (stop.getAttribute("style")??"").split(";")){const index=declaration.indexOf(":");if(index>0)style[declaration.slice(0,index).trim()]=declaration.slice(index+1).trim();}
    const offset=stop.getAttribute("offset")||"0",number=offset.endsWith("%")?scalar(offset.slice(0,-1))/100:scalar(offset);last=Math.max(last,Math.min(1,Math.max(0,number)));
    return {offset:last,color:color(style["stop-color"]||stop.getAttribute("stop-color")||"black",alpha*fraction(style["stop-opacity"]||stop.getAttribute("stop-opacity")||undefined))};
  });
  if(!stops.length)return undefined;if(stops.length===1)return {kind:"solid",color:stops[0]!.color};
  const unit=attrs.gradientUnits??"objectBoundingBox";if(!["objectBoundingBox","userSpaceOnUse"].includes(unit))throw new Error("Invalid SVG gradient units");
  let current:Point=[0,0],start:Point=[0,0],min=[Infinity,Infinity],max=[-Infinity,-Infinity];
  for(const segment of segments){
    if(segment.kind==="move"){current=segment.to;start=current;continue;}
    const bounds=segmentBounds(segment,current,start,[1,0,0,1,0,0]);for(let axis=0;axis<2;axis++){min[axis]=Math.min(min[axis]!,bounds[axis]!);max[axis]=Math.max(max[axis]!,bounds[axis]!+bounds[axis+2]!);}current=segment.kind==="close"?start:segment.to;
  }
  let matrix=drawingTransformToMatrix(parseEditableSvgTransform(attrs.gradientTransform??""));
  if(unit==="objectBoundingBox"){const w=max[0]!-min[0]!,h=max[1]!-min[1]!;if(!(w>0&&h>0))return undefined;matrix=multiply([w,0,0,h,min[0]!,min[1]!],matrix);}
  const coordinate=(name:string,fallback:string,axis:0|1)=>{const source=attrs[name]??fallback;return source.endsWith("%")?scalar(source.slice(0,-1))/100*(unit==="objectBoundingBox"?1:viewport[axis]):unit==="objectBoundingBox"?scalar(source):length(source);};
  const x=coordinate("x1","0%",0),y=coordinate("y1","0%",1),vx=coordinate("x2","100%",0)-x,vy=coordinate("y2","0%",1)-y;
  if(vx===0&&vy===0)return {kind:"solid",color:stops.at(-1)!.color};
  const [a,b,c,d,e,f]=matrix,det=a*d-b*c,len=vx*vx+vy*vy;if(det===0)throw new Error("Singular SVG gradient transform");
  const nx=(d*vx-b*vy)/det/len,ny=(-c*vx+a*vy)/det/len,norm=nx*nx+ny*ny,x1=a*x+c*y+e,y1=b*x+d*y+f,x2=x1+nx/norm,y2=y1+ny/norm;
  if(![x1,y1,x2,y2].every(Number.isFinite))throw new Error("SVG gradient exceeds finite coordinates");
  return {kind:"linearGradient",x1,y1,x2,y2,stops};
}
export class SvgImportJob {
  private gradients=new Map<string,Element>();private userViewport:[number,number];
  private pending:Pending[];private completed=0;private cancelled=false;private failed=false;private document:DrawingSnapshot|undefined;
  constructor(source:string,id:string){
    if(!id)throw new Error("SVG import needs a document id");
    const xml=new DOMParser().parseFromString(source,"image/svg+xml"),root=xml.documentElement;
    if(!root||root.localName!=="svg"||xml.getElementsByTagName("parsererror").length)throw new Error("Invalid SVG document");
    const frame=viewport(root),title=Array.from(root.childNodes).find(n=>n.nodeType===1&&(n as Element).localName==="title")?.textContent?.trim()||"SVG";
    this.document={schema:"drawing.document",id,title,layers:[],assets:{},artboard:{width:frame.width,height:frame.height}};
    const box=root.getAttribute("viewBox");this.userViewport=box?box.trim().split(/[ ,\t\r\n]+/).map(scalar).slice(2) as [number,number]:[frame.width,frame.height];
    for(const node of Array.from(root.getElementsByTagName("*")))if(["linearGradient","radialGradient"].includes(node.localName)&&node.hasAttribute("id")){const id=node.getAttribute("id")!;if(this.gradients.has(id))throw new Error("Duplicate SVG gradient id");this.gradients.set(id,node);}
    this.pending=[{element:root,id:"svg",style:{},parent:this.document.layers as Layer[]}];
  }
  cancel(){this.cancelled=true;this.pending=[];this.gradients.clear();this.document=undefined;}
  step(budget=64):SvgImportProgress {
    if(this.cancelled||this.failed||!this.document)throw new Error("SVG import is not available");
    if(!Number.isSafeInteger(budget)||budget<=0)throw new Error("SVG import budget must be positive");
    try {for(let i=0;i<budget&&this.pending.length;i++){this.visit(this.pending.pop()!);this.completed++;}}
    catch(error){this.failed=true;this.pending=[];this.gradients.clear();this.document=undefined;throw error;}
    return {completed:this.completed,pending:this.pending.length,done:this.pending.length===0};
  }
  take():DrawingSnapshot {if(this.cancelled||this.failed||this.pending.length||!this.document)throw new Error("SVG import is incomplete");const document=this.document;this.document=undefined;this.gradients.clear();return document;}
  private visit(work:Pending){
    const {element,id,parent}=work,tag=element.localName;
    if(["title","desc","metadata","defs","linearGradient","radialGradient","stop"].includes(tag))return;
    const style=properties(element,work.style),name=element.getAttribute("inkscape:label")||element.getAttribute("id")||(id==="svg"?this.document!.title!:tag);
    let transform=parseEditableSvgTransform(element.getAttribute("transform")??"");
    if(id==="svg")transform=drawingMatrixToTransform(multiply(drawingTransformToMatrix(transform),viewport(element).matrix));
    const base={id,name,visible:style.display!=="none"&&((tag==="g"||tag==="svg")||!["hidden","collapse"].includes(style.visibility??"visible")),locked:false,opacity:fraction(style.opacity),blendMode:blendMode(style["mix-blend-mode"]),transform,attributes:attributes(style.fill?.startsWith("url(")?{...style,fill:"none"}:style)};
    const isolated=isolation(style.isolation);
    let layer:Layer;
    if(tag==="g"||id==="svg"){
      layer={kind:"group",...base,...(isolated?{isolation:true}:{}),children:[]};const children=Array.from(element.childNodes).filter(n=>n.nodeType===1) as Element[];
      for(let i=children.length-1;i>=0;i--)this.pending.push({element:children[i]!,style,id:`${id}.${i}`,parent:layer.children!});
    }else if(tag==="text"){
      if(Array.from(element.childNodes).some(n=>n.nodeType===1))throw new Error("Positioned SVG text spans are not supported yet");
      const size=nonnegative(length(style["font-size"],16));layer={kind:"text",...base,x:length(element.hasAttribute("x")?element.getAttribute("x")!:undefined),y:length(element.hasAttribute("y")?element.getAttribute("y")!:undefined)-size,content:element.textContent??"",size};
    }else layer={kind:"path",...base,segments:geometry(element)};
    if(style.fill?.startsWith("url(")&&layer.kind!=="group"){
      if(layer.kind!=="path")throw new Error("SVG text gradients need exact text bounds");
      const fill=gradientFill(style.fill,this.gradients,layer.segments as PathSegment[],this.userViewport,fraction(style["fill-opacity"]));if(fill)base.attributes.fill=fill;
    }
    parent.push(layer);
  }
}
