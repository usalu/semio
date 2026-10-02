/** 🧩️ Handwritten Puzzle5d relational projection and complete owned reconstruction. */
import * as m from "../🟦️.ts";
import { ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type { SqliteDatabase,SqliteRow,SqliteValue } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {parseBinary64,binary64Value,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {PUZZLE5D_SQLITE_SCHEMA} from "./🗄️schema/🟦️.ts";
type Cells=readonly SqliteValue[];
const fail=(why:string):never=>{throw new Error("Puzzle5d SQLite "+why)};
const text=(v:string):string=>typeof v==="string"?v:fail("TEXT required");
const optionalText=(v:string|null):SqliteValue=>v===null?null:text(v);
const boolean=(v:boolean):bigint=>typeof v==="boolean"?(v?1n:0n):fail("Boolean required");
const optionalBoolean=(v:boolean|null):SqliteValue=>v===null?null:boolean(v);
const optionalInteger=(v:number|null):SqliteValue=>v===null?null:Number.isInteger(v)&&v>=-2147483648&&v<=2147483647?BigInt(v):fail("signed32 required");
function floating(v:Binary64):Cells {const bits=parseBinary64(v).bits,n=binary64Value(v),kind=Number.isNaN(n)?"nan":n===Infinity?"positiveInfinity":n===-Infinity?"negativeInfinity":"finite";return[Number.isNaN(n)?null:n,BigInt.asIntN(64,bits),kind]}
const optionalFloat=(v:Binary64|null):Cells=>v===null?[null,null,null]:floating(v);
function xyz(v:m.Puzzle5dVector3):Cells {if(v.length!==3)fail("vector3 width");return[...floating(v[0]),...floating(v[1]),...floating(v[2])]}
function orientation(v:m.Puzzle5dVector4|null):Cells {if(v===null)return[null,null,null,null,null,null,null,null,null,null,null,null];if(v.length!==4)fail("quaternion width");return[...floating(v[0]),...floating(v[1]),...floating(v[2]),...floating(v[3])]}
function scale(v:m.Puzzle5dScale):Cells {return Array.isArray(v)?["vec3",null,null,null,...xyz(v)]:["uniform",...floating(v),null,null,null,null,null,null,null,null,null]}
async function forecast(v:m.Puzzle5dSnapshot,o:ArtifactSqliteOptions):Promise<number>{
 let rows=1,work=0;await artifactSqliteCheckpoint(o,"projectSnapshot",0,0);
 const add=async(n:number)=>{rows+=n;if(!Number.isSafeInteger(rows)||rows>(o.maxRows??1000000))fail("row limit");if(++work%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",work,0)};
 await add(v.kindCatalogs===null?0:1);await add(v.kindCompatibility.length+v.fasteners.length);
 for(const p of v.parts)await add(3+(p["3d"].scale===null?0:1)+p.grips.length*3);
 for(const target of v.targetVolumes)await add(1+(target.scale===null?0:1));
 const e=v.kindCatalogsExtra;if(e!==null){await add(1+e.fasteners.length+e.ropes.length);for(const p of e.parts){await add(1+p.baseKinds.length+p.grips.length+p.attributes.length+p.authors.length);for(const representation of p.representations)await add(1+representation.tags.length)}for(const g of e.grips)await add(1+g.compatibleWith.length)}
 await artifactSqliteCheckpoint(o,"projectSnapshot",work,work);return rows;
}
/** 📤️ Project every actual persisted field after exact borrowed row admission. */
export async function puzzle5dSnapshotToSqliteDatabase(v:m.Puzzle5dSnapshot,o:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(PUZZLE5D_SQLITE_SCHEMA,o),count=await forecast(v,o);p.checkRowsAdditional(count);
 const doc=await p.insert("puzzle5_document",[text(v.schema),text(v.domain),optionalText(v.label),text(v.meta.description)]);
 if(v.kindCatalogs!==null){const c=v.kindCatalogs;await p.insert("puzzle5_catalog_child",[doc,text(c.childId),text(c.target.artifactId),text(c.target.dialect.artifactKind),text(c.target.dialect.standard),text(c.target.dialect.subset)])}
 const extra=v.kindCatalogsExtra;if(extra!==null){
  const e=await p.insert("puzzle5_catalog_extra",[doc]);
  for(let i=0;i<extra.parts.length;i++){const k=extra.parts[i]!,id=await p.insert("puzzle5_part_kind",[e,BigInt(i),text(k.id),text(k.name),text(k.label),text(k.description),text(k.icon),text(k.image),text(k.unit),boolean(k.abstract)]);
   for(let j=0;j<k.baseKinds.length;j++)await p.insert("puzzle5_part_kind_base",[id,BigInt(j),text(k.baseKinds[j]!)]);
   for(let j=0;j<k.representations.length;j++){const r=k.representations[j]!,rid=await p.insert("puzzle5_representation",[id,BigInt(j),text(r.id),text(r.name),text(r.url),text(r.mime),optionalText(r.lod),text(r.description)]);for(let n=0;n<r.tags.length;n++)await p.insert("puzzle5_representation_tag",[rid,BigInt(n),text(r.tags[n]!)])}
   for(let j=0;j<k.grips.length;j++){const t=k.grips[j]!;await p.insert("puzzle5_grip_template",[id,BigInt(j),text(t.id),text(t.name),text(t.label),text(t.description),text(t.icon),optionalText(t.gripKind),...xyz(t.point),...xyz(t.direction),...optionalFloat(t.t),optionalBoolean(t.mandatory),...optionalFloat(t.radius)])}
   for(let j=0;j<k.attributes.length;j++){const a=k.attributes[j]!;await p.insert("puzzle5_attribute",[id,BigInt(j),text(a.id),text(a.key),text(a.value),optionalText(a.definition)])}
   for(let j=0;j<k.authors.length;j++){const a=k.authors[j]!;await p.insert("puzzle5_author",[id,BigInt(j),text(a.id),text(a.name),text(a.email),optionalText(a.role),optionalInteger(a.rank)])}
  }
  for(let i=0;i<extra.grips.length;i++){const g=extra.grips[i]!,id=await p.insert("puzzle5_grip_kind",[e,BigInt(i),text(g.id),optionalText(g.code),optionalText(g.label),optionalInteger(g.order),text(g.description),text(g.icon),text(g.color),text(g.defaultRopeKind)]);for(let j=0;j<g.compatibleWith.length;j++)await p.insert("puzzle5_grip_kind_compatible",[id,BigInt(j),text(g.compatibleWith[j]!)])}
  for(let i=0;i<extra.fasteners.length;i++){const k=extra.fasteners[i]!;await p.insert("puzzle5_fastener_kind",[e,BigInt(i),text(k.id),text(k.name),optionalText(k.label)])}
  for(let i=0;i<extra.ropes.length;i++){const k=extra.ropes[i]!;await p.insert("puzzle5_rope_kind",[e,BigInt(i),text(k.id),text(k.name),text(k.label),text(k.defaultFastenerKind)])}
 }
 for(let i=0;i<v.kindCompatibility.length;i++){const k=v.kindCompatibility[i]!;await p.insert("puzzle5_compatibility",[doc,BigInt(i),text(k.source),text(k.target),boolean(k.bidirectional),boolean(k.important),text(k.specificity)])}
 for(let i=0;i<v.parts.length;i++){
  const part=v.parts[i]!,id=await p.insert("puzzle5_part",[doc,BigInt(i),text(part.id),optionalText(part.partKind),text(part.anchor)]),b=part["2d"],w=part["3d"];
  await p.insert("puzzle5_part_board",[id,...floating(b.x),...floating(b.y),optionalText(b.shape),...optionalFloat(b.radius),...optionalFloat(b.width),...optionalFloat(b.height),optionalText(b.text),optionalText(b.iconKind),optionalBoolean(b.hidden),optionalBoolean(b.locked)]);
  await p.insert("puzzle5_part_world",[id,...xyz(w.origin),optionalText(w.meshUrl),...orientation(w.orientation),optionalText(w.label)]);
  if(w.scale!==null)await p.insert("puzzle5_part_scale",[id,...scale(w.scale)]);
  for(let j=0;j<part.grips.length;j++){const g=part.grips[j]!,gid=await p.insert("puzzle5_grip",[id,BigInt(j),text(g.id),optionalText(g.gripKind)]),b=g["2d"],w=g["3d"];await p.insert("puzzle5_grip_board",[gid,...floating(b.angle),optionalText(b.gripKind),...optionalFloat(b.radius)]);await p.insert("puzzle5_grip_world",[gid,...xyz(w.position),...(w.direction===null?[null,null,null,null,null,null,null,null,null]:xyz(w.direction)),...optionalFloat(w.radius),optionalText(w.label)])}
 }
 for(let i=0;i<v.fasteners.length;i++){const f=v.fasteners[i]!;await p.insert("puzzle5_fastener",[doc,BigInt(i),text(f.id),text(f.source),text(f.target),optionalText(f.fastenerKind),...floating(f.gap),...floating(f.shift),...floating(f.rise),...floating(f.rotation),...floating(f.turn),...floating(f.tilt),...floating(f.x),...floating(f.y)])}
 for(let i=0;i<v.targetVolumes.length;i++){const t=v.targetVolumes[i]!,id=await p.insert("puzzle5_target_volume",[doc,BigInt(i),text(t.id),...xyz(t.origin),...orientation(t.orientation),boolean(t.hidden),boolean(t.locked)]);if(t.scale!==null)await p.insert("puzzle5_target_scale",[id,...scale(t.scale)])}
 return p.finish();
}
class Cursor{
 private index:number;
 constructor(readonly row:SqliteRow,start=1){this.index=start;if(row.values[0]!==row.rowid)fail("rowid alias differs")}
 next():SqliteValue{if(this.index>=this.row.values.length)return fail("missing cell");return this.row.values[this.index++]!}
 text():string{const v=this.next();return typeof v==="string"?v:fail("TEXT required")}
 integer():bigint{const v=this.next();return typeof v==="bigint"?v:fail("INTEGER required")}
 optionalText():string|null{return this.row.values[this.index]===null?(this.next(),null):this.text()}
 boolean():boolean{const n=this.integer();return n===0n?false:n===1n?true:fail("Boolean width")}
 optionalBoolean():boolean|null{return this.row.values[this.index]===null?(this.next(),null):this.boolean()}
 optionalInteger():number|null{if(this.row.values[this.index]===null){this.next();return null}const n=this.integer();return n>=-2147483648n&&n<=2147483647n?Number(n):fail("signed32 width")}
 expect(value:SqliteValue):void{if(this.next()!==value)fail("unused cell differs")}
 f64():Binary64{const query=this.next(),n=this.integer(),kind=this.text(),v={bits:BigInt.asUintN(64,n)},expected=binary64Value(v),classification=Number.isNaN(expected)?"nan":expected===Infinity?"positiveInfinity":expected===-Infinity?"negativeInfinity":"finite";if(kind!==classification)fail("IEEE class differs");if(Number.isNaN(expected)){if(query!==null)fail("NaN query must be NULL")}else if(typeof query==="bigint"){if(!Number.isFinite(expected)||!Number.isInteger(expected)||BigInt(expected)!==query)fail("IEEE integer query differs")}else if(typeof query!=="number"||query!==expected)fail("IEEE query differs");return v}
 optionalFloat():Binary64|null{if(this.row.values[this.index+1]===null){this.expect(null);this.expect(null);this.expect(null);return null}return this.f64()}
 xyz():m.Puzzle5dVector3{return[this.f64(),this.f64(),this.f64()]}
 optionalVector3():m.Puzzle5dVector3|null{const a=this.optionalFloat(),b=this.optionalFloat(),c=this.optionalFloat();if(a===null&&b===null&&c===null)return null;if(a===null||b===null||c===null)fail("partial vector absence");return[a!,b!,c!]}
 orientation():m.Puzzle5dVector4|null{const w=this.optionalFloat(),x=this.optionalFloat(),y=this.optionalFloat(),z=this.optionalFloat();if(w===null&&x===null&&y===null&&z===null)return null;if(w===null||x===null||y===null||z===null)fail("partial quaternion absence");return[w!,x!,y!,z!]}
 symbol<T extends string>(members:readonly T[]):T{const v=this.text();return members.includes(v as T)?v as T:fail("unknown enum")}
 done():void{if(this.index!==this.row.values.length)fail("extra cell")}
}
class Reader{
 readonly used=new Set<SqliteRow>();readonly indexes=new Map<string,Map<bigint,SqliteRow[]>>();
 private constructor(readonly tables:Map<string,readonly SqliteRow[]>,readonly control:NativeDecodeControl,readonly options:ArtifactSqliteOptions){}
 static async create(db:SqliteDatabase,o:ArtifactSqliteOptions):Promise<Reader>{const rows=await artifactSqliteTables(db,PUZZLE5D_SQLITE_SCHEMA,o),control=new NativeDecodeControl(o.maxValueBytes??268435456,p=>{o.onProgress?.({phase:"reconstructSnapshot",completed:p.completed,total:p.total});return!o.signal?.aborted},o.signal);await control.admitSlots(db.tables.length,64);await control.beginStage(rows.reduce((n,r)=>n+r.length,0));return new Reader(new Map(db.tables.map(t=>[t.name.toLowerCase(),t.rows])),control,o)}
 all(name:string):readonly SqliteRow[]{return this.tables.get(name)??fail("missing table "+name)}
 async take(row:SqliteRow,start=1):Promise<Cursor>{if(this.used.has(row))fail("duplicate ownership");await this.control.charge(192);await this.control.step();this.used.add(row);return new Cursor(row,start)}
 async group(name:string,parent:bigint,ordinal:number|null=2):Promise<SqliteRow[]>{let index=this.indexes.get(name);if(index===undefined){const source=this.all(name);await this.control.admitSlots(source.length,48);index=new Map();await this.control.scopedStage(async control=>{await control.beginStage(source.length);for(const row of source){const owner=row.values[1];if(typeof owner!=="bigint")fail("parent INTEGER required");let group=index!.get(owner as bigint);if(group===undefined){group=[];index!.set(owner as bigint,group)}group.push(row);await control.step()}});this.indexes.set(name,index)}const rows=index.get(parent)??[];if(ordinal===null)return rows;await this.control.admitSlots(rows.length,8);return artifactSqliteOrderedRowsControlled(rows,ordinal,this.options)}
 async one(name:string,parent:bigint,required=true):Promise<SqliteRow|null>{const rows=await this.group(name,parent,null);if(rows.length>1||required&&rows.length!==1)fail("cardinality "+name);return rows[0]??null}
 async finish():Promise<void>{await this.control.scopedStage(async control=>{let n=0;for(const rows of this.tables.values())n+=rows.length;await control.beginStage(n);for(const rows of this.tables.values())for(const row of rows){if(!this.used.has(row))fail("unowned entity");await control.step()}await control.checkpoint()})}
}
/** 📥️ Reconstruct only complete authored relationships and exact typed fields. */
export async function puzzle5dSnapshotFromSqliteDatabase(db:SqliteDatabase,o:ArtifactSqliteOptions={}):Promise<m.Puzzle5dSnapshot>{
 const r=await Reader.create(db,o),docs=r.all("puzzle5_document");if(docs.length!==1)fail("one document required");const doc=docs[0]!,d=await r.take(doc),schema=d.text(),domain=d.text(),label=d.optionalText(),meta={description:d.text()};d.done();
 const child=await r.one("puzzle5_catalog_child",doc.rowid,false);let kindCatalogs:m.ArtifactChildHandle|null=null;if(child!==null){const c=await r.take(child,2);kindCatalogs={childId:c.text(),target:{artifactId:c.text(),dialect:{artifactKind:c.text(),standard:c.text(),subset:c.text()}}};c.done()}
 const extra=await r.one("puzzle5_catalog_extra",doc.rowid,false);let kindCatalogsExtra:m.Puzzle5dKindCatalogsExtra|null=null;
 const strings=async(name:string,parent:bigint):Promise<string[]>=>{const values:string[]=[];for(const row of await r.group(name,parent)){const c=await r.take(row,3);values.push(c.text());c.done()}return values};
 if(extra!==null){
  const c=await r.take(extra,2);c.done();const parts:m.Puzzle5dCatalogPartKindExtra[]=[],grips:m.Puzzle5dCatalogGripKindExtra[]=[],fasteners:m.Puzzle5dCatalogFastenerKindExtra[]=[],ropes:m.Puzzle5dCatalogRopeKindExtra[]=[];
  for(const row of await r.group("puzzle5_part_kind",extra.rowid)){
   const c=await r.take(row,3),id=c.text(),name=c.text(),label=c.text(),description=c.text(),icon=c.text(),image=c.text(),unit=c.text(),abstract=c.boolean();c.done();
   const baseKinds=await strings("puzzle5_part_kind_base",row.rowid),representations:m.Puzzle5dRepresentation[]=[],templates:m.Puzzle5dGripTemplate[]=[],attributes:m.Puzzle5dAttribute[]=[],authors:m.Puzzle5dAuthor[]=[];
   for(const value of await r.group("puzzle5_representation",row.rowid)){const c=await r.take(value,3),id=c.text(),name=c.text(),url=c.text(),mime=c.text(),lod=c.optionalText(),description=c.text();c.done();representations.push({id,name,url,mime,lod,description,tags:await strings("puzzle5_representation_tag",value.rowid)})}
   for(const value of await r.group("puzzle5_grip_template",row.rowid)){const c=await r.take(value,3);templates.push({id:c.text(),name:c.text(),label:c.text(),description:c.text(),icon:c.text(),gripKind:c.optionalText(),point:c.xyz(),direction:c.xyz(),t:c.optionalFloat(),mandatory:c.optionalBoolean(),radius:c.optionalFloat()});c.done()}
   for(const value of await r.group("puzzle5_attribute",row.rowid)){const c=await r.take(value,3);attributes.push({id:c.text(),key:c.text(),value:c.text(),definition:c.optionalText()});c.done()}
   for(const value of await r.group("puzzle5_author",row.rowid)){const c=await r.take(value,3);authors.push({id:c.text(),name:c.text(),email:c.text(),role:c.optionalText(),rank:c.optionalInteger()});c.done()}
   parts.push({id,name,label,description,icon,image,unit,abstract,baseKinds,representations,grips:templates,attributes,authors});
  }
  for(const row of await r.group("puzzle5_grip_kind",extra.rowid)){const c=await r.take(row,3),id=c.text(),code=c.optionalText(),label=c.optionalText(),order=c.optionalInteger(),description=c.text(),icon=c.text(),color=c.text(),defaultRopeKind=c.text();c.done();grips.push({id,code,label,order,description,icon,color,defaultRopeKind,compatibleWith:await strings("puzzle5_grip_kind_compatible",row.rowid)})}
  for(const row of await r.group("puzzle5_fastener_kind",extra.rowid)){const c=await r.take(row,3);fasteners.push({id:c.text(),name:c.text(),label:c.optionalText()});c.done()}
  for(const row of await r.group("puzzle5_rope_kind",extra.rowid)){const c=await r.take(row,3);ropes.push({id:c.text(),name:c.text(),label:c.text(),defaultFastenerKind:c.text()});c.done()}
  kindCatalogsExtra={parts,grips,fasteners,ropes};
 }
 const kindCompatibility:m.Puzzle5dKindCompatibility[]=[],parts:m.Puzzle5dPart[]=[],fasteners:m.Puzzle5dFastener[]=[],targetVolumes:m.Puzzle5dTargetVolume[]=[];
 for(const row of await r.group("puzzle5_compatibility",doc.rowid)){const c=await r.take(row,3);kindCompatibility.push({source:c.text(),target:c.text(),bidirectional:c.boolean(),important:c.boolean(),specificity:c.symbol(["general","part","fastener","grip","rope"])});c.done()}
 const readScale=async(name:string,parent:bigint):Promise<m.Puzzle5dScale|null>=>{const row=await r.one(name,parent,false);if(row===null)return null;const c=await r.take(row,2),kind=c.symbol(["uniform","vec3"]),uniform=c.optionalFloat(),axes=c.optionalVector3();c.done();if(kind==="uniform"){if(uniform===null||axes!==null)fail("uniform scale fields");return uniform!}if(uniform!==null||axes===null)fail("vector scale fields");return axes!};
 for(const row of await r.group("puzzle5_part",doc.rowid)){
  const c=await r.take(row,3),id=c.text(),partKind=c.optionalText(),anchor=c.symbol(["fixed","derived"]);c.done();
  const b=await r.take((await r.one("puzzle5_part_board",row.rowid))!,2),board:m.Puzzle5dPart2d={x:b.f64(),y:b.f64(),shape:b.optionalText(),radius:b.optionalFloat(),width:b.optionalFloat(),height:b.optionalFloat(),text:b.optionalText(),iconKind:b.optionalText(),hidden:b.optionalBoolean(),locked:b.optionalBoolean()};b.done();
  const w=await r.take((await r.one("puzzle5_part_world",row.rowid))!,2),world:m.Puzzle5dPart3d={origin:w.xyz(),meshUrl:w.optionalText(),orientation:w.orientation(),label:w.optionalText(),scale:await readScale("puzzle5_part_scale",row.rowid)};w.done();
  const grips:m.Puzzle5dGrip[]=[];
  for(const row of await r.group("puzzle5_grip",c.row.rowid)){
   const g=await r.take(row,3),id=g.text(),gripKind=g.optionalText();g.done();
   const b=await r.take((await r.one("puzzle5_grip_board",row.rowid))!,2),board:m.Puzzle5dGrip2d={angle:b.f64(),gripKind:b.optionalText(),radius:b.optionalFloat()};b.done();
   const w=await r.take((await r.one("puzzle5_grip_world",row.rowid))!,2),world:m.Puzzle5dGrip3d={position:w.xyz(),direction:w.optionalVector3(),radius:w.optionalFloat(),label:w.optionalText()};w.done();grips.push({id,gripKind,"2d":board,"3d":world});
  }
  parts.push({id,partKind,anchor,"2d":board,"3d":world,grips});
 }
 for(const row of await r.group("puzzle5_fastener",doc.rowid)){const c=await r.take(row,3);fasteners.push({id:c.text(),source:c.text(),target:c.text(),fastenerKind:c.optionalText(),gap:c.f64(),shift:c.f64(),rise:c.f64(),rotation:c.f64(),turn:c.f64(),tilt:c.f64(),x:c.f64(),y:c.f64()});c.done()}
 for(const row of await r.group("puzzle5_target_volume",doc.rowid)){const c=await r.take(row,3);targetVolumes.push({id:c.text(),origin:c.xyz(),orientation:c.orientation(),hidden:c.boolean(),locked:c.boolean(),scale:await readScale("puzzle5_target_scale",row.rowid)});c.done()}
 await r.finish();return{schema,domain,label,meta,kindCatalogs,kindCatalogsExtra,kindCompatibility,parts,fasteners,targetVolumes};
}
