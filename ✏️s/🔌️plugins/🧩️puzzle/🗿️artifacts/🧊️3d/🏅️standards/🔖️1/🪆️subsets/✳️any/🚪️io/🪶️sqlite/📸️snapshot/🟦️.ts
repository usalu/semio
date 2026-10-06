/** 🧊️ Literal Puzzle3d relational projection and complete owned reconstruction. */
import * as m from "../../../🧬️schema/📸️snapshot/🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow,SqliteValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {parseBinary64,binary64Value,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {PUZZLE3D_SQLITE_SCHEMA} from "./🗄️schema/🟦️.ts";
type Cells=readonly SqliteValue[];
const fail=(why:string):never=>{throw new Error("Puzzle3d SQLite "+why)};
const text=(v:string):string=>typeof v==="string"?v:fail("TEXT required");
const optionalText=(v:string|null):SqliteValue=>v===null?null:text(v);
const boolean=(v:boolean):bigint=>typeof v==="boolean"?(v?1n:0n):fail("Boolean required");
const optionalBoolean=(v:boolean|null):SqliteValue=>v===null?null:boolean(v);
const optionalInteger=(v:number|null):SqliteValue=>v===null?null:Number.isInteger(v)&&v>=-2147483648&&v<=2147483647?BigInt(v):fail("signed32 required");
function floating(v:Binary64):Cells{const bits=parseBinary64(v).bits,n=binary64Value(v),kind=Number.isNaN(n)?"nan":n===Infinity?"positiveInfinity":n===-Infinity?"negativeInfinity":"finite";return[Number.isNaN(n)?null:n,BigInt.asIntN(64,bits),kind]}
const optionalFloat=(v:Binary64|null):Cells=>v===null?[null,null,null]:floating(v);
function xyz(v:m.Puzzle3dVector3):Cells{if(v.length!==3)fail("vector3 width");return[...floating(v[0]),...floating(v[1]),...floating(v[2])]}
const optionalXyz=(v:m.Puzzle3dVector3|null):Cells=>v===null?[null,null,null,null,null,null,null,null,null]:xyz(v);
function orientation(v:m.Puzzle3dVector4|null):Cells{if(v===null)return[null,null,null,null,null,null,null,null,null,null,null,null];if(v.length!==4)fail("quaternion width");return[...floating(v[0]),...floating(v[1]),...floating(v[2]),...floating(v[3])]}
function scale(v:m.Puzzle3dScale):Cells{return Array.isArray(v)?["vec3",null,null,null,...xyz(v)]:["uniform",...floating(v),null,null,null,null,null,null,null,null,null]}
async function forecast(v:m.Puzzle3dSnapshot,o:ArtifactSqliteOptions):Promise<number>{
 let rows=2,work=0;await artifactSqliteCheckpoint(o,"projectSnapshot",0,0);
 const add=async(n:number)=>{rows+=n;if(!Number.isSafeInteger(rows)||rows>(o.maxRows??1000000))fail("row limit");if(++work%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",work,0)};
 await add(v.meta.kindCompatibility.length+v.attractions.length+v.references.length*2);
 await artifactSqliteCheckpoint(o,"projectSnapshot",0,v.objects.length);
 for(let i=0;i<v.objects.length;i++){const object=v.objects[i]!;await add(1+(object.scale===null?0:1)+object.vortices.length);if((i+1)%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",i+1,v.objects.length)}
 await artifactSqliteCheckpoint(o,"projectSnapshot",0,v.targetVolumes.length);
 for(let i=0;i<v.targetVolumes.length;i++){const target=v.targetVolumes[i]!;await add(1+(target.scale===null?0:1));if((i+1)%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",i+1,v.targetVolumes.length)}
 const e=v.meta.kindCatalogs;if(e!==null){
  await add(1+e.cables.length+e.attractions.length);await artifactSqliteCheckpoint(o,"projectSnapshot",0,e.objects.length);
  for(let i=0;i<e.objects.length;i++){const k=e.objects[i]!;await add(1+k.baseKinds.length+k.vortices.length+k.attributes.length+k.authors.length);await artifactSqliteCheckpoint(o,"projectSnapshot",0,k.representations.length);for(let j=0;j<k.representations.length;j++){const representation=k.representations[j]!;await add(1+representation.tags.length);if((j+1)%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",j+1,k.representations.length)}if((i+1)%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",i+1,e.objects.length)}
  await artifactSqliteCheckpoint(o,"projectSnapshot",0,e.vortices.length);for(let i=0;i<e.vortices.length;i++){await add(1+e.vortices[i]!.compatibleWith.length);if((i+1)%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",i+1,e.vortices.length)}
 }
 await artifactSqliteCheckpoint(o,"projectSnapshot",work,work);return rows;
}
/** 📤️ Admit exact rows before allocating each literal owned relational entity. */
export async function puzzle3dSnapshotToSqliteDatabase(v:m.Puzzle3dSnapshot,o:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(PUZZLE3D_SQLITE_SCHEMA,o),count=await forecast(v,o);p.checkRowsAdditional(count);let written=0;
 const insert=async(name:string,cells:Cells):Promise<bigint>=>{const id=await p.insert(name,cells);if(++written%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",written,count);return id};
 const doc=await insert("puzzle3_document",[text(v.schema),text(v.domain)]),meta=await insert("puzzle3_meta",[doc]);
 const catalog=v.meta.kindCatalogs;
 if(catalog!==null){
  const parent=await insert("puzzle3_catalog",[meta]);
  for(let i=0;i<catalog.objects.length;i++){const k=catalog.objects[i]!,id=await insert("puzzle3_object_kind",[parent,BigInt(i),text(k.id),text(k.name),text(k.label),text(k.description),text(k.icon),text(k.image),text(k.unit),boolean(k.abstract)]);
   for(let j=0;j<k.baseKinds.length;j++)await insert("puzzle3_object_kind_base",[id,BigInt(j),text(k.baseKinds[j]!)]);
   for(let j=0;j<k.representations.length;j++){const r=k.representations[j]!,rid=await insert("puzzle3_representation",[id,BigInt(j),text(r.id),text(r.name),text(r.url),text(r.mime),optionalText(r.lod),text(r.description)]);for(let n=0;n<r.tags.length;n++)await insert("puzzle3_representation_tag",[rid,BigInt(n),text(r.tags[n]!)])}
   for(let j=0;j<k.vortices.length;j++){const t=k.vortices[j]!;await insert("puzzle3_vortex_template",[id,BigInt(j),text(t.id),text(t.name),text(t.label),text(t.description),text(t.icon),optionalText(t.vortexKind),...xyz(t.point),...xyz(t.direction),...optionalFloat(t.t),optionalBoolean(t.mandatory),...optionalFloat(t.radius)])}
   for(let j=0;j<k.attributes.length;j++){const a=k.attributes[j]!;await insert("puzzle3_attribute",[id,BigInt(j),text(a.id),text(a.key),text(a.value),optionalText(a.definition)])}
   for(let j=0;j<k.authors.length;j++){const a=k.authors[j]!;await insert("puzzle3_author",[id,BigInt(j),text(a.id),text(a.name),text(a.email),optionalText(a.role),optionalInteger(a.rank)])}
  }
  for(let i=0;i<catalog.vortices.length;i++){const k=catalog.vortices[i]!,id=await insert("puzzle3_vortex_kind",[parent,BigInt(i),text(k.id),optionalText(k.code),optionalText(k.label),optionalInteger(k.order),text(k.description),text(k.icon),text(k.color),text(k.defaultCableKind)]);for(let j=0;j<k.compatibleWith.length;j++)await insert("puzzle3_vortex_kind_compatible",[id,BigInt(j),text(k.compatibleWith[j]!)])}
  for(let i=0;i<catalog.cables.length;i++){const k=catalog.cables[i]!;await insert("puzzle3_cable_kind",[parent,BigInt(i),text(k.id),text(k.label),text(k.name),text(k.defaultAttractionKind)])}
  for(let i=0;i<catalog.attractions.length;i++){const k=catalog.attractions[i]!;await insert("puzzle3_attraction_kind",[parent,BigInt(i),text(k.id),text(k.label),text(k.name)])}
 }
 for(let i=0;i<v.meta.kindCompatibility.length;i++){const k=v.meta.kindCompatibility[i]!;await insert("puzzle3_compatibility",[meta,BigInt(i),text(k.source),text(k.target),boolean(k.bidirectional),boolean(k.important),m.parsePuzzle3dCompatSpecificity(k.specificity)])}
 for(let i=0;i<v.objects.length;i++){const a=v.objects[i]!,id=await insert("puzzle3_object",[doc,BigInt(i),text(a.id),optionalText(a.label),optionalText(a.objectKind),m.parsePuzzle3dObjectAnchor(a.anchor),...xyz(a.origin),...orientation(a.orientation),optionalText(a.meshUrl),boolean(a.hidden),boolean(a.locked)]);if(a.scale!==null)await insert("puzzle3_object_scale",[id,...scale(a.scale)]);for(let j=0;j<a.vortices.length;j++){const v=a.vortices[j]!;await insert("puzzle3_vortex",[id,BigInt(j),text(v.id),optionalText(v.vortexKind),optionalText(v.label),...xyz(v.position),...optionalXyz(v.direction),...optionalFloat(v.radius),boolean(v.hidden),boolean(v.locked)])}}
 for(let i=0;i<v.attractions.length;i++){const a=v.attractions[i]!;await insert("puzzle3_attraction",[doc,BigInt(i),text(a.id),text(a.attracting),text(a.attracted),...floating(a.gap),...floating(a.shift),...floating(a.rise),...floating(a.rotation),...floating(a.turn),...floating(a.tilt),...floating(a.x),...floating(a.y)])}
 for(let i=0;i<v.targetVolumes.length;i++){const t=v.targetVolumes[i]!,id=await insert("puzzle3_target",[doc,BigInt(i),text(t.id),...xyz(t.origin),...orientation(t.orientation),boolean(t.hidden),boolean(t.locked)]);if(t.scale!==null)await insert("puzzle3_target_scale",[id,...scale(t.scale)])}
 for(let i=0;i<v.references.length;i++){const a=v.references[i]!,id=await insert("puzzle3_reference",[doc,BigInt(i),text(a.id),...xyz(a.origin),...floating(a.widthWorld),boolean(a.locked),boolean(a.hidden)]);await insert("puzzle3_reference_source",[id,text(a.source.url),optionalText(a.source.mediaKind)])}
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
 xyz():m.Puzzle3dVector3{return[this.f64(),this.f64(),this.f64()]}
 optionalVector3():m.Puzzle3dVector3|null{const a=this.optionalFloat(),b=this.optionalFloat(),c=this.optionalFloat();if(a===null&&b===null&&c===null)return null;if(a===null||b===null||c===null)fail("partial vector absence");return[a!,b!,c!]}
 orientation():m.Puzzle3dVector4|null{const w=this.optionalFloat(),x=this.optionalFloat(),y=this.optionalFloat(),z=this.optionalFloat();if(w===null&&x===null&&y===null&&z===null)return null;if(w===null||x===null||y===null||z===null)fail("partial quaternion absence");return[w!,x!,y!,z!]}
 symbol<T extends string>(members:readonly T[]):T{const v=this.text();return members.includes(v as T)?v as T:fail("unknown enum")}
 done():void{if(this.index!==this.row.values.length)fail("extra cell")}
}
class Reader{
 readonly used=new Set<SqliteRow>();readonly indexes=new Map<string,Map<bigint,SqliteRow[]>>();
 private constructor(readonly tables:Map<string,readonly SqliteRow[]>,readonly control:NativeDecodeControl,readonly options:ArtifactSqliteOptions){}
 static async create(db:SqliteDatabase,o:ArtifactSqliteOptions):Promise<Reader>{const rows=await artifactSqliteTables(db,PUZZLE3D_SQLITE_SCHEMA,o),control=new NativeDecodeControl(o.maxValueBytes??268435456,p=>{o.onProgress?.({phase:"reconstructSnapshot",completed:p.completed,total:p.total});return!o.signal?.aborted},o.signal);await control.admitSlots(db.tables.length,64);await control.beginStage(rows.reduce((n,r)=>n+r.length,0));return new Reader(new Map(db.tables.map(t=>[t.name.toLowerCase(),t.rows])),control,o)}
 all(name:string):readonly SqliteRow[]{return this.tables.get(name)??fail("missing table "+name)}
 async take(row:SqliteRow,start=1):Promise<Cursor>{if(this.used.has(row))fail("duplicate ownership");await this.control.charge(192);await this.control.step();this.used.add(row);return new Cursor(row,start)}
 async group(name:string,parent:bigint,ordinal:number|null=2):Promise<SqliteRow[]>{let index=this.indexes.get(name);if(index===undefined){const source=this.all(name);await this.control.admitSlots(source.length,48);index=new Map();await this.control.scopedStage(async control=>{await control.beginStage(source.length);for(const row of source){const owner=row.values[1];if(typeof owner!=="bigint")fail("parent INTEGER required");let group=index!.get(owner as bigint);if(group===undefined){group=[];index!.set(owner as bigint,group)}group.push(row);await control.step()}});this.indexes.set(name,index)}const rows=index.get(parent)??[];if(ordinal===null)return rows;await this.control.admitSlots(rows.length,8);return artifactSqliteOrderedRowsControlled(rows,ordinal,this.options)}
 async one(name:string,parent:bigint,required=true):Promise<SqliteRow|null>{const rows=await this.group(name,parent,null);if(rows.length>1||required&&rows.length!==1)fail("cardinality "+name);return rows[0]??null}
 async finish():Promise<void>{await this.control.scopedStage(async control=>{let n=0;for(const rows of this.tables.values())n+=rows.length;await control.beginStage(n);for(const rows of this.tables.values())for(const row of rows){if(!this.used.has(row))fail("unowned entity");await control.step()}await control.checkpoint()})}
}
/** 📥️ Restore complete literal domain rows, cardinalities and exact IEEE companions. */
export async function puzzle3dSnapshotFromSqliteDatabase(db:SqliteDatabase,o:ArtifactSqliteOptions={}):Promise<m.Puzzle3dSnapshot>{
 const r=await Reader.create(db,o),docs=r.all("puzzle3_document");if(docs.length!==1)fail("one document required");const doc=docs[0]!,d=await r.take(doc),schema=d.text(),domain=d.text();d.done();
 const metaRow=(await r.one("puzzle3_meta",doc.rowid))!,metaCursor=await r.take(metaRow,2);metaCursor.done();
 const catalogRow=await r.one("puzzle3_catalog",metaRow.rowid,false);let kindCatalogs:m.Puzzle3dKindCatalogs|null=null;
 const strings=async(name:string,parent:bigint):Promise<string[]>=>{const values:string[]=[];for(const row of await r.group(name,parent)){const c=await r.take(row,3);values.push(c.text());c.done()}return values};
 if(catalogRow!==null){
  const c=await r.take(catalogRow,2);c.done();const objects:m.Puzzle3dCatalogObjectKind[]=[],vortices:m.Puzzle3dCatalogVortexKind[]=[],cables:m.Puzzle3dCatalogCableKind[]=[],attractions:m.Puzzle3dCatalogAttractionKind[]=[];
  for(const row of await r.group("puzzle3_object_kind",catalogRow.rowid)){
   const c=await r.take(row,3),id=c.text(),name=c.text(),label=c.text(),description=c.text(),icon=c.text(),image=c.text(),unit=c.text(),abstract=c.boolean();c.done();
   const baseKinds=await strings("puzzle3_object_kind_base",row.rowid),representations:m.Puzzle3dRepresentation[]=[],templates:m.Puzzle3dCatalogVortexTemplate[]=[],attributes:m.Puzzle3dAttribute[]=[],authors:m.Puzzle3dAuthor[]=[];
   for(const value of await r.group("puzzle3_representation",row.rowid)){const c=await r.take(value,3),id=c.text(),name=c.text(),url=c.text(),mime=c.text(),lod=c.optionalText(),description=c.text();c.done();representations.push({id,name,url,mime,lod,description,tags:await strings("puzzle3_representation_tag",value.rowid)})}
   for(const value of await r.group("puzzle3_vortex_template",row.rowid)){const c=await r.take(value,3);templates.push({id:c.text(),name:c.text(),label:c.text(),description:c.text(),icon:c.text(),vortexKind:c.optionalText(),point:c.xyz(),direction:c.xyz(),t:c.optionalFloat(),mandatory:c.optionalBoolean(),radius:c.optionalFloat()});c.done()}
   for(const value of await r.group("puzzle3_attribute",row.rowid)){const c=await r.take(value,3);attributes.push({id:c.text(),key:c.text(),value:c.text(),definition:c.optionalText()});c.done()}
   for(const value of await r.group("puzzle3_author",row.rowid)){const c=await r.take(value,3);authors.push({id:c.text(),name:c.text(),email:c.text(),role:c.optionalText(),rank:c.optionalInteger()});c.done()}
   objects.push({id,name,label,description,icon,image,unit,abstract,baseKinds,representations,vortices:templates,attributes,authors});
  }
  for(const row of await r.group("puzzle3_vortex_kind",catalogRow.rowid)){const c=await r.take(row,3),id=c.text(),code=c.optionalText(),label=c.optionalText(),order=c.optionalInteger(),description=c.text(),icon=c.text(),color=c.text(),defaultCableKind=c.text();c.done();vortices.push({id,code,label,order,description,icon,color,defaultCableKind,compatibleWith:await strings("puzzle3_vortex_kind_compatible",row.rowid)})}
  for(const row of await r.group("puzzle3_cable_kind",catalogRow.rowid)){const c=await r.take(row,3);cables.push({id:c.text(),label:c.text(),name:c.text(),defaultAttractionKind:c.text()});c.done()}
  for(const row of await r.group("puzzle3_attraction_kind",catalogRow.rowid)){const c=await r.take(row,3);attractions.push({id:c.text(),label:c.text(),name:c.text()});c.done()}
  kindCatalogs={objects,vortices,cables,attractions};
 }
 const kindCompatibility:m.Puzzle3dKindCompatibility[]=[],objects:m.Puzzle3dObject[]=[],attractions:m.Puzzle3dAttraction[]=[],targetVolumes:m.Puzzle3dTargetVolume[]=[],references:m.Puzzle3dReference[]=[];
 for(const row of await r.group("puzzle3_compatibility",metaRow.rowid)){const c=await r.take(row,3);kindCompatibility.push({source:c.text(),target:c.text(),bidirectional:c.boolean(),important:c.boolean(),specificity:c.symbol(["general","object","attraction","vortex","cable"])});c.done()}
 const readScale=async(name:string,parent:bigint):Promise<m.Puzzle3dScale|null>=>{const row=await r.one(name,parent,false);if(row===null)return null;const c=await r.take(row,2),kind=c.symbol(["uniform","vec3"]),uniform=c.optionalFloat(),axes=c.optionalVector3();c.done();if(kind==="uniform"){if(uniform===null||axes!==null)fail("uniform scale fields");return uniform!}if(uniform!==null||axes===null)fail("vector scale fields");return axes!};
 for(const row of await r.group("puzzle3_object",doc.rowid)){
  const c=await r.take(row,3),id=c.text(),label=c.optionalText(),objectKind=c.optionalText(),anchor=c.symbol(["fixed","derived"]),origin=c.xyz(),orientation=c.orientation(),meshUrl=c.optionalText(),hidden=c.boolean(),locked=c.boolean();c.done();const vortices:m.Puzzle3dVortex[]=[];
  for(const value of await r.group("puzzle3_vortex",row.rowid)){const c=await r.take(value,3);vortices.push({id:c.text(),vortexKind:c.optionalText(),label:c.optionalText(),position:c.xyz(),direction:c.optionalVector3(),radius:c.optionalFloat(),hidden:c.boolean(),locked:c.boolean()});c.done()}
  objects.push({id,label,objectKind,anchor,origin,orientation,meshUrl,hidden,locked,vortices,scale:await readScale("puzzle3_object_scale",row.rowid)});
 }
 for(const row of await r.group("puzzle3_attraction",doc.rowid)){const c=await r.take(row,3);attractions.push({id:c.text(),attracting:c.text(),attracted:c.text(),gap:c.f64(),shift:c.f64(),rise:c.f64(),rotation:c.f64(),turn:c.f64(),tilt:c.f64(),x:c.f64(),y:c.f64()});c.done()}
 for(const row of await r.group("puzzle3_target",doc.rowid)){const c=await r.take(row,3);targetVolumes.push({id:c.text(),origin:c.xyz(),orientation:c.orientation(),hidden:c.boolean(),locked:c.boolean(),scale:await readScale("puzzle3_target_scale",row.rowid)});c.done()}
 for(const row of await r.group("puzzle3_reference",doc.rowid)){const c=await r.take(row,3),id=c.text(),origin=c.xyz(),widthWorld=c.f64(),locked=c.boolean(),hidden=c.boolean();c.done();const sourceCursor=await r.take((await r.one("puzzle3_reference_source",row.rowid))!,2),source={url:sourceCursor.text(),mediaKind:sourceCursor.optionalText()};sourceCursor.done();references.push({id,origin,widthWorld,locked,hidden,source})}
 await r.finish();return{schema,domain,meta:{kindCatalogs,kindCompatibility},objects,attractions,targetVolumes,references};
}
