/** 🖍️ Handwritten Draw layer forests, geometry, paint and logical sample entities. */
import sql from "./🗄️.sql" with {type:"text"};
import type {DrawingSnapshot} from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import type {DrawingLayerNode,DrawingLayerBase,DrawingColor,DrawingAttributes,DrawingFill,DrawingStroke,DrawingPathSegment,DrawingPoint} from "../../../🧬️schema/🟦️.ts";
import type {SqliteDatabase,SqliteValue,SqliteRow} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteInteger,artifactSqliteText,artifactSqliteBoolean,artifactSqliteOrderedRowsControlled,artifactSqliteCheckpoint,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import {encodeIeee754Cells,readBinary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary64,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {artifactSqliteOrderKeyEntries} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🔤️keys/🟦️.ts";
export const DRAW_SQLITE_SCHEMA:string=sql;
const object=(value:unknown):Record<string,unknown>=>{if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Draw record required");return value as Record<string,unknown>;};
const array=(value:unknown):readonly unknown[]=>{if(!Array.isArray(value))throw Error("Draw ordered array required");return value;};
const text=(value:unknown):string=>{if(typeof value!=="string")throw Error("Draw text required");return value;};
const bool=(value:unknown):bigint=>{if(typeof value!=="boolean")throw Error("Draw boolean required");return value?1n:0n;};
const u32=(value:unknown):bigint=>{if(typeof value!=="number"||!Number.isInteger(value)||value<0||value>0xffffffff)throw Error("Draw unsigned32 required");return BigInt(value);};
const member=(value:unknown,choices:readonly string[]):string=>{const result=text(value);if(!choices.includes(result))throw Error("Draw enum value outside declared domain");return result;};
/** 📤️ Project each actual layer variant without serializing a native document or reflective tree. */
export async function drawSnapshotToSqliteDatabase(s:DrawingSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(DRAW_SQLITE_SCHEMA,options),put=(name:string,cells:readonly SqliteValue[],id?:bigint)=>p.insert("draw_"+name,cells,id);
 const scalar=async(value:unknown)=>{return put("scalar",encodeIeee754Cells([1n,parseBinary64(value)],[{index:1,width:64}],options.maxColumns).slice(1));};
 const fields=async(name:string,id:bigint,value:unknown,keys:readonly string[])=>{const row=object(value),cells:SqliteValue[]=[];for(const key of keys)cells.push(await scalar(row[key]));await put(name,cells,id);};
 const vector=async(value:unknown,width:number):Promise<bigint[]>=>{const values=array(value);if(values.length!==width)throw Error("Draw vector width");p.checkRowsAdditional(width);const result:bigint[]=[];for(const item of values)result.push(await scalar(item));return result;};
 await put("document",[text(s.schema),text(s.id),s.title===undefined?null:text(s.title)],1n);
 if(s.artboard!==undefined)await fields("artboard",1n,s.artboard,["width","height"]);
 const assets=Object.entries(s.assets);p.checkRowsAdditional(assets.length);await artifactSqliteOrderKeyEntries(assets,options);
 for(let i=0;i<assets.length;i++){
  const[key,value]=assets[i]!,width=u32(value.width),height=u32(value.height),samples=array(value.samples);
  if(BigInt(samples.length)!==width*height)throw Error("Draw image sample count");
  p.checkRowsAdditional(samples.length);const asset=await put("asset",[1n,BigInt(i),key,width,height]);
  for(let ordinal=0;ordinal<samples.length;ordinal++){
   const sample=array(samples[ordinal]);if(sample.length!==4)throw Error("Draw RGBA width");
   const parts=sample.map(component=>{const v=u32(component);if(v>255n)throw Error("Draw sample component");return v;});
   await put("asset_sample",[asset,BigInt(ordinal),...parts]);
  }
 }
 const paint=async(id:bigint,value:unknown)=>{const attributes=object(value??{fillRule:"evenodd"});const fill=attributes.fill;if(fill!==undefined){const v=object(fill),kind=member(v.kind,["solid","linearGradient","radialGradient"]);await put("fill",[kind],id);if(kind==="solid")await put("fill_solid",await vector(v.color,4),id);else{await fields(kind==="linearGradient"?"fill_linear":"fill_radial",id,v,kind==="linearGradient"?["x1","y1","x2","y2"]:["cx","cy","r"]);const stops=array(v.stops);p.checkRowsAdditional(stops.length);for(let i=0;i<stops.length;i++){const stop=object(stops[i]);await put("gradient_stop",[id,BigInt(i),await scalar(stop.offset),...await vector(stop.color,4)]);}}}
  if(attributes.stroke!==undefined){const v=object(attributes.stroke);await put("stroke",[...await vector(v.color,4),await scalar(v.width),member(v.cap,["butt","round","square"]),member(v.join,["miter","round","bevel"]),bool(v.dash!==undefined)],id);if(v.dash!==undefined){const dash=array(v.dash);p.checkRowsAdditional(dash.length);for(let i=0;i<dash.length;i++)await put("stroke_dash",[id,BigInt(i),await scalar(dash[i])]);}}
 };
 type Pending={node:DrawingLayerNode;parent:bigint|null;ordinal:number};
 p.checkRowsAdditional(s.layers.length);const pending:Pending[]=[];for(let i=s.layers.length-1;i>=0;i--)pending.push({node:s.layers[i]!,parent:null,ordinal:i});
 while(pending.length){await p.checkpoint();const task=pending.pop()!,v=object(task.node),attributes=object(v.attributes??{fillRule:"evenodd"}),kind=member(v.kind,["shape","path","text","image","group","boolean","trace"]);const id=await put("layer",[task.parent===null?1n:null,task.parent,BigInt(task.ordinal),kind,text(v.id),text(v.name),bool(v.visible),bool(v.locked),await scalar(v.opacity),text(v.blendMode),member(attributes.fillRule??"evenodd",["evenodd","nonzero"])]);
  await fields("transform",id,v.transform,["x","y","scaleX","scaleY","shear","rotation"]);await paint(id,attributes);
  switch(kind){
   case"shape":{await put("shape",[text(v.shapeKind)],id);for(const[name,keys]of [["rect",["x","y","width","height"]],["ellipse",["cx","cy","rx","ry"]],["circle",["cx","cy","r"]],["line",["x1","y1","x2","y2"]]] as const)if(v[name]!==undefined)await fields(name,id,v[name],keys);if(v.polygon!==undefined){await put("polygon",[],id);const points=array(object(v.polygon).points);p.checkRowsAdditional(points.length);for(let i=0;i<points.length;i++)await put("polygon_point",[id,BigInt(i),...await vector(points[i],2)]);}break;}
   case"path":{await put("path",[],id);const segments=array(v.segments);p.checkRowsAdditional(segments.length);for(let i=0;i<segments.length;i++){const segment=object(segments[i]),kind=member(segment.kind,["move","line","quad","cubic","arc","close"]),key=await put("segment",[id,BigInt(i),kind]);if(kind!=="close")await put("segment_to",await vector(segment.to,2),key);switch(kind){case"quad":await put("segment_quad",await vector(segment.ctrl,2),key);break;case"cubic":await put("segment_cubic",[...await vector(segment.ctrl1,2),...await vector(segment.ctrl2,2)],key);break;case"arc":await put("segment_arc",[await scalar(segment.rx),await scalar(segment.ry),await scalar(segment.rotation),bool(segment.largeArc),bool(segment.sweep)],key);break;}}break;}
   case"text":await put("text",[await scalar(v.x),await scalar(v.y),text(v.content),await scalar(v.size)],id);break;
   case"image":await put("image",[text(v.imageKey),await scalar(v.width),await scalar(v.height)],id);break;
   case"group":{await put("group",[bool(v.isolation??false)],id);if(task.node.kind!=="group")throw Error("Draw group kind mismatch");const children=task.node.children;p.checkRowsAdditional(pending.length+children.length);for(let i=children.length-1;i>=0;i--)pending.push({node:children[i]!,parent:id,ordinal:i});break;}
   case"boolean":{await put("boolean",[text(v.operation)],id);const children=array(v.children);p.checkRowsAdditional(children.length);for(let i=0;i<children.length;i++)await put("boolean_child",[id,BigInt(i),text(children[i])]);break;}
   case"trace":{const params=object(v.params);await put("trace",[text(v.sourceKey),await scalar(params.threshold),await scalar(params.simplifyEpsilon)],id);break;}
  }
 }
 return p.finish();
}
/** 📥️ Reconstruct authored entities with contiguous owned edges and literal identity checks. */
export async function drawSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<DrawingSnapshot>{
 const rows=await artifactSqliteTables(database,DRAW_SQLITE_SCHEMA,options),names=["document","scalar","artboard","asset","asset_sample","layer","transform","shape","rect","ellipse","circle","line","polygon","polygon_point","path","segment","segment_to","segment_quad","segment_cubic","segment_arc","text","image","group","boolean","boolean_child","trace","fill","fill_solid","fill_linear","fill_radial","gradient_stop","stroke","stroke_dash"],widths=[4,4,3,6,7,12,7,2,5,5,4,5,1,5,1,4,3,3,5,6,5,4,2,2,4,4,2,5,5,4,8,9,4],tables=new Map<string,Map<bigint,SqliteRow>>();
 for(let i=0;i<names.length;i++){const map=new Map<bigint,SqliteRow>();for(let j=0;j<rows[i]!.length;j++){const row=rows[i]![j]!;if(row.rowid<=0n||row.values.length!==widths[i]||artifactSqliteInteger(row,0)!==row.rowid||map.has(row.rowid))throw Error("Draw row identity or fields");map.set(row.rowid,row);if(j%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",j,rows[i]!.length);}tables.set(names[i]!,map);}
 const table=(name:string)=>{const result=tables.get(name);if(result===undefined)throw Error("unknown Draw table");return result;},take=(name:string,id:bigint)=>{const map=table(name),row=map.get(id);if(row===undefined)throw Error("dangling or multiply owned Draw entity");map.delete(id);return row;},has=(name:string,id:bigint)=>table(name).has(id),integer=artifactSqliteInteger,string=artifactSqliteText,boolean=artifactSqliteBoolean;
 const scalar=(row:SqliteRow,index:number)=>readBinary64(take("scalar",integer(row,index)),1,[{index:1,width:64}]);
 const fields=<K extends string>(name:string,id:bigint,keys:readonly K[]):Record<K,Binary64>=>{const row=take(name,id),result={} as Record<K,Binary64>;for(let i=0;i<keys.length;i++)result[keys[i]!]=scalar(row,i+1);return result;};
 const color=(row:SqliteRow,start:number):DrawingColor=>[scalar(row,start),scalar(row,start+1),scalar(row,start+2),scalar(row,start+3)];
 const groups=new Map<string,Map<bigint,SqliteRow[]>>();
 const list=async(name:string,parent:bigint,foreign=1,ordinal=2):Promise<SqliteRow[]>=>{const key=name+":"+foreign;let indexed=groups.get(key);if(indexed===undefined){indexed=new Map();const source=table(name);let completed=0;for(const row of source.values()){if(row.values[foreign]!==null){const owner=integer(row,foreign),items=indexed.get(owner)??[];items.push(row);indexed.set(owner,items);}if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,source.size);}groups.set(key,indexed);}const values=await artifactSqliteOrderedRowsControlled(indexed.get(parent)??[],ordinal,options);indexed.delete(parent);for(const row of values)take(name,row.rowid);return values;};
 const paint=async(id:bigint,rule:string):Promise<DrawingAttributes>=>{const attributes:DrawingAttributes={fillRule:member(rule,["evenodd","nonzero"]) as DrawingAttributes["fillRule"]};if(has("fill",id)){const row=take("fill",id),kind=string(row,1);let fill:DrawingFill;switch(kind){case"solid":fill={kind,color:color(take("fill_solid",id),1)};break;case"linearGradient":fill={kind,...fields("fill_linear",id,["x1","y1","x2","y2"]),stops:[]};break;case"radialGradient":fill={kind,...fields("fill_radial",id,["cx","cy","r"]),stops:[]};break;default:throw Error("unknown Draw fill");}if(fill.kind!=="solid"){const stops:Extract<DrawingFill,{kind:"linearGradient"}>["stops"]=[];for(const row of await list("gradient_stop",id))stops.push({offset:scalar(row,3),color:color(row,4)});fill.stops=stops;}attributes.fill=fill;}
  if(has("stroke",id)){const row=take("stroke",id),stroke:DrawingStroke={color:color(row,1),width:scalar(row,5),cap:member(string(row,6),["butt","round","square"]) as DrawingStroke["cap"],join:member(string(row,7),["miter","round","bevel"]) as DrawingStroke["join"]};if(boolean(row,8)){const dash=[];for(const row of await list("stroke_dash",id))dash.push(scalar(row,3));stroke.dash=dash;}attributes.stroke=stroke;}return attributes;};
 const document=take("document",1n),result:DrawingSnapshot={schema:string(document,1),id:string(document,2),layers:[],assets:{}};if(document.values[3]!==null)result.title=string(document,3);if(has("artboard",1n)){const row=take("artboard",1n);result.artboard={width:scalar(row,1),height:scalar(row,2)};}
 for(const row of await list("asset",1n)){
  const key=string(row,3);if(Object.hasOwn(result.assets,key))throw Error("duplicate Draw asset key");
  const width=integer(row,4),height=integer(row,5);if(width<0n||width>0xffffffffn||height<0n||height>0xffffffffn)throw Error("Draw unsigned32 range");
  const samples:DrawingSnapshot['assets'][string]['samples']=[],source=await list("asset_sample",row.rowid);
  if(BigInt(source.length)!==width*height)throw Error("Draw image sample count");
  for(const sample of source){const parts=[];for(let index=3;index<7;index++){const value=integer(sample,index);if(value<0n||value>255n)throw Error("Draw sample component");parts.push(Number(value));}samples.push(parts as [number,number,number,number]);}
  Object.defineProperty(result.assets,key,{value:{width:Number(width),height:Number(height),samples},writable:true,enumerable:true,configurable:true});
 }
 type Pending={row:SqliteRow;output:DrawingLayerNode[]};const pending:Pending[]=[];for(const row of(await list("layer",1n,1,3)).reverse())pending.push({row,output:result.layers});
 while(pending.length){const {row,output}=pending.pop()!;if((row.values[1]===null)===(row.values[2]===null))throw Error("ambiguous Draw forest ownership");const id=row.rowid,kind=string(row,4),base:DrawingLayerBase={id:string(row,5),name:string(row,6),visible:boolean(row,7),locked:boolean(row,8),opacity:scalar(row,9),blendMode:string(row,10),transform:fields("transform",id,["x","y","scaleX","scaleY","shear","rotation"]),attributes:await paint(id,string(row,11))};
  let node:DrawingLayerNode;
  switch(kind){
   case"shape":{const shape:Extract<DrawingLayerNode,{kind:"shape"}>={...base,kind,shapeKind:string(take("shape",id),1)};if(has("rect",id))shape.rect=fields("rect",id,["x","y","width","height"]);if(has("ellipse",id))shape.ellipse=fields("ellipse",id,["cx","cy","rx","ry"]);if(has("circle",id))shape.circle=fields("circle",id,["cx","cy","r"]);if(has("line",id))shape.line=fields("line",id,["x1","y1","x2","y2"]);if(has("polygon",id)){take("polygon",id);const points:DrawingPoint[]=[];for(const row of await list("polygon_point",id))points.push([scalar(row,3),scalar(row,4)]);shape.polygon={points};}node=shape;break;}
   case"path":{take("path",id);const segments:DrawingPathSegment[]=[];for(const row of await list("segment",id)){const kind=string(row,3);if(kind==="close"){segments.push({kind});continue;}const to=take("segment_to",row.rowid),point:DrawingPoint=[scalar(to,1),scalar(to,2)];switch(kind){case"move":case"line":segments.push({kind,to:point});break;case"quad":{const v=take("segment_quad",row.rowid);segments.push({kind,ctrl:[scalar(v,1),scalar(v,2)],to:point});break;}case"cubic":{const v=take("segment_cubic",row.rowid);segments.push({kind,ctrl1:[scalar(v,1),scalar(v,2)],ctrl2:[scalar(v,3),scalar(v,4)],to:point});break;}case"arc":{const v=take("segment_arc",row.rowid);segments.push({kind,rx:scalar(v,1),ry:scalar(v,2),rotation:scalar(v,3),largeArc:boolean(v,4),sweep:boolean(v,5),to:point});break;}default:throw Error("unknown Draw segment");}}node={...base,kind,segments};break;}
   case"text":{const v=take("text",id);node={...base,kind,x:scalar(v,1),y:scalar(v,2),content:string(v,3),size:scalar(v,4)};break;}
   case"image":{const v=take("image",id);node={...base,kind,imageKey:string(v,1),width:scalar(v,2),height:scalar(v,3)};break;}
   case"group":{const children:DrawingLayerNode[]=[];node={...base,kind,isolation:boolean(take("group",id),1),children};for(const row of(await list("layer",id,2,3)).reverse())pending.push({row,output:children});break;}
   case"boolean":{const children:string[]=[];for(const row of await list("boolean_child",id))children.push(string(row,3));node={...base,kind,operation:string(take("boolean",id),1),children};break;}
   case"trace":{const v=take("trace",id);node={...base,kind,sourceKey:string(v,1),params:{threshold:scalar(v,2),simplifyEpsilon:scalar(v,3)}};break;}
   default:throw Error("unknown Draw layer");
  }
  output.push(node);await artifactSqliteCheckpoint(options,"reconstructSnapshot",0,0,false);
 }
 for(const map of tables.values())if(map.size)throw Error("orphan Draw entities");return result;
}
