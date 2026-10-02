/** 🖐️ Explicit complete Block5d entity projection and owned restoration. */
import * as m from "../🟦️.ts";
import {ArtifactSqliteProjection,artifactSqliteTables,artifactSqliteCheckpoint,artifactSqliteOrderedRowsControlled,type ArtifactSqliteOptions} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🟦️.ts";
import type {SqliteDatabase,SqliteRow,SqliteValue} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {parseBinary64,binary64Value,type Binary64} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {NativeDecodeControl} from "../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🛬️decode/🟦️.ts";
import {BLOCK5D_SQLITE_SCHEMA} from "./🗄️schema/🟦️.ts";
type Cells=readonly SqliteValue[];
const fail=(why:string):never=>{throw new Error("Block5d SQLite "+why)};
const text=(v:string):string=>typeof v==="string"?v:fail("TEXT required");
const opt=(v:string|null):SqliteValue=>v===null?null:text(v);
const boolean=(v:boolean):bigint=>typeof v==="boolean"?(v?1n:0n):fail("Boolean required");
function floating(v:Binary64):Cells{const bits=parseBinary64(v).bits,n=binary64Value(v),kind=Number.isNaN(n)?"nan":n===Infinity?"positiveInfinity":n===-Infinity?"negativeInfinity":"finite";return[Number.isNaN(n)?null:n,BigInt.asIntN(64,bits),kind]}
const optionalFloat=(v:Binary64|null):Cells=>v===null?[null,null,null]:floating(v);
function xyz(v:readonly Binary64[]):Cells{if(v.length!==3)fail("vector3 width");return[...floating(v[0]!),...floating(v[1]!),...floating(v[2]!)]}
const optionalXyz=(v:readonly Binary64[]|null):Cells=>v===null?[null,null,null,null,null,null,null,null,null]:xyz(v);
function quaternion(v:readonly Binary64[]|null):Cells{if(v===null)return[null,null,null,null,null,null,null,null,null,null,null,null];if(v.length!==4)fail("quaternion width");return[...floating(v[0]!),...floating(v[1]!),...floating(v[2]!),...floating(v[3]!)]}
async function forecast(v:m.Block5dSnapshot,o:ArtifactSqliteOptions):Promise<number>{
 let count=7+v.representations.length+v.gripKinds.length+v.grips.length+v.compatibility.length+v.attributes.length+v.authors.length;
 await artifactSqliteCheckpoint(o,"projectSnapshot",0,v.representations.length);
 if(!Number.isSafeInteger(count)||count>(o.maxRows??1000000))fail("row limit");
 for(let i=0;i<v.representations.length;i++){const a=v.representations[i]!;count+=a.tags.length+a.attributes.length;if(!Number.isSafeInteger(count)||count>(o.maxRows??1000000))fail("row limit");if((i+1)%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",i+1,v.representations.length)}
 await artifactSqliteCheckpoint(o,"projectSnapshot",v.representations.length,v.representations.length);return count;
}
/** 📤️ Admit the complete borrowed row workload before owning literal relational rows. */
export async function block5dSnapshotToSqliteDatabase(v:m.Block5dSnapshot,o:ArtifactSqliteOptions={}):Promise<SqliteDatabase>{
 const p=await ArtifactSqliteProjection.create(BLOCK5D_SQLITE_SCHEMA,o),total=await forecast(v,o);p.checkRowsAdditional(total);let completed=0;
 const insert=async(name:string,cells:Cells):Promise<bigint>=>{const id=await p.insert(name,cells);if(++completed%256===0)await artifactSqliteCheckpoint(o,"projectSnapshot",completed,total);return id};
 const doc=await insert("block5_document",[text(v.schema)]),k=v.partKind;
 await insert("block5_kind",[doc,text(k.id),text(k.name),text(k.label),opt(k.variant),text(k.description),opt(k.icon),opt(k.unit)]);
 const two=v.part2d,three=v.part3d;
 await insert("block5_part2d",[doc,opt(two.shape),...optionalFloat(two.radius),...optionalFloat(two.width),...optionalFloat(two.height),opt(two.color),opt(two.iconKind)]);
 await insert("block5_part3d",[doc,...quaternion(three.orientation),...optionalXyz(three.scale)]);
 for(let i=0;i<v.representations.length;i++){const a=v.representations[i]!,id=await insert("block5_representation",[doc,BigInt(i),text(a.id),text(a.name),opt(a.meshUrl),opt(a.lod),text(a.description)]);
  for(let j=0;j<a.tags.length;j++)await insert("block5_representation_tag",[id,BigInt(j),text(a.tags[j]!)]);
  for(let j=0;j<a.attributes.length;j++){const n=a.attributes[j]!;await insert("block5_representation_attribute",[id,BigInt(j),text(n.key),text(n.value),opt(n.definition)])}}
 for(let i=0;i<v.gripKinds.length;i++){const a=v.gripKinds[i]!;await insert("block5_grip_kind",[doc,BigInt(i),text(a.id),text(a.name),text(a.label),text(a.color),text(a.defaultRopeKind)])}
 for(let i=0;i<v.grips.length;i++){const a=v.grips[i]!;await insert("block5_grip",[doc,BigInt(i),text(a.id),text(a.gripKind),...floating(a.angle),...floating(a.radius2d),...xyz(a.position),...xyz(a.direction),...floating(a.radius3d)])}
 for(let i=0;i<v.compatibility.length;i++){const a=v.compatibility[i]!;await insert("block5_compatibility",[doc,BigInt(i),text(a.id),text(a.source),text(a.target),boolean(a.bidirectional)])}
 for(let i=0;i<v.attributes.length;i++){const a=v.attributes[i]!;await insert("block5_attribute",[doc,BigInt(i),text(a.key),text(a.value),opt(a.definition)])}
 for(let i=0;i<v.authors.length;i++){const a=v.authors[i]!;await insert("block5_author",[doc,BigInt(i),text(a.id),text(a.name),opt(a.email)])}
 await insert("block5_camera2d",[doc,...floating(v.camera2d.x),...floating(v.camera2d.y),...floating(v.camera2d.zoom)]);
 await insert("block5_camera3d",[doc,...xyz(v.camera3d.position),...xyz(v.camera3d.target),...floating(v.camera3d.zoom)]);
 await insert("block5_meta",[doc,text(v.meta.description)]);await artifactSqliteCheckpoint(o,"projectSnapshot",completed,total);return p.finish();
}
class Cursor{
 private index:number;
 constructor(readonly row:SqliteRow,start=1){this.index=start;if(row.values[0]!==row.rowid)fail("rowid alias differs")}
 next():SqliteValue{if(this.index>=this.row.values.length)fail("missing cell");return this.row.values[this.index++]!}
 text():string{const v=this.next();return typeof v==="string"?v:fail("TEXT required")}
 integer():bigint{const v=this.next();return typeof v==="bigint"?v:fail("INTEGER required")}
 opt():string|null{return this.row.values[this.index]===null?(this.index++,null):this.text()}
 boolean():boolean{const n=this.integer();if(n!==0n&&n!==1n)fail("Boolean range");return n===1n}
 expect(v:SqliteValue):void{if(this.next()!==v)fail("unexpected inactive value")}
 f64():Binary64{const query=this.next(),bits=this.integer(),kind=this.text(),v={bits:BigInt.asUintN(64,bits)},n=binary64Value(v),expected=Number.isNaN(n)?"nan":n===Infinity?"positiveInfinity":n===-Infinity?"negativeInfinity":"finite";if(kind!==expected)fail("IEEE class differs");if(Number.isNaN(n)){if(query!==null)fail("NaN query must be NULL")}else if(typeof query==="bigint"){if(!Number.isInteger(n)||BigInt(n)!==query)fail("IEEE integer query differs")}else if(typeof query!=="number"||query!==n)fail("IEEE query differs");return v}
 optionalFloat():Binary64|null{if(this.row.values[this.index+1]!==null)return this.f64();this.expect(null);this.expect(null);this.expect(null);return null}
 xyz():[Binary64,Binary64,Binary64]{return[this.f64(),this.f64(),this.f64()]}
 optionalXyz():[Binary64,Binary64,Binary64]|null{const x=this.optionalFloat(),y=this.optionalFloat(),z=this.optionalFloat();if(x===null&&y===null&&z===null)return null;if(x===null||y===null||z===null)fail("partial vector absence");return[x!,y!,z!]}
 quaternion():[Binary64,Binary64,Binary64,Binary64]|null{const x=this.optionalFloat(),y=this.optionalFloat(),z=this.optionalFloat(),w=this.optionalFloat();if(x===null&&y===null&&z===null&&w===null)return null;if(x===null||y===null||z===null||w===null)fail("partial quaternion absence");return[x!,y!,z!,w!]}
 done():void{if(this.index!==this.row.values.length)fail("extra cell")}
}
class Reader{
 readonly used=new Set<SqliteRow>();readonly indexes=new Map<string,Map<bigint,SqliteRow[]>>();
 private constructor(readonly tables:Map<string,readonly SqliteRow[]>,readonly control:NativeDecodeControl,readonly options:ArtifactSqliteOptions){}
 static async create(db:SqliteDatabase,o:ArtifactSqliteOptions):Promise<Reader>{const rows=await artifactSqliteTables(db,BLOCK5D_SQLITE_SCHEMA,o),control=new NativeDecodeControl(o.maxValueBytes??268435456,p=>{o.onProgress?.({phase:"reconstructSnapshot",completed:p.completed,total:p.total});return!o.signal?.aborted},o.signal);await control.admitSlots(db.tables.length,64);await control.beginStage(rows.reduce((n,r)=>n+r.length,0));return new Reader(new Map(db.tables.map(t=>[t.name.toLowerCase(),t.rows])),control,o)}
 all(name:string):readonly SqliteRow[]{return this.tables.get(name)??fail("missing table "+name)}
 async take(row:SqliteRow,start=1):Promise<Cursor>{if(this.used.has(row))fail("duplicate ownership");await this.control.charge(384);await this.control.step();this.used.add(row);return new Cursor(row,start)}
 async group(name:string,parent:bigint,ordinal:number|null=2):Promise<SqliteRow[]>{let index=this.indexes.get(name);if(index===undefined){const source=this.all(name);await this.control.admitSlots(source.length,64);index=new Map();await this.control.scopedStage(async control=>{await control.beginStage(source.length);for(const row of source){const owner=row.values[1];if(typeof owner!=="bigint")fail("parent INTEGER required");let group=index!.get(owner as bigint);if(group===undefined){group=[];index!.set(owner as bigint,group)}group.push(row);await control.step()}});this.indexes.set(name,index)}const rows=index.get(parent)??[];await this.control.admitSlots(rows.length,16);return ordinal===null?rows:artifactSqliteOrderedRowsControlled(rows,ordinal,this.options)}
 async one(name:string,parent:bigint):Promise<SqliteRow>{const rows=await this.group(name,parent,null);if(rows.length!==1)fail("cardinality "+name);return rows[0]!}
 async finish():Promise<void>{await this.control.scopedStage(async control=>{let n=0;for(const rows of this.tables.values())n+=rows.length;await control.beginStage(n);for(const rows of this.tables.values())for(const row of rows){if(!this.used.has(row))fail("unowned entity");await control.step()}await control.checkpoint()})}
}
/** 📥️ Restore every named persisted field while refusing malformed relational ownership. */
export async function block5dSnapshotFromSqliteDatabase(db:SqliteDatabase,o:ArtifactSqliteOptions={}):Promise<m.Block5dSnapshot>{
 const r=await Reader.create(db,o),docs=r.all("block5_document");if(docs.length!==1)fail("one document required");const doc=docs[0]!,d=await r.take(doc),schema=d.text();d.done();
 const k=await r.take(await r.one("block5_kind",doc.rowid),2),partKind:m.BlockKindIdentity={id:k.text(),name:k.text(),label:k.text(),variant:k.opt(),description:k.text(),icon:k.opt(),unit:k.opt()};k.done();
 const two=await r.take(await r.one("block5_part2d",doc.rowid),2),part2d:m.Block5dPart2d={shape:two.opt(),radius:two.optionalFloat(),width:two.optionalFloat(),height:two.optionalFloat(),color:two.opt(),iconKind:two.opt()};two.done();
 const three=await r.take(await r.one("block5_part3d",doc.rowid),2),part3d:m.Block5dPart3d={orientation:three.quaternion(),scale:three.optionalXyz()};three.done();
 const representations:m.BlockRepresentation[]=[],gripKinds:m.Block5dGripKind[]=[],grips:m.Block5dGripTemplate[]=[],compatibility:m.BlockCompatibilityRule[]=[],attributes:m.BlockAttribute[]=[],authors:m.BlockAuthor[]=[];
 for(const row of await r.group("block5_representation",doc.rowid)){const c=await r.take(row,3),id=c.text(),name=c.text(),meshUrl=c.opt(),lod=c.opt(),description=c.text();c.done();const tags:string[]=[],attributes:m.BlockAttribute[]=[];
  for(const tag of await r.group("block5_representation_tag",row.rowid)){const c=await r.take(tag,3);tags.push(c.text());c.done()}
  for(const a of await r.group("block5_representation_attribute",row.rowid)){const c=await r.take(a,3);attributes.push({key:c.text(),value:c.text(),definition:c.opt()});c.done()}representations.push({id,name,meshUrl,lod,description,tags,attributes})}
 for(const row of await r.group("block5_grip_kind",doc.rowid)){const c=await r.take(row,3);gripKinds.push({id:c.text(),name:c.text(),label:c.text(),color:c.text(),defaultRopeKind:c.text()});c.done()}
 for(const row of await r.group("block5_grip",doc.rowid)){const c=await r.take(row,3);grips.push({id:c.text(),gripKind:c.text(),angle:c.f64(),radius2d:c.f64(),position:c.xyz(),direction:c.xyz(),radius3d:c.f64()});c.done()}
 for(const row of await r.group("block5_compatibility",doc.rowid)){const c=await r.take(row,3);compatibility.push({id:c.text(),source:c.text(),target:c.text(),bidirectional:c.boolean()});c.done()}
 for(const row of await r.group("block5_attribute",doc.rowid)){const c=await r.take(row,3);attributes.push({key:c.text(),value:c.text(),definition:c.opt()});c.done()}
 for(const row of await r.group("block5_author",doc.rowid)){const c=await r.take(row,3);authors.push({id:c.text(),name:c.text(),email:c.opt()});c.done()}
 const c2=await r.take(await r.one("block5_camera2d",doc.rowid),2),camera2d:m.BlockCamera2d={x:c2.f64(),y:c2.f64(),zoom:c2.f64()};c2.done();
 const c3=await r.take(await r.one("block5_camera3d",doc.rowid),2),camera3d:m.BlockCamera3d={position:c3.xyz(),target:c3.xyz(),zoom:c3.f64()};c3.done();
 const metaCursor=await r.take(await r.one("block5_meta",doc.rowid),2),meta:m.BlockMeta={description:metaCursor.text()};metaCursor.done();await r.finish();
 return{schema,partKind,part2d,part3d,representations,gripKinds,grips,compatibility,attributes,authors,camera2d,camera3d,meta};
}

