/// <reference path="./🗄️.d.ts" />
import sql from"./🗄️.sql" with{type:"text"};
import type{Puzzle2dSnapshot,Puzzle2dHandle,Puzzle2dNode,Puzzle2dEdge,Puzzle2dTargetRegion,Puzzle2dHandleTemplate,Puzzle2dCatalogNodeKind,Puzzle2dKindCatalogs,Puzzle2dMeta}from"../../../🧬️schema/📸️snapshot/🟦️.ts";
import {encodeIeee754Cells,readBinary64,ieee754IsNull,type Ieee754Column,type Ieee754Cell} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {parseBinary64,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import{ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteInteger,artifactSqliteText,artifactSqliteBoolean,artifactSqliteDocument,artifactSqliteOrderedRowsControlled,artifactSqliteTextBytes,artifactSqliteValueBudget,type ArtifactSqliteOptions}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type{SqliteDatabase,SqliteRow}from"../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
/** 🔘️ Literal node port scalars retain their complete binary64 identity. */
export type Puzzle2dSqliteHandle=Omit<Puzzle2dHandle,"angle"|"radius"|"scale">&{angle:Binary64;radius?:Binary64;scale?:Binary64};
/** 🔵️ Board nodes include their ordered ports and every optional scalar. */
export type Puzzle2dSqliteNode=Omit<Puzzle2dNode,"x"|"y"|"radius"|"width"|"height"|"scale"|"handles">&{x:Binary64;y:Binary64;radius?:Binary64;width?:Binary64;height?:Binary64;scale?:Binary64;handles:Puzzle2dSqliteHandle[]};
/** ➡️ Directed connections retain all eight independently editable native numbers. */
export type Puzzle2dSqliteEdge=Omit<Puzzle2dEdge,"gap"|"shift"|"rise"|"rotation"|"turn"|"tilt"|"x"|"y">&{gap:Binary64;shift:Binary64;rise:Binary64;rotation:Binary64;turn:Binary64;tilt:Binary64;x:Binary64;y:Binary64};
/** 🎯️ Board bounds preserve their authored extents without normalization. */
export type Puzzle2dSqliteRegion=Omit<Puzzle2dTargetRegion,"x"|"y"|"width"|"height">&{x:Binary64;y:Binary64;width:Binary64;height:Binary64};
/** 🌱️ Ordered catalog ports retain angle and optional geometric quantities. */
export type Puzzle2dSqliteTemplate=Omit<Puzzle2dHandleTemplate,"angle"|"t"|"radius">&{angle:Binary64;t?:Binary64;radius?:Binary64};
/** 🧩️ Node catalog ownership includes complete representations and metadata. */
export type Puzzle2dSqliteNodeKind=Omit<Puzzle2dCatalogNodeKind,"handles">&{handles:Puzzle2dSqliteTemplate[]};
/** 🗂️ Optional catalog presence is independent of its four collections. */
export type Puzzle2dSqliteCatalogs=Omit<Puzzle2dKindCatalogs,"nodes">&{nodes:Puzzle2dSqliteNodeKind[]};
/** 🪪️ Metadata preserves literal manifest identity and ordered compatibility. */
export type Puzzle2dSqliteMeta=Omit<Puzzle2dMeta,"kindCatalogs">&{kindCatalogs?:Puzzle2dSqliteCatalogs};
/** 📸️ Every persisted board and catalog field at its owned scalar width. */
export type Puzzle2dSqliteSnapshot=Omit<Puzzle2dSnapshot,"camera"|"nodes"|"edges"|"targetRegions"|"meta">&{camera:{x:Binary64;y:Binary64;zoom:Binary64};nodes:Puzzle2dSqliteNode[];edges:Puzzle2dSqliteEdge[];targetRegions:Puzzle2dSqliteRegion[];meta:Puzzle2dSqliteMeta};
export const PUZZLE2D_SQLITE_SCHEMA:string=sql;
const CAMERA:readonly Ieee754Column[]=[{index:2,width:64},{index:3,width:64},{index:4,width:64}];
const NODE:readonly Ieee754Column[]=[{index:6,width:64},{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64},{index:14,width:64}];
const HANDLE:readonly Ieee754Column[]=[{index:5,width:64},{index:6,width:64},{index:9,width:64}];
const EDGE:readonly Ieee754Column[]=[{index:7,width:64},{index:8,width:64},{index:9,width:64},{index:10,width:64},{index:11,width:64},{index:12,width:64},{index:13,width:64},{index:14,width:64}];
const REGION:readonly Ieee754Column[]=[{index:4,width:64},{index:5,width:64},{index:6,width:64},{index:7,width:64}];
const TEMPLATE:readonly Ieee754Column[]=[{index:9,width:64},{index:10,width:64},{index:12,width:64}];
function record(value:unknown,required:readonly string[],optional:readonly string[]=[]):void{if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Puzzle 2D requires a native record");for(const key in value)if(Object.hasOwn(value,key)&&!required.includes(key)&&!optional.includes(key))throw Error("Puzzle 2D record has undeclared fields");for(const key of required)if(!Object.hasOwn(value,key))throw Error("Puzzle 2D record lacks required fields")}
function text(value:unknown):string{if(typeof value!=="string")throw Error("Puzzle 2D requires literal TEXT");return value}
function optionalText(value:unknown):string|null{return value===undefined?null:text(value)}
function flag(value:unknown):bigint{if(typeof value!=="boolean")throw Error("Puzzle 2D requires a literal Boolean");return value?1n:0n}
function optionalFlag(value:unknown):bigint|null{return value===undefined?null:flag(value)}
function signed(value:unknown):bigint{if(typeof value!=="number"||!Number.isSafeInteger(value)||value< -2147483648||value>2147483647)throw Error("Puzzle 2D requires exact i32");return BigInt(value)}
function optionalSigned(value:unknown):bigint|null{return value===undefined?null:signed(value)}
function word(value:unknown):Binary64{record(value,["bits"]);return parseBinary64(value)}
function optionalWord(value:unknown):Binary64|null{return value===undefined?null:word(value)}
function list(value:unknown):void{if(!Array.isArray(value))throw Error("Puzzle 2D requires an ordered collection")}
function anchor(value:unknown):"fixed"|"derived"{if(value!=="fixed"&&value!=="derived")throw Error("Puzzle 2D node anchor is undeclared");return value}
function specificity(value:unknown):Puzzle2dSnapshot["meta"]["kindCompatibility"][number]["specificity"]{if(value!=="general"&&value!=="node"&&value!=="edge"&&value!=="handle"&&value!=="wire"&&value!=="vortex")throw Error("Puzzle 2D compatibility specificity is undeclared");return value}
function readOptionalText(row:SqliteRow,index:number):string|undefined{return row.values[index]===null?undefined:artifactSqliteText(row,index)}
function readOptionalFlag(row:SqliteRow,index:number):boolean|undefined{return row.values[index]===null?undefined:artifactSqliteBoolean(row,index)}
function readSigned(row:SqliteRow,index:number):number|undefined{if(row.values[index]===null)return;const value=artifactSqliteInteger(row,index);if(value< -2147483648n||value>2147483647n)throw Error("Puzzle 2D native i32 is out of range");return Number(value)}
function readOptionalWord(row:SqliteRow,index:number,columns:readonly Ieee754Column[]):Binary64|undefined{return ieee754IsNull(row,index,columns)?undefined:readBinary64(row,index,columns)}
/** 📤️ Projects every authored board entity and ordered catalog member. */
export async function puzzle2dSnapshotToSqliteDatabase(snapshot:Puzzle2dSqliteSnapshot,options:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const out=await ArtifactSqliteProjection.create(sql,options);
 record(snapshot,["schema","camera","nodes","edges","targetRegions","meta"]);list(snapshot.nodes);list(snapshot.edges);list(snapshot.targetRegions);record(snapshot.meta,["kindCompatibility"],["manifestId","kindCatalogs"]);list(snapshot.meta.kindCompatibility);
 let total=3+snapshot.nodes.length+snapshot.edges.length+snapshot.targetRegions.length+snapshot.meta.kindCompatibility.length;
 for(const row of snapshot.nodes){list(row.handles);total+=row.handles.length;}
 const catalogs=snapshot.meta.kindCatalogs;
 if(catalogs!==undefined){record(catalogs,["nodes","handles","edges","wires"]);list(catalogs.nodes);list(catalogs.handles);list(catalogs.edges);list(catalogs.wires);total+=1+catalogs.nodes.length+catalogs.handles.length+catalogs.edges.length+catalogs.wires.length;for(const row of catalogs.nodes){for(const value of[row.baseKinds,row.representations,row.handles,row.attributes,row.authors])list(value);total+=row.baseKinds.length+row.representations.length+row.handles.length+row.attributes.length+row.authors.length;for(const representation of row.representations){list(representation.tags);total+=representation.tags.length;}}for(const row of catalogs.handles){list(row.compatibleWith);total+=row.compatibleWith.length;}}
 out.checkRowsAdditional(total);let completed=0;
 const insert=async(table:string,cells:Parameters<ArtifactSqliteProjection["insert"]>[1],key?:bigint):Promise<bigint>=>{const id=await out.insert(table,cells,key);if(++completed%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total);return id;};
 const real=async(table:string,cells:readonly Ieee754Cell[],columns:readonly Ieee754Column[]):Promise<bigint>=>insert(table,encodeIeee754Cells(cells,columns,options.maxColumns).slice(1));
 const each=async<T>(rows:readonly T[],visit:(row:T,ordinal:number)=>Promise<void>):Promise<void>=>{await artifactSqliteCheckpoint(options,"projectSnapshot",0,rows.length,false);for(let ordinal=0;ordinal<rows.length;ordinal++){await visit(rows[ordinal]!,ordinal);if((ordinal+1)%256===0)await artifactSqliteCheckpoint(options,"projectSnapshot",ordinal+1,rows.length);}await artifactSqliteCheckpoint(options,"projectSnapshot",rows.length,rows.length,false);};
 await insert("puzzle2d_document",[text(snapshot.schema)],1n);record(snapshot.camera,["x","y","zoom"]);await real("puzzle2d_camera",[1n,1n,word(snapshot.camera.x),word(snapshot.camera.y),word(snapshot.camera.zoom)],CAMERA);const meta=await insert("puzzle2d_meta",[1n,optionalText(snapshot.meta.manifestId)]);
 await each(snapshot.nodes,async(row,ordinal)=>{
  record(row,["id","x","y","anchor","handles"],["nodeKind","shape","radius","width","height","text","iconKind","root","scale","visible","locked"]);
  const id=await real("puzzle2d_node",[1n,1n,BigInt(ordinal),text(row.id),optionalText(row.nodeKind),optionalText(row.shape),word(row.x),word(row.y),optionalWord(row.radius),optionalWord(row.width),optionalWord(row.height),optionalText(row.text),optionalText(row.iconKind),optionalFlag(row.root),optionalWord(row.scale),optionalFlag(row.visible),optionalFlag(row.locked),anchor(row.anchor)],NODE);
  await each(row.handles,async(h,ordinal)=>{record(h,["id","angle"],["handleKind","radius","color","iconKind","scale","visible","locked"]);await real("puzzle2d_handle",[1n,id,BigInt(ordinal),text(h.id),optionalText(h.handleKind),word(h.angle),optionalWord(h.radius),optionalText(h.color),optionalText(h.iconKind),optionalWord(h.scale),optionalFlag(h.visible),optionalFlag(h.locked)],HANDLE);});
 });
 await each(snapshot.edges,async(row,ordinal)=>{record(row,["id","source","target","gap","shift","rise","rotation","turn","tilt","x","y"],["edgeKind","sourceTip","targetTip","visible","locked"]);await real("puzzle2d_edge",[1n,1n,BigInt(ordinal),text(row.id),text(row.source),text(row.target),optionalText(row.edgeKind),word(row.gap),word(row.shift),word(row.rise),word(row.rotation),word(row.turn),word(row.tilt),word(row.x),word(row.y),optionalText(row.sourceTip),optionalText(row.targetTip),optionalFlag(row.visible),optionalFlag(row.locked)],EDGE);});
 await each(snapshot.targetRegions,async(row,ordinal)=>{record(row,["id","x","y","width","height","hidden","locked"],["label"]);await real("puzzle2d_target_region",[1n,1n,BigInt(ordinal),text(row.id),word(row.x),word(row.y),word(row.width),word(row.height),optionalText(row.label),flag(row.hidden),flag(row.locked)],REGION);});
 await each(snapshot.meta.kindCompatibility,async(row,ordinal)=>{record(row,["source","target","bidirectional","important","specificity"]);await insert("puzzle2d_kind_compatibility",[meta,BigInt(ordinal),text(row.source),text(row.target),flag(row.bidirectional),flag(row.important),specificity(row.specificity)]);});
 if(catalogs!==undefined){
  const id=await insert("puzzle2d_kind_catalogs",[meta]);
  await each(catalogs.nodes,async(row,ordinal)=>{
   record(row,["id","name","label","description","icon","image","unit","abstract","baseKinds","representations","handles","attributes","authors"]);
   const node=await insert("puzzle2d_catalog_node_kind",[id,BigInt(ordinal),text(row.id),text(row.name),text(row.label),text(row.description),text(row.icon),text(row.image),text(row.unit),flag(row.abstract)]);
   await each(row.baseKinds,async(value,ordinal)=>{await insert("puzzle2d_base_kind",[node,BigInt(ordinal),text(value)]);});
   await each(row.representations,async(row,ordinal)=>{record(row,["id","name","url","mime","tags","description"],["lod"]);const representation=await insert("puzzle2d_representation",[node,BigInt(ordinal),text(row.id),text(row.name),text(row.url),text(row.mime),optionalText(row.lod),text(row.description)]);await each(row.tags,async(tag,ordinal)=>{await insert("puzzle2d_representation_tag",[representation,BigInt(ordinal),text(tag)]);});});
   await each(row.handles,async(row,ordinal)=>{record(row,["id","name","label","description","icon","angle"],["handleKind","t","mandatory","radius"]);await real("puzzle2d_handle_template",[1n,node,BigInt(ordinal),text(row.id),text(row.name),text(row.label),text(row.description),text(row.icon),optionalText(row.handleKind),word(row.angle),optionalWord(row.t),optionalFlag(row.mandatory),optionalWord(row.radius)],TEMPLATE);});
   await each(row.attributes,async(row,ordinal)=>{record(row,["id","key","value"],["definition"]);await insert("puzzle2d_attribute",[node,BigInt(ordinal),text(row.id),text(row.key),text(row.value),optionalText(row.definition)]);});
   await each(row.authors,async(row,ordinal)=>{record(row,["id","name","email"],["role","rank"]);await insert("puzzle2d_author",[node,BigInt(ordinal),text(row.id),text(row.name),text(row.email),optionalText(row.role),optionalSigned(row.rank)]);});
  });
  await each(catalogs.handles,async(row,ordinal)=>{record(row,["id","compatibleWith","description","icon","color","defaultWireKind"],["code","label","order"]);const handle=await insert("puzzle2d_catalog_handle_kind",[id,BigInt(ordinal),text(row.id),optionalText(row.code),optionalText(row.label),optionalSigned(row.order),text(row.description),text(row.icon),text(row.color),text(row.defaultWireKind)]);await each(row.compatibleWith,async(value,ordinal)=>{await insert("puzzle2d_compatible_kind",[handle,BigInt(ordinal),text(value)]);});});
  await each(catalogs.edges,async(row,ordinal)=>{record(row,["id","name","label","description","icon","color"]);await insert("puzzle2d_catalog_edge_kind",[id,BigInt(ordinal),text(row.id),text(row.name),text(row.label),text(row.description),text(row.icon),text(row.color)]);});
  await each(catalogs.wires,async(row,ordinal)=>{record(row,["id","name","label","description","icon","color","defaultEdgeKind"]);await insert("puzzle2d_catalog_wire_kind",[id,BigInt(ordinal),text(row.id),text(row.name),text(row.label),text(row.description),text(row.icon),text(row.color),text(row.defaultEdgeKind)]);});
 }
 if(completed!==total)throw Error("Puzzle 2D projected workload differs");await artifactSqliteCheckpoint(options,"projectSnapshot",completed,total,false);return out.finish();
}
function present<T extends object,K extends keyof T>(target:T,key:K,value:T[K]|undefined):void{if(value!==undefined)target[key]=value;}
/** 📥️ Rebuilds every owned entity with strict presence, ordinals and admitted workspace. */
export async function puzzle2dSnapshotFromSqliteDatabase(database:SqliteDatabase,options:ArtifactSqliteOptions={}):Promise<Puzzle2dSqliteSnapshot>{
 const tables=await artifactSqliteTables(database,sql,options),total=tables.reduce((n,rows)=>n+rows.length,0);let owned=0,completed=0;
 const charge=(bytes:number):void=>{owned+=bytes;artifactSqliteValueBudget(owned,options);};
 charge(total*512);
 const widths=[2,11,3,30,18,35,19,8,2,11,4,9,4,19,7,8,11,4,9,10];
 for(let index=0;index<tables.length;index++){const ids=new Set<bigint>();for(const row of tables[index]!){if(row.values.length!==widths[index]||row.rowid<=0n||artifactSqliteInteger(row,0)!==row.rowid||ids.has(row.rowid))throw Error("Puzzle 2D requires complete positive aliased unique entities");ids.add(row.rowid);if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,total);}}
 const ownText=async(row:SqliteRow,index:number):Promise<string>=>{const value=artifactSqliteText(row,index),length=artifactSqliteTextBytes(value);charge(length);let completed=0,last=0;for(let i=0;i<value.length;i++){const point=value.codePointAt(i)!;if(point>65535)i++;completed+=point<=127?1:point<=2047?2:point<=65535?3:4;if(completed-last>=65536){await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,length);last=completed;}}return value;};
 const ownOptionalText=async(row:SqliteRow,index:number):Promise<string|undefined>=>row.values[index]===null?undefined:ownText(row,index);
 const groups=async(index:number):Promise<Map<bigint,SqliteRow[]>>=>{const map=new Map<bigint,SqliteRow[]>();let completed=0;for(const row of tables[index]!){const parent=artifactSqliteInteger(row,1),rows=map.get(parent);if(rows)rows.push(row);else map.set(parent,[row]);if(++completed%256===0)await artifactSqliteCheckpoint(options,"reconstructSnapshot",completed,tables[index]!.length);}return map;};
 const handles=await groups(4),compatibility=await groups(7),nodeKinds=await groups(9),bases=await groups(10),representations=await groups(11),tags=await groups(12),templates=await groups(13),attributes=await groups(14),authors=await groups(15),handleKinds=await groups(16),compatible=await groups(17),edgeKinds=await groups(18),wireKinds=await groups(19);
 const members=async(map:Map<bigint,SqliteRow[]>,parent:bigint):Promise<SqliteRow[]>=>{const rows=map.get(parent)??[];map.delete(parent);charge(rows.length*8);return artifactSqliteOrderedRowsControlled(rows,2,options);};
 const checkDocument=(row:SqliteRow):void=>{if(artifactSqliteInteger(row,1)!==1n)throw Error("Puzzle 2D entity has wrong document owner");};
 const document=artifactSqliteDocument(tables[0]!);if(tables[1]!.length!==1||tables[2]!.length!==1)throw Error("Puzzle 2D requires one camera and one metadata entity");const camera=tables[1]![0]!,metaRow=tables[2]![0]!;checkDocument(camera);checkDocument(metaRow);
 const result:Puzzle2dSqliteSnapshot={schema:await ownText(document,1),camera:{x:readBinary64(camera,2,CAMERA),y:readBinary64(camera,3,CAMERA),zoom:readBinary64(camera,4,CAMERA)},nodes:[],edges:[],targetRegions:[],meta:{kindCompatibility:[]}};present(result.meta,"manifestId",await ownOptionalText(metaRow,2));
 for(const row of await artifactSqliteOrderedRowsControlled(tables[3]!,2,options)){
  checkDocument(row);const node:Puzzle2dSqliteNode={id:await ownText(row,3),x:readBinary64(row,6,NODE),y:readBinary64(row,7,NODE),anchor:anchor(artifactSqliteText(row,17)),handles:[]};
  present(node,"nodeKind",await ownOptionalText(row,4));present(node,"shape",await ownOptionalText(row,5));present(node,"radius",readOptionalWord(row,8,NODE));present(node,"width",readOptionalWord(row,9,NODE));present(node,"height",readOptionalWord(row,10,NODE));present(node,"text",await ownOptionalText(row,11));present(node,"iconKind",await ownOptionalText(row,12));present(node,"root",readOptionalFlag(row,13));present(node,"scale",readOptionalWord(row,14,NODE));present(node,"visible",readOptionalFlag(row,15));present(node,"locked",readOptionalFlag(row,16));
  for(const member of await members(handles,row.rowid)){const h:Puzzle2dSqliteHandle={id:await ownText(member,3),angle:readBinary64(member,5,HANDLE)};present(h,"handleKind",await ownOptionalText(member,4));present(h,"radius",readOptionalWord(member,6,HANDLE));present(h,"color",await ownOptionalText(member,7));present(h,"iconKind",await ownOptionalText(member,8));present(h,"scale",readOptionalWord(member,9,HANDLE));present(h,"visible",readOptionalFlag(member,10));present(h,"locked",readOptionalFlag(member,11));node.handles.push(h);}
  result.nodes.push(node);
 }
 for(const row of await artifactSqliteOrderedRowsControlled(tables[5]!,2,options)){
  checkDocument(row);const edge:Puzzle2dSqliteEdge={id:await ownText(row,3),source:await ownText(row,4),target:await ownText(row,5),gap:readBinary64(row,7,EDGE),shift:readBinary64(row,8,EDGE),rise:readBinary64(row,9,EDGE),rotation:readBinary64(row,10,EDGE),turn:readBinary64(row,11,EDGE),tilt:readBinary64(row,12,EDGE),x:readBinary64(row,13,EDGE),y:readBinary64(row,14,EDGE)};
  present(edge,"edgeKind",await ownOptionalText(row,6));present(edge,"sourceTip",await ownOptionalText(row,15));present(edge,"targetTip",await ownOptionalText(row,16));present(edge,"visible",readOptionalFlag(row,17));present(edge,"locked",readOptionalFlag(row,18));result.edges.push(edge);
 }
 for(const row of await artifactSqliteOrderedRowsControlled(tables[6]!,2,options)){checkDocument(row);const region:Puzzle2dSqliteRegion={id:await ownText(row,3),x:readBinary64(row,4,REGION),y:readBinary64(row,5,REGION),width:readBinary64(row,6,REGION),height:readBinary64(row,7,REGION),hidden:artifactSqliteBoolean(row,9),locked:artifactSqliteBoolean(row,10)};present(region,"label",await ownOptionalText(row,8));result.targetRegions.push(region);}
 for(const row of await members(compatibility,metaRow.rowid))result.meta.kindCompatibility.push({source:await ownText(row,3),target:await ownText(row,4),bidirectional:artifactSqliteBoolean(row,5),important:artifactSqliteBoolean(row,6),specificity:specificity(artifactSqliteText(row,7))});
 if(tables[8]!.length>1)throw Error("Puzzle 2D metadata owns at most one catalog");
 if(tables[8]!.length===1){
  const catalogRow=tables[8]![0]!;if(artifactSqliteInteger(catalogRow,1)!==metaRow.rowid)throw Error("Puzzle 2D catalog has wrong metadata owner");const catalog:Puzzle2dSqliteCatalogs={nodes:[],handles:[],edges:[],wires:[]};result.meta.kindCatalogs=catalog;
  for(const row of await members(nodeKinds,catalogRow.rowid)){
   const node:Puzzle2dSqliteNodeKind={id:await ownText(row,3),name:await ownText(row,4),label:await ownText(row,5),description:await ownText(row,6),icon:await ownText(row,7),image:await ownText(row,8),unit:await ownText(row,9),abstract:artifactSqliteBoolean(row,10),baseKinds:[],representations:[],handles:[],attributes:[],authors:[]};
   for(const member of await members(bases,row.rowid))node.baseKinds.push(await ownText(member,3));
   for(const member of await members(representations,row.rowid)){const representation:NonNullable<Puzzle2dMeta["kindCatalogs"]>["nodes"][number]["representations"][number]={id:await ownText(member,3),name:await ownText(member,4),url:await ownText(member,5),mime:await ownText(member,6),tags:[],description:await ownText(member,8)};present(representation,"lod",await ownOptionalText(member,7));for(const tag of await members(tags,member.rowid))representation.tags.push(await ownText(tag,3));node.representations.push(representation);}
   for(const member of await members(templates,row.rowid)){const h:Puzzle2dSqliteTemplate={id:await ownText(member,3),name:await ownText(member,4),label:await ownText(member,5),description:await ownText(member,6),icon:await ownText(member,7),angle:readBinary64(member,9,TEMPLATE)};present(h,"handleKind",await ownOptionalText(member,8));present(h,"t",readOptionalWord(member,10,TEMPLATE));present(h,"mandatory",readOptionalFlag(member,11));present(h,"radius",readOptionalWord(member,12,TEMPLATE));node.handles.push(h);}
   for(const member of await members(attributes,row.rowid)){const attribute:NonNullable<Puzzle2dMeta["kindCatalogs"]>["nodes"][number]["attributes"][number]={id:await ownText(member,3),key:await ownText(member,4),value:await ownText(member,5)};present(attribute,"definition",await ownOptionalText(member,6));node.attributes.push(attribute);}
   for(const member of await members(authors,row.rowid)){const author:NonNullable<Puzzle2dMeta["kindCatalogs"]>["nodes"][number]["authors"][number]={id:await ownText(member,3),name:await ownText(member,4),email:await ownText(member,5)};present(author,"role",await ownOptionalText(member,6));present(author,"rank",readSigned(member,7));node.authors.push(author);}
   catalog.nodes.push(node);
  }
  for(const row of await members(handleKinds,catalogRow.rowid)){const h:NonNullable<Puzzle2dMeta["kindCatalogs"]>["handles"][number]={id:await ownText(row,3),compatibleWith:[],description:await ownText(row,7),icon:await ownText(row,8),color:await ownText(row,9),defaultWireKind:await ownText(row,10)};present(h,"code",await ownOptionalText(row,4));present(h,"label",await ownOptionalText(row,5));present(h,"order",readSigned(row,6));for(const member of await members(compatible,row.rowid))h.compatibleWith.push(await ownText(member,3));catalog.handles.push(h);}
  for(const row of await members(edgeKinds,catalogRow.rowid))catalog.edges.push({id:await ownText(row,3),name:await ownText(row,4),label:await ownText(row,5),description:await ownText(row,6),icon:await ownText(row,7),color:await ownText(row,8)});
  for(const row of await members(wireKinds,catalogRow.rowid))catalog.wires.push({id:await ownText(row,3),name:await ownText(row,4),label:await ownText(row,5),description:await ownText(row,6),icon:await ownText(row,7),color:await ownText(row,8),defaultEdgeKind:await ownText(row,9)});
 }
 if([handles,compatibility,nodeKinds,bases,representations,tags,templates,attributes,authors,handleKinds,compatible,edgeKinds,wireKinds].some(map=>map.size))throw Error("Puzzle 2D contains unmatched owned entities");await artifactSqliteCheckpoint(options,"reconstructSnapshot",total,total,false);return result;
}
